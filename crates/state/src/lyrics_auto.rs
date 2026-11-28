use std::f32::consts::PI;
use std::io::{BufWriter, Cursor, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result};
use icu_segmenter::{WordSegmenter, options::WordBreakInvariantOptions};
use music::{LyricsLine, LyricsWord};
use rusqlite::OptionalExtension as _;
use serde::Deserialize;
use sha2::{Digest as _, Sha256};
use storage::Database;

pub const SOURCE: &str = "Auto sync";
const MODEL: &str = "ggml-small-q5_1.bin";
const RATE: u32 = 16_000;
const TIME_LIMIT: Duration = Duration::from_secs(12 * 60);
/// How far before/after a provider's line onset word timing may be matched in recognized audio.
const WORD_ONSET_LOOKBACK_MS: i64 = 950;
const WORD_ONSET_LOOKAHEAD_MS: i64 = 350;
/// Whisper DTW peaks near the vowel center; pull back to the consonant onset.
const DTW_LEAD_MS: i64 = 80;
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
    #[serde(default)]
    text: String,
    #[serde(default)]
    offsets: Option<Offsets>,
    tokens: Vec<Token>,
}

#[derive(Deserialize)]
struct Token {
    text: String,
    offsets: Option<Offsets>,
    #[serde(default)]
    t_dtw: Option<i64>,
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
    // Preview versions have no published tag. The engine is shared by stable and Dev.
    let release_path = if crate::VERSION.contains("-nightly") {
        "tags/nightly"
    } else {
        "latest"
    };
    let release: Published = client
        .get(format!("{RELEASES}/{release_path}"))
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
    let checksums_url = &release
        .assets
        .iter()
        .find(|asset| asset.name == "SHA256SUMS")
        .context("this release has no checksums")?
        .browser_download_url;
    let checksums = client
        .get(checksums_url.as_str())
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
        crate::sha256_hex(&Sha256::digest(&archive_bytes)) == expected,
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
        crate::sha256_hex(&digest.finalize()) == MODEL_SHA256,
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
    format!("v5:{}", crate::sha256_hex(&Sha256::digest(text.as_bytes())))
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

/// Runs the recognizer on vocal-filtered audio. Its words are anchors for the supplied lyrics only.
pub(crate) fn generate(engine: &Path, pcm: &[i16], text: &str) -> Result<Option<String>> {
    let filtered = prepare_vocal_pcm(pcm);
    let transcript = recognize(engine, &filtered, text)?;
    let duration_ms = (pcm.len() as u64 * 1_000 / u64::from(RATE)) as i64;
    Ok(align(transcript, text, duration_ms))
}

/// Match each line near its supplied onset, preserving the sheet and adding timing where
/// recognized speech and vocal-band energy confirm the sung words.
pub(crate) fn generate_words(
    engine: &Path,
    pcm: &[i16],
    lines: &[LyricsLine],
) -> Result<Option<Vec<LyricsLine>>> {
    let text = lines
        .iter()
        .map(|line| line.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let filtered = prepare_vocal_pcm(pcm);
    let transcript = recognize(engine, &filtered, &text)?;
    let flux = vocal_flux_10ms(pcm);
    Ok(align_words(
        transcript,
        lines,
        (pcm.len() as u64 * 1_000 / u64::from(RATE)) as i64,
        &flux,
    ))
}

struct Biquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    z1: f32,
    z2: f32,
}

impl Biquad {
    fn from_coeffs(b0: f32, b1: f32, b2: f32, a0: f32, a1: f32, a2: f32) -> Self {
        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            z1: 0.0,
            z2: 0.0,
        }
    }

    fn high_pass(fs: f32, freq: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * freq / fs;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);
        Self::from_coeffs(
            (1.0 + cos_w0) * 0.5,
            -(1.0 + cos_w0),
            (1.0 + cos_w0) * 0.5,
            1.0 + alpha,
            -2.0 * cos_w0,
            1.0 - alpha,
        )
    }

    fn low_pass(fs: f32, freq: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * freq / fs;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);
        Self::from_coeffs(
            (1.0 - cos_w0) * 0.5,
            1.0 - cos_w0,
            (1.0 - cos_w0) * 0.5,
            1.0 + alpha,
            -2.0 * cos_w0,
            1.0 - alpha,
        )
    }

    fn peaking(fs: f32, freq: f32, q: f32, gain_db: f32) -> Self {
        let w0 = 2.0 * PI * freq / fs;
        let cos_w0 = w0.cos();
        let alpha = w0.sin() / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);
        Self::from_coeffs(
            1.0 + alpha * a,
            -2.0 * cos_w0,
            1.0 - alpha * a,
            1.0 + alpha / a,
            -2.0 * cos_w0,
            1.0 - alpha / a,
        )
    }

    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }
}

