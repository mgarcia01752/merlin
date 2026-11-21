# Copyright 2026 MORGANA contributors
# SPDX-License-Identifier: Apache-2.0
# Adapted for MERLIN from 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
# Changes: namespaces, fixture owners and shared typed error categories.
"""Actual native execution and independent Rust-example parity; no Python DSP."""
from __future__ import annotations

import ast
import json
import inspect
import hashlib
import os
import signal
import subprocess
import sys
from pathlib import Path
import threading
import time
import unittest

import merlin.upstream as upstream


def us_configuration() -> upstream.UsConfiguration:
    """Construct exactly the independently run Rust example's typed settings."""
    waveform = upstream.Waveform(50, 192, 64, 6, 20_000_000)
    slots = [upstream.Minislot(128 + 8 * index, 4, 1) for index in range(8)]
    profile = upstream.Profile(224, 5, 0x7FFFFF, slots)
    grant = upstream.SyntheticGrant(1, 224, 0, 0, 2, list(range(8)))
    us = upstream.UsConfiguration(waveform, [profile], grant, [0, 1])
    return us


def configuration() -> upstream.ExperimentConfig:
    """Compose the shared domain candidate without Python scientific algorithms."""
    return upstream.ExperimentConfig(us_configuration(), 1, upstream.ReceiverConfig())


