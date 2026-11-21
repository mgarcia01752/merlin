# Copyright 2026 MORGANA contributors
# SPDX-License-Identifier: Apache-2.0
# Adapted for MERLIN from 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
# Changes: namespaces, fixture owners and shared typed error categories.
"""Native source adapters and actual captured-plan scientific execution."""
import hashlib
import json
import os
from pathlib import Path
import unittest
import merlin.upstream as upstream

ROOT = Path(__file__).resolve().parents[3]

class MeasuredTests(unittest.TestCase):
    def test_native_25khz_mixed_grid_actual_rust_process_parity(self):
        """Captured4K fields and mixed/zero loading execute through both consumers."""
        session=upstream.Simulation()
        session.configure_measured_json((ROOT/"examples/measurements/upstream/measured/capture-driven-25khz.json").read_bytes())
        actual=json.loads(session.run().to_json())
        expected=json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_CAPTURES25_JSON"]).read_text())
        for key in ("configuration","measurement_provenance_json","planned","bit_errors","reference_recovered"):
            self.assertEqual(actual[key],expected[key],key)
        self.assertTrue(actual["reference_recovered"])

    def test_native_multiple_capture_policies_and_cancelled_provenance(self):
        """All capture selection policies execute in shared Rust and retain sources."""
        original=json.loads((ROOT/"examples/measurements/upstream/measured/capture-driven.json").read_text())
        for policy in ("average","hold","interpolate"):
            plan=json.loads(json.dumps(original))
            second=json.loads(json.dumps(plan["rxmer"][0]));second["id"]=2
            second["bytes"][297:]=[80]*400
            plan["rxmer"].append(second);plan["rxmer_policy"]=policy
            if policy!="average":plan["rxmer_offsets"]=[0,6]
            session=upstream.Simulation();session.configure_measured_json(json.dumps(plan).encode())
            report=json.loads(session.run().to_json())
            self.assertEqual(len(report["grants"]),1)
            sources=json.loads(report["measurement_provenance_json"])["plan"]["rxmer"]
            self.assertEqual(sources,plan["rxmer"])
            control=upstream.RunControl();control.cancel()
            partial=json.loads(session.run_with_control(control).to_json())
            self.assertEqual(partial["status"],"Cancelled")
            self.assertIsNotNone(partial["measurement_provenance_json"])

    def test_pre_eq_literals_errors_originals_and_lifetimes(self):
        """Native calls retain exact signed source values and independently owned bytes."""
        for name, minimum in (("applied", -4.0), ("last-update", -2.0)):
            raw=bytes.fromhex((ROOT/f"examples/measurements/upstream/pre-eq/{name}.hex").read_text())
            capture=upstream.PreEqCapture.from_pnm(raw)
            self.assertEqual(capture.coefficients()[:2],[(1.0,0.0),(1.125,-0.25)])
            self.assertEqual(capture.coefficients()[2][0],minimum)
            self.assertEqual(capture.original_sha256(),hashlib.sha256(raw).hexdigest())
            original, exported=capture.original_bytes(),capture.to_json()
            del capture
            self.assertEqual(original,raw)
            self.assertEqual(json.loads(exported)["schema_version"],1)
            with self.assertRaises(upstream.ApiError) as rejected:
                upstream.PreEqCapture.from_pnm(raw[:-1])
            self.assertEqual((rejected.exception.code,rejected.exception.field,rejected.exception.offset),(upstream.ErrorCode.InvalidInput,"pre_eq.data_length",30))

    def test_actual_capture_driven_rust_python_science_and_provenance(self):
        """Separate Rust/native processes agree on decoded inputs and actual outcomes."""
        raw=(ROOT/"examples/measurements/upstream/measured/capture-driven.json").read_bytes()
        session=upstream.Simulation()
        session.configure_measured_json(raw)
        report=session.run()
        actual=json.loads(report.to_json())
        expected=json.loads(Path(os.environ["MERLIN_UPSTREAM_RUST_CAPTURES_JSON"]).read_text())
        self.assertTrue(report.reference_recovered)
        for key in ("measurement_provenance_json","configuration","options","generated_source_bits","compared_source_bits","bit_errors","ber","reference_recovered"):
            self.assertEqual(actual[key],expected[key],key)
        self.assertEqual(len(actual["grants"]),1)
        for observed, reference in zip(actual["grants"],expected["grants"]):
            self.assertGreater(observed["identity"]["run"],0)
            self.assertGreater(observed["identity"]["generation"],0)
            self.assertEqual(observed["identity"]["grant"],reference["identity"]["grant"])
            for key in observed.keys()-{"identity"}:
                self.assertEqual(observed[key],reference[key],key)
        bad=json.loads(raw);bad["pre_eq_events"][0]["base_bank"]=99
        with self.assertRaises(upstream.ApiError):
            session.configure_measured_json(json.dumps(bad).encode())
        self.assertTrue(session.run().reference_recovered)
        del session
        self.assertTrue(report.reference_recovered)
