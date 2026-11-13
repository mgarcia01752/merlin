#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Desktop MSI identity, safety, shortcuts and native runtime staging contracts."""
import hashlib
import json
import os
from pathlib import Path
import struct
import tempfile
import unittest
import xml.etree.ElementTree as ET
from unittest.mock import patch
from common import ROOT, version
from asset_names import asset_name
from windows_desktop import ARTWORK, NS, UPGRADE_CODE, author, identifier, package, smoke

NAMESPACE = {'w': NS}


class WindowsDesktopTests(unittest.TestCase):
    def test_stable_components_upgrade_icon_and_unchecked_shortcut(self):
        """Adding a file must not change identities for existing installed paths."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            payload = root / 'payload'
            (payload / 'qml').mkdir(parents=True)
            (payload / 'merlin-qt.exe').write_bytes(b'synthetic executable')
            (payload / 'qml/Module.dll').write_bytes(b'synthetic module')
            first = author(payload, root / 'first.wxs', '0.2.0')
            (payload / 'a.txt').write_text('new file', encoding='ascii')
            second = author(payload, root / 'second.wxs', '0.2.1')
            original = ET.parse(first)
            updated = ET.parse(second)
            for relative in ('merlin-qt.exe', 'qml/Module.dll'):
                query = f".//w:Component[@Id='{identifier(Path(relative))}']"
                self.assertEqual(original.find(query, NAMESPACE).attrib,
                                 updated.find(query, NAMESPACE).attrib)
            product = original.find('w:Product', NAMESPACE)
            self.assertEqual(product.attrib['UpgradeCode'], UPGRADE_CODE)
            self.assertNotEqual(UPGRADE_CODE, __import__('windows').UPGRADE_CODE)
            self.assertIsNotNone(product.find('w:MajorUpgrade', NAMESPACE))
            self.assertEqual(product.find("w:Icon[@Id='ApplicationIcon']", NAMESPACE).attrib['SourceFile'], str(ARTWORK / 'merlin.ico'))
            choice = product.find("w:Property[@Id='DESKTOP_SHORTCUT']", NAMESPACE)
            self.assertEqual(choice.attrib, {'Id': 'DESKTOP_SHORTCUT', 'Secure': 'yes'})
            self.assertEqual(product.find(".//w:Component[@Id='DesktopShortcut']/w:Condition", NAMESPACE).text,
                             'DESKTOP_SHORTCUT = "1"')
            self.assertIsNotNone(product.find(".//w:Control[@Type='CheckBox'][@Property='DESKTOP_SHORTCUT']", NAMESPACE))
            self.assertIsNotNone(product.find(".//w:Shortcut[@Id='MenuLink']", NAMESPACE))
            with self.assertRaises(FileExistsError):
                author(payload, first, '0.2.0')

    def test_invalid_msi_versions_case_collisions_and_existing_output_reject(self):
        """Unsafe versions/Windows filename aliases cannot become an MSI payload."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'merlin-qt.exe').touch()
            for number in ('256.0.0', '0.256.0', '1.0.65536', '01.0.0', '../0.1.0'):
                with self.assertRaises(ValueError):
                    author(root, root / 'out.wxs', number)
            (root / 'A.txt').touch()
            (root / 'a.txt').touch()
            with self.assertRaises(ValueError):
                author(root, root / 'out.wxs', '0.2.0')
            sentinel = root / 'keep'
            sentinel.write_bytes(b'keep')
            with self.assertRaises(FileExistsError):
                package(root / 'missing', root / 'sdk', root / 'crt', root)
            self.assertEqual(sentinel.read_bytes(), b'keep')

    def test_native_payload_deploys_qml_crt_notices_and_only_desktop_asset(self):
        """Mock only native tools; execute the real authoring/staging pipeline."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / 'merlin-qt.exe'
            binary.write_bytes(b'fixture')
            crt = root / 'crt'
            crt.mkdir()
            for name in ('msvcp140.dll', 'vcruntime140.dll', 'vcruntime140_1.dll'):
                (crt / name).write_bytes(b'CRT fixture')
            output = root / 'assets'
            calls = []
            def run(arguments, **kwargs):
                calls.append(arguments)
                if str(arguments[0]).endswith('windeployqt.exe'):
                    stage = Path(arguments[-1]).parent
                    for name in ('Qt6Core.dll', 'Qt6Quick.dll', 'platforms/qwindows.dll'):
                        path = stage / name
                        path.parent.mkdir(parents=True, exist_ok=True)
                        path.write_bytes(b'Qt fixture')
                if str(arguments[0]).endswith('light.exe'):
                    Path(arguments[arguments.index('-out') + 1]).write_bytes(b'MSI fixture')
            def populate(stage):
                (stage / 'notices').mkdir()
            def check_output(arguments, **kwargs):
                return '6.8.3\n' if '-query' in arguments else f'MERLIN {__import__("common").version()}\n'
            with patch('windows_desktop.platform.system', return_value='Windows'), \
                    patch('windows_desktop.platform.machine', return_value='AMD64'), \
                    patch('windows_desktop.subprocess.run', side_effect=run), \
                    patch('windows_desktop.subprocess.check_output', side_effect=check_output), \
                    patch('windows_desktop.populate', side_effect=populate), \
                    patch('windows_desktop.smoke') as smoke:
                artifact = package(binary, root / 'sdk', crt, output)
            self.assertEqual({p.name for p in output.iterdir()}, {artifact.name, 'SHA256SUMS'})
            self.assertEqual(artifact.name, asset_name('desktop', __import__('common').version(), 'windows', 'amd64', 'msi'))
            self.assertIn('--qmldir', calls[0])
            self.assertTrue((root / 'assets-stage/msvcp140.dll').is_file())
            self.assertTrue((root / 'assets-stage/notices/wix-ui-LICENSE.txt').is_file())
            smoke.assert_called_once()
            self.assertIn(hashlib.sha256(artifact.read_bytes()).hexdigest(), (output / 'SHA256SUMS').read_text())

    def test_smoke_uses_windows_environment_keys_and_rejects_qml_errors(self):
        """Windows environment copies uppercase keys; SDK paths cannot leak in."""
        with tempfile.TemporaryDirectory() as directory:
            payload = Path(directory)
            examples = payload / 'files/examples/workbench'
            examples.mkdir(parents=True)
            (examples / 'manifest.json').write_text(
                json.dumps({'bundles': [{'workbench': 'one.merlin.json'}]}), encoding='utf-8')
            with patch.dict(os.environ, {'SYSTEMROOT': 'C:/Windows', 'PATH': 'developer-sdk',
                                         'QT_PLUGIN_PATH': 'developer-plugins'}, clear=True), \
                    patch('windows_desktop.subprocess.check_output', return_value=f'MERLIN {version()}\n'), \
                    patch('windows_desktop.subprocess.run') as run:
                run.return_value.stderr = ''
                smoke(payload)
                for call in run.call_args_list:
                    environment = call.kwargs['env']
                    self.assertNotIn('developer-sdk', environment['PATH'])
                    self.assertNotIn('QT_PLUGIN_PATH', environment)
                    self.assertEqual(environment['QT_QPA_PLATFORM'], 'windows')
                run.return_value.stderr = 'A required QML module is not installed'
                with self.assertRaises(ValueError):
                    smoke(payload)

    def test_branding_formats_sizes_and_source_provenance(self):
        """Installer assets have the exact native dimensions and verified origin."""
        data = (ARTWORK / 'merlin.ico').read_bytes()
        reserved, kind, count = struct.unpack_from('<HHH', data)
        self.assertEqual((reserved, kind), (0, 1))
        sizes = {data[6 + 16 * index] or 256 for index in range(count)}
        self.assertEqual(sizes, {16, 20, 24, 32, 40, 48, 64, 128, 256})
        for name, expected in [('banner.bmp', (493, 58)), ('dialog.bmp', (493, 312))]:
            data = (ARTWORK / name).read_bytes()
            self.assertEqual(data[:2], b'BM')
            self.assertEqual(struct.unpack_from('<ii', data, 18), expected)
            self.assertEqual(struct.unpack_from('<H', data, 28)[0], 24)
        provenance = json.loads((ARTWORK / 'provenance.json').read_text())
        self.assertEqual(hashlib.sha256((ROOT / provenance['source']).read_bytes()).hexdigest(), provenance['source_sha256'])
        for name, digest in provenance['outputs'].items():
            self.assertEqual(hashlib.sha256((ARTWORK / name).read_bytes()).hexdigest(), digest)
