mod install;

use std::io::Write as _;
use std::path::PathBuf;

use anyhow::{Context as _, Result};
use gpui::{Context, Entity, SharedString, Task};
use i18n::t;
use serde::Deserialize;
use sha2::{Digest as _, Sha256};

use crate::{AppSettings, Io, join};
use install::Install;

const RELEASES: &str = "https://api.github.com/repos/RoxyZ7/SonoraX/releases";
const RELEASES_PAGE: &str = "https://github.com/RoxyZ7/SonoraX/releases";
const SUMS: &str = "SHA256SUMS";
const CHECK_EVERY: std::time::Duration = std::time::Duration::from_secs(6 * 60 * 60);
/// What the nightly workflow puts before the version label in the nightly release's title.
/// The version this build shows and reports. The nightly workflow sets `SONORA_VERSION` at
/// compile time to a label such as `0.44.2-nightly.20261026`, and every other build falls back
/// to the Cargo version.
pub const VERSION: &str = match option_env!("SONORA_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};
const AGENT: &str = concat!(
    "SonoraX/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/RoxyZ7/SonoraX)"
);

/// The builds Sonora updates to. Stable follows tagged releases, and Nightly follows the build
/// of main the nightly workflow publishes every night.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    Stable,
    Nightly,
}

impl Channel {
    pub const ALL: [Self; 2] = [Self::Stable, Self::Nightly];

    /// The channel the running build came from.
    pub fn running() -> Self {
        match VERSION.contains("-nightly") {
            true => Self::Nightly,
            false => Self::Stable,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Nightly => "nightly",
        }
    }

    /// Reads a stored id. An empty or unknown one means the channel of the running build.
    pub fn from_id(id: &str) -> Self {
        match id {
            "stable" => Self::Stable,
            "nightly" => Self::Nightly,
            _ => Self::running(),
        }
    }

    pub fn label(self) -> SharedString {
        match self {
            Self::Stable => t!("update-channel-stable"),
            Self::Nightly => t!("update-channel-nightly"),
        }
    }

    fn url(self) -> String {
        format!("{RELEASES}/latest")
    }
}

/// Whether this copy of Sonora installs its own updates, which is when a channel can be chosen.
/// A copy a package manager installed, or a portable one, does not.
pub fn updates_itself() -> bool {
    Install::current().updates_itself()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    pub page: String,
    /// The asset this install updates from, when the release carries one.
    package: Option<Asset>,
    sums: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateState {
    Quiet,
    Offered(Release),
    Fetching,
    Failed,
}

pub struct Updates {
    state: UpdateState,
    http: reqwest::Client,
    io: Io,
    task: Option<Task<()>>,
    /// SonoraX currently follows stable releases.
    channel: Channel,
    first_check: bool,
    _poll: Task<()>,
}

impl Updates {
    pub fn new(_settings: Entity<AppSettings>, io: Io, cx: &mut Context<Self>) -> Self {
        let channel = Channel::Stable;
        let mut updates = Self {
            state: UpdateState::Quiet,
            http: reqwest::Client::new(),
            io,
            task: None,
            channel,
            first_check: true,
            _poll: cx.spawn(async move |this, cx| {
                loop {
                    cx.background_executor().timer(CHECK_EVERY).await;
                    if this.update(cx, |this, cx| this.look(cx)).is_err() {
                        break;
                    }
                }
            }),
        };
        updates.look(cx);
        updates
    }

    pub fn state(&self) -> &UpdateState {
        &self.state
    }

    pub fn offered(&self) -> Option<&Release> {
        match &self.state {
            UpdateState::Offered(release) => Some(release),
            _ => None,
        }
    }

    pub fn installable(&self) -> bool {
        updates_itself()
            && self
                .offered()
                .is_some_and(|release| release.package.is_some() && release.sums.is_some())
    }

    pub fn releases_page(&self) -> &'static str {
        RELEASES_PAGE
    }

    pub fn dismiss(&mut self, cx: &mut Context<Self>) {
        self.task = None;
        self.state = UpdateState::Quiet;
        cx.notify();
    }

    /// Downloads the offered build, checks it against the release's checksums and puts it in
    /// place, then quits so the new build can start. A failure leaves the running build as it
    /// was and shows the failure instead of the offer.
    pub fn install(&mut self, cx: &mut Context<Self>) {
        let Some(release) = self.offered().cloned() else {
            return;
        };
        let (Some(package), Some(sums)) = (release.package, release.sums) else {
            return;
        };
        self.state = UpdateState::Fetching;
        cx.notify();

        let http = self.http.clone();
        let io = self.io.clone();
        let version = release.version;
        self.task = Some(cx.spawn(async move |this, cx| {
            let fetched =
                join(io.spawn(async move { download(&http, &package, &sums, &version).await }))
                    .await;
            let applied = match fetched {
                Ok(path) => join(io.spawn_blocking(move || Install::current().apply(&path))).await,
                Err(error) => Err(error),
            };
            match applied {
                Ok(()) => {
                    cx.update(|cx| cx.quit());
                }
                Err(error) => {
                    log::warn!("updates: cannot install the new version: {error:#}");
                    this.update(cx, |this, cx| {
                        this.task = None;
                        this.state = UpdateState::Failed;
                        cx.notify();
                    })
                    .ok();
                }
            }
        }));
    }

    /// Asks GitHub for the newest stable SonoraX build.
    fn look(&mut self, cx: &mut Context<Self>) {
        let channel = match updates_itself() {
            true => self.channel,
            false if Channel::running() == Channel::Stable => Channel::Stable,
            false => return,
        };
        let startup = std::mem::replace(&mut self.first_check, false);
        let http = self.http.clone();
        let io = self.io.clone();
        self.task = Some(cx.spawn(async move |this, cx| {
            let found = join(io.spawn(async move { latest(&http, channel).await })).await;
            this.update(cx, |this, cx| {
                this.task = None;
                match found {
                    Ok(Some(release)) => {
                        if startup
                            && updates_itself()
                            && release.package.is_some()
                            && release.sums.is_some()
                        {
                            this.state = UpdateState::Offered(release);
                            this.install(cx);
                        } else {
                            this.state = UpdateState::Offered(release);
                            cx.notify();
                        }
                    }
                    Ok(None) => {}
                    Err(error) => log::warn!("updates: cannot ask github: {error:#}"),
                }
            })
            .ok();
        }));
    }
}

