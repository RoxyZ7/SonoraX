use std::{
    f32::consts::TAU,
    ops::Range,
    sync::Arc,
    time::{Duration, Instant},
};

use gpui::accesskit::Role;
use gpui::prelude::*;
use gpui::{
    App, Div, FontWeight, Hsla, PathBuilder, Pixels, Point, SharedString, TextShadow, Window,
    canvas, div, linear_color_stop, linear_gradient, point, px,
};
use icu_segmenter::{WordSegmenter, options::WordBreakInvariantOptions};
use music::{LyricsLine, Track};
use state::{Playback, Sonora};
use ui::{ActiveTheme as _, Artwork, GLASS_BLUR, Theme};
use unicode_segmentation::UnicodeSegmentation as _;

const STAR_SCALES: [f32; 8] = [1.08, 0.88, 1.16, 0.94, 1.12, 0.86, 1.04, 0.90];
const ANCHOR_GLOW_SCALE: f32 = 2.6;
const ANCHOR_GLOW_OPACITY: f32 = 0.22;
const ANCHOR_INNER_RATIO: f32 = 0.28;
const HALO_STEPS: usize = 3;
const HALO_POINTS: usize = 24;
const FLARE_RING_POINTS: usize = 33;
const GROUP_LIMIT: usize = 14;

/// Shapes expand organically from left-center toward the right, forming crisp interlocking triangles.
const GROUP_SHAPES: [[(f32, f32); GROUP_LIMIT]; 3] = [
    [
        (0.18, 0.54),
        (0.27, 0.28),
        (0.32, 0.64),
        (0.41, 0.30),
        (0.47, 0.58),
        (0.56, 0.24),
        (0.61, 0.56),
        (0.71, 0.34),
        (0.76, 0.66),
        (0.84, 0.44),
        (0.67, 0.77),
        (0.49, 0.78),
        (0.30, 0.78),
        (0.52, 0.42),
    ],
    [
        (0.22, 0.40),
        (0.16, 0.66),
        (0.35, 0.64),
        (0.38, 0.32),
        (0.51, 0.24),
        (0.54, 0.58),
        (0.66, 0.36),
        (0.74, 0.64),
        (0.84, 0.48),
        (0.70, 0.22),
        (0.60, 0.78),
        (0.43, 0.79),
        (0.26, 0.76),
        (0.47, 0.44),
    ],
    [
        (0.24, 0.56),
        (0.33, 0.26),
        (0.42, 0.62),
        (0.48, 0.34),
        (0.58, 0.66),
        (0.65, 0.28),
        (0.75, 0.56),
        (0.83, 0.36),
        (0.70, 0.78),
        (0.51, 0.78),
        (0.31, 0.76),
        (0.17, 0.42),
        (0.56, 0.46),
        (0.38, 0.44),
    ],
];

const STAR_TRAIL_WIDTH: Pixels = px(5.5);
/// Compensates for audio output buffer + visual attack so lyrics land right on the vocal beat.
const CONSTELLATION_LEAD: Duration = Duration::from_millis(60);
const CLOCK_BEVEL: Pixels = px(0.75);
const COVER_BLUR_FLOOR: f32 = 0.20;
const FLARE_BEAMS: [(f32, f32, f32); 2] = [(1., 0.22, 0.34), (0.70, 0.065, 0.86)];
const WORD_GLOW: Pixels = px(10.);
const PHRASE_FADE: Duration = Duration::from_millis(420);
const HANDOFF_LEAD: Duration = Duration::from_millis(210);
const HANDOFF_TAIL: Duration = Duration::from_millis(240);
const SONG_HANDOFF_LEAD: Duration = Duration::from_millis(480);
const SONG_HANDOFF_IMMEDIATE: Duration = Duration::from_millis(160);
const RESPONSE_LANES: usize = 3;
const INSTRUMENTAL_GAP: Duration = Duration::from_millis(2400);
const COVER_RETURN: Duration = Duration::from_millis(950);
const COVER_RELEASE: Duration = Duration::from_millis(620);
const STAR_BLOOM: Duration = Duration::from_millis(340);
const STAR_ATTACK: Duration = Duration::from_millis(60);
const LINK_REVEAL: Duration = Duration::from_millis(170);
const SONG_DISSOLVE: Duration = Duration::from_millis(820);
const PALETTE_DISSOLVE: Duration = Duration::from_millis(580);

const FONT: &str = "Noto Sans";
const THAI_FONT: &str = "Noto Sans Thai";

/// Clear glass uses neutral shading and keeps the album tint in the rim reflection.
pub(super) fn glass_controls(element: Div, tint: Option<Hsla>, cx: &App) -> Div {
    let dark = Theme::dark();
    let tint = tint.unwrap_or(dark.foreground);
    let blur = ui::blurring(cx);
    let rim = Hsla {
        s: tint.s * 0.40,
        l: 0.86,
        ..tint
    };
    element
        .font_family(FONT)
        .rounded(dark.radius * 2.2)
        .border_1()
        .border_color(rim.opacity(0.30))
        .when(blur, |this| this.backdrop_blur(GLASS_BLUR * 0.75))
        .bg(linear_gradient(
            168.,
            linear_color_stop(rim.opacity(0.11), 0.),
            linear_color_stop(dark.background.opacity(if blur { 0.30 } else { 0.46 }), 1.),
        ))
}

fn prism(tint: Hsla, angle: f32) -> Hsla {
    Hsla {
        h: (tint.h + angle).rem_euclid(1.),
        s: tint.s.max(0.46),
        l: tint.l.clamp(0.58, 0.84),
        ..tint
    }
}

#[derive(Clone)]
struct LyricNode {
    text: SharedString,
    font: &'static str,
    start: Duration,
    end: Duration,
}

#[derive(Clone)]
struct LyricGroup {
    nodes: Vec<LyricNode>,
    follows_break: bool,
    links: [[Option<usize>; 2]; GROUP_LIMIT],
}

/// The art and title displayed at the end of a track, held while the next enters.
#[derive(Clone)]
struct SongArt {
    id: Option<SharedString>,
    title: SharedString,
    artists: SharedString,
    cover: Option<SharedString>,
}

impl SongArt {
    fn new(track: &Track, cx: &App) -> Self {
        let cover = Sonora::global(cx).cover.read(cx).backdrop_for(track, cx);
        Self {
            id: track.id.as_deref().map(SharedString::from),
            title: SharedString::from(track.name.as_str()),
            artists: SharedString::from(track.artists.as_str()),
            cover: cover.map(SharedString::from),
        }
    }

    fn same_track(&self, other: &Self) -> bool {
        match (&self.id, &other.id) {
            (Some(left), Some(right)) => left == right,
            (None, None) => self.title == other.title && self.artists == other.artists,
            _ => false,
        }
    }
}

/// The last visible cover and lyric group, frozen at their playback position.
struct SongExit {
    art: SongArt,
    cover_focus: f32,
    sky: Option<(LyricGroup, GraphMotion, Duration)>,
    drift: f32,
    colors: Option<SceneColors>,
    started: Instant,
}

/// The colors shared by the rim, clock and stars during a cover change.
#[derive(Clone, Copy, PartialEq)]
struct SceneColors {
    glow: Hsla,
    edge: [Hsla; 2],
    clock: [Hsla; 2],
    contrast: f32,
}

impl SceneColors {
    fn mix(self, other: Self, amount: f32) -> Self {
        let tint = |a, b| ui::motion::mix(a, b, amount);
        Self {
            glow: tint(self.glow, other.glow),
            edge: std::array::from_fn(|i| tint(self.edge[i], other.edge[i])),
            clock: std::array::from_fn(|i| tint(self.clock[i], other.clock[i])),
            contrast: self.contrast + (other.contrast - self.contrast) * amount,
        }
    }
}

/// Keep the current blended colors as the starting point when late artwork arrives.
struct PaletteFade {
    from: SceneColors,
    to: SceneColors,
    started: Instant,
}

impl PaletteFade {
    fn sample(&self, now: Instant) -> (SceneColors, bool) {
        if self.from == self.to {
            return (self.to, false);
        }
        let fraction = progress(now - self.started, PALETTE_DISSOLVE);
        (
            self.from.mix(self.to, ui::ease_in_out_cubic(fraction)),
            fraction < 1.,
        )
    }
}

#[derive(Default)]
struct LyricSky {
    source: Option<Arc<[LyricsLine]>>,
    groups: Vec<LyricGroup>,
    vocals: Vec<Range<Duration>>,
    responses: Vec<LyricResponse>,
}

/// A timed backing vocal that can run alongside the main constellation.
struct LyricResponse {
    span: Range<Duration>,
    nodes: Vec<LyricNode>,
    lane: Option<usize>,
}

#[derive(Default)]
pub(super) struct LyricTimeline {
    sky: LyricSky,
    incoming_sky: LyricSky,
    incoming_id: Option<SharedString>,
    energy: f32,
    sampled_at: Option<Instant>,
    art: Option<SongArt>,
    exit: Option<SongExit>,
    palette: Option<PaletteFade>,
    incoming_palette: Option<PaletteFade>,
    pub(super) controls_tint: Option<Hsla>,
    last_lyric: Option<Duration>,
    last_cover_focus: f32,
    drift: f32,
    frame_at: Option<Instant>,
}

impl LyricTimeline {
    /// Clear scene history when the user leaves fullscreen or opens a different panel.
    pub(super) fn reset_scene(&mut self) {
        self.art = None;
        self.incoming_id = None;
        self.incoming_sky = LyricSky::default();
        self.incoming_palette = None;
        self.exit = None;
        self.palette = None;
        self.last_lyric = None;
        self.frame_at = None;
        self.drift = 0.;
    }

    /// Retain one outgoing scene even when the user skips through several tracks quickly.
    fn follow_song(&mut self, art: Option<SongArt>, now: Instant, motion: bool) {
        if !motion
            || art.is_none()
            || self
                .exit
                .as_ref()
                .is_some_and(|exit| now - exit.started >= SONG_DISSOLVE)
        {
            self.exit = None;
        }
        let previous = std::mem::replace(&mut self.art, art);
        let Some((previous, _)) = previous
            .zip(self.art.as_ref())
            .filter(|(old, new)| !old.same_track(new))
        else {
            return;
        };
        let joined = self
            .incoming_id
            .take()
            .is_some_and(|id| self.art.as_ref().and_then(|art| art.id.as_ref()) == Some(&id));
        if joined {
            std::mem::swap(&mut self.sky, &mut self.incoming_sky);
            self.palette = self.incoming_palette.take().or_else(|| self.palette.take());
            self.exit = None;
        } else if motion {
            let keep_old = self
                .exit
                .as_ref()
                .is_some_and(|exit| progress(now - exit.started, SONG_DISSOLVE) < 0.42);
            if !keep_old {
                let sky = self
                    .last_lyric
                    .filter(|_| self.last_cover_focus < 0.75)
                    .and_then(|at| {
                        let shape = self
                            .sky
                            .groups
                            .partition_point(|group| group.nodes[0].start <= at)
                            .checked_sub(1)?;
                        let group = &self.sky.groups[shape];
                        let age = (at - group.nodes[0].start).as_secs_f32();
                        Some((
                            group.clone(),
                            GraphMotion {
                                shape,
                                age,
                                enabled: false,
                                drift: self.drift,
                                audio: [0.; 2],
                                energy: self.energy,
                            },
                            at,
                        ))
                    });
                self.exit = Some(SongExit {
                    art: previous,
                    cover_focus: self.last_cover_focus,
                    sky,
                    drift: self.drift,
                    colors: self.palette.as_ref().map(|palette| palette.sample(now).0),
                    started: now,
                });
            }
        }
        self.last_lyric = None;
    }

