use std::io::{BufWriter, Cursor, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use rusqlite::OptionalExtension as _;
use serde::Deserialize;
use sha2::{Digest as _, Sha256};
use storage::Database;

pub const SOURCE: &str = "Auto sync";
const MODEL: &str = "ggml-small-q5_1.bin";
const RATE: u32 = 16_000;
const TIME_LIMIT: Duration = Duration::from_secs(12 * 60);
const ENGINE_VERSION: &str = "v1.9.2";
const ENGINE_EXE: &str = if cfg!(windows) {
    "whisper-cli.exe"
} else {
    "whisper-cli"
};
const MODEL_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/98aa99a0a9db05ae2342309f5096248665f7cba3/ggml-small-q5_1.bin";
const MODEL_SHA256: &str = "ae85e4a935d7a567bd102fe55afc16bb595bdb618e11b2fc7591bc08120411bb";
const MODEL_MAX_BYTES: u64 = 250 * 1024 * 1024;
const RELEASES: &str = "https://api.github.com/repos/RoxyZ7/SonoraX/releases";

#[derive(Deserialize)]
struct Published {
    assets: Vec<ReleaseAsset>,
}

#[derive(Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct Transcript {
    transcription: Vec<Segment>,
}

#[derive(Deserialize)]
struct Segment {
    tokens: Vec<Token>,
}

#[derive(Deserialize)]
struct Token {
    text: String,
    offsets: Option<Offsets>,
}

#[derive(Deserialize)]
struct Offsets {
    from: i64,
    to: i64,
}

/// Finds the platform engine, whether it was shipped beside the app or downloaded once to cache.
pub(crate) fn engine() -> Option<PathBuf> {
    let installed = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|parent| parent.join("lyrics-engine")));
    let cached =
        dirs::cache_dir().map(|cache| cache.join("SonoraX/lyrics-engine").join(ENGINE_VERSION));
    installed
        .into_iter()
        .chain(cached)
        .map(|dir| dir.join(ENGINE_EXE))
        .find(|exe| exe.is_file() && exe.parent().is_some_and(|dir| dir.join(MODEL).is_file()))
}

pub(crate) fn supported() -> bool {
    engine_target().is_some() || engine().is_some()
}

