#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Enforce Qt's facade boundary while freezing reviewed legacy migration debt."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess

from implementation_evidence import rust_code

ROOT = Path(__file__).resolve().parents[2]
RECORD = 'docs/development/features/unified-qt-workbench/legacy-qt-boundary.json'
SCIENTIFIC_PACKAGES = {
    'docsis-ofdm-fec', 'docsis-ofdma', 'fec-codec', 'merlin-dsp',
    'merlin-api', 'pnm-input', 'pnm',
}
INFRASTRUCTURE_PACKAGES = {
    'merlin': 'authoritative public simulation facade',
    'base64': 'bounded project byte transport',
    'cxx': 'native QObject interoperability',
    'cxx-qt': 'QObject bridge runtime',
    'cxx-qt-lib': 'Qt presentation value types',
    'cxx-gen': 'bridge build generation',
    'cxx-qt-build': 'Qt resource and bridge registration',
    'serde': 'typed request/result transport',
    'serde_json': 'bounded presentation and project transport',
    'sha2': 'input/download identity checks',
    'url': 'native file and fixed project/update links',
    'log': 'application diagnostics',
    'env_logger': 'application log configuration',
    'ureq': 'application update HTTP only',
}
BUILD_INFRASTRUCTURE_PACKAGES = {
    'cc': 'compile the Qt branding initializer as a retained build object',
    'qt-build-utils': 'generate Qt branding resources with explicit file watches',
}


def digest(data):
    """Identify exact bytes without changing caller-owned input.

    Args:
        data: Bytes to identify.
    Returns:
        Lowercase SHA-256 hexadecimal digest.
    """
    return hashlib.sha256(data).hexdigest()


def cargo_metadata(root):
    """Read locked direct Qt dependency identities without compiling Qt.

    Args:
        root: Repository root using its existing Cargo helper/toolchain.
    Returns:
        Cargo metadata document including actual package and rename fields.
    Raises:
        Subprocess/JSON errors; missing metadata never weakens enforcement.
    """
    command = [str(root / 'scripts/cargo-local.sh'), 'metadata', '--locked',
               '--no-deps', '--format-version', '1', '--manifest-path',
               str(root / 'apps/qt/Cargo.toml')]
    return json.loads(subprocess.run(command, cwd=root, check=True,
                                    capture_output=True, timeout=90).stdout)


def dependency_records(root, metadata):
    """Normalize scientific direct dependencies, including path/package aliases.

    Args:
        root: Repository or isolated fixture root.
        metadata: Actual Cargo document, injected only by independent tests.
    Returns:
        Stable records for scientific normal/build/dev/target dependencies.
    Raises:
        ValueError/OSError for missing root identity or invalid local manifests.
    """
    manifest = (root / 'apps/qt/Cargo.toml').resolve()
    packages = [p for p in metadata['packages']
                if Path(p['manifest_path']).resolve() == manifest]
    if len(packages) != 1 or packages[0]['name'] != 'merlin-qt':
        raise ValueError('Cargo metadata must identify the actual merlin-qt manifest')
    records = []
    for dependency in packages[0]['dependencies']:
        actual = dependency['name']
        path = dependency.get('path')
        relative = None
        if path:
            package_manifest = Path(path).resolve() / 'Cargo.toml'
            text = package_manifest.read_text(encoding='utf-8')
            # Isolate [package], then its quoted name, avoiding binary/target names.
            section = re.search(r'^\[package\]\s*\n([^\[]*)', text, re.MULTILINE)
            name = re.search(r'^name\s*=\s*"([^"]+)"', section[1], re.MULTILINE) if section else None
            if not name:
                raise ValueError(f'Missing local package identity: {package_manifest}')
            actual = name[1]
            resolved = Path(path).resolve()
            relative = str(resolved.relative_to(root.resolve())) if resolved.is_relative_to(root.resolve()) else str(resolved)
        if actual in BUILD_INFRASTRUCTURE_PACKAGES:
            if dependency.get('kind') != 'build':
                raise ValueError(f'Qt resource infrastructure must be build-only: {actual}')
            continue
        if actual not in SCIENTIFIC_PACKAGES and actual not in INFRASTRUCTURE_PACKAGES:
            raise ValueError(f'Unreviewed Qt dependency identity: {actual}')
        if actual not in SCIENTIFIC_PACKAGES:
            continue
        records.append({
            'package': actual, 'declared_package': dependency['name'],
            'rename': dependency.get('rename'), 'kind': dependency.get('kind'),
            'target': dependency.get('target'), 'path': relative,
            'source': dependency.get('source'), 'requirement': dependency.get('req'),
            'features': sorted(dependency.get('features', [])),
            'default_features': dependency.get('uses_default_features', True),
            'optional': dependency.get('optional', False),
        })
    return sorted(records, key=lambda record: json.dumps(record, sort_keys=True))


