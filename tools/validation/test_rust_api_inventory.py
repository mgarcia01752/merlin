#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Guard compiler inventory against wildcard, trait and stale-file omissions."""
from pathlib import Path
import json
import tempfile
import unittest
from unittest.mock import patch

from rust_api_inventory import inventory, main, resolve_reexport_owners


class RustdocExportTests(unittest.TestCase):
    """Use independent minimal rustdoc shapes to verify public export discovery."""

    def common_facade(self, root):
        """Prepare a source-less inline facade trait and its original compiler page.

        Args:
            root: Independent temporary source/documentation root.
        Returns:
            Fresh inventory, facade source, documentation root and canonical page.
        """
        source = root / 'crates/merlin/src/lib.rs'
        source.parent.mkdir(parents=True)
        source.write_text('pub mod common { pub use merlin_api::{Cancellation}; }')
        docs = root / 'doc'
        facade = docs / 'merlin/common'
        facade.mkdir(parents=True)
        (facade / 'index.html').write_text(
            '<a class="trait" href="trait.Cancellation.html" '
            'title="trait merlin::common::Cancellation">Cancellation</a>')
        (facade / 'trait.Cancellation.html').write_text(
            '<h4 class="code-header">fn is_cancelled(&amp;self) -&gt; bool</h4>')
        canonical = docs / 'merlin_api'
        canonical.mkdir()
        (canonical / 'index.html').write_text(
            '<a class="trait" href="trait.Cancellation.html" '
            'title="trait merlin_api::Cancellation">Cancellation</a>')
        page = canonical / 'trait.Cancellation.html'
        page.write_text(
            '<a class="src" href="../src/merlin_api/control.rs.html#18">Source</a>'
            '<h4 class="code-header">fn is_cancelled(&amp;self) -&gt; bool</h4>')
        data = {'common': inventory('merlin::common', facade / 'index.html', docs)}
        return data, source, docs, page

    def test_inline_common_facade_without_source_uses_exact_original_trait_docs(self):
        """The raw merlin_api origin avoids resolving back to the facade itself."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            data, _, docs, _ = self.common_facade(root)
            method = data['common'][0]['methods'][0]
            self.assertIsNone(method['source_file'])
            resolve_reexport_owners(data, root, docs)
            self.assertEqual(method['source_file'], 'crates/phy-api/src/control.rs')
            self.assertEqual(method['reexport_origin'], 'merlin_api::Cancellation')
            self.assertEqual(method['rustdoc'], 'merlin/common/trait.Cancellation.html')

    def test_inline_common_facade_rejects_imports_in_other_or_nested_modules(self):
        """Same-named imports outside the exact inline module never prove origin."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            data, source, docs, _ = self.common_facade(root)
            for code in [
                'pub mod other { pub use merlin_api::{Cancellation}; } pub mod common {}',
                'pub mod other { pub mod common { pub use merlin_api::{Cancellation}; } }',
                'pub mod common { mod other { pub use merlin_api::{Cancellation}; } }',
                'pub mod common { /* pub use merlin_api::{Cancellation}; */ '
                'const TEXT: &str = "pub use merlin_api::{Cancellation};"; }',
            ]:
                with self.subTest(source=code):
                    source.write_text(code)
                    with self.assertRaisesRegex(ValueError, 'unique explicit source'):
                        resolve_reexport_owners(data, root, docs)
            source.write_text('pub mod common { pub use merlin_api::different::{Cancellation}; }')
            with self.assertRaisesRegex(ValueError, 'compiler inventory'):
                resolve_reexport_owners(data, root, docs)

    def test_inline_common_facade_rejects_ambiguous_import_and_compiler_sources(self):
        """Neither source imports nor same-export compiler pages may be guessed."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            data, source, docs, page = self.common_facade(root)
            source.write_text('pub mod common { pub use merlin_api::{Cancellation}; '
                              'pub use docsis_ofdma::api::{Cancellation}; }')
            with self.assertRaisesRegex(ValueError, 'unique explicit source'):
                resolve_reexport_owners(data, root, docs)
            source.write_text('pub mod common { pub use merlin_api::{Cancellation}; }')
            (page.parent / 'index.html').write_text(
                '<a class="trait" href="trait.Cancellation.html" '
                'title="trait merlin_api::Cancellation">Cancellation</a>'
                '<a class="trait" href="trait.Other.html" '
                'title="trait merlin_api::Cancellation">Cancellation</a>')
            (page.parent / 'trait.Other.html').write_text(
                '<a class="src" href="../src/merlin_api/other.rs.html#18">Source</a>'
                '<h4 class="code-header">fn is_cancelled(&amp;self) -&gt; bool</h4>')
            with self.assertRaisesRegex(ValueError, 'compiler inventory'):
                resolve_reexport_owners(data, root, docs)

    def test_inline_common_facade_rejects_signature_kind_and_missing_source_drift(self):
        """An exact import still requires the original compiler kind/signature/source."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            data, _, docs, page = self.common_facade(root)
            page.write_text(
                '<a class="src" href="../src/merlin_api/control.rs.html#18">Source</a>'
                '<h4 class="code-header">fn is_cancelled(&amp;self) -&gt; usize</h4>')
            with self.assertRaisesRegex(ValueError, 'exact canonical owner'):
                resolve_reexport_owners(data, root, docs)
            page.write_text('<h4 class="code-header">fn is_cancelled(&amp;self) -&gt; bool</h4>')
            with self.assertRaisesRegex(ValueError, 'exact canonical owner'):
                resolve_reexport_owners(data, root, docs)
            (page.parent / 'index.html').write_text(
                '<a class="struct" href="struct.Cancellation.html" '
                'title="struct merlin_api::Cancellation">Cancellation</a>')
            (page.parent / 'struct.Cancellation.html').write_text(
                '<a class="src" href="../src/merlin_api/control.rs.html#18">Source</a>'
                '<h4 class="code-header">pub fn is_cancelled(&amp;self) -&gt; bool</h4>')
            with self.assertRaisesRegex(ValueError, 'kind/identity'):
                resolve_reexport_owners(data, root, docs)

    def project_facade(self, root, exposed_name='Project'):
        """Prepare compiler constructor spellings over one proven nominal import.

        Args:
            root: Independent temporary source/documentation root.
            exposed_name: Exact public name, optionally an explicit import alias.
        Returns:
            Documentation root and source-less facade item page.
        """
        source = root / 'crates/merlin/src/lib.rs'
        source.parent.mkdir(parents=True)
        entry = 'Project' if exposed_name == 'Project' else f'Project as {exposed_name}'
        source.write_text(f'pub mod common {{ pub use pnm_input::project::{{{entry}}}; }}')
        docs = root / 'doc'
        facade = docs / 'merlin/common'
        facade.mkdir(parents=True)
        (facade / 'index.html').write_text(
            f'<a class="struct" href="struct.{exposed_name}.html" '
            f'title="struct merlin::common::{exposed_name}">{exposed_name}</a>')
        page = facade / f'struct.{exposed_name}.html'
        page.write_text('<h4 class="code-header">pub fn from_bytes(bytes: &amp;[u8]) '
                        f'-&gt; Result&lt;{exposed_name}, ProjectError&gt;</h4>')
        canonical = docs / 'pnm_input/project'
        canonical.mkdir(parents=True)
        (canonical / 'index.html').write_text(
            '<a class="struct" href="struct.Project.html" '
            'title="struct pnm_input::project::Project">Project</a>')
        (canonical / 'struct.Project.html').write_text(
            '<a class="src" href="../../src/pnm_input/project/mod.rs.html#99">Source</a>'
            '<h4 class="code-header">pub fn from_bytes(bytes: &amp;[u8]) '
            '-&gt; Result&lt;Self, ProjectError&gt;</h4>')
        return docs, page

    def test_nominal_constructor_self_spelling_keeps_both_compiler_signatures(self):
        """A concrete facade return and canonical Self both remain exact compiler evidence."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            docs, page = self.project_facade(root)
            data = {'common': inventory('merlin::common', page.parent / 'index.html', docs)}
            method = data['common'][0]['methods'][0]
            resolve_reexport_owners(data, root, docs)
            self.assertEqual(method['signature'], 'pub fn from_bytes(bytes: &[u8]) -> Result<Project, ProjectError>')
            self.assertEqual(method['reexport_canonical_signature'],
                             'pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProjectError>')
            self.assertEqual(method['source_file'], 'crates/pnm-input/src/project/mod.rs')
            self.assertEqual(method['reexport_origin'], 'pnm_input::project::Project')

    def test_nominal_alias_constructor_uses_only_its_proven_original_type(self):
        """An explicit public alias has the same Self identity without losing either spelling."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            docs, page = self.project_facade(root, 'SavedProject')
            data = {'common': inventory('merlin::common', page.parent / 'index.html', docs)}
            method = data['common'][0]['methods'][0]
            resolve_reexport_owners(data, root, docs)
            self.assertEqual(method['operation'], 'merlin::common::SavedProject::from_bytes')
            self.assertEqual(method['signature'], 'pub fn from_bytes(bytes: &[u8]) -> Result<SavedProject, ProjectError>')
            self.assertEqual(method['reexport_canonical_signature'],
                             'pub fn from_bytes(bytes: &[u8]) -> Result<Self, ProjectError>')
            self.assertEqual(method['reexport_origin'], 'pnm_input::project::Project')

    def test_nominal_signature_equivalence_rejects_other_types_namespaces_and_parameters(self):
        """Only whole nominal tokens change; all other compiler signature text must agree."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            docs, page = self.project_facade(root)
            for signature in [
                'pub fn from_bytes(bytes: &[u8]) -> Result<OtherProject, ProjectError>',
                'pub fn from_bytes(bytes: &[u8]) -> Result<elsewhere::Project, ProjectError>',
                'pub fn from_bytes(bytes: &[u8]) -> Result<Selfish, ProjectError>',
                'pub fn from_bytes(bytes: &mut [u8]) -> Result<Project, ProjectError>',
                'pub fn from_bytes(input: &[u8]) -> Result<Project, ProjectError>',
                'pub fn from_bytes(bytes: &[u8]) -> Result<Project, OtherError>',
            ]:
                with self.subTest(signature=signature):
                    page.write_text('<h4 class="code-header">' + signature.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;') + '</h4>')
                    data = {'common': inventory('merlin::common', page.parent / 'index.html', docs)}
                    with self.assertRaisesRegex(ValueError, 'exact canonical owner'):
                        resolve_reexport_owners(data, root, docs)
                    method = data['common'][0]['methods'][0]
                    self.assertEqual(method['signature'], signature)
                    self.assertNotIn('reexport_canonical_signature', method)

    def test_main_discovers_adapter_reexports_from_the_actual_common_facade(self):
        """A neutral leaf alone cannot enumerate additional facade-owned public exports."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for module in ['docsis_ofdm_fec/api', 'docsis_ofdma/api', 'merlin_api',
                           'pnm_input', 'merlin/common']:
                path = root / module / 'index.html'
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('')
            (root / 'merlin/common/index.html').write_text(
                '<a class="struct" href="../../pnm_input/struct.Project.html" '
                'title="struct pnm_input::Project">Project</a>')
            (root / 'pnm_input/struct.Project.html').write_text(
                '<a class="src" href="../src/pnm_input/project/mod.rs.html">Source</a>'
                '<h4 class="code-header">pub fn direction(&amp;self) -&gt; ProjectDirection</h4>')
            output = root / 'exports.json'
            with patch('sys.argv', ['inventory', '--docs', str(root), '--output', str(output)]):
                main()
            common = json.loads(output.read_text())['common']
            self.assertEqual([item['export'] for item in common], ['merlin::common::Project'])
            self.assertEqual(common[0]['methods'][0]['operation'], 'merlin::common::Project::direction')
            self.assertEqual(common[0]['methods'][0]['source_file'], 'crates/pnm-input/src/project/mod.rs')

    def test_compiler_explicit_rename_preserves_public_name_and_canonical_method_owner(self):
        """A compiler pub-use alias names the caller API; origin names are not extra exports."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'index.html').write_text(
                '<code>pub use owner::<a class="struct" href="struct.Original.html" '
                'title="struct owner::Original">Original</a> as PublicName;</code>')
            (root / 'struct.Original.html').write_text(
                '<h4 class="code-header">pub fn new() -&gt; Self</h4>')
            rows = inventory('public', root / 'index.html', root)
            self.assertEqual([row['export'] for row in rows], ['public::PublicName'])
            self.assertEqual(rows[0]['methods'][0]['operation'], 'public::PublicName::new')
            self.assertEqual(rows[0]['methods'][0]['rustdoc'], 'struct.Original.html')
            (root / 'index.html').write_text(
                '<code>owner::<a class="struct" href="struct.Original.html" '
                'title="struct owner::Original">Original</a> as PublicName</code>')
            self.assertEqual(inventory('public',root / 'index.html',root)[0]['export'],
                             'public::Original')

    def test_glob_keeps_namespace_modules_extend_it_and_stale_items_are_absent(self):
        """A wildcard imports items directly; an actual module adds one component."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "index.html").write_text(
                '<code>pub use crate::<a class="mod" href="config.html" title="mod owner::config">config</a>::*;</code>'
                '<a class="mod" href="nested.html" title="mod owner::nested">nested</a>'
                '<a class="struct" href="https://invalid.test/struct.Remote.html" title="struct Remote">Remote</a>'
            )
            (root / "config.html").write_text('<a class="struct" href="struct.Value.html" title="struct owner::Value">Value</a>')
            (root / "nested.html").write_text('<a class="struct" href="struct.Other.html" title="struct owner::Other">Other</a>')
            for name in ("Value", "Other", "Stale"):
                (root / f"struct.{name}.html").write_text('<h4 class="code-header">pub fn new(value: usize) -&gt; Self</h4>')
            rows = inventory("public", root / "index.html", root)
            self.assertEqual([row["export"] for row in rows], ["public::Value", "public::nested::Other"])
            self.assertEqual(rows[0]["methods"][0]["signature"], "pub fn new(value: usize) -> Self")

    def test_macro_generated_methods_and_required_trait_declarations_are_complete(self):
        """Expanded methods count once; private and implicit trait methods do not."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "index.html").write_text(
                '<a class="struct" href="struct.Count.html" title="struct owner::Count">Count</a>'
                '<a class="trait" href="trait.Stop.html" title="trait owner::Stop">Stop</a>'
            )
            (root / "struct.Count.html").write_text(
                '<a class="src" href="../src/docsis_ofdm_fec/api/units.rs.html#10">Source</a>'
                '<h4 class="code-header">pub const fn new(value: usize) -&gt; Self</h4>'
                '<h4 class="code-header">pub fn value(&amp;self) -&gt; usize</h4>'
                '<h4 class="code-header">fn clone(&amp;self) -&gt; Self</h4>'
                '<h4 class="code-header">pub(crate) fn private(&amp;self)</h4>'
            )
            (root / "trait.Stop.html").write_text(
                '<h4 class="code-header">fn is_cancelled(&amp;self) -&gt; bool</h4>'
                '<h4 class="code-header">fn is_cancelled(&amp;self) -&gt; bool</h4>'
            )
            rows = inventory("public", root / "index.html", root)
            self.assertEqual([method["operation"] for method in rows[0]["methods"]],
                             ["public::Count::new", "public::Count::value"])
            self.assertEqual(rows[0]["methods"][0]["source_file"], "crates/ofdm/src/api/units.rs")
            self.assertEqual(len(rows[1]["methods"]), 1)

    def test_free_functions_keep_one_name_and_missing_current_targets_fail(self):
        """A missing linked function fails instead of falling back to stale files."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "index.html").write_text('<a class="fn" href="fn.load.html" title="fn owner::load">load</a>')
            with self.assertRaises(FileNotFoundError):
                inventory("public", root / "index.html", root)
            (root / "fn.load.html").write_text('<pre class="rust item-decl"><code>pub fn load(bytes: &amp;[u8]) -&gt; Result&lt;Value, Error&gt;</code></pre>')
            rows = inventory("public", root / "index.html", root)
            self.assertEqual(rows[0]["methods"][0]["operation"], "public::load")
            self.assertEqual(rows[0]["methods"][0]["signature"], "pub fn load(bytes: &[u8]) -> Result<Value, Error>")

    def test_external_reexport_without_source_links_uses_exact_origin_not_type_name(self):
        """Two same-named PHY types remain distinct when rustdoc omits source links."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'crates/pnm-input/src/upstream/mod.rs'
            source.parent.mkdir(parents=True)
            source.write_text('pub use docsis_ofdma::api::{ApiError, ApiResult};\n')
            def item(export, owner):
                return {'export': export, 'kind': 'enum', 'methods': [
                    {'operation': export + '::code', 'signature': 'pub fn code(&self) -> Code', 'source_file': owner}]}
            data = {'downstream': [item('merlin::downstream::ApiError', 'crates/ofdm/src/api/error.rs')],
                    'upstream': [item('merlin::upstream::ApiError', 'crates/ofdma/src/api/error.rs')],
                    'native-input': [item('pnm_input::upstream::ApiError', None)]}
            resolve_reexport_owners(data, root)
            method = data['native-input'][0]['methods'][0]
            self.assertEqual(method['source_file'], 'crates/ofdma/src/api/error.rs')
            self.assertEqual(method['reexport_origin'], 'merlin::upstream::ApiError')
            method['source_file'] = None
            source.write_text('// pub use docsis_ofdma::api::{ApiError};\n')
            with self.assertRaisesRegex(ValueError, 'unique explicit source'):
                resolve_reexport_owners(data, root)

    def test_explicit_scientific_reexport_resolves_canonical_compiler_owner_and_rejects_drift(self):
        """Only the declared module and exact compiler signature establish provenance."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'crates/pnm-input/src/lib.rs'
            source.parent.mkdir(parents=True)
            source.write_text('pub use docsis_ofdm_fec::signal_processing::rxmer_designer::{CaptureScope};')
            docs = root / 'doc'
            module = docs / 'docsis_ofdm_fec/signal_processing/rxmer_designer'
            module.mkdir(parents=True)
            (module / 'index.html').write_text('<a class="enum" href="enum.CaptureScope.html" title="enum owner::CaptureScope">CaptureScope</a>')
            page = module / 'enum.CaptureScope.html'
            page.write_text('<a class="src" href="../../../src/docsis_ofdm_fec/signal_processing/rxmer_designer/config.rs.html#10">Source</a><h4 class="code-header">pub fn validate(&amp;self) -&gt; Result&lt;(), Error&gt;</h4>')
            method = {'operation': 'pnm_input::CaptureScope::validate', 'signature': 'pub fn validate(&self) -> Result<(), Error>', 'source_file': None}
            data = {'native-input': [{'export': 'pnm_input::CaptureScope', 'kind': 'enum', 'methods': [method]}]}
            resolve_reexport_owners(data, root, docs)
            self.assertEqual(method['source_file'], 'crates/ofdm/src/signal_processing/rxmer_designer/config.rs')
            self.assertEqual(method['reexport_origin'], 'docsis_ofdm_fec::signal_processing::rxmer_designer::CaptureScope')
            method['source_file'] = None
            method['signature'] = 'pub fn validate(&self, replacement: bool) -> Result<(), Error>'
            with self.assertRaisesRegex(ValueError, 'exact canonical owner'):
                resolve_reexport_owners(data, root, docs)
            source.write_text('pub use unknown::signal_processing::{CaptureScope};')
            with self.assertRaisesRegex(ValueError, 'unique explicit source'):
                resolve_reexport_owners(data, root, docs)

    def test_two_explicit_import_origins_never_choose_a_same_named_type(self):
        """Ambiguous explicit imports reject instead of guessing source ownership."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'crates/pnm-input/src/lib.rs'
            source.parent.mkdir(parents=True)
            source.write_text('pub use docsis_ofdm_fec::api::{ApiError}; pub use docsis_ofdma::api::{ApiError};')
            data = {'native-input': [{'export': 'pnm_input::ApiError', 'kind': 'enum', 'methods': [
                {'operation': 'pnm_input::ApiError::code', 'signature': 'pub fn code(&self) -> Code', 'source_file': None}]}]}
            with self.assertRaisesRegex(ValueError, 'unique explicit source'):
                resolve_reexport_owners(data, root)

    def test_compiler_linked_primitive_alias_allows_exact_expansion_but_not_type_drift(self):
        """Rustdoc may expand an imported alias; only its actual declaration resolves it."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'crates/pnm-input/src/lib.rs'
            source.parent.mkdir(parents=True)
            source.write_text('pub use docsis_ofdm_fec::signal_processing::rxmer_designer::{RippleSpacing};')
            docs = root / 'doc'
            module = docs / 'docsis_ofdm_fec/signal_processing/rxmer_designer'
            module.mkdir(parents=True)
            (module / 'index.html').write_text('<a class="enum" href="enum.RippleSpacing.html" title="enum owner::RippleSpacing">RippleSpacing</a>')
            (module / 'enum.RippleSpacing.html').write_text('<a class="src" href="../../../src/docsis_ofdm_fec/signal_processing/rxmer_designer.rs.html#10">Source</a><h4 class="code-header">pub fn frequency_hz(&amp;self) -&gt; <a class="type" href="../../types/type.FrequencyHz.html" title="type owner::FrequencyHz">FrequencyHz</a></h4>')
            types = docs / 'docsis_ofdm_fec/types'
            types.mkdir()
            alias = types / 'type.FrequencyHz.html'
            alias.write_text('<pre class="item-decl">pub type FrequencyHz = u32;</pre>')
            method = {'operation': 'pnm_input::RippleSpacing::frequency_hz', 'signature': 'pub fn frequency_hz(&self) -> u32', 'source_file': None}
            data = {'native-input': [{'export': 'pnm_input::RippleSpacing', 'kind': 'enum', 'methods': [method]}]}
            resolve_reexport_owners(data, root, docs)
            self.assertEqual(method['source_file'], 'crates/ofdm/src/signal_processing/rxmer_designer.rs')
            method['source_file'] = None
            alias.write_text('<pre class="item-decl">pub type FrequencyHz = u64;</pre>')
            with self.assertRaisesRegex(ValueError, 'exact canonical owner'):
                resolve_reexport_owners(data, root, docs)


if __name__ == "__main__":
    unittest.main()
