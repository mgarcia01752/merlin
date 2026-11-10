#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Synchronize project-owned version metadata from the root VERSION file."""
import argparse
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]
MANIFESTS = ('crates/ofdm/Cargo.toml', 'crates/fec-codec/Cargo.toml',
             'crates/cli/Cargo.toml', 'apps/qt/Cargo.toml')
LOCKS = ('Cargo.lock', 'apps/qt/Cargo.lock')


def validate(version):
    """Validate one whole stable version, raising ValueError for invalid input.

    Accept MAJOR.MINOR.PATCH with no leading zeros; return the unchanged text.
    """
    # Whole-string validation: "1.2.3" passes; "01.2.3" and suffixes fail.
    # Noncapturing alternatives accept zero or a nonzero-leading decimal part.
    if not re.fullmatch(r'(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)', version):
        raise ValueError('Expected MAJOR.MINOR.PATCH without leading zeros')
    return version


def bumped(version: str, part: str) -> str:
    """Return a bumped three-component version without modifying files.

    Args:
        version: Whole numeric MAJOR.MINOR.PATCH string without leading zeros.
        part: Component to increment: major, minor or patch.
    Returns:
        Incremented version with lower components reset to zero.
    Raises:
        ValueError: Version syntax or component name is invalid.
    """
    validate(version)
    parts = [int(value) for value in version.split(".")]
    index = ("major", "minor", "patch").index(part)
    parts[index] += 1
    parts[index + 1:] = [0] * (2 - index)
    return ".".join(map(str, parts))


def cargo_field(text, field):
    """Read one quoted top-level Cargo field or raise ValueError if absent.

    Used only on controlled package manifests/lock blocks, never arbitrary TOML.
    """
    # Match a complete quoted field line, e.g. name = "viewer" -> viewer.
    # Field names are internal literals; capture group 1 is the quoted value.
    match = re.search(r'^' + re.escape(field) + r' = "([^"\n]+)"$', text, re.MULTILINE)
    if match is None:
        raise ValueError(f'Missing Cargo field: {field}')
    return match.group(1)


def metadata(root):
    """Read package names and metadata; fail before writing missing inputs.

    root is a complete repository or synthetic test checkout. Return all files
    and the set of local Cargo package names; filesystem/parse failures propagate.
    """
    names = set()
    texts = {}
    for name in MANIFESTS + LOCKS:
        texts[name] = (root / name).read_text(encoding='utf-8')
    for name in MANIFESTS:
        names.add(cargo_field(texts[name], 'name'))
    return texts, names


def synchronize(root, version, write=False):
    """Check or synchronize all local package versions to VERSION.

    root is the repository path; version is the validated target. With write
    false return stale paths. With write true update only package versions and
    VERSION after preparing every replacement; dependencies remain untouched.
    Read/parse/validation errors propagate before any files are written.
    """
    validate(version)
    texts, names = metadata(root)
    replacements = {}
    for name, text in texts.items():
        updated = text
        if name in MANIFESTS:
            # Replace the single top-level package version, not dependency pins.
            updated, count = re.subn(r'(?m)^version = "[^"]+"$',
                                     f'version = "{version}"', text, count=1)
            if count != 1:
                raise ValueError(f'Missing package version: {name}')
        elif name in LOCKS:
            blocks = text.split('[[package]]')
            for index, block in enumerate(blocks[1:], 1):
                if cargo_field(block, 'name') in names and '\nsource = ' not in block:
                    # Anchored version line inside one parsed local package block.
                    blocks[index] = re.sub(r'(?m)^version = "[^"]+"$',
                                           f'version = "{version}"', block, count=1)
            updated = '[[package]]'.join(blocks)
        if updated != text:
            replacements[name] = updated
    if write:
        for name, text in replacements.items():
            (root / name).write_text(text, encoding='utf-8')
        (root / 'VERSION').write_text(version + '\n', encoding='utf-8')
    return sorted(replacements)


def main():
    """Check current metadata and optional release tag; exit nonzero on drift."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--tag')
    args = parser.parse_args()
    try:
        version = validate((ROOT / 'VERSION').read_text(encoding='utf-8').strip())
        if args.tag is not None and args.tag != f'v{version}':
            raise ValueError('Release tag does not match VERSION')
        stale = synchronize(ROOT, version)
        if stale:
            raise ValueError('Version drift: ' + ', '.join(stale))
    except (OSError, ValueError, KeyError) as error:
        parser.exit(1, f'ERROR: {error}\n')
    print(f'Version metadata agrees: {version}')


if __name__ == '__main__':
    main()
