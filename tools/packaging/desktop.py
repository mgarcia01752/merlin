#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build a standalone Ubuntu 22.04 amd64 desktop DEB without installing it."""
import argparse
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess

from common import ROOT, checksums, copy_tree, version
from notices import collect_notices
from qt_runtime import deploy
from qt_notices import qt_notices, qt

DEPENDENCIES = (
    'pkexec, libc6 (>= 2.35), libstdc++6, libgcc-s1, libgl1, libegl1, libopengl0, '
    'libdbus-1-3, libfontconfig1, libfreetype6, libx11-6, libx11-xcb1, '
    'libxkbcommon0, libxkbcommon-x11-0, libxcb1, libxcb-cursor0, libxcb-icccm4, '
    'libxcb-image0, libxcb-keysyms1, libxcb-render-util0, libxcb-shape0, '
    'libxcb-randr0, libxcb-xfixes0, libxcb-sync1, libxcb-render0, libxcb-shm0, '
    'libxcb-xkb1, libxcb-xinerama0, libsm6, libice6, libglib2.0-0, libgtk-3-0, '
    'libzstd1, zlib1g, libpcre2-16-0, libb2-1, ca-certificates, xdg-utils, xwayland, fonts-dejavu-core'
)


def system_packages(libraries):
    """Resolve every observed system ELF dependency to an installed DEB owner.

    Args:
        libraries: Absolute host paths reported by ldd outside the bundled SDK.
    Returns:
        Sorted dependency package names; usr-merged aliases are checked too.
    Raises:
        ValueError when a required library has no known Debian owner.
    """
    owners = set()
    for library in libraries:
        candidates = {str(library), str(library.resolve())}
        for path in list(candidates):
            if path.startswith('/usr/lib/'):
                candidates.add(path.removeprefix('/usr'))
            elif path.startswith('/lib/'):
                candidates.add('/usr' + path)
        found = False
        for candidate in sorted(candidates):
            result = subprocess.run(['dpkg-query', '--search', candidate], capture_output=True, text=True)
            if result.returncode == 0:
                for line in result.stdout.splitlines():
                    owner = line.split(': ', 1)[0].split(':', 1)[0]
                    if owner != 'diversion by glx-diversions':
                        owners.add(owner)
                        found = True
                break
        if not found:
            raise ValueError(f'No DEB dependency owner for {library}')
    owners.discard('libc6')
    return sorted(owners)