/// Strips bass/kick rumble, sharpens vocal consonants, and normalizes level for Whisper.
fn prepare_vocal_pcm(pcm: &[i16]) -> Vec<i16> {
    if pcm.is_empty() {
        return Vec::new();
    }
    let fs = RATE as f32;
    let mut hp = Biquad::high_pass(fs, 180.0, 0.707);
    let mut presence = Biquad::peaking(fs, 2_600.0, 0.9, 3.5);
    let mut lp = Biquad::low_pass(fs, 6_200.0, 0.707);

    let mut filtered = Vec::with_capacity(pcm.len());
    let mut active_sq = 0.0_f64;
    let mut active_count = 0usize;
    for &sample in pcm {
        let x = f32::from(sample) / 32_768.0;
        let y = lp.process(presence.process(hp.process(x)));
        if y.abs() > 0.008 {
            active_sq += f64::from(y) * f64::from(y);
            active_count += 1;
        }
        filtered.push(y);
    }

    let rms = if active_count > 0 {
        (active_sq / active_count as f64).sqrt() as f32
    } else {
        0.16
    };
    let gain = (0.16 / rms.max(0.01)).clamp(0.5, 4.5);

    filtered
        .into_iter()
        .map(|y| {
            let v = y * gain;
            let limited = if v.abs() <= 0.75 {
                v
            } else {
                v.signum() * (0.75 + 0.25 * ((v.abs() - 0.75) / 0.25).tanh())
            };
            (limited.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16
        })
        .collect()
}

/// Builds a concise vocabulary prompt from the lyrics to bias Whisper toward sung Thai/English words.
fn lyrics_prompt(text: &str) -> String {
    let mut out = String::new();
    let mut char_count = 0usize;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || (trimmed.starts_with('[') && trimmed.ends_with(']'))
            || (trimmed.starts_with('(') && trimmed.ends_with(')'))
        {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
            char_count += 1;
        }
        for ch in trimmed.chars() {
            if ch.is_control() {
                continue;
            }
            out.push(ch);
            char_count += 1;
            if char_count >= 180 {
                return out;
            }
        }
    }
    out
}

fn run_whisper(
    engine: &Path,
    input: &Path,
    output: &Path,
    thai: bool,
    threads: &str,
    prompt: Option<&str>,
    enhanced: bool,
) -> Result<()> {
    let mut command = Command::new(engine);
    command
        .current_dir(engine.parent().context("missing lyrics engine folder")?)
        .args([
            "-m",
            MODEL,
            "-t",
            threads,
            "-bs",
            if enhanced { "2" } else { "1" },
            "-bo",
            if enhanced { "2" } else { "1" },
            "-ng",
            "-l",
            if thai { "th" } else { "auto" },
        ]);
    if enhanced {
        command.args(["-dtw", "small"]);
        if let Some(prompt) = prompt.filter(|p| !p.is_empty()) {
            command.args(["--prompt", prompt]);
        }
    }
    // Thai UTF-8 takes 3 bytes per char; 96 bytes matches ~32 English chars without chopping mid-word.
    command
        .args(["-ojf", "-ml", if thai { "96" } else { "32" }, "-f"])
        .arg(input)
        .arg("-of")
        .arg(output)
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
                return Ok(());
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
}

fn recognize(engine: &Path, pcm: &[i16], text: &str) -> Result<Transcript> {
    let scratch = tempfile::tempdir()?;
    let input = scratch.path().join("input.wav");
    let output = scratch.path().join("timing");
    wav(&input, pcm)?;
    let thai = text
        .chars()
        .filter(|&c| matches!(c, '\u{0e01}'..='\u{0e4e}'))
        .count()
        >= 8;
    // Keep half the logical cores available for playback and the rest of the desktop.
    let threads = std::thread::available_parallelism()
        .map_or(2, |count| (count.get() / 2).clamp(1, 6))
        .to_string();
    let prompt = lyrics_prompt(text);
    let json_path = output.with_extension("json");
    if run_whisper(
        engine,
        &input,
        &output,
        thai,
        &threads,
        Some(prompt.as_str()),
        true,
    )
    .is_err()
        || !json_path.is_file()
    {
        run_whisper(engine, &input, &output, thai, &threads, None, false)?;
    }
    Ok(serde_json::from_reader(std::fs::File::open(json_path)?)?)
}