def scientific_sites(text, roots):
    """Locate scientific crate identifiers in Rust code, preserving line offsets.

    Args:
        text: Original source, including possible comments and string literals.
        roots: Known scientific crate names and actual renamed identifiers.
    Returns:
        Exact root/line pairs; whole-file freezes catch later alias invocations.
    """
    masked = rust_code(text)
    candidates = {}
    names = r'(?:' + '|'.join(re.escape(root) for root in sorted(roots)) + r')'
    # A crate-qualified path, e.g. ::docsis_ofdma::api, is scientific anywhere
    # in code. A boolean parameter named pnm is not a crate-qualified access.
    qualified = r'\b' + names + r'\b(?=\s*::)'
    for match in re.finditer(qualified, masked):
        candidates[match.start()] = match[0]
    # Whole identifiers inside use trees include bare and renamed imports:
    # use {pnm, fec_codec as codec}; the semicolon bounds one declaration.
    for declaration in re.finditer(r'\buse\s+[^;]+;', masked):
        for match in re.finditer(r'\b' + names + r'\b', declaration[0]):
            candidates[declaration.start() + match.start()] = match[0]
    # extern crate fec_codec as codec exposes the crate without a use path.
    external = r'\bextern\s+crate\s+(' + names + r')\b'
    for match in re.finditer(external, masked):
        candidates[match.start(1)] = match[1]
    return [{'line': text.count('\n', 0, offset) + 1, 'root': name}
            for offset, name in sorted(candidates.items())]


def escaped_modules(root, path, text):
    """Reject source inclusion of Rust modules outside Qt's owned source tree.

    Args:
        root: Repository root.
        path: Source file containing path/include declarations.
        text: Original Rust source; ordinary non-source resources remain allowed.
    Returns:
        Diagnostics for literal external Rust includes or module paths.
    """
    errors = []
    masked = rust_code(text)
    qt_root = (root / 'apps/qt').resolve()
    build_root = qt_root / 'build_support'
    build_source = path.resolve() == qt_root / 'build.rs' or path.resolve().is_relative_to(build_root)
    # Capture literal module paths and include!("file.rs"); resource include_str!
    # is presentation data rather than Rust module inclusion.
    pattern = r'#\s*\[\s*path\s*=\s*"([^"]+)"\s*\]|\binclude!\s*\(\s*"([^"]+\.rs)"'
    for match in re.finditer(pattern, text):
        if masked[match.start()] != text[match.start()]:
            continue
        relative = match[1] or match[2]
        included = (path.parent / relative).resolve()
        owned_source = included.is_relative_to(qt_root / 'src')
        owned_build_source = build_source and included.is_relative_to(build_root)
        if not owned_source and not owned_build_source:
            errors.append(f'{path.relative_to(root)}: external Rust module inclusion {relative}')
    return errors


def rust_sources(root):
    """Collect owned Rust sources without traversing Cargo build artifacts.

    Args:
        root: Repository or isolated fixture root.
    Returns:
        Sorted source paths under Qt, excluding target/cache directories.
    """
    paths = []
    for directory, children, names in os.walk(root / 'apps/qt'):
        children[:] = [name for name in children if name not in ('target', 'local', '.git')]
        paths.extend(Path(directory) / name for name in names if name.endswith('.rs'))
    return sorted(paths)


def audit(root, metadata, legacy, final=False):
    """Reject new scientific dependencies/calls and stale or changed exceptions.

    Args:
        root: Repository or isolated fixture root.
        metadata: Locked Cargo dependency metadata.
        legacy: Reviewed baseline records; never modified by this operation.
        final: Require zero remaining dependency/source exceptions.
    Returns:
        JSON-compatible result with diagnostics and exact debt counts.
    """
    errors = []
    if legacy.get('schema_version') != 1:
        raise ValueError('Unsupported Qt boundary record schema')
    dependencies = dependency_records(root, metadata)
    if dependencies != legacy['dependencies']:
        errors.append('Qt scientific dependency set changed; migrate/review exact dependency records')
    roots = {name.replace('-', '_') for name in SCIENTIFIC_PACKAGES}
    roots.update((d['rename'] or d['declared_package']).replace('-', '_') for d in dependencies)
    frozen = {entry['path']: entry for entry in legacy['files']}
    if len(frozen) != len(legacy['files']):
        errors.append('Duplicate legacy source entries')
    checked = set()
    for path in rust_sources(root):
        if not path.resolve().is_relative_to((root / 'apps/qt').resolve()):
            errors.append(f'{path.relative_to(root)}: redirected Qt Rust source')
            continue
        name = str(path.relative_to(root))
        source = path.read_bytes()
        text = source.decode('utf-8')
        errors.extend(escaped_modules(root, path, text))
        sites = scientific_sites(text, roots)
        if name in frozen:
            checked.add(name)
            entry = frozen[name]
            if entry.get('sunset_task') != 'UIU-DS-01':
                errors.append(f'{name}: missing reviewed migration sunset task')
            if digest(source) != entry['sha256'] or sites != entry['sites']:
                errors.append(f'{name}: changed frozen scientific source; adopt public API before changing legacy calls')
        elif sites:
            errors.append(f'{name}: new direct scientific crate access')
    for name in frozen.keys() - checked:
        errors.append(f'{name}: stale legacy source exception')
    if final and (dependencies or frozen):
        errors.append('Final Qt acceptance requires zero scientific migration exceptions')
    return {'errors': errors, 'scientific_dependencies': len(dependencies),
            'frozen_legacy_files': len(frozen), 'final': final}


def main():
    """Run the non-mutating boundary gate using actual checkout metadata.

    Returns:
        Zero only when all dependency and source-boundary assertions pass.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--final', action='store_true')
    arguments = parser.parse_args()
    legacy = json.loads((ROOT / RECORD).read_text(encoding='utf-8'))
    result = audit(ROOT, cargo_metadata(ROOT), legacy, arguments.final)
    print(json.dumps(result, indent=2))
    return bool(result['errors'])


if __name__ == '__main__':
    raise SystemExit(main())
