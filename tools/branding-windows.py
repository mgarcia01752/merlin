#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Reproduce Windows icon and WiX artwork from the existing MERLIN icon."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
SIZES = (16, 20, 24, 32, 40, 48, 64, 128, 256)


def generate(destination):
    """Write ICO/BMP assets and source provenance into a build asset directory.

    Args:
        destination: Explicit output directory; generated branding may be replaced.
    Returns:
        None. Source art is read-only and transparency is preserved in the ICO.
    Raises:
        OSError for unavailable source art or an unwritable destination.
    """
    source = ROOT / 'docs/images/logo/merlin-icon-dark.png'
    destination.mkdir(parents=True, exist_ok=True)
    icon = Image.open(source).convert('RGBA')
    icon.save(destination / 'merlin.ico', sizes=[(size, size) for size in SIZES])
    # WiX3 uses a 164px left panel on welcome/exit and a small upper-right icon.
    dialog = Image.new('RGB', (493, 312), '#ffffff')
    draw = ImageDraw.Draw(dialog)
    draw.rectangle((0, 0, 163, 311), fill='#0b1523')
    draw.rectangle((0, 0, 163, 3), fill='#38d6cb')
    emblem = icon.resize((140, 140), getattr(Image, 'Resampling', Image).LANCZOS)
    dialog.paste(emblem, (12, 70), emblem)
    dialog.save(destination / 'dialog.bmp')
    banner = Image.new('RGB', (493, 58), '#ffffff')
    emblem = icon.resize((48, 48), getattr(Image, 'Resampling', Image).LANCZOS)
    banner.paste(emblem, (438, 4), emblem)
    ImageDraw.Draw(banner).line((0, 57, 492, 57), fill='#38d6cb')
    banner.save(destination / 'banner.bmp')
    provenance = {'source': str(source.relative_to(ROOT)),
                  'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                  'ico_sizes': list(SIZES),
                  'outputs': {name: hashlib.sha256((destination / name).read_bytes()).hexdigest()
                              for name in ('merlin.ico', 'dialog.bmp', 'banner.bmp')}}
    (destination / 'provenance.json').write_text(json.dumps(provenance, indent=2) + '\n', encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT / 'packaging/windows')
    generate(parser.parse_args().output)