fn is_non_speech_token(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.is_empty()
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || (trimmed.starts_with('(') && trimmed.ends_with(')'))
}

/// Computes vocal-band (320Hz-3400Hz) onset flux every 10ms to lock phrase entries to actual singing.
fn vocal_flux_10ms(pcm: &[i16]) -> Vec<f32> {
    let frame = (RATE / 100) as usize; // 160 samples = 10ms
    if pcm.len() < frame * 2 {
        return Vec::new();
    }
    let frames = pcm.len() / frame;
    let mut env = Vec::with_capacity(frames);
    let mut hp = 0.0_f32;
    let mut prev_x = 0.0_f32;
    let mut lp = 0.0_f32;
    for chunk in pcm.chunks_exact(frame) {
        let mut energy = 0.0_f32;
        for &sample in chunk {
            let x = f32::from(sample) / 32_768.0;
            hp = 0.88 * (hp + x - prev_x);
            prev_x = x;
            lp += 0.57 * (hp - lp);
            energy += lp * lp;
        }
        let rms = (energy / frame as f32).sqrt();
        env.push((1.0 + rms * 28.0).ln());
    }
    let mut flux = vec![0.0_f32; frames];
    for i in 1..frames {
        let prev_base = if i >= 2 {
            env[i - 1].min(env[i - 2])
        } else {
            env[i - 1]
        };
        flux[i] = (env[i] - prev_base).max(0.0);
    }
    flux
}

fn snap_to_vocal_onset(ms: i64, min_ms: i64, max_ms: i64, flux: &[f32]) -> i64 {
    if flux.is_empty() || max_ms <= min_ms {
        return ms.clamp(min_ms, max_ms);
    }
    let center = (ms / 10).clamp(0, flux.len() as i64 - 1) as usize;
    let lo = ((ms - 110).max(min_ms) / 10).clamp(0, flux.len() as i64 - 1) as usize;
    let hi = ((ms + 25).min(max_ms) / 10).clamp(0, flux.len() as i64 - 1) as usize;
    if hi <= lo {
        return ms.clamp(min_ms, max_ms);
    }
    let avg = flux[lo..=hi].iter().sum::<f32>() / (hi - lo + 1) as f32;
    if avg <= 0.015 {
        return ms.clamp(min_ms, max_ms);
    }
    let mut best_idx = center;
    let mut best_score = 0.0_f32;
    for idx in lo..=hi {
        let dist = idx.abs_diff(center) as f32;
        let weight = 1.0 - dist * 0.04;
        let score = flux[idx] * weight;
        if score > best_score {
            best_score = score;
            best_idx = idx;
        }
    }
    if flux[best_idx] > avg * 1.75 {
        (best_idx as i64 * 10).clamp(min_ms, max_ms)
    } else {
        ms.clamp(min_ms, max_ms)
    }
}

fn sample_timeline(points: &[i64], frac: f64) -> i64 {
    if points.is_empty() {
        return 0;
    }
    if points.len() == 1 {
        return points[0];
    }
    let pos = frac.clamp(0.0, 1.0) * (points.len() - 1) as f64;
    let idx = (pos.floor() as usize).min(points.len() - 2);
    let rem = pos - idx as f64;
    points[idx] + ((points[idx + 1] - points[idx]) as f64 * rem).round() as i64
}