/// Gets the platform engine and pinned model once, keeping the large model out of app updates.
pub(crate) async fn ensure_engine() -> Result<PathBuf> {
    if let Some(engine) = engine() {
        return Ok(engine);
    }
    let target = engine_target().context("automatic lyrics alignment is unsupported here")?;
    let root = dirs::cache_dir()
        .context("cannot locate the application cache directory")?
        .join("SonoraX/lyrics-engine");
    let destination = root.join(ENGINE_VERSION);
    tokio::fs::create_dir_all(&root).await?;
    let stage = tempfile::Builder::new()
        .prefix("lyrics-engine-")
        .tempdir_in(&root)?;
    let engine_dir = stage.path().join("engine");
    tokio::fs::create_dir(&engine_dir).await?;

    let client = reqwest::Client::builder()
        .user_agent(concat!("SonoraX/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(30 * 60))
        .build()?;
    let tag = match crate::VERSION.contains("-nightly") {
        true => "nightly".to_owned(),
        false if crate::VERSION == env!("CARGO_PKG_VERSION") => "latest".to_owned(),
        false => format!("v{}", crate::VERSION),
    };
    let release: Published = client
        .get(format!("{RELEASES}/{tag}"))
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let archive_name = format!("SonoraX-lyrics-engine-{target}.tar.gz");
    let archive = release
        .assets
        .iter()
        .find(|asset| asset.name == archive_name)
        .context("this release has no lyrics engine for the current platform")?;
    let checksums_url =
        format!("https://github.com/RoxyZ7/SonoraX/releases/download/{tag}/SHA256SUMS");
    let checksums = client
        .get(checksums_url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let expected = checksums
        .lines()
        .filter_map(|line| line.split_once(char::is_whitespace))
        .find(|(_, name)| name.trim() == archive_name)
        .map(|(hash, _)| hash.trim().to_ascii_lowercase())
        .context("the release has no checksum for its lyrics engine")?;
    let archive_bytes = client
        .get(&archive.browser_download_url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    anyhow::ensure!(
        format!("{:x}", Sha256::digest(&archive_bytes)) == expected,
        "lyrics engine archive failed its SHA-256 check"
    );
    tokio::task::spawn_blocking({
        let engine_dir = engine_dir.clone();
        move || extract_engine(&archive_bytes, &engine_dir)
    })
    .await??;

    let model_path = engine_dir.join(MODEL);
    let mut model_file = tokio::fs::File::create(&model_path).await?;
    let mut response = client.get(MODEL_URL).send().await?.error_for_status()?;
    let mut digest = Sha256::new();
    let mut size = 0u64;
    while let Some(chunk) = response.chunk().await? {
        size = size
            .checked_add(chunk.len() as u64)
            .context("lyrics model size overflow")?;
        anyhow::ensure!(
            size <= MODEL_MAX_BYTES,
            "lyrics model download is too large"
        );
        digest.update(&chunk);
        tokio::io::AsyncWriteExt::write_all(&mut model_file, &chunk).await?;
    }
    tokio::io::AsyncWriteExt::flush(&mut model_file).await?;
    anyhow::ensure!(
        format!("{:x}", digest.finalize()) == MODEL_SHA256,
        "lyrics model failed its SHA-256 check"
    );
    drop(model_file);

    if let Some(engine) = engine() {
        return Ok(engine);
    }
    if destination.exists() {
        std::fs::remove_dir_all(&destination)
            .context("cannot replace an incomplete lyrics engine cache")?;
    }
    match std::fs::rename(&engine_dir, &destination) {
        Ok(()) => {}
        Err(error) if destination.is_dir() => {
            log::debug!("lyrics: another process installed the engine: {error}");
        }
        Err(error) => return Err(error).context("cannot install the lyrics engine cache"),
    }
    engine().context("lyrics engine installation did not produce the expected files")
}

fn engine_target() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => Some("x86_64-pc-windows-msvc"),
        ("windows", "aarch64") => Some("aarch64-pc-windows-msvc"),
        ("linux", "x86_64") => Some("x86_64-unknown-linux-gnu"),
        ("linux", "aarch64") => Some("aarch64-unknown-linux-gnu"),
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        _ => None,
    }
}

fn extract_engine(archive: &[u8], destination: &Path) -> Result<()> {
    let decoder = flate2::read::GzDecoder::new(Cursor::new(archive));
    let mut archive = tar::Archive::new(decoder);
    let mut found_executable = false;
    let mut found_license = false;
    for entry in archive.entries()? {
        let mut entry = entry?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let entry_path = entry.path()?;
        let Some(name) = entry_path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let destination_name = match name {
            name if name == ENGINE_EXE => {
                found_executable = true;
                ENGINE_EXE
            }
            "LICENSE.whisper.cpp" => {
                found_license = true;
                "LICENSE.whisper.cpp"
            }
            _ => continue,
        };
        let mut file = std::fs::File::create(destination.join(destination_name))?;
        std::io::copy(&mut entry, &mut file)?;
        if destination_name == ENGINE_EXE {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                file.set_permissions(std::fs::Permissions::from_mode(0o755))?;
            }
        }
    }
    anyhow::ensure!(
        found_executable,
        "lyrics archive is missing the engine executable"
    );
    anyhow::ensure!(
        found_license,
        "lyrics archive is missing the engine license"
    );
    Ok(())
}

/// Invalidates a timing if the lyrics text or the alignment implementation changes.
pub(crate) fn fingerprint(text: &str) -> String {
    format!("v1:{:x}", Sha256::digest(text.as_bytes()))
}

pub(crate) fn cached(db: &Database, key: &str, version: &str) -> Result<Option<String>> {
    Ok(db
        .open()?
        .query_row(
            "SELECT lrc FROM lyrics_generated WHERE track_key = ?1 AND fingerprint = ?2",
            (key, version),
            |row| row.get(0),
        )
        .optional()?)
}

pub(crate) fn save(db: &Database, key: &str, version: &str, lrc: &str) -> Result<()> {
    db.open()?.execute(
        "INSERT INTO lyrics_generated (track_key, fingerprint, lrc) VALUES (?1, ?2, ?3)
         ON CONFLICT(track_key) DO UPDATE SET fingerprint = excluded.fingerprint, lrc = excluded.lrc",
        (key, version, lrc),
    )?;
    Ok(())
}

/// Runs the recognizer on transient audio. Its words are anchors for the supplied lyrics only.
pub(crate) fn generate(engine: &Path, pcm: &[i16], text: &str) -> Result<Option<String>> {
    let scratch = tempfile::tempdir()?;
    let input = scratch.path().join("input.wav");
    let output = scratch.path().join("timing");
    wav(&input, pcm)?;
    let thai = text
        .chars()
        .filter(|&c| matches!(c, '\u{0e01}'..='\u{0e4e}'))
        .count()
        >= 8;
    let mut command = Command::new(engine);
    command
        .current_dir(engine.parent().context("missing lyrics engine folder")?)
        .args([
            "-m",
            MODEL,
            "-t",
            "2",
            "-ng",
            "-l",
            if thai { "th" } else { "auto" },
        ])
        .args(["-ojf", "-ml", "32", "-f"])
        .arg(input)
        .arg("-of")
        .arg(&output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt as _;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        const BELOW_NORMAL_PRIORITY: u32 = 0x0000_4000;
        command.creation_flags(CREATE_NO_WINDOW | BELOW_NORMAL_PRIORITY);
    }
    let mut child = command.spawn().context("cannot launch lyrics recognizer")?;
    let deadline = Instant::now() + TIME_LIMIT;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                anyhow::ensure!(status.success(), "lyrics recognizer exited with {status}");
                break;
            }
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(250)),
            result => {
                child.kill().ok();
                child.wait().ok();
                result.context("cannot wait for lyrics recognizer")?;
                anyhow::bail!("lyrics recognizer timed out");
            }
        }
    }
    let transcript: Transcript =
        serde_json::from_reader(std::fs::File::open(output.with_extension("json"))?)?;
    Ok(align(transcript, text))
}