class NativeTests(unittest.TestCase):
    def test_actual_native_exports_defaults_errors_and_lifetime(self) -> None:
        """Runtime imports/structured errors are real; reports outlive sessions."""
        self.assertTrue(upstream.Simulation.__module__.startswith("merlin.upstream"))
        sim = upstream.Simulation()
        with self.assertRaises(upstream.ApiError) as missing:
            sim.preview()
        self.assertEqual(missing.exception.code, upstream.ErrorCode.MissingSetting)
        sim.configure(configuration())
        self.assertEqual(sim.preview().profile_ids, [224])
        report = sim.run()
        self.assertTrue(report.reference_recovered)
        self.assertEqual(report.bit_errors, 0)
        self.assertEqual(report.ber, 0.0)
        before = report.to_json()
        sim.run()
        del sim
        self.assertEqual(report.to_json(), before)
        self.assertTrue(report.qam("RxEqualized", 4, 20).values)
        self.assertEqual(len(report.spectrum("TxOutput", 50).power_per_hz), 2048)
        with self.assertRaises((OverflowError, ValueError)):
            upstream.ExperimentOptions(grants=-1)
        with self.assertRaises((OverflowError, ValueError)):
            upstream.Minislot(0, 999, 1)
        with self.assertRaises(upstream.ApiError) as unsupported:
            upstream.Waveform(100, 192, 64, 6, 20_000_000)
        self.assertEqual(unsupported.exception.code, upstream.ErrorCode.UnsupportedCapability)

    def test_independently_executed_rust_and_python_scientific_parity(self) -> None:
        """Compare actual native runs with a separate Rust example process."""
        rust = json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_REFERENCE_JSON"]).read_text())
        sim = upstream.Simulation()
        sim.configure(configuration())
        native = json.loads(sim.run().to_json())
        for key in ["configuration", "options", "planned", "status", "generated_source_bits", "compared_source_bits", "bit_errors", "ber", "reference_recovered", "not_started_grants"]:
            self.assertEqual(native[key], rust[key], key)
        self.assertEqual([grant["words"] for grant in native["grants"]], [grant["words"] for grant in rust["grants"]])

    def test_gil_release_busy_and_cancel_restore_native_session(self) -> None:
        """Python polls/cancels during real Rust work and session calls report Busy."""
        sim = upstream.Simulation(upstream.ExperimentOptions(grants=128, chunk_samples=313))
        sim.configure(configuration())
        control = upstream.RunControl()
        results = []
        worker = threading.Thread(target=lambda: results.append(sim.run_with_control(control)))
        worker.start()
        deadline = time.monotonic() + 10
        while json.loads(control.progress_json())["compared_bits"] == 0:
            self.assertLess(time.monotonic(), deadline)
            time.sleep(0.001)
        with self.assertRaises(upstream.ApiError) as busy:
            sim.preview()
        self.assertEqual(busy.exception.code, upstream.ErrorCode.Busy)
        control.cancel()
        worker.join(10)
        self.assertFalse(worker.is_alive())
        report = results[0]
        self.assertEqual(report.status, "Cancelled")
        self.assertGreater(report.compared_source_bits, 0)
        self.assertFalse(report.reference_recovered)
        self.assertEqual(report.bit_errors, 0)
        sim.set_options(upstream.ExperimentOptions())
        self.assertTrue(sim.run().reference_recovered)

    def test_pause_resume_and_cancel_from_pause(self) -> None:
        """Acknowledged pause retains native work and cancellation wakes it."""
        sim = upstream.Simulation(upstream.ExperimentOptions(grants=32, chunk_samples=127))
        sim.configure(configuration())
        control = upstream.RunControl()
        control.pause()
        results = []
        worker = threading.Thread(target=lambda: results.append(sim.run_with_control(control)))
        worker.start()
        deadline = time.monotonic() + 10
        while json.loads(control.progress_json())["status"] != "Paused":
            self.assertLess(time.monotonic(), deadline)
            time.sleep(0.001)
        frozen = control.progress_json()
        time.sleep(0.05)
        self.assertEqual(control.progress_json(), frozen)
        control.resume()
        while json.loads(control.progress_json())["compared_bits"] == 0:
            self.assertLess(time.monotonic(), deadline)
            time.sleep(0.001)
        control.pause()
        while json.loads(control.progress_json())["status"] != "Paused":
            self.assertLess(time.monotonic(), deadline)
            time.sleep(0.001)
        control.cancel()
        worker.join(10)
        self.assertFalse(worker.is_alive())
        self.assertEqual(results[0].status, "Cancelled")
        self.assertGreater(results[0].compared_source_bits, 0)

    def test_bounded_conversion_and_actual_interpreter_interrupt(self) -> None:
        """Oversized input rejects before conversion; SIGINT returns a report."""
        with self.assertRaises(upstream.ApiError):
            upstream.ChannelConfig(taps=[(1.0, 0.0)] * 257)
        with self.assertRaises(upstream.ApiError):
            upstream.ExperimentOptions(retained_taps=["TxOutput"] * 10)
        with self.assertRaises(TypeError):
            upstream.ExperimentOptions(unknown_setting=1)
        code = """import json, os, signal, threading, time
from test_upstream_experiment import configuration
import merlin.upstream as upstream
sim = upstream.Simulation(upstream.ExperimentOptions(grants=128, chunk_samples=127))
sim.configure(configuration())
control = upstream.RunControl()
def interrupt():
    while json.loads(control.progress_json())["compared_bits"] == 0:
        time.sleep(0.001)
    os.kill(os.getpid(), signal.SIGINT)
threading.Thread(target=interrupt, daemon=True).start()
report = sim.run_with_control(control)
assert report.status == "Cancelled", report.status
assert report.compared_source_bits > 0
assert sim.preview().source_bits > 0
print("partial retained")
"""
        environment = dict(os.environ)
        environment["PYTHONPATH"] = os.pathsep.join([str(Path(__file__).parent), environment.get("PYTHONPATH", "")])
        result = subprocess.run([sys.executable, "-c", code], env=environment, text=True, capture_output=True, timeout=20)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("partial retained", result.stdout)

    def test_measurement_playback_actual_rust_native_parity(self) -> None:
        """Compare native scheduled residual science with a separate Rust process."""
        us = us_configuration()
        first = upstream.SyntheticRxmer(us, 1, [35.0] * 2048)
        second = upstream.SyntheticRxmer(us, 2, [45.0] * 2048)
        playback = upstream.RxmerPlayback(us, [first, second], "interpolate", [0, 6])
        self.assertEqual(json.loads(playback.at_json(3))["contributors"], [[1, 0.5], [2, 0.5]])
        unity = upstream.PreEqBank.unity(us)
        updates = upstream.PreEqPlayback(us, 1, unity, [upstream.PreEqEvent.snapshot(2, 6, unity)], [])
        self.assertEqual(updates.coefficients(6)[128], (1.0, 0.0))
        sim = upstream.Simulation()
        sim.configure(upstream.ExperimentConfig(us, 1, rxmer=playback, pre_eq_updates=updates))
        actual = json.loads(sim.run().to_json())
        expected = json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_MEASUREMENTS_JSON"]).read_text())
        for key in ["configuration", "options", "planned", "status", "compared_source_bits", "bit_errors", "reference_recovered", "noise_generator"]:
            self.assertEqual(actual[key], expected[key], key)
        for key in ["words", "matched_data_mer", "matched_data_mer_value"]:
            self.assertEqual(actual["grants"][0][key], expected["grants"][0][key], key)
        probe = upstream.ProbeReference(10, 1)
        update = upstream.PreEqEvent.adjust(3, 6, 1, 10, [(128, (-1.0, 0.0))])
        nonunity = upstream.PreEqPlayback(us, 1, unity, [update], [probe])
        self.assertEqual(nonunity.coefficients(6)[128], (-1.0, 0.0))
        with self.assertRaises(upstream.ApiError) as incompatible:
            upstream.ExperimentConfig(us, 1, rxmer=playback, pre_eq_updates=nonunity)
        self.assertEqual(incompatible.exception.code, upstream.ErrorCode.UnsupportedCapability)
        with self.assertRaises(upstream.ApiError):
            upstream.ChannelConfig(noise_seed=b"bad")

    def test_generic_json_actual_native_load_inventory_and_run_parity(self) -> None:
        """The same versioned producer fixture runs through independent consumers."""
        root = Path(__file__).resolve().parents[3]
        data = (root / "examples/cli/upstream/us-50khz.json").read_bytes()
        document = upstream.ConfigurationDocument.from_json(data)
        self.assertEqual(document.inventory(), ["generic-50khz"])
        self.assertEqual(document.original_bytes(), data)
        self.assertEqual(document.original_sha256(), hashlib.sha256(data).hexdigest())
        self.assertEqual(json.loads(document.to_json())["schema_version"], 1)
        sim = upstream.Simulation()
        sim.configure_document(document)
        expected = json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_DOCUMENT_JSON"]).read_text())
        actual = json.loads(sim.run().to_json())
        for key in ["configuration", "options", "planned", "source_configuration_json", "source_configuration_original", "source_configuration_sha256", "status", "compared_source_bits", "bit_errors", "reference_recovered"]:
            self.assertEqual(actual[key], expected[key], key)
        self.assertEqual(actual["grants"][0]["words"], expected["grants"][0]["words"])
        bad = json.loads(data)
        bad["schema_version"] = 2
        with self.assertRaises(upstream.ApiError):
            sim.configure_json(json.dumps(bad).encode())
        self.assertTrue(sim.run().reference_recovered)
        with self.assertRaises(upstream.ApiError) as unsupported:
            sim.configure_json((root / "examples/cli/upstream/us-controls-inspection.json").read_bytes())
        self.assertEqual(unsupported.exception.code, upstream.ErrorCode.UnsupportedCapability)
        self.assertTrue(sim.run().reference_recovered)

    def test_json_synthetic_targets_updates_guards_and_originals_match_rust(self) -> None:
        """One producer carries all synthetic controls through both real consumers."""
        root = Path(__file__).resolve().parents[3]
        data = (root / "examples/cli/upstream/us-synthetic-playback.json").read_bytes()
        simulation = upstream.Simulation()
        simulation.configure_json(data)
        actual = json.loads(simulation.run().to_json())
        expected = json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_SYNTHETIC_JSON"]).read_text())
        for key in ["configuration", "options", "planned", "source_configuration_json", "source_configuration_original", "source_configuration_sha256", "status", "compared_source_bits", "bit_errors", "reference_recovered"]:
            self.assertEqual(actual[key], expected[key], key)
        self.assertTrue(actual["reference_recovered"])
        self.assertEqual(actual["grants"][0]["words"], expected["grants"][0]["words"])
        self.assertEqual(actual["grants"][0]["matched_data_mer"], expected["grants"][0]["matched_data_mer"])
        self.assertEqual(simulation.document().original_bytes(), data)
        self.assertEqual(actual["source_configuration_sha256"], hashlib.sha256(data).hexdigest())

    def test_stub_public_classes_and_members_match_actual_extension(self) -> None:
        """Declarations describe native runtime objects rather than fake stubs."""
        stub = Path(upstream.__file__).with_name("upstream.pyi")
        tree = ast.parse(stub.read_text())
        classes = [node for node in tree.body if isinstance(node, ast.ClassDef)]
        self.assertEqual({node.name for node in classes}, set(upstream.__all__) - {"__version__", "ApiError", "ErrorCode"})
        for node in classes:
            native = getattr(upstream, node.name)
            for item in node.body:
                if isinstance(item, ast.FunctionDef) and item.name != "__init__":
                    self.assertTrue(hasattr(native, item.name), (node.name, item.name))
                if isinstance(item, ast.FunctionDef) and not any(isinstance(decorator, ast.Name) and decorator.id == "property" for decorator in item.decorator_list):
                    target = native if item.name == "__init__" else getattr(native, item.name)
                    try:
                        actual = inspect.signature(target)
                    except (ValueError, TypeError):
                        self.fail(f"Missing inspectable native signature: {node.name}.{item.name}")
                    expected = list(item.args.args)
                    if item.name == "__init__":
                        expected = expected[1:]
                    self.assertEqual(list(actual.parameters), [argument.arg for argument in expected], (node.name, item.name))
                    defaults = [inspect.Parameter.empty] * (len(expected) - len(item.args.defaults)) + [ast.literal_eval(value) for value in item.args.defaults]
                    self.assertEqual([parameter.default for parameter in actual.parameters.values()], defaults, (node.name, item.name))


if __name__ == "__main__":
    unittest.main()