fn extract_transcript_chars(
    transcript: Transcript,
    duration_ms: i64,
) -> (Vec<char>, Vec<(i64, i64)>) {
    let mut chars = Vec::new();
    let mut times: Vec<(i64, i64)> = Vec::new();
    for segment in transcript.transcription {
        let mut raw_tokens = Vec::new();
        let mut has_broken_utf8 = false;
        let mut token_letter_count = 0usize;

        for token in segment.tokens {
            if is_non_speech_token(&token.text) {
                continue;
            }
            let Some(span) = token
                .offsets
                .filter(|span| span.from >= 0 && span.to >= span.from && span.from < duration_ms)
            else {
                continue;
            };
            if token.text.contains('\u{fffd}') {
                has_broken_utf8 = true;
            }
            let spoken = phonetic_letters(&token.text);
            token_letter_count += spoken.len();
            let dtw_ms = token
                .t_dtw
                .filter(|&dtw| dtw >= 0)
                .map(|dtw| (dtw * 10).saturating_sub(DTW_LEAD_MS))
                .filter(|&ms| {
                    ms >= span.from.saturating_sub(450)
                        && ms <= span.to.saturating_add(350)
                        && ms < duration_ms
                });
            raw_tokens.push((spoken, span.from, span.to.min(duration_ms), dtw_ms));
        }

        let seg_letters = phonetic_letters(&segment.text);
        let split_multibyte = !seg_letters.is_empty()
            && (has_broken_utf8 || token_letter_count * 100 < seg_letters.len() * 85);

        if split_multibyte {
            // Whisper BPE splits 3-byte Thai chars across tokens; recover full chars from segment.text.
            let mut tok_times: Vec<(i64, i64, bool)> = raw_tokens
                .iter()
                .map(|&(_, raw_from, raw_to, dtw_ms)| {
                    let from = dtw_ms.unwrap_or(raw_from);
                    (from, raw_to.max(from), dtw_ms.is_some())
                })
                .collect();
            if tok_times.is_empty() {
                if let Some(span) = segment.offsets.filter(|span| {
                    span.from >= 0 && span.to >= span.from && span.from < duration_ms
                }) {
                    tok_times.push((span.from, span.to.min(duration_ms), false));
                }
            }
            if tok_times.is_empty() {
                continue;
            }
            if !tok_times[0].2 {
                if tok_times.len() >= 2 && tok_times[1].0 - tok_times[0].0 > 1_200 {
                    tok_times[0].0 = tok_times[1].0 - 250;
                } else if tok_times.len() == 1 {
                    let max_leading = (seg_letters.len() as i64 * 220).clamp(600, 2_000);
                    if tok_times[0].1 - tok_times[0].0 > max_leading {
                        tok_times[0].0 = tok_times[0].1 - max_leading;
                    }
                }
            }
            let mut points = Vec::with_capacity(tok_times.len() + 1);
            let mut cur = tok_times[0].0.max(times.last().map_or(0, |&(at, _)| at));
            points.push(cur);
            for &(s, _, _) in tok_times.iter().skip(1) {
                cur = s.max(cur);
                points.push(cur);
            }
            points.push(tok_times.last().unwrap().1.max(cur));

            let count = seg_letters.len();
            for (index, ch) in seg_letters.into_iter().enumerate() {
                let f0 = index as f64 / count as f64;
                let f1 = (index + 1) as f64 / count as f64;
                let start = sample_timeline(&points, f0).max(times.last().map_or(0, |&(at, _)| at));
                let end = sample_timeline(&points, f1).max(start);
                chars.push(ch);
                times.push((start, end));
            }
            continue;
        }

        let speech: Vec<_> = raw_tokens
            .into_iter()
            .filter(|(spoken, _, _, _)| !spoken.is_empty())
            .collect();
        for (tok_idx, (spoken, raw_from, raw_to, dtw_ms)) in speech.iter().enumerate() {
            let count = spoken.len() as i64;
            let next_anchor = speech
                .get(tok_idx + 1)
                .map(|(_, nf, _, nd)| nd.unwrap_or(*nf));

            let effective_from = if let Some(dtw) = *dtw_ms {
                dtw
            } else {
                let max_leading_ms = (count * 220).clamp(500, 1_500);
                if tok_idx == 0 && raw_to - raw_from > max_leading_ms {
                    raw_to - max_leading_ms
                } else {
                    *raw_from
                }
            };
            let effective_to = next_anchor
                .filter(|&next| next > effective_from)
                .map_or(*raw_to, |next| (*raw_to).min(next))
                .max(effective_from);
            let span_ms = (effective_to - effective_from).max(0);
            for (index, &ch) in spoken.iter().enumerate() {
                let start = effective_from + span_ms * index as i64 / count;
                let end = effective_from + span_ms * (index as i64 + 1) / count;
                let start = start.max(times.last().map_or(0, |(at, _)| *at));
                chars.push(ch);
                times.push((start, end.max(start)));
            }
        }
    }
    (chars, times)
}

