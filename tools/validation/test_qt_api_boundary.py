#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Independent fixtures proving the Qt boundary rejects direct scientific access."""
import copy
from pathlib import Path
import tempfile
import unittest

from qt_api_boundary import audit, dependency_records, digest, scientific_sites


class QtApiBoundaryTests(unittest.TestCase):
    """Exercise package identity, aliases, frozen calls and final acceptance."""

    def setUp(self):
        """Create a minimal disposable Qt tree without invoking Cargo or Qt."""
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        (self.root / 'apps/qt/src').mkdir(parents=True)
        self.metadata = {'packages': [{
            'name': 'merlin-qt',
            'manifest_path': str(self.root / 'apps/qt/Cargo.toml'),
            'dependencies': [],
        }]}
        self.legacy = {'schema_version': 1, 'dependencies': [], 'files': []}

    def write(self, text, name='main.rs'):
        """Write one exact source fixture, returning its repository path."""
        path = self.root / 'apps/qt/src' / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding='utf-8')
        return path

    def result(self, final=False):
        """Run only the pure audit against this fixture's injected metadata."""
        return audit(self.root, self.metadata, self.legacy, final)

    def test_facade_and_transitive_science_are_admitted(self):
        """A facade call and non-root science metadata do not violate the boundary."""
        self.write('use merlin::{downstream, upstream}; fn main() {}')
        self.metadata['packages'][0]['dependencies'].append({'name': 'merlin'})
        self.metadata['packages'].append({
            'name': 'docsis-ofdma', 'manifest_path': '/transitive/Cargo.toml',
            'dependencies': [],
        })
        self.assertEqual(self.result(final=True)['errors'], [])

    def test_renamed_scientific_dependency_is_rejected(self):
        """The actual Cargo package, not a harmless alias, selects the boundary."""
        self.metadata['packages'][0]['dependencies'].append({
            'name': 'fec-codec', 'rename': 'chart_helper', 'kind': 'normal',
        })
        self.assertTrue(self.result()['errors'])
        self.assertEqual(self.result()['scientific_dependencies'], 1)

    def test_unknown_scientific_package_cannot_bypass_a_name_blacklist(self):
        """A new FFT/codec package also needs a narrow infrastructure justification."""
        self.metadata['packages'][0]['dependencies'] = [{
            'name': 'rustfft', 'rename': 'screen_helper',
        }]
        with self.assertRaisesRegex(ValueError, 'Unreviewed Qt dependency identity'):
            self.result()

    def test_resource_build_tools_are_narrow_infrastructure_exceptions(self):
        """Only the reviewed build-only resource tool identities are admitted."""
        self.metadata['packages'][0]['dependencies'] = [
            {'name': 'cc', 'kind': 'build'},
            {'name': 'qt-build-utils', 'kind': 'build'},
        ]
        self.assertEqual(self.result(final=True)['errors'], [])
        self.assertEqual(self.result(final=True)['scientific_dependencies'], 0)

    def test_resource_build_tools_cannot_become_runtime_or_dev_dependencies(self):
        """Aliases and platform guards cannot widen the build-only exceptions."""
        for name in ('cc', 'qt-build-utils'):
            for kind in (None, 'normal', 'dev'):
                with self.subTest(name=name, kind=kind):
                    self.metadata['packages'][0]['dependencies'] = [{
                        'name': name, 'kind': kind, 'rename': 'resource_helper',
                        'target': 'cfg(windows)',
                    }]
                    with self.assertRaisesRegex(ValueError, 'must be build-only'):
                        self.result()

    def test_path_identity_detects_alias_without_scientific_declared_name(self):
        """A local package name is inspected even when the requested name differs."""
        crate = self.root / 'library'
        crate.mkdir()
        (crate / 'Cargo.toml').write_text('[package]\nname = "merlin-dsp"\nversion = "0.1.0"\n')
        self.metadata['packages'][0]['dependencies'].append({
            'name': 'screen_utils', 'path': str(crate),
        })
        self.assertTrue(self.result()['errors'])
        records = dependency_records(self.root, self.metadata)
        self.assertEqual(records[0]['package'], 'merlin-dsp')
        self.assertEqual(records[0]['declared_package'], 'screen_utils')

    def test_dev_build_and_target_specific_dependencies_are_not_exempt(self):
        """Scientific access from tests/build scripts/another target is also debt."""
        for kind in ('dev', 'build', None):
            with self.subTest(kind=kind):
                self.metadata['packages'][0]['dependencies'] = [{
                    'name': 'docsis-ofdma', 'kind': kind, 'target': 'cfg(windows)',
                }]
                self.assertTrue(self.result()['errors'])

    def test_qualified_import_and_extern_alias_are_rejected(self):
        """All three independent Rust spellings reveal the actual crate root."""
        for source in (
                'use docsis_ofdm_fec::simulation::channel;',
                'fn main() { ::docsis_ofdma::simulation::run(); }',
                'extern crate fec_codec as decoder;'):
            with self.subTest(source=source):
                self.write(source)
                self.assertTrue(self.result()['errors'])

    def test_renamed_crate_root_is_detected_in_source(self):
        """A dependency-level alias is also a scientific source identifier."""
        self.metadata['packages'][0]['dependencies'] = [{
            'name': 'docsis-ofdma', 'rename': 'hidden_engine',
        }]
        self.legacy['dependencies'] = dependency_records(self.root, self.metadata)
        self.write('use hidden_engine as science;')
        self.assertTrue(any('new direct scientific' in e for e in self.result()['errors']))

    def test_comments_raw_strings_and_lifetimes_are_not_imports(self):
        """Examples in literals/comments are inert; real Rust lifetimes survive."""
        self.write('''// docsis_ofdma::simulation::run()
/* fec_codec */
const HELP: &str = r#"use pnm_input;"#;
fn view<'input>(input: &'input str) { let pnm_status = input; }
''')
        self.assertEqual(self.result()['errors'], [])

    def test_pnm_picker_boolean_is_not_a_scientific_crate_reference(self):
        """A native chooser's existing format flag must remain infrastructure."""
        self.write('fn chooser(pnm: bool) { if pnm { println!("PNM"); } }')
        self.assertEqual(self.result()['errors'], [])

    def test_grouped_bare_imports_and_multiline_paths_are_detected(self):
        """Grouped aliases and whitespace cannot hide the actual crate root."""
        for text in ('use {pnm, merlin::common};',
                     'use {fec_codec as codec, merlin::common};',
                     'fn main() { docsis_ofdma\n ::simulation::run(); }'):
            with self.subTest(text=text):
                self.write(text)
                self.assertTrue(self.result()['errors'])

    def test_legacy_file_is_exact_and_cannot_gain_an_alias_call(self):
        """An unchanged import with a new later alias invocation still fails."""
        source = 'use docsis_ofdma::simulation as legacy;\nfn main() {}\n'
        path = self.write(source)
        self.legacy['files'] = [{
            'path': 'apps/qt/src/main.rs', 'sha256': digest(path.read_bytes()),
            'sites': [{'line': 1, 'root': 'docsis_ofdma'}],
            'sunset_task': 'UIU-DS-01',
        }]
        self.assertEqual(self.result()['errors'], [])
        self.write(source + 'fn new_operation() { legacy::run(); }\n')
        self.assertTrue(any('changed frozen' in e for e in self.result()['errors']))

    def test_stale_file_record_and_missing_sunset_are_rejected(self):
        """Deleted/renamed files and unspecified exception owners cannot linger."""
        self.legacy['files'] = [{
            'path': 'apps/qt/src/removed.rs', 'sha256': '0' * 64, 'sites': [],
        }]
        self.assertTrue(any('stale' in e for e in self.result()['errors']))
        path = self.write('use pnm;')
        self.legacy['files'] = [{
            'path': 'apps/qt/src/main.rs', 'sha256': digest(path.read_bytes()),
            'sites': [{'line': 1, 'root': 'pnm'}],
        }]
        self.assertTrue(any('sunset' in e for e in self.result()['errors']))

    def test_final_mode_refuses_remaining_legacy_debt(self):
        """A valid migration snapshot is still not final API acceptance."""
        self.metadata['packages'][0]['dependencies'] = [{'name': 'pnm-input'}]
        self.legacy['dependencies'] = dependency_records(self.root, self.metadata)
        self.assertEqual(self.result()['errors'], [])
        self.assertTrue(any('zero scientific' in e for e in self.result(final=True)['errors']))

    def test_external_module_paths_and_include_are_rejected(self):
        """Literal source injection cannot bypass dependency-root inspection."""
        for source in ('#[path = "../../../crates/ofdm/src/lib.rs"] mod engine;',
                       'include!("../../../crates/ofdma/src/lib.rs");'):
            with self.subTest(source=source):
                self.write(source)
                self.assertTrue(any('external Rust' in e for e in self.result()['errors']))

    def test_owned_build_modules_remain_scientifically_audited(self):
        """Build helpers are owned sources, with the same scientific-call guard."""
        build = self.root / 'apps/qt/build.rs'
        build.write_text('#[path = "build_support/branding.rs"] mod branding;')
        helper = self.root / 'apps/qt/build_support/branding.rs'
        helper.parent.mkdir()
        helper.write_text('fn resources() {}')
        self.assertEqual(self.result(final=True)['errors'], [])
        helper.write_text('fn resources() { docsis_ofdma::simulation::run(); }')
        self.assertTrue(any('new direct scientific' in e for e in self.result()['errors']))
        helper.write_text('include!("../../../crates/ofdma/src/lib.rs");')
        self.assertTrue(any('external Rust' in e for e in self.result()['errors']))

    def test_runtime_sources_cannot_include_build_helpers(self):
        """The build-support source exception does not widen runtime includes."""
        self.write('#[path = "../build_support/branding.rs"] mod branding;')
        self.assertTrue(any('external Rust' in e for e in self.result()['errors']))

    def test_wrong_metadata_identity_and_record_schema_fail_closed(self):
        """Absent metadata or an unknown exception schema is never a pass."""
        self.metadata['packages'][0]['name'] = 'unrelated'
        with self.assertRaises(ValueError):
            self.result()
        self.metadata['packages'][0]['name'] = 'merlin-qt'
        self.legacy['schema_version'] = 99
        with self.assertRaises(ValueError):
            self.result()

    def test_original_buffers_are_unchanged(self):
        """Auditing owns no caller metadata or exception mutation rights."""
        before = copy.deepcopy((self.metadata, self.legacy))
        self.write('fn main() {}')
        self.assertEqual(self.result()['errors'], [])
        self.assertEqual((self.metadata, self.legacy), before)
        self.assertEqual(scientific_sites('\nuse pnm::parser;', {'pnm'}),
                         [{'line': 2, 'root': 'pnm'}])


if __name__ == '__main__':
    unittest.main()
