#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Prepare the exact local nextest runner without system installation."""
import hashlib
import os
from pathlib import Path
import platform
import tarfile
import tempfile
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
VERSION = '0.9.151'
SHA256 = '6b284d6fde9670dfb6ee10ea9d9abbc85f95643d3f592472d420d82e257b3a6c'
URL = (f'https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-{VERSION}/'
       f'cargo-nextest-{VERSION}-x86_64-unknown-linux-musl.tar.gz')
MAX_ARCHIVE_BYTES = 64 * 1024 * 1024


def verify(archive):
    """Require a bounded regular archive matching the reviewed release hash.

    Args:
        archive: Cached or freshly downloaded release archive.
    Returns:
        None for the exact pinned bytes.
    Raises:
        ValueError for redirected, oversized or mismatched content.
    """
    if archive.is_symlink() or not archive.is_file() or archive.stat().st_size > MAX_ARCHIVE_BYTES:
        raise ValueError('Invalid nextest archive')
    if hashlib.sha256(archive.read_bytes()).hexdigest() != SHA256:
        raise ValueError('Nextest archive checksum mismatch; no executable was run')


def prepare(root=ROOT):
    """Download/reverify nextest and atomically prepare its local executable.

    Args:
        root: Owning checkout; tests may supply an isolated temporary root.
    Returns:
        Absolute executable path; no global installation or source compilation.
    Raises:
        ValueError for unsupported hosts, redirects or invalid archive members.
        Network, archive and filesystem errors propagate before execution.
    """
    if platform.system() != 'Linux' or platform.machine() != 'x86_64':
        raise ValueError('Local nextest bootstrap requires Linux x86_64')
    directory = root / f'local/test-tools/nextest-{VERSION}'
    if not directory.resolve().is_relative_to(root.resolve()):
        raise ValueError('Refusing redirected nextest tool storage')
    directory.mkdir(parents=True, exist_ok=True)
    archive = directory / 'nextest.tar.gz'
    if not archive.exists():
        with tempfile.NamedTemporaryFile(dir=directory, delete=False) as stream:
            temporary = Path(stream.name)
            try:
                with urllib.request.urlopen(URL, timeout=60) as response:
                    count = 0
                    while True:
                        chunk = response.read(1024 * 1024)
                        if not chunk:
                            break
                        count += len(chunk)
                        if count > MAX_ARCHIVE_BYTES:
                            raise ValueError('Nextest download exceeds size limit')
                        stream.write(chunk)
                stream.flush()
                verify(temporary)
                temporary.replace(archive)
            finally:
                temporary.unlink(missing_ok=True)
    verify(archive)
    with tarfile.open(archive, 'r:gz') as source:
        matches = [member for member in source.getmembers() if member.name == 'cargo-nextest']
        if len(matches) != 1 or not matches[0].isfile() or matches[0].size > MAX_ARCHIVE_BYTES:
            raise ValueError('Invalid nextest executable member')
        binary = source.extractfile(matches[0]).read(MAX_ARCHIVE_BYTES + 1)
    executable = directory / 'cargo-nextest'
    if executable.is_symlink():
        raise ValueError('Refusing redirected nextest executable')
    if not executable.is_file() or executable.read_bytes() != binary:
        with tempfile.NamedTemporaryFile(dir=directory, delete=False) as stream:
            temporary = Path(stream.name)
            try:
                stream.write(binary)
                stream.flush()
                temporary.chmod(0o755)
                os.replace(temporary, executable)
            finally:
                temporary.unlink(missing_ok=True)
    return executable


if __name__ == '__main__':
    print(prepare())