/// Estimates a consistent song-wide offset between provider LRC timestamps and the YouTube recording.
fn estimate_global_offset(chars: &[char], times: &[(i64, i64)], lines: &[LyricsLine]) -> i64 {
    let mut deltas = Vec::new();
    for line in lines {
        if line.text.trim().is_empty() || line.words.as_ref().is_some_and(|w| !w.is_empty()) {
            continue;
        }
        let target = phonetic_letters(&line.text);
        if target.len() < 6 {
            continue;
        }
        let raw_start = line.start.as_millis().min(i64::MAX as u128) as i64;
        let lo = times.partition_point(|&(at, _)| at < raw_start.saturating_sub(3_500));
        let hi = times.partition_point(|&(at, _)| at < raw_start.saturating_add(3_500));
        if hi <= lo {
            continue;
        }
        let is_thai = line
            .text
            .chars()
            .any(|c| matches!(c, '\u{0e01}'..='\u{0e5b}'));
        let max_err = if is_thai { 3 } else { 2 };
        if let Some((mapped, _)) =
            align_line_chars(&chars[lo..hi], &times[lo..hi], &target, raw_start, max_err)
        {
            if let Some(first_h) = mapped.iter().find_map(|&m| m) {
                let matched_ms = times[lo + first_h].0;
                deltas.push(matched_ms - raw_start);
            }
        }
    }
    if deltas.len() < 4 {
        return 0;
    }
    deltas.sort_unstable();
    let median = deltas[deltas.len() / 2];
    let q1 = deltas[deltas.len() / 4];
    let q3 = deltas[(deltas.len() * 3) / 4];
    if q3 - q1 <= 950 && median.abs() >= 60 {
        median.clamp(-3_000, 3_000)
    } else {
        0
    }
}