    fn follow_colors(
        palette: &mut Option<PaletteFade>,
        target: SceneColors,
        now: Instant,
        motion: bool,
    ) -> (SceneColors, bool) {
        let previous = motion.then_some(palette.as_ref()).flatten();
        if let Some(previous) = previous.filter(|fade| fade.to == target) {
            return previous.sample(now);
        }
        let from = previous.map_or(target, |fade| fade.sample(now).0);
        *palette = Some(PaletteFade {
            from,
            to: target,
            started: now,
        });
        (from, from != target)
    }

    /// Keep the ambient light moving in the same direction across track changes and seeks.
    fn advance_drift(&mut self, now: Instant, moving: bool) -> f32 {
        let elapsed = self
            .frame_at
            .replace(now)
            .map_or(0., |then| (now - then).as_secs_f32());
        if moving {
            self.drift += elapsed.min(0.05) * 0.28;
        }
        self.drift
    }

    /// Smooth the footprint over a phrase so bass hits light the stars without jostling words.
    fn follow_energy(&mut self, audio: [f32; 2], playing: bool) -> f32 {
        let now = Instant::now();
        let elapsed = self.sampled_at.replace(now).map(|then| now - then);
        let target = (audio[0] * 0.76 + audio[1] * 0.24).clamp(0., 1.).sqrt();
        if let Some(elapsed) = elapsed.filter(|elapsed| *elapsed < Duration::from_secs(1)) {
            if playing {
                let seconds = if target > self.energy { 0.30 } else { 0.75 };
                self.energy +=
                    (target - self.energy) * (1. - (-elapsed.as_secs_f32() / seconds).exp());
            }
        } else {
            self.energy = target;
        }
        self.energy
    }
}

impl LyricSky {
    fn update(&mut self, source: Option<&Arc<[LyricsLine]>>) {
        match (self.source.as_ref(), source) {
            (Some(previous), Some(next)) if Arc::ptr_eq(previous, next) => return,
            (None, None) => return,
            _ => {}
        }
        self.source = source.cloned();
        self.groups.clear();
        self.vocals.clear();
        self.responses.clear();
        let Some(lines) = source else { return };
        let mut starts_group = true;
        let mut explicit_break = false;
        for (index, line) in lines.iter().enumerate() {
            let next = lines.get(index + 1).map(|next| next.start);
            for voice in &line.secondary {
                let tokens = lyric_tokens(&voice.text);
                let end = estimated_line_end(
                    &voice.text,
                    &tokens,
                    voice.start,
                    voice.sung_end(),
                    next.filter(|next| *next > voice.start),
                );
                let nodes = timed_nodes(&voice.text, voice.words.as_deref(), voice.start, end);
                if let Some(first) = nodes.first() {
                    let span_end = nodes
                        .iter()
                        .map(|node| node.end)
                        .max()
                        .unwrap_or(end)
                        .max(first.start);
                    self.responses.push(LyricResponse {
                        span: first.start..span_end,
                        nodes,
                        lane: None,
                    });
                }
            }
            if line.text.trim().is_empty() {
                explicit_break = true;
                continue;
            }
            let tokens = lyric_tokens(&line.text);
            let end = estimated_line_end(&line.text, &tokens, line.start, line.sung_end(), next);
            let nodes = timed_nodes(&line.text, line.words.as_deref(), line.start, end);
            if nodes.is_empty() {
                continue;
            }
            let vocal_start = nodes[0].start;
            let vocal_end = nodes
                .iter()
                .map(|node| node.end)
                .max()
                .unwrap_or(end)
                .max(vocal_start);
            let mut follows_break = self.vocals.last().is_none_or(|span| {
                vocal_start.saturating_sub(span.end) >= INSTRUMENTAL_GAP
                    || (explicit_break && vocal_start > span.end)
            });
            if follows_break {
                self.vocals.push(vocal_start..vocal_end);
            } else if let Some(span) = self.vocals.last_mut() {
                span.start = span.start.min(vocal_start);
                span.end = span.end.max(vocal_end);
            }
            explicit_break = false;
            let mut new_group = starts_group
                || follows_break
                || self
                    .groups
                    .last()
                    .is_none_or(|group| group.nodes.len() + nodes.len() > GROUP_LIMIT);
            for node in nodes {
                if new_group {
                    self.groups.push(LyricGroup {
                        nodes: Vec::new(),
                        follows_break,
                        links: [[None; 2]; GROUP_LIMIT],
                    });
                    follows_break = false;
                }
                let group = self.groups.last_mut().expect("a lyric group is open");
                group.nodes.push(node);
                new_group = group.nodes.len() == GROUP_LIMIT;
            }
            starts_group = false;
        }
        self.responses.sort_by_key(|response| response.span.start);
        let mut occupied = [Duration::ZERO; RESPONSE_LANES];
        for response in &mut self.responses {
            response.lane = occupied.iter().position(|end| *end <= response.span.start);
            if let Some(lane) = response.lane {
                occupied[lane] = response.span.end.saturating_add(RESPONSE_RELEASE);
            }
        }
        for group in &mut self.groups {
            group.nodes.sort_by_key(|node| node.start);
        }
        self.groups.sort_by_key(|group| group.nodes[0].start);
        for (index, group) in self.groups.iter_mut().enumerate() {
            let points = &GROUP_SHAPES[index % GROUP_SHAPES.len()][..group.nodes.len()];
            for (end, link) in group
                .links
                .iter_mut()
                .enumerate()
                .take(points.len())
                .skip(2)
            {
                *link = cross_links(points, end);
            }
        }
    }

    /// Fade from the last constellation into the cover and out again before the next vocal.
    fn cover_focus(&self, at: Duration, motion: bool) -> f32 {
        if self.active_responses(at).next().is_some() {
            return 0.;
        }
        let next = self.vocals.partition_point(|span| span.start <= at);
        let previous = next.checked_sub(1).map(|index| &self.vocals[index]);
        let upcoming = self.vocals.get(next);
        if previous.is_some_and(|span| at < span.end) {
            return 0.;
        }
        if !motion {
            return 1.;
        }
        let Some(previous) = previous else {
            return upcoming.map_or(1., |span| {
                ui::ease_in_out_cubic(progress(span.start.saturating_sub(at), COVER_RELEASE))
            });
        };
        let return_time = cover_return_time(previous.end, upcoming);
        let release_time = upcoming.map_or(COVER_RELEASE, |span| {
            COVER_RELEASE.min(span.start.saturating_sub(previous.end) / 2)
        });
        let fade_in = ui::ease_in_out_cubic(progress(at - previous.end, return_time));
        let fade_out = upcoming.map_or(1., |span| {
            ui::ease_in_out_cubic(progress(span.start.saturating_sub(at), release_time))
        });
        fade_in.min(fade_out)
    }

    fn active_responses(&self, at: Duration) -> impl Iterator<Item = &LyricResponse> {
        let started = self
            .responses
            .partition_point(|response| response.span.start <= at);
        self.responses[..started]
            .iter()
            .rev()
            .filter(move |response| at.saturating_sub(response.span.end) < RESPONSE_RELEASE)
    }

    /// Keep the old group only during the first portion of an instrumental break.
    fn instrumental_return(&self, at: Duration) -> Option<(Duration, Duration)> {
        let next = self.vocals.partition_point(|span| span.start <= at);
        let previous = self.vocals.get(next.checked_sub(1)?)?;
        let elapsed = at.checked_sub(previous.end)?;
        let return_time = cover_return_time(previous.end, self.vocals.get(next));
        (elapsed < return_time).then_some((elapsed, return_time))
    }
}

fn cover_return_time(end: Duration, next: Option<&Range<Duration>>) -> Duration {
    next.map_or(COVER_RETURN, |span| {
        COVER_RETURN.min(span.start.saturating_sub(end) / 2)
    })
}

