#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Guard compiler inventory against wildcard, trait and stale-file omissions."""
from pathlib import Path
import tempfile
import unittest

from rust_api_inventory import inventory, resolve_reexport_owners


class RustdocExportTests(unittest.TestCase):
    """Use independent minimal rustdoc shapes to verify public export discovery."""

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


if __name__ == "__main__":
    unittest.main()