fn align_words(
    transcript: Transcript,
    lines: &[LyricsLine],
    duration_ms: i64,
    flux: &[f32],
) -> Option<Vec<LyricsLine>> {
    let (chars, times) = extract_transcript_chars(transcript, duration_ms);
    if chars.is_empty() {
        return None;
    }
    let global_offset = estimate_global_offset(&chars, &times, lines);
    let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
    let mut result = lines.to_vec();
    let mut changed = false;
    let mut consumed = 0;
    let mut prev_line_start_ms: i64 = 0;
    let mut prev_line_end_ms: i64 = 0;
    let mut matched_lines = 0usize;
    let mut total_lines = 0usize;

    for (index, line) in result.iter_mut().enumerate() {
        let base_start_ms = line.start.as_millis().min(i64::MAX as u128) as i64;
        let expected_start_ms = (base_start_ms + global_offset).clamp(0, duration_ms);
        if line.text.trim().is_empty() || line.words.as_ref().is_some_and(|words| !words.is_empty())
        {
            prev_line_start_ms = prev_line_start_ms.max(expected_start_ms);
            continue;
        }
        total_lines += 1;
        let next_start_ms = lines
            .get(index + 1)
            .map(|next| {
                let raw = next.start.as_millis().min(i64::MAX as u128) as i64;
                (raw + global_offset).clamp(0, duration_ms)
            })
            .unwrap_or(duration_ms);
        let end_cap = next_start_ms
            .min(expected_start_ms.saturating_add(12_000))
            .max(expected_start_ms);
        let search_end = next_start_ms
            .saturating_add(WORD_ONSET_LOOKAHEAD_MS)
            .min(expected_start_ms.saturating_add(12_000))
            .min(duration_ms);
        let lower = consumed.max(times.partition_point(|&(at, _)| {
            at < expected_start_ms
                .saturating_sub(WORD_ONSET_LOOKBACK_MS)
                .max(prev_line_start_ms)
        }));
        let upper = times.partition_point(|&(at, _)| at < search_end);
        if upper <= lower {
            if global_offset != 0 {
                line.start = Duration::from_millis(expected_start_ms as u64);
                changed = true;
            }
            prev_line_start_ms = prev_line_start_ms.max(expected_start_ms);
            continue;
        }
        let target = phonetic_letters(&line.text);
        if target.len() < 2 {
            prev_line_start_ms = prev_line_start_ms.max(expected_start_ms);
            continue;
        }
        let is_thai = line
            .text
            .chars()
            .any(|c| matches!(c, '\u{0e01}'..='\u{0e5b}'));
        let max_err = if is_thai { 4 } else { 3 };
        let Some((mapped, best_end)) = align_line_chars(
            &chars[lower..upper],
            &times[lower..upper],
            &target,
            expected_start_ms,
            max_err,
        ) else {
            if global_offset != 0 {
                line.start = Duration::from_millis(expected_start_ms as u64);
                changed = true;
            }
            prev_line_start_ms = prev_line_start_ms.max(expected_start_ms);
            continue;
        };

        let anchors: Vec<usize> = mapped
            .iter()
            .enumerate()
            .filter_map(|(i, pos)| pos.map(|_| i))
            .collect();
        let mut char_times = vec![(expected_start_ms, expected_start_ms); target.len()];
        for &i in &anchors {
            char_times[i] = times[lower + mapped[i].unwrap()];
        }
        let first_a = anchors[0];
        let last_a = *anchors.last().unwrap();
        let ms_per_char = if last_a > first_a {
            ((char_times[last_a].0 - char_times[first_a].0) / (last_a - first_a) as i64)
                .clamp(40, 130)
        } else {
            80
        };
        let min_start_ms = expected_start_ms
            .saturating_sub(WORD_ONSET_LOOKBACK_MS)
            .max(prev_line_start_ms);
        for i in 0..first_a {
            let dist = (first_a - i) as i64;
            let s = (char_times[first_a].0 - dist * ms_per_char).max(min_start_ms);
            let e = (char_times[first_a].0 - (dist - 1) * ms_per_char).max(s);
            char_times[i] = (s, e);
        }
        for pair in anchors.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            if b <= a + 1 {
                continue;
            }
            let t0 = char_times[a].0;
            let t1 = char_times[b].0.max(t0);
            let span = (b - a) as f64;
            for i in (a + 1)..b {
                let f0 = (i - a) as f64 / span;
                let f1 = (i + 1 - a) as f64 / span;
                let s = t0 + ((t1 - t0) as f64 * f0).round() as i64;
                let e = t0 + ((t1 - t0) as f64 * f1).round() as i64;
                char_times[i] = (s, e.max(s));
            }
        }
        for i in (last_a + 1)..target.len() {
            let dist = (i - last_a) as i64;
            let s = (char_times[last_a].1 + (dist - 1) * ms_per_char).min(end_cap);
            let e = (char_times[last_a].1 + dist * ms_per_char)
                .min(end_cap)
                .max(s);
            char_times[i] = (s, e);
        }

        let mut t_cursor = 0;
        let mut byte = 0;
        let mut words = Vec::new();
        for next in segmenter.segment_str(&line.text) {
            let text = &line.text[byte..next];
            byte = next;
            let width = letters(text).count();
            if width == 0 || t_cursor >= char_times.len() {
                continue;
            }
            let last_idx = (t_cursor + width - 1).min(char_times.len() - 1);
            let from = char_times[t_cursor].0.max(prev_line_start_ms);
            let to = char_times[last_idx].1.min(end_cap).max(from);
            words.push(LyricsWord {
                start: Duration::from_millis(from as u64),
                end: Duration::from_millis(to as u64),
                text: text.to_owned(),
            });
            t_cursor += width;
        }

        // Spread words that landed on identical timestamps instead of discarding the whole line.
        let mut run = 0;
        while run < words.len() {
            let mut run_end = run + 1;
            while run_end < words.len() && words[run_end].start <= words[run].start {
                run_end += 1;
            }
            if run_end - run > 1 {
                let base = words[run].start;
                let limit = words
                    .get(run_end)
                    .map(|next| next.start)
                    .unwrap_or_else(|| words[run_end - 1].end.max(base))
                    .max(base);
                let count = (run_end - run) as u32;
                let gap = limit.saturating_sub(base);
                for (offset, word) in words[run..run_end].iter_mut().enumerate() {
                    let step = if gap >= Duration::from_millis(count as u64) {
                        gap * offset as u32 / count
                    } else {
                        Duration::from_millis(offset as u64 * 20).min(gap)
                    };
                    word.start = base + step;
                    word.end = word.end.max(word.start);
                }
            }
            run = run_end;
        }

        // Snap phrase entries after pauses onto the actual vocal onset in the PCM waveform.
        let second_start = words
            .get(1)
            .map(|w| w.start.as_millis().min(i64::MAX as u128) as i64 - 20);
        if let Some(first) = words.first_mut() {
            let s = first.start.as_millis().min(i64::MAX as u128) as i64;
            if s - prev_line_end_ms >= 220 {
                let upper_snap = second_start.unwrap_or(end_cap).max(s);
                let snapped = snap_to_vocal_onset(s, min_start_ms, upper_snap, flux);
                first.start = Duration::from_millis(snapped as u64);
                first.end = first.end.max(first.start);
            }
        }

        if !words.is_empty() {
            let first_start = words[0].start;
            let last_end = words
                .iter()
                .map(|word| word.end)
                .max()
                .unwrap_or(first_start)
                .max(first_start);
            line.start = first_start;
            line.end = Some(last_end);
            line.words = Some(words);
            changed = true;
            matched_lines += 1;
            consumed = lower + best_end;
            prev_line_start_ms = first_start.as_millis().min(i64::MAX as u128) as i64;
            prev_line_end_ms = last_end.as_millis().min(i64::MAX as u128) as i64;
        } else {
            prev_line_start_ms = prev_line_start_ms.max(expected_start_ms);
        }
    }
    log::debug!(
        "lyrics: word-aligned {matched_lines}/{total_lines} lines (global_offset={global_offset}ms)"
    );
    changed.then_some(result)
}