/// A fullscreen lyric scene driven by the same offset and clock as the lyrics panel.
pub(super) fn scene(
    playback: &Playback,
    timeline: &mut LyricTimeline,
    audio: [f32; 2],
    window: &mut Window,
    cx: &App,
) -> Div {
    let dark = Theme::dark();
    let track = playback.shown_track(cx);
    let frame_now = Instant::now();
    let motion = ui::motion::animates(cx);
    let (playhead, playing) = playback.visual_position(cx);
    let (rem, viewport) = (window.rem_size(), window.viewport_size());
    let screen = point(viewport.width, viewport.height);
    let width = viewport.width * 0.84;
    let height = viewport.height * 0.48;
    let size = point(width, height);
    let clock = (viewport.height * 0.145).clamp(rem * 2.1, rem * 7.2);
    let album_side = (viewport.height * 0.30).min(rem * 15.);

    timeline.follow_song(
        track.as_ref().map(|track| SongArt::new(track, cx)),
        frame_now,
        motion,
    );
    let crossfade = playback.shown_crossfade(cx);
    let incoming_art = crossfade.map(|(track, _)| SongArt::new(track, cx));
    let lookahead = crossfade
        .is_none()
        .then(|| {
            playback
                .crossfade_target(cx)
                .map(|track| SongArt::new(track, cx))
        })
        .flatten();
    if timeline.incoming_id != incoming_art.as_ref().and_then(|art| art.id.clone()) {
        timeline.incoming_palette = None;
        timeline.incoming_id = incoming_art.as_ref().and_then(|art| art.id.clone());
    }
    if crossfade.is_some() {
        timeline.exit = None;
    }
    let (lyric_time, source) = scene_lyrics(track.as_ref(), playhead, cx);
    let lyric_time = lyric_time.map(|at| at.saturating_add(CONSTELLATION_LEAD));
    timeline.sky.update(source);
    let incoming_lyrics = crossfade.and_then(|(track, fade)| {
        let (at, source) = scene_lyrics(Some(track), fade.incoming, cx);
        timeline.incoming_sky.update(source);
        at.map(|at| at.saturating_add(CONSTELLATION_LEAD))
    });

    let moving = playing && motion;
    let now = jiff::Zoned::now();
    let audio = if moving { audio } else { [0.; 2] };
    let energy = timeline.follow_energy(audio, moving);
    let ambient_drift =
        timeline.advance_drift(frame_now, moving || (motion && timeline.exit.is_some()));
    let drift = if motion { ambient_drift } else { 0. };
    let cover_focus = lyric_time.map_or(1., |at| timeline.sky.cover_focus(at, motion));
    let incoming_focus =
        incoming_lyrics.map_or(1., |at| timeline.incoming_sky.cover_focus(at, motion));
    timeline.last_lyric = lyric_time;
    timeline.last_cover_focus = cover_focus;

    let (palette, contrast) = cover_colors(timeline.art.as_ref(), track.as_ref(), screen, cx);
    let mut target = scene_colors(palette, contrast, &dark, cx);
    if palette.is_none()
        && timeline.art.as_ref().is_some_and(|art| art.cover.is_some())
        && let Some(colors) = timeline.exit.as_ref().and_then(|exit| exit.colors)
    {
        target = colors;
    }
    let (current_colors, palette_moving) =
        LyricTimeline::follow_colors(&mut timeline.palette, target, frame_now, motion);
    let change = crossfade.map_or(1., |(_, fade)| fade.progress);
    let blend = change * change * (3. - 2. * change);
    let (incoming_colors, incoming_colors_moving) = if let Some((track, _)) = crossfade {
        let (palette, contrast) = cover_colors(incoming_art.as_ref(), Some(track), screen, cx);
        let incoming =
            if palette.is_none() && incoming_art.as_ref().is_some_and(|art| art.cover.is_some()) {
                current_colors
            } else {
                scene_colors(palette, contrast, &dark, cx)
            };
        let (incoming, palettes_move) = LyricTimeline::follow_colors(
            &mut timeline.incoming_palette,
            incoming,
            frame_now,
            motion,
        );
        (incoming, palettes_move)
    } else {
        (current_colors, false)
    };
    let colors = current_colors.mix(incoming_colors, blend);
    timeline.controls_tint = Some(colors.clock[0]);
    let glow = colors.glow;
    let light = Hsla {
        l: (glow.l + 0.25).min(0.94),
        ..glow
    };
    let star_palette = |colors: SceneColors| StarLight {
        primary: colors.edge[0],
        secondary: colors.edge[1],
        foreground: dark.foreground,
    };
    let star_light = star_palette(current_colors);
    let incoming_light = star_palette(incoming_colors);
    let clock_colors = colors.clock;
    let clock_contrast = colors.contrast;
    let exit = timeline.exit.as_ref();
    let art = timeline.art.as_ref();
    let same_cover = exit
        .zip(art)
        .is_some_and(|(exit, art)| exit.art.cover == art.cover);
    let same_incoming_cover = art
        .zip(incoming_art.as_ref())
        .is_some_and(|(old, new)| old.cover == new.cover);
    let change = exit.map_or(change, |exit| {
        progress(frame_now - exit.started, SONG_DISSOLVE)
    });
    let arrival = ui::ease_out_cubic(((change - 0.08) / 0.92).clamp(0., 1.));
    let departure = 1. - ui::ease_in_out_cubic((change / 0.78).min(1.));
    let primary_opacity = if crossfade.is_some() {
        1. - blend
    } else {
        arrival
    };
    let rim_position = song_progress(playhead, playback.shown_time(cx).1);
    let rim_position = crossfade.map_or(rim_position, |(track, fade)| {
        rim_position + (song_progress(fade.incoming, track.duration) - rim_position) * blend
    });
    if moving
        || (playing && (lyric_time.is_some() || incoming_lyrics.is_some()))
        || palette_moving
        || incoming_colors_moving
        || exit.is_some()
        || (playing && playback.crossfade_target(cx).is_some())
    {
        window.request_animation_frame();
    }
    let effects = crate::shared::effects();
    let frame = SkyFrame {
        size,
        drift,
        audio,
        energy,
        motion,
        light: star_light,
        rem,
        song_handoff: false,
    };
    let song_handoff = (motion && crossfade.is_some())
        .then(|| {
            song_handoff(
                &timeline.sky,
                &timeline.incoming_sky,
                lyric_time?,
                incoming_lyrics?,
                frame,
                incoming_light,
            )
        })
        .flatten();
    let ambient_focus =
        cover_focus + (incoming_focus - cover_focus) * if crossfade.is_some() { blend } else { 0. };

    div()
        .flex_1()
        .min_h_0()
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .font_family(FONT)
        .when_some(lookahead.and_then(|art| art.cover), |this, cover| {
            this.child(ui::Prefetch::new([
                Artwork::new(Some(cover.clone()))
                    .size(screen.x * 1.12)
                    .wide(screen.y * 1.12)
                    .into_any_element(),
                Artwork::new(Some(cover))
                    .size(album_side)
                    .into_any_element(),
            ]))
        })
        .child(
            div()
                .absolute()
                .inset_0()
                .overflow_hidden()
                .bg(dark.background)
                .when_some(
                    exit.filter(|_| departure > 0. && !same_cover),
                    |this, exit| {
                        this.when_some(exit.art.cover.clone(), |this, cover| {
                            this.child(cover_backdrop(
                                cover,
                                "constellation-old-cover",
                                screen,
                                exit.drift,
                                exit.cover_focus,
                                departure,
                                effects,
                            ))
                        })
                    },
                )
                .when_some(
                    timeline.art.as_ref().and_then(|art| art.cover.clone()),
                    |this, cover| {
                        this.child(cover_backdrop(
                            cover,
                            "constellation-cover",
                            screen,
                            drift,
                            cover_focus,
                            if same_cover || same_incoming_cover {
                                1.
                            } else {
                                primary_opacity
                            },
                            effects,
                        ))
                    },
                )
                .when_some(
                    incoming_art
                        .as_ref()
                        .and_then(|art| art.cover.clone())
                        .filter(|_| !same_incoming_cover),
                    |this, cover| {
                        this.child(cover_backdrop(
                            cover,
                            "constellation-next-cover",
                            screen,
                            drift,
                            incoming_focus,
                            blend,
                            effects,
                        ))
                    },
                )
                .when(effects, |this| {
                    this.child(
                        div()
                            .absolute()
                            .inset_0()
                            .opacity(0.17 + ambient_focus * 0.18)
                            .child(
                                ui::Waves::new([
                                    prism(glow, -0.12),
                                    prism(glow, 0.03),
                                    prism(glow, 0.20),
                                    prism(glow, 0.29),
                                    prism(glow, -0.19),
                                ])
                                .stage(dark.background)
                                .time(drift),
                            ),
                    )
                })
                .child(div().absolute().inset_0().bg(linear_gradient(
                    180.,
                    linear_color_stop(dark.background.opacity(0.32), 0.),
                    linear_color_stop(dark.background.opacity(0.74), 1.),
                )))
                .when(effects, |this| {
                    this.child(ui::constellation_light(
                        drift,
                        rim_position,
                        [
                            audio[0] * (1. - ambient_focus * 0.36),
                            audio[1] * (1. - ambient_focus * 0.28),
                        ],
                        [light, colors.edge[0], colors.edge[1]],
                        change,
                    ))
                }),
        )
        .child(
            div()
                .absolute()
                .left_0()
                .right_0()
                .top(viewport.height * 0.055)
                .flex()
                .flex_col()
                .items_center()
                .text_color(dark.foreground)
                .child(
                    div()
                        .text_size(rem * 0.92)
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(
                            ui::motion::mix(dark.foreground, clock_colors[0], 0.08).opacity(0.84),
                        )
                        .text_shadow(clock_reflections(
                            dark.foreground,
                            dark.background,
                            clock_colors[0],
                            0.72 + clock_contrast * 0.28,
                        ))
                        .child(now.strftime("%a %d %b").to_string()),
                )
                .child(glass_clock(
                    now.strftime("%H:%M").to_string(),
                    clock,
                    clock_colors,
                    clock_contrast,
                    drift,
                )),
        )
        .when_some(exit.filter(|_| departure > 0.), |this, exit| {
            this.when_some(exit.sky.as_ref(), |this, (group, animation, at)| {
                let outgoing_light = exit.colors.map_or(star_light, |colors| StarLight {
                    primary: colors.edge[0],
                    secondary: colors.edge[1],
                    foreground: dark.foreground,
                });
                this.child(
                    div()
                        .absolute()
                        .left((viewport.width - width) / 2.)
                        .top(
                            (viewport.height - height) / 2. + viewport.height * 0.11
                                - rem * (1. - departure) * 0.75,
                        )
                        .w(width)
                        .h(height)
                        .opacity(departure * 0.82)
                        .child(graph(
                            group,
                            *at,
                            size,
                            *animation,
                            outgoing_light,
                            rem,
                            true,
                        )),
                )
            })
        })
        .when_some(lyric_time.filter(|_| primary_opacity > 0.), |this, at| {
            this.child(timeline.sky.draw(at, frame, primary_opacity, screen))
        })
        .when_some(incoming_lyrics.filter(|_| blend > 0.), |this, at| {
            this.child(timeline.incoming_sky.draw(
                at,
                SkyFrame {
                    light: incoming_light,
                    song_handoff: song_handoff.is_some(),
                    ..frame
                },
                blend,
                screen,
            ))
        })
        .when_some(song_handoff, |this, handoff| {
            this.child(sky_layer(size, screen).child(handoff_glow(handoff, size, star_light, rem)))
        })
        .when_some(
            exit.filter(|exit| departure > 0. && exit.cover_focus > 0.),
            |this, exit| {
                this.child(album_card(
                    &exit.art,
                    "constellation-old-album",
                    screen,
                    album_side,
                    AlbumReveal {
                        focus: exit.cover_focus,
                        opacity: departure,
                        lift: -rem * (1. - departure) * 0.8,
                        pulse: 0.,
                        reflection: -10. * (1. - departure),
                    },
                    colors.clock[1],
                    rem,
                ))
            },
        )
        .when_some(art.filter(|_| cover_focus > 0.), |this, art| {
            this.child(album_card(
                art,
                "constellation-album",
                screen,
                album_side,
                AlbumReveal {
                    focus: cover_focus,
                    opacity: primary_opacity,
                    lift: rem * (1. - arrival) * 1.5,
                    pulse: energy * cover_focus,
                    reflection: drift.sin() * 16. + (1. - arrival) * 38.,
                },
                colors.clock[1],
                rem,
            ))
        })
        .when_some(
            incoming_art
                .as_ref()
                .filter(|_| incoming_focus > 0. && blend > 0.),
            |this, art| {
                this.child(album_card(
                    art,
                    "constellation-next-album",
                    screen,
                    album_side,
                    AlbumReveal {
                        focus: incoming_focus,
                        opacity: blend,
                        lift: rem * (1. - blend) * 0.9,
                        pulse: energy * incoming_focus,
                        reflection: drift.sin() * 16. + (1. - blend) * 24.,
                    },
                    colors.clock[1],
                    rem,
                ))
            },
        )
}

/// Frame data shared by the two lyric clocks during the overlap.
#[derive(Clone, Copy)]
struct SkyFrame {
    size: Point<Pixels>,
    drift: f32,
    audio: [f32; 2],
    energy: f32,
    motion: bool,
    light: StarLight,
    rem: Pixels,
    song_handoff: bool,
}

fn sky_layer(size: Point<Pixels>, screen: Point<Pixels>) -> Div {
    div()
        .absolute()
        .left((screen.x - size.x) / 2.)
        .top((screen.y - size.y) / 2. + screen.y * 0.11)
        .w(size.x)
        .h(size.y)
}

