#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Classify every outgoing commit before allowing a docs-only branch push."""
import argparse
from pathlib import PurePosixPath
import subprocess


def git(root, *arguments):
    """Read Git metadata as bytes, propagating inspection failures."""
    return subprocess.run(['git', '-C', str(root), *arguments], check=True,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout


def documentation_path(path):
    """Allow maintained prose and documentation images, excluding executable inputs."""
    name = PurePosixPath(path)
    return (path == 'README.md'
            or (path.startswith('docs/') and name.suffix == '.md')
            or (path.startswith('docs/images/')
                and name.suffix.lower() in {'.png', '.jpg', '.jpeg', '.svg', '.webp', '.gif'}))


def docs_only(root, base, head):
    """Require a nonempty outgoing range containing documentation changes only.

    Inspect each commit and every merge-parent diff without rename detection.
    This preserves both paths of a rename and code edits reverted later in the
    range. Missing ancestry or unreadable objects raise instead of allowing skip.
    """
    subprocess.run(['git', '-C', str(root), 'merge-base', '--is-ancestor', base, head],
                   check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    commits = git(root, 'rev-list', f'{base}..{head}').splitlines()
    changed = False
    for commit in commits:
        paths = git(root, 'diff-tree', '--root', '-r', '-m', '--no-renames',
                    '--no-commit-id', '--name-only', '-z', commit.decode('ascii'))
        for raw_path in paths.split(b'\0'):
            if raw_path:
                changed = True
                if not documentation_path(raw_path.decode('utf-8', errors='surrogateescape')):
                    return False
    return changed


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', required=True)
    parser.add_argument('--base', required=True)
    parser.add_argument('--head', required=True)
    args = parser.parse_args()
    print('docs' if docs_only(args.root, args.base, args.head) else 'full')
