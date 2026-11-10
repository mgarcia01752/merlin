#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify docs-only policy against real, isolated Git histories."""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    'docs_only', Path(__file__).resolve().parents[1] / 'docs_only.py')
POLICY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(POLICY)


class DocsOnlyTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.git('init', '-b', 'master')
        self.git('config', 'user.name', 'Test')
        self.git('config', 'user.email', 'test@example.invalid')
        self.commit('source.rs', 'initial')
        self.base = self.git('rev-parse', 'HEAD').strip()

    def git(self, *arguments):
        return subprocess.run(['git', '-C', str(self.root), *arguments], check=True,
                              capture_output=True, text=True).stdout

    def commit(self, name, content):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)
        self.git('add', '--all')
        self.git('commit', '-m', 'test')

    def classify(self):
        return POLICY.docs_only(self.root, self.base, 'HEAD')

    def test_docs_and_images_skip(self):
        self.commit('README.md', 'readme')
        self.commit('docs/user/guide.md', 'guide')
        self.commit('docs/images/workspace.png', 'image')
        self.assertTrue(self.classify())

    def test_mixed_range_requires_gate(self):
        self.commit('source.rs', 'changed')
        self.commit('README.md', 'readme')
        self.assertFalse(self.classify())

    def test_reverted_code_still_requires_gate(self):
        self.commit('source.rs', 'changed')
        self.commit('source.rs', 'initial')
        self.commit('README.md', 'readme')
        self.assertFalse(self.classify())

    def test_code_renamed_to_markdown_requires_gate(self):
        self.git('mv', 'source.rs', 'README.md')
        self.git('commit', '-m', 'rename')
        self.assertFalse(self.classify())

    def test_empty_range_requires_gate(self):
        self.assertFalse(self.classify())

    def test_missing_object_fails(self):
        with self.assertRaises(subprocess.CalledProcessError):
            POLICY.docs_only(self.root, 'missing-ref', 'HEAD')

    def test_merged_code_requires_gate(self):
        self.git('checkout', '-b', 'topic')
        self.commit('source.rs', 'topic')
        self.git('checkout', 'master')
        self.commit('README.md', 'readme')
        self.git('merge', '--no-ff', 'topic', '-m', 'merge')
        self.assertFalse(self.classify())

    def test_non_documentation_paths_require_gate(self):
        for name in ['AGENTS.md', 'skills/example/SKILL.md', 'Cargo.toml',
                     '.github/workflows/ci.yml', 'docs/input.json', 'docs/run.sh',
                     'tests/fixture.md', 'docs/images/source.rs']:
            self.assertFalse(POLICY.documentation_path(name), name)
