#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise an extracted or installed desktop package without capture/network traffic."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

from common import version


def validate(prefix, scratch):
    """Check resources, private dependency resolution, offline UI and WB admission.

    Args:
        prefix: Extracted filesystem root or Path('/') for installed checks.
        scratch: Writable test-only directory for Qt runtime/cache/config files.
    Returns:
        None after all executable/resource checks pass.
    Raises:
        AssertionError/subprocess errors for missing or inconsistent package data.
    """
    payload = prefix / 'usr/lib/merlin'
    executable = prefix / 'usr/bin/merlin'
    environment = os.environ.copy()
    environment.pop('LD_LIBRARY_PATH', None)
    for key in ('QT_PLUGIN_PATH', 'QML_IMPORT_PATH', 'QML2_IMPORT_PATH'):
        environment.pop(key, None)
    environment.update(QT_QPA_PLATFORM='offscreen', QT_QUICK_BACKEND='software',
                       XDG_CONFIG_HOME=str(scratch / 'config'), XDG_CACHE_HOME=str(scratch / 'cache'))
    assert subprocess.check_output([str(executable), '--version'], env=environment, text=True).strip() == f'MERLIN {version()}'
    examples = payload / 'files/examples/workbench'
    manifest = json.loads((examples / 'manifest.json').read_text())
    assert {bundle['id'] for bundle in manifest['bundles']} == {'single', 'multi-rxmer', 'multi-rxmer-2'}
    for bundle in manifest['bundles']:
        wb = examples / bundle['workbench']
        provenance = json.loads((examples / bundle['provenance']).read_text())
        for record in provenance['files']:
            data = (wb.parent / record['file']).read_bytes()
            assert len(data) == record['bytes']
            assert hashlib.sha256(data).hexdigest() == record['sha256']
        for name in ('LICENSE', 'NOTICE'):
            assert (wb.parent / name).is_file()
        subprocess.run([str(executable), '--check-workbench', str(wb)], env=environment, check=True, timeout=30)
    assert (payload / 'notices/LICENSE-GPL-3.0').is_file()
    assert (payload / 'notices/qt/qt-source-provenance.json').is_file()
    assert (prefix / 'usr/share/icons/hicolor/256x256/apps/merlin.png').is_file()
    subprocess.run(['desktop-file-validate', str(prefix / 'usr/share/applications/merlin.desktop')], check=True)
    link_environment = environment | {'LD_LIBRARY_PATH': str(payload / 'lib')}
    for path in payload.rglob('*'):
        if path.is_file():
            with path.open('rb') as stream:
                elf = stream.read(4) == b'\x7fELF'
            if elf:
                dependencies = subprocess.check_output(['ldd', str(path)], env=link_environment, text=True)
                assert 'not found' not in dependencies, (path, dependencies)
                for line in dependencies.splitlines():
                    fields = line.split()
                    if fields and fields[0].startswith(('libQt6', 'libicu')) and len(fields) >= 3:
                        assert Path(fields[2]).is_relative_to(payload), line
    startup = subprocess.run([str(executable), '--smoke-test'], env=environment, capture_output=True, text=True, timeout=30)
    assert startup.returncode == 0, startup.stderr
    assert not any(term in startup.stderr for term in ('failed to load', 'is not installed', 'ReferenceError', 'TypeError', 'Cannot assign')), startup.stderr
    print('Desktop resources, all workbenches, ELF closure and offline UI passed.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--deb', type=Path)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--installed', action='store_true')
    arguments = parser.parse_args()
    if arguments.installed:
        # CI owns this directory; local invocation never installs/removes packages.
        scratch = Path('docs/development/local/desktop-installed-check').resolve()
        scratch.mkdir(parents=True, exist_ok=True)
        validate(Path('/'), scratch)
    else:
        if arguments.deb is None or arguments.output is None:
            parser.error('Provide --deb/--output or --installed')
        arguments.output.mkdir(parents=True, exist_ok=False)
        prefix = arguments.output.resolve() / 'root'
        subprocess.run(['dpkg-deb', '--extract', str(arguments.deb), str(prefix)], check=True)
        fields = subprocess.check_output(['dpkg-deb', '--field', str(arguments.deb), 'Package', 'Version', 'Architecture'], text=True)
        assert f'Package: merlin\n' in fields and f'Version: {version()}\n' in fields and 'Architecture: amd64' in fields, fields
        validate(prefix, arguments.output.resolve())
