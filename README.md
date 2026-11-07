<div align="center">

<img src="./.github/readme-icon.svg" alt="" width="60" height="60">

<h1>SonoraX</h1>

<p align="center">
  <a href="https://sonorahq.org/changelog">Changelog</a>
  ·
  <a href="https://sonorahq.org/docs">Docs</a>
  ·
  <a href="https://sonorahq.org/docs/roadmap">Roadmap</a>
  ·
  <a href="https://sonorahq.org/faq">FAQ</a>
</p>

[![Build](https://img.shields.io/github/actions/workflow/status/RoxyZ7/SonoraX/release.yml?style=flat-square&label=build)](https://github.com/RoxyZ7/SonoraX/actions/workflows/release.yml)
[![License](https://img.shields.io/github/license/RoxyZ7/SonoraX?style=flat-square&label=license&cacheSeconds=3600)](./COPYING)

### A Thai-localized fork of Sonora

Stream from your favorite services and play local files — all in one **native** app.
SonoraX is an unofficial community fork with its own public source, Windows, Linux and macOS releases, and automatic updates from this repository.
</div>

<div align="center">
    <table>
      <tr>
        <td colspan="2">
          <img width="1556" height="967" alt="image" src="https://github.com/user-attachments/assets/c797cb19-9631-45b0-91b6-695f28f4143d" />
        </td>
      </tr>
      <tr>
        <td width="50%">
          <img width="1555" height="966" alt="image" src="https://github.com/user-attachments/assets/b556bcd0-2823-485f-83f3-41e1099fdc45" />
        </td>
        <td width="50%">
          <img width="1555" height="966" alt="image" src="https://github.com/user-attachments/assets/f27400ab-9264-4d82-9a70-e0a60e0c3a5f" />
        </td>
      </tr>
    </table>
</div>
<div align="center">
    <sub>
      Adaptive themes are optional. Everything is (or will be) customizable.
    </sub>
</div>

> [!IMPORTANT]
> **SonoraX is not a piracy tool.**
>
> SonoraX is not a platform for obtaining or sharing copyrighted material. We will not implement any functions that can be used to export decrypted streams, DRM licenses, content keys, or to convert protected streams into media files.
>
> SonoraX is not designed to circumvent subscriptions or other restrictions put in place by music streaming platforms. If the service demands that you have a valid subscription in order to play back their tracks, so will SonoraX.
>
> Features aimed at ripping, downloading, distributing, or gaining access to protected streaming content are out of scope for the project.

## Installation

SonoraX is based on [Sonora](https://github.com/sonorahq/sonora), an open-source music player. Please see the upstream project for its original contributors and documentation.

## Download

All SonoraX installers and builds are published on [SonoraX Releases](https://github.com/RoxyZ7/SonoraX/releases/latest). SonoraX checks for a newer release at startup and installs it automatically on supported self-updating installs. If an update is published while SonoraX is running, it shows an update notice.

### Windows

Download and run `SonoraX-Setup.exe` (or `SonoraX-Setup-arm64.exe` on Windows on ARM). Portable executables are also available for both architectures.

### macOS

Download the macOS disk image from [SonoraX Releases](https://github.com/RoxyZ7/SonoraX/releases/latest), open it and move SonoraX to Applications.

### Linux

Download the AppImage or Flatpak bundle for your architecture. Make an AppImage executable before running it:

```sh
chmod +x SonoraX-*.AppImage
./SonoraX-*.AppImage
```

### Nightly

Preview builds for Windows, Linux and macOS are published at [SonoraX Nightly](https://github.com/RoxyZ7/SonoraX/releases/tag/nightly). Nightly builds update from the nightly channel.
## Community

Feel free to join our [Discord](https://discord.gg/a8N8Tx23rV) server and [Matrix](https://matrix.to/#/#sonora:nolight.dev) space.
Discord is the primary one, but we do have a Matrix bridge.

## Translations

<!-- i18n:start -->

| Language | Translated | Coverage |
| --- | --- | --- |
| English (`en-US`) | 1235/1235 | 100% |
| Deutsch (`de`) | 1105/1235 | 89% |
| Español (`es`) | 1105/1235 | 89% |
| Français (`fr`) | 1106/1235 | 90% |
| Italiano (`it`) | 1105/1235 | 89% |
| Bahasa Indonesia (`id`) | 1105/1235 | 89% |
| 日本語 (`ja`) | 1105/1235 | 89% |
| Русский (`ru`) | 1151/1235 | 93% |
| Українська (`uk`) | 1107/1235 | 90% |
| Polski (`pl`) | 1107/1235 | 90% |
| Čeština (`cs`) | 1105/1235 | 89% |
| Português (Brasil) (`pt-BR`) | 1105/1235 | 89% |
| 简体中文 (`zh-CN`) | 1105/1235 | 89% |
| Türkçe (`tr`) | 1105/1235 | 89% |
| Shqip (`sq`) | 1105/1235 | 89% |
| ภาษาไทย (`th`) | 1235/1235 | 100% |

<!-- i18n:end -->

## Star History

<a href="https://www.star-history.com/?repos=roxyz7%2Fsonorax&type=date&logscale=&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=roxyz7/sonorax&type=date&theme=dark&logscale&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=roxyz7/sonorax&type=date&logscale&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=roxyz7/sonorax&type=date&logscale&legend=top-left" />
 </picture>
</a>

## Credits

SonoraX is built on Sonora and uses the help of some incredible open-source projects, including:

- [Zed](https://github.com/zed-industries/zed) — a wonderful editor (~~ab~~)used by all core team members. Conveniently provides `gpui` — their native Rust rendering stack.
- [librespot](https://github.com/librespot-org/librespot) — Spotify playback and library integration.
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) — certain YouTube ideas implemented in [ytmusic-rs](https://github.com/sonorahq/ytmusic-rs) :)

## Code signing policy

Free code signing provided by [SignPath.io](https://signpath.io), certificate by
[SignPath Foundation](https://signpath.org).

The Windows executable and installer are built by GitHub Actions from this repository and signed
through SignPath.

- Reviewers: members of sonorahq
- Approvers: [@nolight132](https://github.com/nolight132)

## License

Copyright (C) 2026 Sonora Contributors.

SonoraX is free software, released under the [GNU General Public License version
3 or later](COPYING).

SonoraX is an unofficial client and is not affiliated with, endorsed by, or
sponsored by Spotify AB.

The binary also embeds the [Inter](https://github.com/rsms/inter) typeface (SIL
Open Font License 1.1) and four interchangeable icon sets:
[Lucide](https://lucide.dev) (ISC), [Iconoir](https://iconoir.com) (MIT),
[Remix Icon](https://remixicon.com) 4.8.0 (Apache 2.0) and the
[Solar](https://www.figma.com/community/file/1166831539721848736) Linear set
(CC BY 4.0, by 480 Design). Each pack keeps its licence beside its files in
`assets/icons`. `THIRD-PARTY.md` lists every bundled dependency.