impl LyricSky {
    fn draw(&self, at: Duration, frame: SkyFrame, opacity: f32, screen: Point<Pixels>) -> Div {
        let SkyFrame {
            size,
            drift,
            audio,
            energy,
            motion,
            light: star_light,
            rem,
            song_handoff,
        } = frame;
        let groups = &self.groups;
        let next = groups.partition_point(|group| group.nodes[0].start <= at);
        let current = next.checked_sub(1);
        let vocal_index = self.vocals.partition_point(|span| span.start <= at);
        let in_vocals = vocal_index
            .checked_sub(1)
            .is_some_and(|index| at < self.vocals[index].end);
        let make_motion = |shape, age, enabled| GraphMotion {
            shape,
            age,
            enabled,
            drift,
            audio,
            energy,
        };
        let handoff = (motion && !song_handoff)
            .then(|| {
                [next, next.saturating_sub(1)]
                    .into_iter()
                    .find_map(|index| phrase_handoff(groups, index, at, drift, energy, star_light))
            })
            .flatten();
        sky_layer(size, screen)
            .opacity(opacity)
            .when_some(current, |this, shape| {
                let group = &groups[shape];
                let elapsed = at.saturating_sub(group.nodes[0].start);
                let age = if motion { elapsed.as_secs_f32() } else { 1. };
                let outgoing = shape.checked_sub(1).filter(|_| {
                    motion && in_vocals && !group.follows_break && elapsed < PHRASE_FADE
                });
                this.when_some(outgoing, |this, previous| {
                    let fade = 1. - ui::ease_in_out_cubic(progress(elapsed, PHRASE_FADE));
                    this.child(
                        graph(
                            &groups[previous],
                            group.nodes[0].start,
                            size,
                            make_motion(
                                previous,
                                at.saturating_sub(groups[previous].nodes[0].start)
                                    .as_secs_f32(),
                                false,
                            ),
                            star_light,
                            rem,
                            true,
                        )
                        .opacity(fade),
                    )
                })
                .when(in_vocals, |this| {
                    this.child(graph(
                        group,
                        at,
                        size,
                        make_motion(shape, age, motion),
                        star_light,
                        rem,
                        false,
                    ))
                })
                .when(motion && !in_vocals, |this| {
                    let Some((gap_age, return_time)) = self.instrumental_return(at) else {
                        return this;
                    };
                    let dissolve = ui::ease_in_out_cubic(progress(gap_age, return_time));
                    this.child(
                        graph(
                            group,
                            at,
                            size,
                            GraphMotion {
                                energy: energy * (1. - dissolve),
                                ..make_motion(shape, age, false)
                            },
                            star_light,
                            rem,
                            true,
                        )
                        .opacity(1. - dissolve)
                        .top(rem * dissolve * 0.8),
                    )
                })
            })
            .when_some(handoff, |this, handoff| {
                this.child(handoff_glow(handoff, size, star_light, rem))
            })
            .children(
                self.active_responses(at).filter_map(|response| {
                    response.lane.map(|lane| response.draw(at, frame, lane))
                }),
            )
    }
}

const RESPONSE_RELEASE: Duration = Duration::from_millis(220);

impl LyricResponse {
    fn draw(&self, at: Duration, frame: SkyFrame, row: usize) -> Div {
        let spoken = self.nodes.partition_point(|node| node.start <= at);
        let Some(active) = spoken.checked_sub(1) else {
            return div();
        };
        let release = if frame.motion {
            1. - ui::ease_in_out_cubic(progress(at.saturating_sub(self.span.end), RESPONSE_RELEASE))
        } else {
            f32::from(at < self.span.end)
        };
        let light = frame.light;
        let tint = ui::motion::mix(light.secondary, light.foreground, 0.72);
        let first = active.saturating_sub(3);
        let last = self.nodes.len().min(active + 3);
        let visible = last - first;
        let step = (frame.size.x * 0.46 / visible as f32).min(frame.rem * 4.4);
        let shift = frame.size.x * 0.16;
        let top = frame.size.y * 0.84 + frame.rem * (row as f32 * 2.15);
        let entry = if frame.motion {
            ui::ease_out_cubic(progress(at.saturating_sub(self.span.start), STAR_ATTACK))
        } else {
            1.
        };
        let sweep = frame
            .motion
            .then(|| edge_sweep(&self.nodes, active, at))
            .flatten();
        let stars = canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let y = bounds.top() + frame.rem * 0.38;
                let x = |index: usize| {
                    bounds.left() + step * (index.saturating_sub(first) as f32 + 0.5)
                };
                for index in first..active.min(last - 1) {
                    stroke(
                        &[point(x(index), y), point(x(index + 1), y)],
                        tint.opacity(0.38),
                        window,
                    );
                }
                if let Some(sweep) =
                    sweep.filter(|sweep| sweep.target > first && sweep.target < last)
                {
                    let from = x(sweep.target - 1);
                    let head = from + step * sweep.travel;
                    let tail = (head - step * 0.32).max(from);
                    let strength = 1. - sweep.settle;
                    line(
                        &[point(tail, y), point(head, y)],
                        frame.rem * 0.16,
                        tint.opacity(strength * 0.22),
                        window,
                    );
                    stroke(
                        &[point(tail, y), point(head, y)],
                        light.foreground.opacity(strength * 0.88),
                        window,
                    );
                }
            },
        )
        .absolute()
        .inset_0();
        div()
            .absolute()
            .left(if row.is_multiple_of(2) {
                shift * 2.
            } else {
                Pixels::ZERO
            })
            .right(if row.is_multiple_of(2) {
                Pixels::ZERO
            } else {
                shift * 2.
            })
            .top(top + frame.rem * (1. - entry) * 0.36)
            .flex()
            .justify_center()
            .opacity(release * (0.68 + 0.26 * entry))
            .child(
                div()
                    .relative()
                    .flex()
                    .items_end()
                    .w(step * visible as f32)
                    .child(stars)
                    .children(
                        self.nodes[first..last]
                            .iter()
                            .enumerate()
                            .map(|(index, node)| {
                                let index = first + index;
                                let current = active == index;
                                let bloom = if current && frame.motion {
                                    1. - progress(at.saturating_sub(node.start), STAR_BLOOM)
                                } else {
                                    0.
                                };
                                let color = if current { light.foreground } else { tint };
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .w(step)
                                    .gap_y_1()
                                    .opacity(if current {
                                        if frame.motion {
                                            0.35 + 0.65
                                                * ui::ease_out_cubic(progress(
                                                    at.saturating_sub(node.start),
                                                    Duration::from_millis(90),
                                                ))
                                        } else {
                                            1.
                                        }
                                    } else if index < active {
                                        if frame.motion {
                                            1. - 0.42
                                                * ui::ease_out_cubic(progress(
                                                    at.saturating_sub(node.end),
                                                    Duration::from_millis(160),
                                                ))
                                        } else {
                                            0.58
                                        }
                                    } else {
                                        0.32
                                    })
                                    .child(
                                        div()
                                            .h(frame.rem * 0.76)
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(frame.rem * (0.56 + 0.22 * bloom))
                                            .text_color(light.foreground)
                                            .when(current, |this| {
                                                this.text_shadow(lyric_shadows(
                                                    Theme::dark().background,
                                                    tint,
                                                ))
                                            })
                                            .child(if current { "✦" } else { "·" }),
                                    )
                                    .child(
                                        div()
                                            .font_family(node.font)
                                            .max_w_full()
                                            .truncate()
                                            .text_size(
                                                frame.rem * if current { 1.12 } else { 0.86 },
                                            )
                                            .text_color(color)
                                            .text_shadow(trail_shadows(Theme::dark().background))
                                            .child(node.text.clone()),
                                    )
                            }),
                    ),
            )
    }
}

fn scene_lyrics<'a>(
    track: Option<&Track>,
    position: Duration,
    cx: &'a App,
) -> (Option<Duration>, Option<&'a Arc<[LyricsLine]>>) {
    let Some(id) = track.and_then(|track| track.id.as_deref()) else {
        return (None, None);
    };
    let (source, time) = Sonora::global(cx)
        .lyrics
        .read(cx)
        .visual_lyrics(id, position);
    (time, source)
}

fn song_progress(position: Duration, duration: Duration) -> f32 {
    if duration.is_zero() {
        0.
    } else {
        progress(position, duration)
    }
}

fn cover_colors(
    art: Option<&SongArt>,
    track: Option<&Track>,
    screen: Point<Pixels>,
    cx: &App,
) -> (Option<ui::CoverPalette>, f32) {
    let palette = art
        .and_then(|art| art.cover.as_deref())
        .and_then(|cover| ui::cover_palette(cover, cx))
        .or_else(|| ui::cover_palette(track?.cover.as_deref()?, cx));
    let brightness = palette.map_or(0., |palette| {
        let aspect = f32::from(screen.x) / f32::from(screen.y).max(1.);
        let y = (0.15 + 0.055) / 1.08;
        palette.cover_brightness_at(aspect, y)
    });
    (
        palette,
        ui::ease_in_out_cubic(((brightness - 0.30) / 0.45).clamp(0., 1.)),
    )
}

fn scene_colors(
    palette: Option<ui::CoverPalette>,
    contrast: f32,
    dark: &Theme,
    cx: &App,
) -> SceneColors {
    let tint = palette
        .and_then(|palette| palette.primary)
        .or_else(|| cx.theme().tint);
    let glow = tint.map_or(dark.foreground, |tint| dark.primary_of(tint));
    let edge_color = |color: Hsla| Hsla {
        l: color.l.clamp(0.49, 0.69),
        s: (color.s * 1.16).min(1.),
        a: 1.,
        ..color
    };
    let neutral = Hsla {
        s: 0.,
        ..dark.foreground
    };
    let first = palette
        .map(|palette| palette.primary.unwrap_or(neutral))
        .or_else(|| cx.theme().tint)
        .unwrap_or(neutral);
    let second = palette
        .and_then(|palette| palette.secondary)
        .unwrap_or_else(|| Hsla {
            h: (first.h + 0.085).rem_euclid(1.),
            l: (first.l + 0.14).min(1.),
            ..first
        });
    SceneColors {
        glow,
        edge: [edge_color(first), edge_color(second)],
        clock: [
            edge_color(
                palette
                    .and_then(|palette| palette.primary)
                    .unwrap_or(neutral),
            ),
            edge_color(
                palette
                    .and_then(|palette| palette.secondary.or(palette.primary))
                    .unwrap_or(neutral),
            ),
        ],
        contrast,
    }
}

/// Crystal-sharp vivid artwork while lyrics are playing; smoothly blurs out during breaks.
fn cover_backdrop(
    cover: SharedString,
    id: &'static str,
    screen: Point<Pixels>,
    drift: f32,
    focus: f32,
    opacity: f32,
    effects: bool,
) -> Div {
    let blur = GLASS_BLUR * 1.4 * (COVER_BLUR_FLOOR + (1. - COVER_BLUR_FLOOR) * focus);
    div()
        .absolute()
        .left(-screen.x * (0.075 + drift.sin() * 0.045))
        .top(-screen.y * (0.075 + (drift * 0.76).cos() * 0.045))
        .opacity((0.46 - focus * 0.21) * opacity)
        .when(effects, |this| this.blur(blur))
        .child(
            Artwork::new(Some(cover))
                .id(id)
                .size(screen.x * 1.15)
                .wide(screen.y * 1.15)
                .corner_radius(Pixels::ZERO)
                .overlay(),
        )
}