/// Writes the recognizer's 16-bit mono WAV input.
fn wav(path: &Path, pcm: &[i16]) -> Result<()> {
    let size = u32::try_from(pcm.len())? * 2;
    let mut out = BufWriter::new(std::fs::File::create(path)?);
    out.write_all(b"RIFF")?;
    out.write_all(&(size + 36).to_le_bytes())?;
    out.write_all(b"WAVEfmt ")?;
    out.write_all(&16u32.to_le_bytes())?;
    out.write_all(&1u16.to_le_bytes())?;
    out.write_all(&1u16.to_le_bytes())?;
    out.write_all(&RATE.to_le_bytes())?;
    out.write_all(&(RATE * 2).to_le_bytes())?;
    out.write_all(&2u16.to_le_bytes())?;
    out.write_all(&16u16.to_le_bytes())?;
    out.write_all(b"data")?;
    out.write_all(&size.to_le_bytes())?;
    for &sample in pcm {
        out.write_all(&sample.to_le_bytes())?;
    }
    out.flush()?;
    Ok(())
}

fn letters(text: &str) -> impl Iterator<Item = char> + '_ {
    text.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
}

/// Matches each lyric against the next stretch of recognized vocals, in song order.
fn align(transcript: Transcript, text: &str) -> Option<String> {
    let mut chars = Vec::new();
    let mut time = Vec::new();
    for token in transcript
        .transcription
        .into_iter()
        .flat_map(|segment| segment.tokens)
    {
        let Some(offsets) = token
            .offsets
            .filter(|span| span.from >= 0 && span.to >= span.from)
        else {
            continue;
        };
        if token.text.starts_with("[_") {
            continue;
        }
        for letter in letters(&token.text) {
            chars.push(letter);
            time.push(offsets.from);
        }
    }
    let mut cursor = 0;
    let mut last = -1;
    let mut eligible = 0;
    let mut consecutive_misses = 0;
    let mut matched = Vec::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let target: Vec<_> = letters(line).collect();
        if target.len() < 2 {
            continue;
        }
        eligible += 1;
        let end = (cursor + target.len() * 2 + 120).min(chars.len());
        let remaining = &chars[cursor..end];
        let position = match_line(&target, remaining);
        match position {
            Some((start, end)) if time[cursor + start] > last => {
                last = time[cursor + start];
                cursor += end;
                consecutive_misses = 0;
                matched.push(format!(
                    "[{:02}:{:02}.{:02}]{line}",
                    last / 60_000,
                    last / 1_000 % 60,
                    last / 10 % 100
                ));
            }
            _ => {
                consecutive_misses += 1;
                if consecutive_misses > 2 {
                    return None;
                }
            }
        }
    }
    (matched.len() >= 4 && matched.len() * 100 >= eligible * 85).then(|| matched.join("\n"))
}

/// Finds the lowest edit cost for the line in a bounded window, preferring an earlier match.
fn match_line(needle: &[char], haystack: &[char]) -> Option<(usize, usize)> {
    if let Some(start) = haystack
        .windows(needle.len())
        .position(|span| span == needle)
    {
        return Some((start, start + needle.len()));
    }
    if needle.len() < 6 {
        return None;
    }
    let mut previous: Vec<_> = (0..=haystack.len()).map(|column| (0, column)).collect();
    let mut row = previous.clone();
    for (index, letter) in needle.iter().enumerate() {
        row[0] = (index + 1, 0);
        for (column, other) in haystack.iter().enumerate() {
            let (sub, start) = previous[column];
            let (drop, drop_start) = previous[column + 1];
            let (add, add_start) = row[column];
            row[column + 1] = [
                (sub + usize::from(letter != other), start),
                (drop + 1, drop_start),
                (add + 1, add_start),
            ]
            .into_iter()
            .min()
            .unwrap();
        }
        std::mem::swap(&mut previous, &mut row);
    }
    previous
        .into_iter()
        .enumerate()
        .filter(|(end, (cost, start))| end > start && *cost * 4 <= needle.len())
        .min_by_key(|(end, (cost, start))| (cost * 8 + start / 8, *end))
        .map(|(end, (_, start))| (start, end))
}
