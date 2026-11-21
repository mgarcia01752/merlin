#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""WB import, exact saved settings, native parity and shared cancellation."""
import gc
import json
import os
from pathlib import Path
import tempfile
import threading
import time
import unittest
import merlin as m

ROOT = Path(__file__).resolve().parents[3]
WB = ROOT / "examples/workbench/multi-rxmer-2/workbench.merlin.json"
# Independent native UI observation for the current bundled WB.
UI_FEC = [(83, 0, 0), (114, 5, 0), (124, 19, 0)]


def controller_relative_sources(sources):
    """Compare opaque controller identities by relationships across processes.

    Scope numbers come from each process's controller counter. Preserve slots,
    scope equality/distinctness, policies and all other source metadata exactly.
    """
    scopes = {}

    def relative(value):
        if isinstance(value, list):
            return [relative(item) for item in value]
        if isinstance(value, dict):
            if set(value) == {"scope", "slot"}:
                scope = scopes.setdefault(value["scope"], len(scopes))
                return {"scope": scope, "slot": value["slot"]}
            return {key: relative(value[key]) for key in sorted(value)}
        return value

    return relative(sources)


class WorkbenchTests(unittest.TestCase):
    def test_self_contained_file_bytes_and_copied_settings(self):
        """Load away from fixture paths without running or reading sidecar captures."""
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "isolated.json"
            path.write_bytes(WB.read_bytes())
            wb = m.Workbench.from_file(path)
            path.unlink()
        self.assertEqual([v.value for v in wb.profile_ids], [0, 3, 4])
        self.assertEqual([v.value for v in m.Workbench.from_bytes(WB.read_bytes()).profile_ids], [0, 3, 4])
        options = wb.options
        self.assertEqual(options[0].receiver_mode, m.ReceiverMode.CM_CHANNEL_ESTIMATED)
        self.assertEqual(options[0].observation.value, 1)
        self.assertEqual(options[0].noise_seed.value, 41)
        self.assertEqual(options[0].processing["mce"]["averaging_symbols"], 16)
        options[0].receiver_mode = m.ReceiverMode.PILOT_ESTIMATED
        self.assertEqual(wb.options[0].receiver_mode, m.ReceiverMode.CM_CHANNEL_ESTIMATED)
        control = m.RunControl()
        control.cancel()
        for batch in wb.run_with_control(control):
            self.assertTrue(all(entry.report is None for entry in batch))

    def test_saved_mode_mce_tuning_selection_and_structured_failures(self):
        """Settings come from WB bytes; unsupported/corrupt data never silently defaults."""
        document = json.loads(WB.read_bytes())
        document["schema_version"] = 2
        document["receiver"].update(selected_profile_ids=[4], receiver_mode="merlin_channel_estimated",
                                    mce={"averaging_symbols": 32, "minimum_variance": 1e-7})
        wb = m.Workbench.from_bytes(json.dumps(document).encode())
        self.assertEqual([v.value for v in wb.profile_ids], [4])
        self.assertEqual(wb.options[0].receiver_mode, m.ReceiverMode.MERLIN_CHANNEL_ESTIMATED)
        self.assertEqual(wb.options[0].processing["mce"]["averaging_symbols"], 32)
        document["sections"]["measurements"]["options"]["mode"] = "hold"
        with self.assertRaises(m.ApiError) as caught:
            m.Workbench.from_bytes(json.dumps(document).encode())
        self.assertEqual(caught.exception.code, m.ErrorCode.UnsupportedCapability)
        document["sections"]["measurements"]["options"]["mode"] = "average"
        document["sections"]["profile"]["file"]["sha256"] = "bad"
        with self.assertRaises(m.ApiError) as caught:
            m.Workbench.from_bytes(json.dumps(document).encode())
        self.assertEqual(caught.exception.code, m.ErrorCode.InvalidInput)

    def test_actual_saved_run_owned_results_and_rust_ui_parity(self):
        """Compare simulated results to native UI anchors and optional fresh Rust JSON."""
        wb = m.Workbench.from_file(WB)
        batches = wb.run()
        del wb
        gc.collect()
        rows = []
        for batch in batches:
            entry = next(iter(batch))
            self.assertEqual(entry.status, m.RunStatus.Complete)
            report = entry.report
            rows.append((report.fec.total_codewords.value, report.fec.corrected_codewords.value,
                         report.fec.uncorrectable_codewords.value))
            self.assertEqual(report.profile.post_fec_ber.value, 0)
            self.assertEqual(report.options.receiver_mode, m.ReceiverMode.CM_CHANNEL_ESTIMATED)
        self.assertEqual(rows, UI_FEC)
        baseline = os.environ.get("MERLIN_WB_PARITY")
        if baseline:
            rust = json.loads(Path(baseline).read_text())
            self.assertEqual(len(batches), len(rust))
            for batch, expected in zip(batches, rust):
                actual = batch.to_dict()
                self.assertEqual(actual["status"], expected["status"])
                for got, want in zip(actual["entries"], expected["entries"]):
                    self.assertEqual(got["status"], want["status"])
                    for key in ("fec", "all_completed_fec", "profile", "options"):
                        self.assertEqual(got["report"][key], want["report"][key])
                    self.assertEqual(controller_relative_sources(got["report"]["sources"]),
                                     controller_relative_sources(want["report"]["sources"]))

    def test_other_python_thread_can_cancel_and_later_profiles_are_not_started(self):
        """GIL release lets a caller cancel native work and retain all profile entries."""
        wb = m.Workbench.from_file(WB)
        control = m.RunControl()
        def stop():
            time.sleep(0.05)
            control.cancel()
        worker = threading.Thread(target=stop)
        worker.start()
        batches = wb.run_with_control(control)
        worker.join()
        self.assertEqual(len(batches), 3)
        # Cancellation can arrive during native preflight or after engine startup.
        self.assertIn(next(iter(batches[0])).status,
                      (m.RunStatus.NotStarted, m.RunStatus.Cancelled))
        self.assertTrue(all(next(iter(batch)).status == m.RunStatus.NotStarted
                            for batch in batches[1:]))


if __name__ == "__main__":
    unittest.main()