/// The glass cover card follows its own lift and reflection as the song changes.
struct AlbumReveal {
    focus: f32,
    opacity: f32,
    lift: Pixels,
    pulse: f32,
    reflection: f32,
}

fn album_card(
    art: &SongArt,
    id: &'static str,
    screen: Point<Pixels>,
    album_side: Pixels,
    reveal: AlbumReveal,
    tint: Hsla,
    rem: Pixels,
) -> Div {
    let dark = Theme::dark();
    let side = album_side
        * (0.90 + reveal.focus * 0.10 + reveal.pulse * 0.045)
        * (0.93 + reveal.opacity * 0.07);
    let corner = dark.radius * 2.5;
    div()
        .absolute()
        .left_0()
        .right_0()
        .top(screen.y * 0.34 - rem * (1. - reveal.focus) + reveal.lift)
        .opacity(reveal.focus * reveal.opacity)
        .flex()
        .flex_col()
        .items_center()
        .gap_3()
        .px_8()
        .when_some(art.cover.clone(), |this, cover| {
            this.child(
                div()
                    .relative()
                    .size(side)
                    .rounded(corner)
                    .overflow_hidden()
                    .child(
                        Artwork::new(Some(cover))
                            .id(id)
                            .size(side)
                            .corner_radius(corner)
                            .overlay(),
                    )
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .rounded(corner)
                            .border_1()
                            .border_color(dark.foreground.opacity(0.24))
                            .bg(linear_gradient(
                                143. + reveal.reflection,
                                linear_color_stop(dark.foreground.opacity(0.18), 0.),
                                linear_color_stop(tint.opacity(0.02), 0.75),
                            )),
                    ),
            )
        })
        .child(
            div()
                .text_center()
                .font_family(lyric_font(&art.title))
                .text_size(rem * 0.92)
                .font_weight(FontWeight::MEDIUM)
                .text_color(dark.foreground.opacity(0.86))
                .text_shadow(trail_shadows(dark.background))
                .child(art.title.clone()),
        )
}

fn spoken_units(text: &str) -> f64 {
    let mut units = 0.0_f64;
    for grapheme in text.graphemes(true) {
        let Some(first) = grapheme.chars().next() else {
            continue;
        };
        if !first.is_alphanumeric() {
            continue;
        }
        let step = match first {
            '\u{0e30}' | '\u{0e32}' | '\u{0e33}' | '\u{0e40}'..='\u{0e46}' => 0.35,
            _ => 1.0,
        };
        units += step;
    }
    units
}

fn spoken_weight(text: &str) -> f64 {
    let units = spoken_units(text);
    if units <= 0.0 {
        return 1.0;
    }
    0.55 + units.powf(0.60)
}

fn max_spoken_duration(text: &str, tokens: &[Range<usize>]) -> Duration {
    let total_weight: f64 = tokens
        .iter()
        .map(|token| spoken_weight(&text[token.clone()]))
        .sum();
    let secs = (0.9 + total_weight * 0.34).clamp(1.6, 12.0);
    Duration::from_secs_f64(secs)
}

fn estimated_line_end(
    text: &str,
    tokens: &[Range<usize>],
    start: Duration,
    sung_end: Option<Duration>,
    next: Option<Duration>,
) -> Duration {
    if let Some(end) = sung_end.filter(|end| *end > start) {
        return end;
    }
    let max_span = max_spoken_duration(text, tokens);
    let Some(next) = next.filter(|next| *next > start) else {
        return start.saturating_add(max_span);
    };
    let gap = next.saturating_sub(start);
    if gap > max_span {
        return start.saturating_add(max_span);
    }
    let breath = if gap > Duration::from_millis(600) {
        (gap / 10).min(Duration::from_millis(200))
    } else {
        Duration::ZERO
    };
    start.saturating_add(gap.saturating_sub(breath))
}

struct SanitizedWord<'a> {
    text: &'a str,
    start: Duration,
    end: Duration,
    interp_end: Duration,
}

fn sanitize_words<'a>(
    words: &'a [music::LyricsWord],
    line_end: Duration,
) -> Vec<SanitizedWord<'a>> {
    let filtered: Vec<_> = words
        .iter()
        .filter_map(|word| {
            let trimmed = word.text.trim();
            (!trimmed.is_empty()).then_some((trimmed, word.start, word.end))
        })
        .collect();
    let mut prev_start = Duration::ZERO;
    let mut out = Vec::with_capacity(filtered.len());
    for (index, &(text, raw_start, raw_end)) in filtered.iter().enumerate() {
        let start = raw_start.max(prev_start);
        prev_start = start;
        let next_start = filtered.get(index + 1).map(|&(_, next, _)| next);
        let (end, interp_end) = if raw_end > start {
            let interp = next_start
                .filter(|next| *next > start)
                .map_or(raw_end, |next| raw_end.min(next));
            (raw_end, interp)
        } else {
            let inferred = next_start
                .filter(|next| *next > start)
                .unwrap_or_else(|| line_end.max(start));
            let max_span = max_spoken_duration(text, &lyric_tokens(text));
            let interp = inferred.min(start.saturating_add(max_span));
            (inferred, interp)
        };
        out.push(SanitizedWord {
            text,
            start,
            end,
            interp_end,
        });
    }
    out
}

fn timed_nodes(
    text: &str,
    words: Option<&[music::LyricsWord]>,
    from: Duration,
    end: Duration,
) -> Vec<LyricNode> {
    let tokens = lyric_tokens(text);
    if let Some(words) = words {
        let sanitized = sanitize_words(words, end);
        if !sanitized.is_empty() {
            if let Some(nodes) = join_timed_fragments(text, &tokens, &sanitized) {
                return nodes;
            }
            let nodes: Vec<_> = sanitized
                .iter()
                .flat_map(|word| {
                    let sub_tokens = lyric_tokens(word.text);
                    if sub_tokens.len() <= 1 {
                        vec![lyric_node(word.text, word.start, word.end)]
                    } else {
                        let mut sub =
                            estimated_nodes(word.text, &sub_tokens, word.start, word.interp_end);
                        if let Some(last) = sub.last_mut() {
                            last.end = last.end.max(word.end);
                        }
                        sub
                    }
                })
                .collect();
            if !nodes.is_empty() {
                return nodes;
            }
        }
    }
    estimated_nodes(text, &tokens, from, end)
}

/// Find written words in scripts with or without spaces. Keep punctuation beside its word.
fn lyric_tokens(text: &str) -> Vec<Range<usize>> {
    let mut tokens: Vec<Range<usize>> = Vec::new();
    let mut prefix = None;
    let mut start = 0;
    let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
    let mut breaks = segmenter.segment_str(text);
    while let Some(end) = breaks.next() {
        let segment = &text[start..end];
        if !segment.is_empty() {
            if breaks.is_word_like() {
                tokens.push(prefix.take().unwrap_or(start)..end);
            } else if segment.trim().is_empty() {
                prefix = None;
            } else if let Some(previous) = tokens.last_mut().filter(|token| token.end == start) {
                previous.end = end;
            } else {
                prefix.get_or_insert(start);
            }
        }
        start = end;
    }
    if tokens.is_empty() {
        let trimmed = text.trim_start();
        if !trimmed.trim_end().is_empty() {
            let start = text.len() - trimmed.len();
            tokens.push(start..start + trimmed.trim_end().len());
        }
    }
    tokens
}

/// Estimate onsets only when a source supplied the containing interval without word times.
fn estimated_nodes(
    text: &str,
    tokens: &[Range<usize>],
    start: Duration,
    end: Duration,
) -> Vec<LyricNode> {
    if tokens.is_empty() {
        return Vec::new();
    }
    let weights: Vec<f64> = tokens
        .iter()
        .map(|token| spoken_weight(&text[token.clone()]))
        .collect();
    let total: f64 = weights.iter().sum();
    let span = end.saturating_sub(start);
    let mut elapsed = 0.0;
    tokens
        .iter()
        .zip(weights)
        .map(|(token, weight)| {
            let onset = start + span.mul_f64((elapsed / total).clamp(0.0, 1.0));
            elapsed += weight;
            let finish = start + span.mul_f64((elapsed / total).clamp(0.0, 1.0));
            lyric_node(&text[token.clone()], onset, finish)
        })
        .collect()
}

fn lyric_node(text: &str, start: Duration, end: Duration) -> LyricNode {
    LyricNode {
        text: SharedString::from(text),
        font: lyric_font(text),
        start,
        end: end.max(start),
    }
}

fn find_fragment(haystack: &str, cursor: usize, fragment: &str) -> Option<(usize, usize)> {
    let rest = haystack.get(cursor..)?;
    if let Some(offset) = rest.find(fragment) {
        return Some((cursor + offset, cursor + offset + fragment.len()));
    }
    let frag_lower = fragment.to_lowercase();
    for (byte_idx, _) in rest.char_indices() {
        let slice = &rest[byte_idx..];
        let mut chars = slice.char_indices();
        let mut matched = String::new();
        while matched.len() < frag_lower.len() {
            let Some((_, ch)) = chars.next() else {
                break;
            };
            for lc in ch.to_lowercase() {
                matched.push(lc);
            }
        }
        if matched == frag_lower {
            let end_offset = chars.next().map_or(slice.len(), |(i, _)| i);
            return Some((cursor + byte_idx, cursor + byte_idx + end_offset));
        }
    }
    None
}

fn weighted_position(
    text: &str,
    range: &Range<usize>,
    from: Duration,
    to: Duration,
    at: usize,
) -> Duration {
    let at = at.clamp(range.start, range.end);
    if at <= range.start || to <= from {
        return from;
    }
    if at >= range.end {
        return to;
    }
    let total = spoken_units(&text[range.clone()]);
    if total <= 0.0 {
        return from;
    }
    let before = spoken_units(&text[range.start..at]);
    from + to
        .saturating_sub(from)
        .mul_f64((before / total).clamp(0.0, 1.0))
}

struct TimedSpan {
    range: Range<usize>,
    from: Duration,
    to: Duration,
    interp_to: Duration,
}