/// Semi-global Levenshtein alignment returning exact matched haystack indices for each target char.
fn align_line_chars(
    haystack: &[char],
    h_times: &[(i64, i64)],
    target: &[char],
    line_start: i64,
    max_error_tenths: usize,
) -> Option<(Vec<Option<usize>>, usize)> {
    let m = target.len();
    let n = haystack.len();
    if m < 2 || n == 0 {
        return None;
    }
    let cols = n + 1;
    let mut dp = vec![0u16; (m + 1) * cols];
    let mut start_col = vec![0usize; (m + 1) * cols];
    let mut back = vec![0u8; (m + 1) * cols];

    for j in 0..=n {
        start_col[j] = j;
    }
    for i in 1..=m {
        dp[i * cols] = i as u16;
        back[i * cols] = 2;
    }
    for i in 1..=m {
        for j in 1..=n {
            let idx = i * cols + j;
            let diag = dp[(i - 1) * cols + (j - 1)]
                .saturating_add(u16::from(target[i - 1] != haystack[j - 1]));
            let up = dp[(i - 1) * cols + j].saturating_add(1);
            let left = dp[i * cols + (j - 1)].saturating_add(1);
            if diag <= up && diag <= left {
                dp[idx] = diag;
                start_col[idx] = start_col[(i - 1) * cols + (j - 1)];
                back[idx] = 1;
            } else if up <= left {
                dp[idx] = up;
                start_col[idx] = start_col[(i - 1) * cols + j];
                back[idx] = 2;
            } else {
                dp[idx] = left;
                start_col[idx] = start_col[i * cols + (j - 1)];
                back[idx] = 3;
            }
        }
    }

    let max_cost = if m < 5 {
        1
    } else {
        (m * max_error_tenths / 10).max(1)
    } as u16;
    let mut best: Option<(i64, usize)> = None;
    for j in 1..=n {
        let cost = dp[m * cols + j];
        if cost > max_cost {
            continue;
        }
        let h_start = start_col[m * cols + j];
        if j <= h_start {
            continue;
        }
        let start_ms = h_times[h_start].0;
        let end_ms = h_times[j - 1].1;
        if end_ms - start_ms > 15_000 || start_ms - line_start > 3_800 {
            continue;
        }
        let drift = (start_ms - line_start).abs();
        let score = i64::from(cost) * 1_000 + drift / 4;
        if best.is_none_or(|(best_score, _)| score < best_score) {
            best = Some((score, j));
        }
    }
    let (_, best_j) = best?;

    let mut mapped = vec![None; m];
    let (mut i, mut j) = (m, best_j);
    while i > 0 {
        match back[i * cols + j] {
            1 if j > 0 => {
                if target[i - 1] == haystack[j - 1] {
                    mapped[i - 1] = Some(j - 1);
                }
                i -= 1;
                j -= 1;
            }
            3 if j > 0 => {
                j -= 1;
            }
            _ => {
                i -= 1;
            }
        }
    }
    let matched = mapped.iter().filter(|pos| pos.is_some()).count();
    (matched >= 2 && matched * 10 >= m * 5).then_some((mapped, best_j))
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

/// Normalizes matching text without changing the displayed lyrics.
fn letters(text: &str) -> impl Iterator<Item = char> + '_ {
    text.chars()
        .filter(|c| !matches!(c, '\u{0e47}'..='\u{0e4c}'))
        .filter(|c| c.is_alphanumeric() || matches!(c, '\u{0e31}' | '\u{0e34}'..='\u{0e3a}'))
        .flat_map(char::to_lowercase)
}

/// Maps Thai homophones and sung vowel-length pairs 1:1 so alignment survives Whisper spelling drift.
fn phonetic_char(ch: char) -> char {
    match ch {
        'ข' | 'ฃ' | 'ค' | 'ฅ' | 'ฆ' => 'ค',
        'ฉ' | 'ช' | 'ฌ' => 'ช',
        'ซ' | 'ศ' | 'ษ' | 'ส' => 'ส',
        'ญ' | 'ย' => 'ย',
        'ฎ' | 'ด' => 'ด',
        'ฏ' | 'ต' => 'ต',
        'ฐ' | 'ฑ' | 'ฒ' | 'ถ' | 'ท' | 'ธ' => 'ท',
        'ณ' | 'น' => 'น',
        'ผ' | 'พ' | 'ภ' => 'พ',
        'ฝ' | 'ฟ' => 'ฟ',
        'ร' | 'ล' | 'ฬ' => 'ล',
        'ห' | 'ฮ' => 'ห',
        'ะ' | 'ั' | 'า' => 'า',
        'ิ' | 'ี' => 'ี',
        'ึ' | 'ื' => 'ื',
        'ุ' | 'ู' => 'ู',
        'เ' | 'แ' => 'เ',
        'ใ' | 'ไ' => 'ไ',
        other => other,
    }
}

