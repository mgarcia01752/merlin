#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Shared public desktop examples, guides and complete third-party notices."""
import shutil
from common import ROOT, copy_tree
from notices import collect_notices
from qt_notices import qt_notices


def populate(payload):
    """Copy the desktop resource allowlist into a fresh native payload.

    Args:
        payload: Caller-owned staging directory beside the native executable.
    Returns:
        None after examples, documentation and license notices are staged.
    Raises:
        Filesystem, Cargo metadata or notice-source failures; input stays read-only.
    """
    files = payload / 'files'
    files.mkdir()
    copy_examples(files / 'examples')
    for relative in ('docs/user', 'docs/reference', 'docs/images'):
        destination = files / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        copy_tree(ROOT / relative, destination)
    shutil.copy2(ROOT / 'docs/architecture.md', files / 'docs/architecture.md')
    notices = payload / 'notices'
    notices.mkdir()
    for source, name in [('apps/qt/LICENSE', 'LICENSE-GPL-3.0'), ('LICENSE', 'LICENSE-Apache-2.0'),
                         ('apps/qt/NOTICE', 'NOTICE'), ('crates/fec-codec/LICENSE-MIT', 'fec-codec-LICENSE-MIT')]:
        shutil.copy2(ROOT / source, notices / name)
    copy_tree(ROOT / 'crates/ofdm/resources/qam-table', notices / 'qam-table')
    collect_notices(ROOT, notices / 'cargo', manifest=ROOT / 'apps/qt/Cargo.toml')
    qt_notices(notices / 'qt')
    for name in ('icu-73.2-LICENSE', 'icu-73.2-provenance.json'):
        shutil.copy2(ROOT / 'tools/packaging/resources' / name, notices / name)


def copy_examples(destination):
    """Stage the desktop example allowlist, excluding standalone CLI requests.

    Args:
        destination: New example directory beneath the caller-owned payload.
    Returns:
        None after copying workbench/measurement fixtures and a desktop index.
    Raises:
        FileExistsError for existing output; unsafe resource/copy errors propagate.
    """
    destination.mkdir()
    for name in ('workbench', 'measurements'):
        copy_tree(ROOT / 'examples' / name, destination / name)
    (destination / 'README.md').write_text(
        '# MERLIN desktop examples\n\n'
        'Open [workbench](workbench/README.md) for ready-to-load UI configurations.\n'
        '[Measurements](measurements/README.md) contains shared capture data and provenance.\n'
        'Standalone CLI requests under examples/cli are excluded from this installer.\n',
        encoding='utf-8')