fn join_timed_fragments(
    text: &str,
    tokens: &[Range<usize>],
    words: &[SanitizedWord<'_>],
) -> Option<Vec<LyricNode>> {
    let mut cursor = 0;
    let mut spans = Vec::with_capacity(words.len());
    for word in words {
        let (start, end) = find_fragment(text, cursor, word.text)?;
        cursor = end;
        spans.push(TimedSpan {
            range: start..end,
            from: word.start,
            to: word.end,
            interp_to: word.interp_end,
        });
    }
    let mut nodes = Vec::with_capacity(tokens.len());
    let mut remaining = spans.as_slice();
    for token in tokens {
        remaining = &remaining[remaining.partition_point(|span| span.range.end <= token.start)..];
        let first = remaining.first()?;
        if first.range.start >= token.end {
            return None;
        }
        let start = if token.start <= first.range.start {
            first.from
        } else {
            weighted_position(text, &first.range, first.from, first.interp_to, token.start)
        };
        let finish = remaining
            .iter()
            .take_while(|span| span.range.start < token.end)
            .map(|span| {
                if token.end >= span.range.end {
                    span.to
                } else {
                    weighted_position(text, &span.range, span.from, span.interp_to, token.end)
                }
            })
            .max()?;
        nodes.push(lyric_node(&text[token.clone()], start, finish));
    }
    (!nodes.is_empty()).then_some(nodes)
}

/// Connect up to two nearby earlier stars with good angular separation to build crisp triangles.
fn cross_links(points: &[(f32, f32)], end: usize) -> [Option<usize>; 2] {
    let (x, y) = points[end];
    let (prev_x, prev_y) = points[end - 1];
    let incoming = (prev_x - x, prev_y - y);
    let incoming_len = incoming.0.hypot(incoming.1).max(0.001);

    let mut candidates: Vec<(usize, f32, (f32, f32))> = (end.saturating_sub(6)..end - 1)
        .filter_map(|start| {
            let (sx, sy) = points[start];
            let branch = (sx - x, sy - y);
            let len = branch.0.hypot(branch.1);
            let sin_angle =
                (incoming.0 * branch.1 - incoming.1 * branch.0).abs() / (incoming_len * len);
            (len >= 0.11 && len <= 0.42 && sin_angle >= 0.28).then_some((
                start,
                len / sin_angle.sqrt(),
                (branch.0 / len, branch.1 / len),
            ))
        })
        .collect();
    candidates.sort_by(|a, b| a.1.total_cmp(&b.1));

    let first = candidates.first().copied();
    let second = first.and_then(|(_, _, dir1)| {
        candidates
            .iter()
            .skip(1)
            .find(|(_, _, dir2)| (dir1.0 * dir2.1 - dir1.1 * dir2.0).abs() >= 0.26)
            .copied()
    });
    [
        first.map(|(start, _, _)| start),
        second.map(|(start, _, _)| start),
    ]
}

#[derive(Clone, Copy)]
struct EdgeSweep {
    target: usize,
    travel: f32,
    settle: f32,
}

fn edge_sweep(nodes: &[LyricNode], active: usize, at: Duration) -> Option<EdgeSweep> {
    [active, active + 1]
        .into_iter()
        .filter_map(|target| {
            let previous = nodes.get(target.checked_sub(1)?)?.start;
            let onset = nodes.get(target)?.start;
            let gap = onset.saturating_sub(previous);
            let lead = (gap / 2).min(Duration::from_millis(145));
            let settle = (gap / 4).min(Duration::from_millis(85));
            if lead.is_zero() || settle.is_zero() {
                return None;
            }
            let start = onset.saturating_sub(lead);
            let end = onset.saturating_add(settle);
            if at <= start || at >= end {
                return None;
            }
            Some(EdgeSweep {
                target,
                travel: (at.min(onset) - start).as_secs_f32() / lead.as_secs_f32(),
                settle: at.saturating_sub(onset).as_secs_f32() / settle.as_secs_f32(),
            })
        })
        .next_back()
}

/// Shares the rim's cover palette with the stars while keeping crisp pure-white geometry.
#[derive(Clone, Copy)]
struct StarLight {
    primary: Hsla,
    secondary: Hsla,
    foreground: Hsla,
}

impl StarLight {
    fn node(self, shape: usize, index: usize) -> (Hsla, Hsla) {
        if (shape % 3 + index) % 3 == 2 {
            (self.secondary, self.primary)
        } else {
            (self.primary, self.secondary)
        }
    }

    fn highlight(self, tint: Hsla) -> Hsla {
        ui::motion::mix(tint, self.foreground, 0.40)
    }
}

/// The playback time and sound levels that move one constellation.
#[derive(Clone, Copy)]
struct GraphMotion {
    shape: usize,
    age: f32,
    enabled: bool,
    drift: f32,
    audio: [f32; 2],
    energy: f32,
}

/// Convert stable group layouts to screen fractions with subtle floating drift.
fn star_positions(
    shape: usize,
    single: bool,
    age: f32,
    drift: Option<f32>,
    energy: f32,
) -> [(f32, f32); GROUP_LIMIT] {
    let variation = ((shape as u32).wrapping_mul(0x9e37_79b9).rotate_right(13) >> 24) as f32 / 255.;
    let spread = 0.92 + variation * 0.14;
    let direction = if shape.is_multiple_of(2) { 1. } else { -1. };
    let entry = 0.96 + ui::ease_out_cubic((age / 0.22).clamp(0., 1.)) * 0.04;
    let footprint = entry * (0.94 + energy * 0.14);
    let sway = 0.45 + energy * 0.35;
    GROUP_SHAPES[shape % GROUP_SHAPES.len()].map(|(x, y)| {
        let (x_float, y_float) = drift.map_or((0., 0.), |drift| {
            let float = drift * (1.8 + energy * 0.18) + x * TAU;
            (
                float.sin() * 0.008 * sway,
                (float * 1.6).sin() * 0.014 * sway,
            )
        });
        let (x, y) = if single {
            (0., -0.02)
        } else {
            (
                (x - 0.5) * spread * direction,
                (y - 0.5) * (0.92 + variation * 0.10),
            )
        };
        (0.5 + x * footprint + x_float, 0.5 + y * footprint + y_float)
    })
}

fn progress(elapsed: Duration, duration: Duration) -> f32 {
    if duration.is_zero() {
        1.
    } else {
        (elapsed.as_secs_f32() / duration.as_secs_f32()).min(1.)
    }
}

/// The brief light arc connecting the outgoing group to the next group or returning from the cover.
struct PhraseHandoff {
    from: (f32, f32),
    to: (f32, f32),
    from_tint: Hsla,
    to_tint: Hsla,
    travel: f32,
    fade: f32,
}

fn song_handoff(
    outgoing: &LyricSky,
    incoming: &LyricSky,
    outgoing_at: Duration,
    incoming_at: Duration,
    frame: SkyFrame,
    incoming_light: StarLight,
) -> Option<PhraseHandoff> {
    let first = incoming.groups.first()?;
    let onset = first.nodes[0].start;
    let start = onset.saturating_sub(SONG_HANDOFF_LEAD);
    if incoming_at < start || incoming_at >= onset.saturating_add(HANDOFF_TAIL) {
        return None;
    }
    let departure = outgoing_at.saturating_sub(incoming_at - start);
    let shape = outgoing
        .groups
        .partition_point(|group| group.nodes[0].start <= departure)
        .checked_sub(1)?;
    let group = &outgoing.groups[shape];
    let index = group.nodes.partition_point(|node| node.start <= departure) - 1;
    let position = |shape, group: &LyricGroup, at: Duration, index| {
        star_positions(
            shape,
            group.nodes.len() == 1,
            at.saturating_sub(group.nodes[0].start).as_secs_f32(),
            Some(frame.drift),
            frame.energy,
        )[index]
    };
    let elapsed = incoming_at.saturating_sub(onset);
    let travel = if onset.is_zero() {
        progress(incoming_at, SONG_HANDOFF_IMMEDIATE)
    } else {
        progress(incoming_at.min(onset) - start, onset - start)
    };
    Some(PhraseHandoff {
        from: position(shape, group, outgoing_at, index),
        to: position(0, first, incoming_at, 0),
        from_tint: frame.light.node(shape, index).0,
        to_tint: incoming_light.node(0, 0).0,
        travel: ui::ease_in_out_cubic(travel),
        fade: 1. - ui::ease_in_out_cubic(progress(elapsed, HANDOFF_TAIL)),
    })
}

fn phrase_handoff(
    groups: &[LyricGroup],
    index: usize,
    at: Duration,
    drift: f32,
    energy: f32,
    light: StarLight,
) -> Option<PhraseHandoff> {
    let group = groups.get(index)?;
    let onset = group.nodes[0].start;
    let previous = index.checked_sub(1).and_then(|previous| {
        let last = groups[previous].nodes.last()?;
        (!group.follows_break).then_some((previous, last.start))
    });
    let lead = previous.map_or(HANDOFF_LEAD, |(_, last)| {
        HANDOFF_LEAD.min(onset.saturating_sub(last) / 2)
    });
    let start = onset.saturating_sub(lead);
    let travel_time = onset - start;
    if at < start || at >= onset.saturating_add(HANDOFF_TAIL) || travel_time.is_zero() {
        return None;
    }
    let next_age = at.saturating_sub(onset);
    let to = star_positions(
        index,
        group.nodes.len() == 1,
        next_age.as_secs_f32(),
        Some(drift),
        energy,
    )[0];
    let previous = previous.map(|(previous, _)| (previous, groups[previous].nodes.len() - 1));
    let from = previous.map_or((0.50, 0.42), |(previous, last)| {
        let age = at
            .saturating_sub(groups[previous].nodes[0].start)
            .as_secs_f32();
        star_positions(previous, last == 0, age, Some(drift), energy)[last]
    });
    let travel = ui::ease_in_out_cubic(progress(at.min(onset) - start, travel_time));
    let fade = 1. - ui::ease_out_cubic(progress(next_age, HANDOFF_TAIL));
    Some(PhraseHandoff {
        from,
        to,
        from_tint: previous.map_or(light.secondary, |(shape, index)| light.node(shape, index).0),
        to_tint: light.node(index, 0).0,
        travel,
        fade,
    })
}

fn handoff_glow(
    handoff: PhraseHandoff,
    size: Point<Pixels>,
    light: StarLight,
    rem: Pixels,
) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |bounds, (), window, _| {
            let from = bounds.origin + point(size.x * handoff.from.0, size.y * handoff.from.1);
            let to = bounds.origin + point(size.x * handoff.to.0, size.y * handoff.to.1);
            let delta = to.relative_to(&from);
            let lift = (delta.x.abs() * 0.10 + delta.y.abs() * 0.05).clamp(rem * 0.45, rem * 2.0);
            let along = |t: f32| {
                from + delta * t - point(Pixels::ZERO, lift * (t * std::f32::consts::PI).sin())
            };
            let tint = ui::motion::mix(handoff.from_tint, handoff.to_tint, handoff.travel);
            let head = along(handoff.travel);
            let tail = handoff.travel.min(0.42);
            if tail > 0. {
                let points = std::array::from_fn::<_, 15, _>(|n| {
                    along(handoff.travel - tail * (1. - n as f32 / 14.))
                });
                for (index, segment) in points.windows(2).enumerate() {
                    let intensity = ((index + 1) as f32 / 14.).powi(2) * handoff.fade;
                    line(
                        segment,
                        STAR_TRAIL_WIDTH * 0.72,
                        tint.opacity(intensity * 0.28),
                        window,
                    );
                    stroke(
                        segment,
                        light.highlight(tint).opacity(intensity * 0.84),
                        window,
                    );
                }
            }
            let glow = handoff.fade * (handoff.travel * 7.).min(1.);
            if glow > 0. {
                star_halo(head, rem * 0.82, tint, glow, window);
                paint_star_node(head, rem * 0.30, tint, light.highlight(tint), glow, window);
            }
        },
    )
    .absolute()
    .size_full()
}

