#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Refresh or check the downstream Python installation guide without Git mutations."""
import argparse
import os
from pathlib import Path
import stat
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
DOCUMENT = 'docs/user/python/integration.md'
TEMPLATE = 'tools/python/templates/integration.md.in'
INPUTS = (TEMPLATE, 'bindings/python/pyproject.toml')
sys.path.insert(0, str(ROOT / 'tools/release'))
from versions import cargo_field, validate


def checked_path(root, name):
    """Resolve a controlled repository path while rejecting redirected components.

    Args:
        root: Absolute checkout or disposable fixture root.
        name: Relative repository-owned input/output path.
    Returns:
        The unredirected path without creating or writing it.
    Raises:
        ValueError for any symlink component or an escaping path.
    """
    root = Path(root).absolute()
    path = root / name
    if path.resolve() != path or root not in path.parents:
        raise ValueError(f'Refuse redirected integration-document path: {name}')
    return path


def render(root):
    """Produce deterministic guide text from matching source/package versions.

    Args:
        root: Complete checkout with VERSION, adapter manifests and template.
    Returns:
        Generated Markdown with the exact version and expected source tag.
    Raises:
        OSError or ValueError for missing, redirected or inconsistent inputs.
        This operation never writes, queries a remote or asserts publication.
    """
    version = validate(checked_path(root, 'VERSION').read_text().strip())
    manifest = checked_path(root, 'bindings/python/Cargo.toml').read_text()
    if cargo_field(manifest, 'version') != version:
        raise ValueError('Python adapter version does not match VERSION')
    project = checked_path(root, 'bindings/python/pyproject.toml').read_text()
    if cargo_field(project, 'name') != 'merlin-ofdm' or 'module-name = "merlin"' not in project:
        raise ValueError('Python package/import names changed; update integration template')
    template = checked_path(root, TEMPLATE).read_text(encoding='utf-8')
    if '{{VERSION}}' not in template:
        raise ValueError('Integration template must contain {{VERSION}}')
    result = template.replace('{{VERSION}}', version)
    if '{{' in result or '}}' in result:
        raise ValueError('Unresolved integration template token')
    return result


def update(root, write=False):
    """Check or atomically refresh the generated guide, preserving unrelated files.

    Args:
        root: Checkout or disposable fixture root.
        write: Replace only the generated document when stale; false is read-only.
    Returns:
        True if the document differed, false for an unchanged document.
    Raises:
        OSError or ValueError for invalid inputs or redirected output paths.
        No file is written until all rendering/validation has succeeded.
    """
    content = render(root)
    path = checked_path(root, DOCUMENT)
    previous = path.read_text(encoding='utf-8') if path.exists() else None
    if previous == content:
        return False
    if write:
        path.parent.mkdir(parents=True, exist_ok=True)
        mode = stat.S_IMODE(path.stat().st_mode) if path.exists() else 0o644
        temporary = None
        try:
            with tempfile.NamedTemporaryFile(mode='w', encoding='utf-8',
                                             dir=path.parent, delete=False) as output:
                temporary = Path(output.name)
                output.write(content)
            temporary.chmod(mode)
            os.replace(temporary, path)
        finally:
            if temporary is not None and temporary.exists():
                temporary.unlink()
    return True


def main():
    """Parse write/check mode and fail stale checks without modifying Git state.

    Returns through the process exit code: zero for a fresh/updated guide and
    nonzero for stale check mode or invalid inputs. No network/install operations.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    modes = parser.add_mutually_exclusive_group(required=True)
    modes.add_argument('--write', action='store_true', help='Refresh generated Markdown')
    modes.add_argument('--check', action='store_true', help='Fail if generated Markdown is stale')
    arguments = parser.parse_args()
    try:
        changed = update(ROOT, write=arguments.write)
    except (OSError, ValueError) as error:
        parser.exit(1, f'ERROR: {error}\n')
    if changed and arguments.check:
        parser.exit(1, 'ERROR: Python integration guide is stale; run '
                    'python3 tools/python/update_integration_doc.py --write and commit the guide.\n')
    print('Python integration guide: ' + ('refreshed' if changed else 'current'))


if __name__ == '__main__':
    main()