fn phonetic_letters(text: &str) -> Vec<char> {
    letters(text).map(phonetic_char).collect()
}

/// Uses reliable recognition hits as anchors and interpolates short gaps between them.
fn align(transcript: Transcript, text: &str, duration_ms: i64) -> Option<String> {
    let (chars, spans) = extract_transcript_chars(transcript, duration_ms);
    let time: Vec<i64> = spans.into_iter().map(|(start, _)| start).collect();
    let lines: Vec<_> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    let mut anchors = Vec::new();
    let mut cursor = 0;
    let mut last = -1;
    for (index, line) in lines.iter().enumerate() {
        let target = phonetic_letters(line);
        if target.len() < 2 || cursor >= chars.len() {
            continue;
        }
        let missed = anchors.last().map_or(index, |&(at, _)| index - at - 1);
        let until = last.max(0)
            + if anchors.is_empty() {
                80_000
            } else {
                28_000 + (missed.min(5) as i64 * 6_000)
            };
        let end = time
            .partition_point(|&stamp| stamp <= until)
            .min(cursor + target.len() * 2 + 120 + missed.min(5) * 45);
        if end <= cursor {
            continue;
        }
        let Some((start, end)) = match_line(&target, &chars[cursor..end]) else {
            continue;
        };
        let at = time[cursor + start];
        if at > last {
            last = at;
            anchors.push((index, at));
            cursor = (cursor + end).max(time.partition_point(|&stamp| stamp <= last));
        }
    }

    let (first, first_at) = *anchors.first()?;
    let (last, last_at) = *anchors.last()?;
    let enough = anchors.len() >= 4 && anchors.len() * 2 >= lines.len();
    let covers = first <= 3
        && lines.len() - 1 - last <= 3
        && last_at - first_at >= duration_ms / 3
        && anchors.windows(2).all(|pair| pair[1].0 - pair[0].0 <= 6);
    if !enough || !covers {
        log::debug!(
            "lyrics: recognizer aligned {}/{} lines, too few usable anchors",
            anchors.len(),
            lines.len()
        );
        return None;
    }
    let mut stamps = vec![0; lines.len()];
    let interval = ((last_at - first_at) / (last - first) as i64).clamp(700, 5_000);
    for (index, stamp) in stamps[..first].iter_mut().enumerate() {
        let step = (first_at / first.max(1) as i64).min(interval);
        *stamp = first_at - (first - index) as i64 * step;
    }
    for pair in anchors.windows(2) {
        let [(from, start), (to, end)] = [pair[0], pair[1]];
        for (offset, stamp) in stamps[from..to].iter_mut().enumerate() {
            *stamp = start + (end - start) * offset as i64 / (to - from) as i64;
        }
    }
    let tail_step = ((duration_ms - last_at) / (lines.len() - last) as i64).min(interval);
    for (offset, stamp) in stamps[last..].iter_mut().enumerate() {
        *stamp = last_at + offset as i64 * tail_step;
    }
    if !stamps.windows(2).all(|pair| pair[0] < pair[1]) {
        return None;
    }
    log::debug!(
        "lyrics: matched {}/{} lines and filled short gaps",
        anchors.len(),
        lines.len()
    );
    Some(
        lines
            .iter()
            .zip(stamps)
            .map(|(line, ms)| {
                format!(
                    "[{:02}:{:02}.{:03}]{line}",
                    ms / 60_000,
                    ms / 1_000 % 60,
                    ms % 1_000
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

/// Finds the lowest edit cost for the line in a bounded window, preferring an earlier match.
fn match_line(needle: &[char], haystack: &[char]) -> Option<(usize, usize)> {
    if let Some(start) = haystack
        .windows(needle.len())
        .position(|span| span == needle)
    {
        return Some((start, start + needle.len()));
    }
    if needle.len() < 5 {
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
        .filter(|(end, (cost, start))| end > start && *cost * 3 <= needle.len())
        .min_by_key(|(end, (cost, start))| (cost * 8 + start / 8, *end))
        .map(|(end, (_, start))| (start, end))
}