def package(binary, sdk, output):
    """Stage public desktop assets and produce an immutable DEB/checksum pair.

    Args:
        binary: Native executable built on Ubuntu 22.04 amd64.
        sdk: Existing Qt 6.8.3 SDK; read-only throughout staging.
        output: New artifact directory; an existing directory is rejected.
    Returns:
        Path to the generated desktop DEB.
    Raises:
        ValueError for unsupported hosts, mismatched versions or missing resources.
        FileExistsError for existing outputs; build/SDK failures propagate.
    """
    if output.exists():
        raise FileExistsError(output)
    if platform.machine() != 'x86_64' or platform.system() != 'Linux':
        raise ValueError('Desktop DEB builds require Ubuntu 22.04 amd64')
    os_release = dict(line.split('=', 1) for line in Path('/etc/os-release').read_text().splitlines() if '=' in line)
    if os_release.get('ID', '').strip('"') != 'ubuntu' or os_release.get('VERSION_ID', '').strip('"') != '22.04':
        raise ValueError('Build the desktop DEB on Ubuntu 22.04')
    environment = os.environ.copy()
    environment['LD_LIBRARY_PATH'] = str(sdk / 'lib')
    qt.check_sdk(sdk)
    actual = subprocess.check_output([str(binary.resolve()), '--version'], env=environment, text=True).strip()
    if actual != f'MERLIN {version()}':
        raise ValueError('Desktop binary/version mismatch')
    if (output.parent / (output.name + '-stage')).exists():
        raise FileExistsError('Package stage already exists')
    output.mkdir(parents=True)
    stage = output / 'stage'
    payload = stage / 'usr/lib/merlin'
    payload.mkdir(parents=True)
    shutil.copy2(binary, payload / 'merlin-qt')
    (payload / 'merlin-qt').chmod(0o755)
    dependencies = sorted(set(DEPENDENCIES.split(', ')) | set(system_packages(deploy(payload, sdk, ROOT / 'apps/qt/qml'))))
    dependency_text = ', '.join(dependencies)
    files = payload / 'files'
    files.mkdir()
    copy_tree(ROOT / 'examples', files / 'examples')
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
    launcher = stage / 'usr/bin/merlin'
    launcher.parent.mkdir(parents=True)
    launcher.write_text('#!/bin/sh\nset -eu\n'
                        'app=$(CDPATH= cd -- "$(dirname -- "$(readlink -f -- "$0")")/../lib/merlin" && pwd)\n'
                        'export LD_LIBRARY_PATH="$app/lib"\n'
                        'export QT_PLUGIN_PATH="$app/plugins"\n'
                        'export QML_IMPORT_PATH="$app/qml"\n'
                        'export QML2_IMPORT_PATH="$app/qml"\n'
                        'export QT_QPA_PLATFORM="${QT_QPA_PLATFORM:-xcb}"\n'
                        'exec "$app/merlin-qt" "$@"\n', encoding='utf-8')
    launcher.chmod(0o755)
    desktop = stage / 'usr/share/applications/merlin.desktop'
    desktop.parent.mkdir(parents=True)
    template = (ROOT / 'apps/qt/merlin.desktop.in').read_text()
    desktop.write_text(template.replace('@LAUNCHER@', '/usr/bin/merlin').replace('@ICON@', 'merlin'), encoding='utf-8')
    icon = stage / 'usr/share/icons/hicolor/256x256/apps/merlin.png'
    icon.parent.mkdir(parents=True)
    shutil.copy2(ROOT / 'docs/images/logo/merlin-icon-dark.png', icon)
    for path in stage.rglob('*'):
        if path.is_dir():
            path.chmod(0o755)
        elif path.is_file():
            with path.open('rb') as stream:
                executable_file = stream.read(4) == b'\x7fELF'
            path.chmod(0o755 if executable_file or path == launcher else 0o644)
            if executable_file:
                subprocess.run(['strip', '--strip-unneeded', str(path)], check=True)
    control = stage / 'DEBIAN'
    control.mkdir(mode=0o755)
    control.chmod(0o755)
    installed_kib = (sum(path.stat().st_size for path in stage.rglob('*') if path.is_file()) + 1023) // 1024
    (control / 'control').write_text(
        f'Package: merlin\nVersion: {version()}\nArchitecture: amd64\n'
        'Maintainer: MERLIN contributors\nSection: education\nPriority: optional\n'
        f'Installed-Size: {installed_kib}\nDepends: {dependency_text}\n'
        'Homepage: https://github.com/mgarcia01752/merlin\n'
        'Description: MERLIN DOCSIS OFDM Simulation Toolkit\n'
        ' Native single-channel downstream OFDM simulation workbench.\n', encoding='utf-8')
    (control / 'control').chmod(0o644)
    artifact = output / f'merlin_{version()}_ubuntu22.04_amd64.deb'
    subprocess.run(['dpkg-deb', '--root-owner-group', '-Zxz', '--build', str(stage), str(artifact)], check=True)
    # Checksums inventory contains deliverables only; staging is retained separately.
    stage.rename(output.parent / (output.name + '-stage'))
    checksums(output)
    print(json.dumps({'deb': str(artifact), 'installer_bytes': artifact.stat().st_size,
                      'installed_bytes': installed_kib * 1024, 'external_prerequisites': dependency_text}))
    return artifact


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--qt-sdk', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    arguments = parser.parse_args()
    package(arguments.binary.resolve(), arguments.qt_sdk.resolve(), arguments.output.resolve())