fn graph(
    group: &LyricGroup,
    at: Duration,
    size: Point<Pixels>,
    animation: GraphMotion,
    light: StarLight,
    rem: Pixels,
    outgoing: bool,
) -> Div {
    let Point {
        x: width,
        y: height,
    } = size;
    let GraphMotion {
        shape,
        age,
        enabled: motion,
        drift,
        audio: [bass, highs],
        energy,
    } = animation;
    let foreground = light.foreground;
    let nodes = &group.nodes;
    let Some(active) = nodes
        .partition_point(|node| node.start <= at)
        .checked_sub(1)
    else {
        return div();
    };
    let count = nodes.len();
    let links = group.links;
    let single = count == 1;
    let reveal = 0.65 + 0.35 * ui::ease_out_cubic((age / 0.16).clamp(0., 1.));
    let word_age = at.saturating_sub(nodes[active].start);
    let emphasis = if motion {
        ui::ease_out_cubic((word_age.as_secs_f32() / 0.085).clamp(0., 1.))
    } else {
        1.
    };
    let sweep = motion.then(|| edge_sweep(nodes, active, at)).flatten();
    let radius = rem * (0.86 + bass * 0.16 + energy * 0.10);
    let (tint, refraction) = light.node(shape, active);
    let bloom = if motion {
        let rise = (word_age.as_secs_f32() / STAR_ATTACK.as_secs_f32()).clamp(0., 1.);
        let decay = (word_age.saturating_sub(STAR_ATTACK).as_secs_f32()
            / (STAR_BLOOM - STAR_ATTACK).as_secs_f32())
        .clamp(0., 1.);
        ui::ease_out_cubic(rise) * (1. - ui::ease_out_cubic(decay))
    } else {
        0.
    };
    let scales = std::array::from_fn::<_, GROUP_LIMIT, _>(|index| {
        if single {
            1.
        } else {
            STAR_SCALES[(index + shape % STAR_SCALES.len()) % STAR_SCALES.len()]
        }
    });
    let positions = star_positions(
        shape,
        single,
        age,
        (motion || outgoing).then_some(drift),
        energy,
    );
    let held = nodes[active].end.saturating_sub(nodes[active].start);
    let sustain = if motion && held >= Duration::from_millis(650) && word_age < held {
        let rise = ui::ease_in_out_cubic(progress(word_age, STAR_BLOOM));
        let fall = ui::ease_in_out_cubic(progress(held - word_age, Duration::from_millis(190)));
        rise * fall
            * (0.36 + 0.24 * (word_age.as_secs_f32() * 8.).sin().powi(2))
            * (0.7 + energy * 0.5)
    } else {
        0.
    };
    let hero = |index: usize, depth: f32, opacity: f32| {
        let node = &nodes[index];
        let (x, y) = positions[index];
        let span = if single {
            width
        } else {
            (width * 0.56).max(rem * 4.).min(width)
        };
        let scale = if single { 2.55 } else { 2.65 };
        let type_size = (rem * scale).min(height * 0.23) * (0.72 + scales[index] * 0.28);
        div()
            .absolute()
            .w(span)
            .left(width * x - span / 2.)
            .top(height * y + rem * (1.68 - depth * 0.24))
            .text_center()
            .font_family(node.font)
            .text_size(type_size * (0.91 + depth * 0.09))
            .font_weight(FontWeight::BOLD)
            .text_color(foreground)
            .opacity(opacity)
            .text_shadow(lyric_shadows(
                Theme::dark().background,
                light.highlight(light.node(shape, index).0),
            ))
            .child(node.text.clone())
    };
    let current_word = hero(active, emphasis, 0.86 + emphasis * 0.14);
    let departing = active
        .checked_sub(1)
        .filter(|_| motion && word_age < LINK_REVEAL)
        .map(|index| {
            let fade = 1. - ui::ease_out_cubic(word_age.as_secs_f32() / LINK_REVEAL.as_secs_f32());
            hero(index, fade, fade * 0.50)
        });
    let link_progress = std::array::from_fn::<_, GROUP_LIMIT, _>(|index| {
        if motion && index <= active {
            let elapsed = at.saturating_sub(nodes[index].start);
            ui::ease_out_cubic((elapsed.as_secs_f32() / LINK_REVEAL.as_secs_f32()).min(1.))
        } else {
            1.
        }
    });
    div()
        .absolute()
        .left_0()
        .top_0()
        .w(width)
        .h(height)
        .child(
            canvas(
                |_, _, _| {},
                move |bounds, (), window, _| {
                    let dots = positions.map(|(x, y)| bounds.origin + point(width * x, height * y));
                    let completed = sweep.map_or(active, |sweep| sweep.target - 1);
                    let mesh_white = foreground;
                    // Primary constellation polyline (crisp white wireframe like in the video)
                    if completed > 0 {
                        line(
                            &dots[..=completed],
                            px(1.15),
                            mesh_white.opacity(0.62 * reveal),
                            window,
                        );
                    }
                    // Triangulating cross-edges forming geometric constellation facets
                    for (end, pair) in links.into_iter().enumerate().take(active + 1) {
                        let progress = link_progress[end];
                        if progress == 0. {
                            continue;
                        }
                        for (slot, start) in pair.into_iter().enumerate() {
                            let Some(start) = start else { continue };
                            let tip = dots[end] + dots[start].relative_to(&dots[end]) * progress;
                            let base_alpha = if slot == 0 { 0.42 } else { 0.25 };
                            line(
                                &[dots[end], tip],
                                px(0.95),
                                mesh_white.opacity(reveal * base_alpha * progress),
                                window,
                            );
                        }
                    }
                    // Inactive star pinpoints
                    for (index, dot) in dots.into_iter().take(active + 1).enumerate() {
                        if index != active {
                            let (node_tint, _) = light.node(shape, index);
                            paint_star_node(
                                dot,
                                rem * 0.24 * scales[index],
                                node_tint,
                                light.highlight(node_tint),
                                reveal * 0.92,
                                window,
                            );
                        }
                    }
                    // Fading ring left behind on the previous active star
                    if motion && active > 0 && emphasis < 0.96 {
                        let prev_r = radius * 0.85 * scales[active - 1];
                        arc(
                            dots[active - 1],
                            prev_r,
                            0.,
                            TAU,
                            mesh_white.opacity((1. - emphasis) * 0.45),
                            window,
                        );
                    }
                    // Snappy comet sweep arriving right on the word onset
                    if let Some(sweep) = sweep {
                        let from = dots[sweep.target - 1];
                        let to = dots[sweep.target];
                        let from_tint = light.node(shape, sweep.target - 1).0;
                        let to_tint = light.node(shape, sweep.target).0;
                        let delta = to.relative_to(&from);
                        let along = |t| from + delta * t;
                        let head = ui::ease_in_out_cubic(sweep.travel);
                        let marker = along(head);
                        let span = head.min(0.36 + bass * 0.10);
                        let strength = 1. - ui::ease_out_cubic(sweep.settle);
                        let trail = ui::motion::mix(from_tint, to_tint, head);
                        line(
                            &[from, marker],
                            px(1.15),
                            mesh_white.opacity(0.62 * reveal),
                            window,
                        );
                        for step in 0..8 {
                            let a = step as f32 / 8.;
                            let b = (step + 1) as f32 / 8.;
                            let segment =
                                [along(head - span * (1. - a)), along(head - span * (1. - b))];
                            let flare = b * b * strength;
                            line(
                                &segment,
                                STAR_TRAIL_WIDTH * (1. + bass * 0.35),
                                trail.opacity(flare * 0.26),
                                window,
                            );
                            line(&segment, px(1.6), mesh_white.opacity(flare * 0.96), window);
                        }
                        if sweep.target > active {
                            let spark = rem * (0.14 + head * 0.18) * scales[sweep.target];
                            paint_star_node(
                                to,
                                spark,
                                to_tint,
                                light.highlight(to_tint),
                                head * 0.85,
                                window,
                            );
                        }
                        let head_light = strength * (head * 5.).min(1.);
                        star_halo(
                            marker,
                            radius * (0.65 + highs * 0.25),
                            trail,
                            head_light * 0.72,
                            window,
                        );
                        paint_star_node(
                            marker,
                            rem * (0.18 + highs * 0.10),
                            trail,
                            light.highlight(trail),
                            head_light,
                            window,
                        );
                    }
                    // Active reticle starburst (crisp white ring + sharp 4-point needle flare)
                    let radius = radius * scales[active];
                    let intensity = (0.90 + 0.10 * highs + 0.14 * bloom + sustain * 0.16).min(1.15);
                    if bloom > 0. {
                        let spread = ui::ease_out_cubic(
                            (word_age.as_secs_f32() / STAR_BLOOM.as_secs_f32()).clamp(0., 1.),
                        );
                        star_glints(
                            dots[active],
                            radius,
                            spread,
                            bloom * intensity,
                            refraction,
                            light.highlight(refraction),
                            window,
                        );
                    }
                    if sustain > 0.02 {
                        let spin = word_age.as_secs_f32() * 1.9;
                        arc(
                            dots[active],
                            radius * 1.38,
                            spin,
                            TAU * 0.32,
                            mesh_white.opacity(sustain * 0.75),
                            window,
                        );
                        arc(
                            dots[active],
                            radius * 1.38,
                            spin + TAU * 0.5,
                            TAU * 0.22,
                            tint.opacity(sustain * 0.60),
                            window,
                        );
                    }
                    let tilt = if motion {
                        0.10 * (drift * 1.6).sin()
                    } else {
                        0.
                    };
                    star_flare(
                        dots[active],
                        radius * (0.95 + bloom * 0.16),
                        [tint, refraction, mesh_white],
                        tilt,
                        intensity,
                        window,
                    );
                },
            )
            .absolute()
            .size_full(),
        )
        .children(
            nodes
                .iter()
                .enumerate()
                .take(active + 1)
                .map(|(index, node)| {
                    let selected = active == index;
                    let (x, y) = positions[index];
                    let span = if single {
                        width
                    } else {
                        (width * 0.24).max(rem * 3.).min(width)
                    };
                    let label = div()
                        .absolute()
                        .w(span)
                        .left(if single {
                            Pixels::ZERO
                        } else {
                            width * x - span / 2.
                        })
                        .top(height * y + rem * (0.38 + scales[index] * 0.12))
                        .text_center()
                        .font_family(node.font)
                        .text_size(rem * (0.58 + scales[index] * 0.26))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(foreground.opacity(reveal * 0.78))
                        .text_shadow(trail_shadows(Theme::dark().background))
                        .when(selected, |this| this.opacity(0.))
                        .child(node.text.clone());
                    if outgoing {
                        label.into_any_element()
                    } else {
                        label
                            .id(("constellation-word", index))
                            .role(Role::Label)
                            .aria_label(node.text.clone())
                            .into_any_element()
                    }
                }),
        )
        .children(departing)
        .child(current_word)
}

fn lyric_font(text: &str) -> &'static str {
    if text
        .chars()
        .any(|ch| ('\u{0e01}'..='\u{0e5b}').contains(&ch))
    {
        THAI_FONT
    } else {
        FONT
    }
}

/// Crisp drop shadow under active lyrics so bold white text pops over vivid artwork.
fn lyric_shadows(shade: Hsla, tint: Hsla) -> [TextShadow; 3] {
    let edge = WORD_GLOW / 7.;
    [
        TextShadow {
            color: shade.opacity(0.88),
            offset: point(Pixels::ZERO, edge * 2.),
            blur: WORD_GLOW,
        },
        TextShadow {
            color: tint.opacity(0.41),
            offset: Point::default(),
            blur: WORD_GLOW * 1.8,
        },
        TextShadow {
            color: tint.opacity(0.62),
            offset: Point::default(),
            blur: WORD_GLOW * 0.62,
        },
    ]
}