#[derive(Deserialize)]
struct Published {
    tag_name: String,
    html_url: String,
    #[serde(default)]
    assets: Vec<Asset>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

/// The newest build on `channel`, or `None` when the running build should stay.
async fn latest(http: &reqwest::Client, channel: Channel) -> Result<Option<Release>> {
    let published: Published = http
        .get(channel.url())
        .header("User-Agent", AGENT)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .context("cannot reach github")?
        .error_for_status()
        .context("github refused the release request")?
        .json()
        .await
        .context("cannot read the github release")?;

    let version = published.tag_name.trim_start_matches('v').to_owned();
    if !wanted(channel, &version) {
        return Ok(None);
    }
    let install = Install::current();
    let package = published
        .assets
        .iter()
        .find(|asset| install.wants(&asset.name))
        .cloned();
    let sums = published
        .assets
        .iter()
        .find(|asset| asset.name == SUMS)
        .map(|asset| asset.browser_download_url.clone());

    Ok(Some(Release {
        version,
        page: published.html_url,
        package,
        sums,
    }))
}

/// Streams the asset into the temporary folder while hashing it, and keeps it only when the
/// hash matches the one the release lists.
async fn download(
    http: &reqwest::Client,
    package: &Asset,
    sums: &str,
    version: &str,
) -> Result<PathBuf> {
    let listed = http
        .get(sums)
        .header("User-Agent", AGENT)
        .send()
        .await
        .context("cannot reach the checksums")?
        .error_for_status()
        .context("the checksums are missing")?
        .text()
        .await
        .context("cannot read the checksums")?;
    let expected = listed
        .lines()
        .filter_map(|line| line.split_once(char::is_whitespace))
        .find(|(_, name)| name.trim() == package.name)
        .map(|(sum, _)| sum.trim().to_ascii_lowercase())
        .context("the release lists no checksum for the update")?;

    let mut response = http
        .get(&package.browser_download_url)
        .header("User-Agent", AGENT)
        .send()
        .await
        .context("cannot reach the update")?
        .error_for_status()
        .context("the update is missing")?;

    let path = std::env::temp_dir().join(format!("sonora-{version}-{}", package.name));
    let mut file = std::fs::File::create(&path).context("cannot keep the update")?;
    let mut hasher = Sha256::new();
    while let Some(chunk) = response.chunk().await.context("cannot read the update")? {
        hasher.update(&chunk);
        file.write_all(&chunk).context("cannot keep the update")?;
    }
    drop(file);

    if format!("{:x}", hasher.finalize()) != expected {
        std::fs::remove_file(&path).ok();
        anyhow::bail!("the update does not match its checksum");
    }
    Ok(path)
}

/// Whether `offered` on `channel` should replace the running build. Within the running build's
/// own channel only a newer build counts. From the other channel any build does, since choosing
/// that channel asks to move to it.
fn wanted(channel: Channel, offered: &str) -> bool {
    if channel != Channel::running() {
        return true;
    }
    match (ordered(offered), ordered(VERSION)) {
        (Some(offered), Some(running)) => offered > running,
        _ => false,
    }
}

/// Splits a version into its release numbers and, for a nightly label such as
/// `0.44.2-nightly.20261026`, the date it was built. A stable version has 0 for the date.
fn ordered(version: &str) -> Option<(u64, u64, u64, u64)> {
    let (core, rest) = version
        .trim()
        .split_once(['-', '+'])
        .unwrap_or((version.trim(), ""));
    let mut parts = core.split('.').map(|part| part.parse::<u64>().ok());
    let date = rest
        .strip_prefix("nightly.")
        .and_then(|date| date.parse().ok())
        .unwrap_or(0);
    Some((parts.next()??, parts.next()??, parts.next()??, date))
}
