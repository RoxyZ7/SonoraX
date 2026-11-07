#!/usr/bin/env python3

import shutil
import struct
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MASTER = ROOT / "assets" / "icon.svg"

LINUX_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]
WINDOWS_SIZES = [16, 24, 32, 48, 64, 128, 256]
TRAY_SIZES = [32, 64]
ICNS_TYPES = [
    ("icp4", 16),
    ("icp5", 32),
    ("ic11", 32),
    ("ic12", 64),
    ("ic07", 128),
    ("ic13", 256),
    ("ic08", 256),
    ("ic14", 512),
    ("ic09", 512),
    ("ic10", 1024),
]

class Master:
    def __init__(self, path):
        if not path.exists():
            sys.exit(f"icons: {path} is missing")

        self.source = path.read_text(encoding="utf-8")
        svg = ET.fromstring(self.source)
        box = svg.get("viewBox")
        if box is None:
            sys.exit("icons: master has no viewBox")

        _, _, width, height = (float(n) for n in box.split())
        if width != height:
            sys.exit("icons: master viewBox is not square")
        self.span = width


def rasterize(svg_text, pixels, workdir, name):
    source = workdir / f"{name}.svg"
    target = workdir / f"{name}.png"
    source.write_text(svg_text, encoding="utf-8")
    subprocess.run(
        [
            "rsvg-convert",
            "--width", str(pixels),
            "--height", str(pixels),
            "--keep-aspect-ratio",
            "--background-color", "none",
            "--output", str(target),
            str(source),
        ],
        check=True,
    )
    return target.read_bytes()


def write_ico(entries, path):
    header = struct.pack("<HHH", 0, 1, len(entries))
    offset = len(header) + 16 * len(entries)
    directory = b""
    payload = b""
    for pixels, blob in entries:
        directory += struct.pack(
            "<BBBBHHII",
            0 if pixels >= 256 else pixels,
            0 if pixels >= 256 else pixels,
            0,
            0,
            1,
            32,
            len(blob),
            offset,
        )
        payload += blob
        offset += len(blob)
    path.write_bytes(header + directory + payload)


def write_icns(entries, path):
    body = b""
    for kind, blob in entries:
        body += kind.encode("ascii") + struct.pack(">I", len(blob) + 8) + blob
    path.write_bytes(b"icns" + struct.pack(">I", len(body) + 8) + body)


def main():
    if not shutil.which("rsvg-convert"):
        sys.exit("icons: rsvg-convert missing, run under `nix shell nixpkgs#librsvg`")

    master = Master(MASTER)
    assets = ROOT / "assets"

    with tempfile.TemporaryDirectory() as tmp:
        workdir = Path(tmp)

        for destination in (
            ROOT / ".github" / "readme-icon.svg",
            assets / "icons" / "common" / "sonora.svg",
            assets / "linux" / "sonora.svg",
            assets / "macos" / "sonora.svg",
            assets / "windows" / "sonora.svg",
        ):
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(master.source, encoding="utf-8")

        for pixels in LINUX_SIZES:
            blob = rasterize(master.source, pixels, workdir, f"l{pixels}")
            target = (
                assets
                / "linux"
                / "icons"
                / "hicolor"
                / f"{pixels}x{pixels}"
                / "apps"
                / "sonora.png"
            )
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(blob)

        icns = []
        rendered = {}
        for kind, pixels in ICNS_TYPES:
            if pixels not in rendered:
                rendered[pixels] = rasterize(master.source, pixels, workdir, f"m{pixels}")
            icns.append((kind, rendered[pixels]))
        write_icns(icns, assets / "macos" / "sonora.icns")

        ico = [
            (
                pixels,
                rasterize(master.source, pixels, workdir, f"w{pixels}"),
            )
            for pixels in WINDOWS_SIZES
        ]
        windows = assets / "windows"
        windows.mkdir(parents=True, exist_ok=True)
        write_ico(ico, windows / "sonora.ico")

        tray = assets / "tray"
        tray.mkdir(parents=True, exist_ok=True)
        for pixels in TRAY_SIZES:
            blob = rasterize(master.source, pixels, workdir, f"t{pixels}")
            (tray / f"template-{pixels}.png").write_bytes(blob)
        (tray / "sonora.png").write_bytes(
            rasterize(master.source, 32, workdir, "tray")
        )

    print(
        f"icons: {len(LINUX_SIZES)} linux png, 1 linux svg, 1 icns, 1 ico,"
        f" {len(TRAY_SIZES) + 1} tray png, 1 readme svg, 1 macos svg, 1 windows svg"
    )


if __name__ == "__main__":
    main()
