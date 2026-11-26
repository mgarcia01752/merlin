#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native namespaces, legacy imports, typing and shared cancellation."""
import ast
import importlib
import importlib.util
import inspect
import json
from pathlib import Path
import pkgutil
import sys
import threading
import time
import unittest

import merlin
from merlin import common, downstream, upstream

ROOT = Path(__file__).resolve().parents[3]


class NamespaceImports(unittest.TestCase):
    def test_installed_imports_and_legacy_identity(self):
        """Real files load outside the checkout and old classes retain identity."""
        native = merlin._native
        self.assertTrue(Path(native.__file__).resolve().is_relative_to(Path(sys.prefix).resolve()))
        self.assertTrue(str(native.__file__).endswith(('.so', '.pyd')))
        legacy = importlib.import_module('merlin.merlin')
        for name in downstream.__all__:
            self.assertIs(getattr(merlin, name), getattr(downstream, name), name)
            self.assertIs(getattr(legacy, name), getattr(downstream, name), name)
        self.assertIs(merlin.Simulation, merlin.Merlin)
        self.assertIs(common.RunControl, merlin.RunControl)
        self.assertIs(common.ErrorCode, merlin.ErrorCode)
        self.assertIs(upstream.ErrorCode, common.ErrorCode)
        self.assertEqual(merlin.Simulation.__module__, 'merlin')
        self.assertEqual(upstream.Simulation.__module__, 'merlin.upstream')
        self.assertIsNot(upstream.Simulation, downstream.Simulation)
        with self.assertRaises(upstream.ApiError) as missing:
            upstream.Simulation().preview()
        self.assertEqual(missing.exception.code, common.ErrorCode.MissingSetting)
        self.assertEqual(missing.exception.legacy_code, 'MissingSetting')
        self.assertEqual(missing.exception.engine_code, 'MissingSetting')
        self.assertIsNone(missing.exception.field)
        self.assertIsNone(missing.exception.offset)

    def test_discovery_and_typing_payload(self):
        """Installed files expose real import specs, discovery and PEP561 metadata."""
        package = Path(merlin.__file__).parent
        discovered = {entry.name for entry in pkgutil.iter_modules(merlin.__path__)}
        for name in ('downstream', 'upstream', 'common', 'merlin'):
            self.assertIn(name, discovered)
            spec = importlib.util.find_spec('merlin.' + name)
            self.assertIsNotNone(spec)
            self.assertEqual(Path(spec.origin).suffix, '.py')
            self.assertTrue((package / (name + '.pyi')).is_file())
        self.assertTrue((package / '__init__.pyi').is_file())
        self.assertTrue((package / 'py.typed').is_file())
        binaries = [p for p in package.iterdir() if p.suffix in ('.so', '.pyd')]
        self.assertEqual(len(binaries), 1)
        self.assertTrue(binaries[0].name.startswith('_native.'))

    def test_common_control_bridge(self):
        """Cancelling the common token releases a real paused native worker."""
        shared = common.RunControl()
        control = upstream.RunControl(cancellation=shared)
        self.assertFalse(shared.is_cancelled())
        self.assertFalse(control.is_cancelled())
        control.pause()
        session = upstream.Simulation()
        session.configure_json((ROOT / 'examples/cli/upstream/us-50khz.json').read_bytes())
        reports = []
        failures = []
        def run():
            try:
                reports.append(session.run_with_control(control))
            except BaseException as failure:
                failures.append(failure)
        worker = threading.Thread(target=run)
        worker.start()
        deadline = time.monotonic() + 5
        while json.loads(control.progress_json())['status'] != 'Paused':
            self.assertLess(time.monotonic(), deadline)
            time.sleep(0.001)
        self.assertEqual(json.loads(control.progress_json())['emitted_samples'], 0)
        shared.cancel()
        worker.join(5)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertTrue(shared.is_cancelled())
        self.assertTrue(control.is_cancelled())
        self.assertEqual(reports[0].status, 'Cancelled')
        self.assertEqual(reports[0].generated_source_bits, 0)
        original = reports[0].to_json()
        self.assertTrue(session.run().reference_recovered)
        self.assertEqual(reports[0].to_json(), original)
        another = common.RunControl()
        upstream.RunControl(another).cancel()
        self.assertTrue(another.is_cancelled())

    def test_stub_signatures_and_root_alias(self):
        """Declarations preserve every parameter/default of callable native exports."""
        for module, name in [(merlin, '__init__.pyi'), (upstream, 'upstream.pyi')]:
            tree = ast.parse(Path(merlin.__file__).with_name(name).read_text())
            for cls in (item for item in tree.body if isinstance(item, ast.ClassDef)):
                value = getattr(module, cls.name)
                for item in cls.body:
                    if not isinstance(item, ast.FunctionDef):
                        continue
                    if any(isinstance(d, ast.Name) and d.id == 'property' for d in item.decorator_list):
                        self.assertTrue(hasattr(value, item.name), (cls.name, item.name))
                        continue
                    target = value if item.name == '__init__' else getattr(value, item.name)
                    actual = inspect.signature(target)
                    expected = [*item.args.posonlyargs, *item.args.args]
                    if item.name == '__init__':
                        expected = expected[1:]
                    expected += item.args.kwonlyargs
                    self.assertEqual(list(actual.parameters), [a.arg for a in expected], (cls.name, item.name))
                    defaults = [inspect.Parameter.empty] * (len(item.args.posonlyargs) + len(item.args.args) - len(item.args.defaults))
                    defaults += [ast.literal_eval(v) for v in item.args.defaults]
                    if item.name == '__init__':
                        defaults = defaults[1:]
                    defaults += [inspect.Parameter.empty if v is None else ast.literal_eval(v) for v in item.args.kw_defaults]
                    self.assertEqual([p.default for p in actual.parameters.values()], defaults, (cls.name, item.name))
        self.assertIs(merlin.Merlin, merlin.Simulation)
