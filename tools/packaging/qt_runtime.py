#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Stage private Qt runtime imports using the pinned SDK, never system installs."""
import json
import os
from pathlib import Path
import shutil
import subprocess


def deploy(payload, sdk, qml_source):
    """Copy scanned QML modules, selected plugins and their SDK ELF closure.

    Args:
        payload: Fresh package application directory, already containing its ELF.
        sdk: Read-only pinned Qt SDK root.
        qml_source: Project QML root for Qt's own dependency scanner.
    Returns:
        Sorted paths of required system libraries outside the private Qt SDK.
    Raises:
        ValueError for unresolved dependencies or scanner paths outside the SDK.
        OSError/subprocess errors for missing tools, SDK files or copy failures.
    """
    sdk = sdk.resolve()
    environment = os.environ.copy()
    environment['LD_LIBRARY_PATH'] = str(sdk / 'lib')
    result = subprocess.run([str(sdk / 'libexec/qmlimportscanner'), '-rootPath', str(qml_source),
                             '-importPath', str(sdk / 'qml')], env=environment,
                            capture_output=True, text=True, check=True)
    for module in json.loads(result.stdout):
        source = Path(module.get('path', ''))
        if not source.is_dir() or not source.is_relative_to(sdk / 'qml'):
            continue
        shutil.copytree(source, payload / 'qml' / source.relative_to(sdk / 'qml'),
                        dirs_exist_ok=True, ignore=shutil.ignore_patterns('*.debug', '*.a', '*.prl'))
    for category in ('platforms', 'imageformats', 'iconengines', 'xcbglintegrations'):
        source = sdk / 'plugins' / category
        for plugin in source.glob('*.so'):
            if category == 'platforms' and plugin.name not in ('libqxcb.so', 'libqoffscreen.so', 'libqminimal.so'):
                continue
            target = payload / 'plugins' / category / plugin.name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(plugin, target)
    libraries = payload / 'lib'
    libraries.mkdir()
    pending = []
    for path in payload.rglob('*'):
        if path.is_file():
            with path.open('rb') as stream:
                if stream.read(4) == b'\x7fELF':
                    pending.append(path)
    examined = set()
    system_libraries = set()
    while pending:
        path = pending.pop()
        if path in examined:
            continue
        examined.add(path)
        result = subprocess.run(['ldd', str(path)], env=environment, capture_output=True, text=True, check=True)
        if 'not found' in result.stdout:
            raise ValueError(f'Unresolved runtime dependency: {path}\n{result.stdout}')
        for line in result.stdout.splitlines():
            fields = line.split()
            if len(fields) < 3 or fields[1] != '=>':
                continue
            dependency = Path(fields[2])
            if dependency.is_relative_to(sdk):
                destination = libraries / fields[0]
                if not destination.exists():
                    shutil.copy2(dependency.resolve(), destination)
                    pending.append(destination)
            elif not dependency.is_relative_to(payload):
                system_libraries.add(dependency)
    (payload / 'qt.conf').write_text('[Paths]\nPrefix=.\nLibraries=lib\nPlugins=plugins\nQmlImports=qml\n', encoding='utf-8')
    return sorted(system_libraries)