fn trail_shadows(shade: Hsla) -> [TextShadow; 2] {
    [
        TextShadow {
            color: shade.opacity(0.92),
            offset: point(Pixels::ZERO, px(1.5)),
            blur: px(5.),
        },
        TextShadow {
            color: shade.opacity(0.65),
            offset: Point::default(),
            blur: px(2.5),
        },
    ]
}

/// Reflected light along the top edge and a soft shadow below give the translucent glyph depth.
fn clock_reflections(white: Hsla, shade: Hsla, light: Hsla, strength: f32) -> [TextShadow; 3] {
    [
        TextShadow {
            color: shade.opacity(0.45 * strength),
            offset: point(Pixels::ZERO, CLOCK_BEVEL * 2.2),
            blur: WORD_GLOW * 0.65,
        },
        TextShadow {
            color: light.opacity(0.16 * strength),
            offset: point(CLOCK_BEVEL, Pixels::ZERO),
            blur: CLOCK_BEVEL * 2.,
        },
        TextShadow {
            color: white.opacity(0.35 * strength),
            offset: point(Pixels::ZERO, -CLOCK_BEVEL),
            blur: CLOCK_BEVEL,
        },
    ]
}

struct ClockOutline<'a> {
    path: &'a mut PathBuilder,
    origin: Point<Pixels>,
    scale: f32,
}

impl ClockOutline<'_> {
    fn at(&self, x: f32, y: f32) -> Point<Pixels> {
        self.origin + point(px(x * self.scale * 0.85), px(-y * self.scale))
    }
}

impl ttf_parser::OutlineBuilder for ClockOutline<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to(self.at(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(self.at(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.curve_to(self.at(x, y), self.at(x1, y1));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path
            .cubic_bezier_to(self.at(x, y), self.at(x1, y1), self.at(x2, y2));
    }

    fn close(&mut self) {
        self.path.close();
    }
}

/// Paint sleek frosted-glass digits at the top center like in the reference video.
fn glass_clock(
    time: String,
    height: Pixels,
    [primary, secondary]: [Hsla; 2],
    contrast: f32,
    drift: f32,
) -> Div {
    let dark = Theme::dark();
    let pearl = ui::motion::mix(dark.foreground, primary, 0.37);
    let glass = ui::motion::mix(dark.foreground, secondary, 0.52);
    let shade = Hsla {
        l: 0.27 - contrast * 0.15,
        ..glass
    };
    let angle = 149. + (drift * 0.55).sin() * 13.;
    let fallback = || {
        div()
            .text_size(height)
            .text_color(pearl.opacity(0.68))
            .child(time.clone())
    };
    let Ok(mut face) =
        ttf_parser::Face::parse(include_bytes!("../../../../assets/fonts/NotoSans.ttf"), 0)
    else {
        return fallback();
    };
    face.set_variation(ttf_parser::Tag::from_bytes(b"wght"), 560.);
    let glyphs = time
        .chars()
        .filter_map(|ch| {
            let id = face.glyph_index(ch)?;
            Some((id, face.glyph_hor_advance(id)? as f32))
        })
        .collect::<Vec<_>>();
    let Some(cap) = face
        .glyph_index('0')
        .and_then(|id| face.glyph_bounding_box(id))
    else {
        return fallback();
    };
    let scale = f32::from(height) / f32::from(cap.height());
    let width = px(glyphs.iter().map(|(_, advance)| advance).sum::<f32>() * scale * 0.85);
    div().child(
        div()
            .id("constellation-clock")
            .role(Role::Label)
            .aria_label(time)
            .pt_1()
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, (), window, _| {
                        let baseline =
                            bounds.origin + point(Pixels::ZERO, px(cap.y_max as f32 * scale));
                        let outline = |mut path: PathBuilder, offset| {
                            let mut origin = baseline + offset;
                            for &(glyph, advance) in &glyphs {
                                face.outline_glyph(
                                    glyph,
                                    &mut ClockOutline {
                                        path: &mut path,
                                        origin,
                                        scale,
                                    },
                                );
                                origin.x += px(advance * scale * 0.85);
                            }
                            path.build()
                        };
                        for (width, opacity) in [(6.5, 0.12), (3.2, 0.24)] {
                            if let Ok(path) = outline(
                                PathBuilder::stroke(CLOCK_BEVEL * width),
                                point(Pixels::ZERO, CLOCK_BEVEL * 2.0),
                            ) {
                                window.paint_path(
                                    path,
                                    dark.background.opacity((0.35 + contrast * 0.65) * opacity),
                                );
                            }
                        }
                        if let Ok(path) = outline(PathBuilder::fill(), Point::default()) {
                            window.paint_path(
                                path,
                                linear_gradient(
                                    angle,
                                    linear_color_stop(pearl.opacity(0.17 + contrast * 0.10), 0.),
                                    linear_color_stop(glass.opacity(0.20 + contrast * 0.09), 1.),
                                ),
                            );
                        }
                        for (width, strength) in
                            [(8., 0.038), (5., 0.062), (2.8, 0.105), (1.3, 0.20)]
                        {
                            if let Ok(path) =
                                outline(PathBuilder::stroke(CLOCK_BEVEL * width), Point::default())
                            {
                                window.paint_path(
                                    path,
                                    linear_gradient(
                                        angle - 8.,
                                        linear_color_stop(
                                            pearl.opacity(strength * (1. + contrast * 0.25)),
                                            0.,
                                        ),
                                        linear_color_stop(
                                            shade.opacity(strength * (0.45 + contrast * 0.20)),
                                            1.,
                                        ),
                                    ),
                                );
                            }
                        }
                    },
                )
                .w(width)
                .h(height),
            ),
    )
}

/// Crisp expanding white shockwave ring when a new word lands on the beat.
/// Send a soft ring and tiny 4-point star glints out as the sung word arrives.
fn star_glints(
    center: Point<Pixels>,
    radius: Pixels,
    spread: f32,
    intensity: f32,
    tint: Hsla,
    core: Hsla,
    window: &mut Window,
) {
    let ring = radius * (1.18 + spread * 1.15);
    arc(
        center,
        ring,
        0.,
        TAU,
        tint.opacity(intensity * 0.42),
        window,
    );
    for (angle, reach, size) in [
        (-1.08_f32, 1., 0.22),
        (-2.77, 0.80, 0.16),
        (0.38, 1.14, 0.19),
    ] {
        let distance = ring * reach;
        let position = center + point(distance * angle.cos(), distance * angle.sin());
        paint_star_node(position, radius * size, tint, core, intensity, window);
    }
}

/// Crisp circular reticle ring sliced by an upright 4-point needle star,
/// layered over the original 7-layer cover-tinted halo.
fn star_flare(
    center: Point<Pixels>,
    radius: Pixels,
    [tint, refraction, core]: [Hsla; 3],
    tilt: f32,
    intensity: f32,
    window: &mut Window,
) {
    star_halo(center, radius * 2.65, tint, intensity * 0.88, window);

    let ring_radius = radius * 1.06;
    let ring = std::array::from_fn::<_, 41, _>(|step| {
        let angle = step as f32 * TAU / 40.;
        center + point(ring_radius * angle.cos(), ring_radius * angle.sin())
    });
    for (width, color, alpha) in [
        (0.28, refraction, 0.14),
        (0.15, refraction, 0.28),
        (0.082, core, 0.90),
    ] {
        line(
            &ring,
            radius * width,
            color.opacity((alpha * intensity).min(1.)),
            window,
        );
    }

    for axis in 0..2 {
        let angle = tilt + axis as f32 * TAU / 4.;
        let length = radius * 1.56;
        let tip = point(length * angle.cos(), length * angle.sin());
        for (reach, width, color, alpha) in [
            (1.0, 0.22, tint, 0.24),
            (0.92, 0.13, tint, 0.48),
            (0.84, 0.078, core, 0.78),
            (0.68, 0.042, core, 0.98),
        ] {
            let width = radius * width;
            let normal = point(-width * angle.sin(), width * angle.cos());
            let mut path = PathBuilder::fill();
            path.add_polygon(
                &[
                    center - tip * reach,
                    center + normal,
                    center + tip * reach,
                    center - normal,
                ],
                true,
            );
            if let Ok(path) = path.build() {
                window.paint_path(path, color.opacity((alpha * intensity).min(1.)));
            }
        }
    }

    star_halo(center, radius * 0.40, core, intensity, window);
}

/// Crisp slender 4-point concave star (`✦`) rendered on every node in the constellation.
fn paint_star_node(
    dot: Point<Pixels>,
    size: Pixels,
    tint: Hsla,
    core: Hsla,
    opacity: f32,
    window: &mut Window,
) {
    star_halo(dot, size * 1.45, tint, opacity * 0.38, window);
    for (scale, inner_ratio, color, strength) in [(1.0, 0.18, tint, 0.52), (0.88, 0.14, core, 0.96)]
    {
        let r = size * scale;
        let inner = r * inner_ratio;
        let points = [
            dot + point(Pixels::ZERO, -r),
            dot + point(inner, -inner),
            dot + point(r, Pixels::ZERO),
            dot + point(inner, inner),
            dot + point(Pixels::ZERO, r),
            dot + point(-inner, inner),
            dot + point(-r, Pixels::ZERO),
            dot + point(-inner, -inner),
        ];
        let mut star = PathBuilder::fill();
        star.add_polygon(&points, true);
        if let Ok(path) = star.build() {
            window.paint_path(path, color.opacity((opacity * strength).min(1.)));
        }
    }
}

/// Expanding translucent disks feather the star without blurring its sharp center.
fn star_halo(center: Point<Pixels>, radius: Pixels, tint: Hsla, opacity: f32, window: &mut Window) {
    for step in (1..=HALO_STEPS).rev() {
        let distance = step as f32 / HALO_STEPS as f32;
        let ring = radius * distance;
        let points = std::array::from_fn::<_, HALO_POINTS, _>(|index| {
            let angle = index as f32 * TAU / HALO_POINTS as f32;
            center + point(ring * angle.cos(), ring * angle.sin())
        });
        let mut path = PathBuilder::fill();
        path.add_polygon(&points, true);
        if let Ok(path) = path.build() {
            window.paint_path(
                path,
                tint.opacity(opacity * (1. - distance * 0.9) / HALO_STEPS as f32),
            );
        }
    }
}

fn arc(
    center: Point<Pixels>,
    radius: Pixels,
    start: f32,
    sweep: f32,
    color: Hsla,
    window: &mut Window,
) {
    let points = std::array::from_fn::<_, 29, _>(|step| {
        let angle = start + sweep * step as f32 / 28.;
        center + point(radius * angle.cos(), radius * angle.sin())
    });
    stroke(&points, color, window);
}

fn stroke(points: &[Point<Pixels>], color: Hsla, window: &mut Window) {
    line(points, px(1.1), color, window);
}

fn line(points: &[Point<Pixels>], width: Pixels, color: Hsla, window: &mut Window) {
    let mut path = PathBuilder::stroke(width);
    path.add_polygon(points, false);
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
}
