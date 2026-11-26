#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Fixture-backed native lifecycle and conversion checks; no production science."""
import json
import os
from pathlib import Path
import signal
import tempfile
import threading
import time
import unittest
import merlin as m

ROOT = Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "examples/workbench/downstream/single"


def configured():
    """Create a validated controller without running simulation, returning its ID."""
    dpd = m.DPD.from_file(FIXTURE / "modulation-profile.bin", m.ProfileId.PROFILE_A)
    ocd = m.OCD.from_profile(m.ProtocolScope.DOCSIS31, dpd,
                           m.OcdOverrides(bandwidth=m.BandwidthHz(192_000_000)))
    sim = m.Merlin(ocd)
    sim.set_profile(dpd)
    first = sim.add_rxmer(m.RxMer.from_file(FIXTURE / "rxmer.bin"))
    sim.set_options(m.EvaluationOptions(
        channel_selection=m.ChannelSelection.unity(),
        observation=m.ObservationDuration(1),
        profile_tolerance=m.AssociationTolerance(90_000),
        limits=m.RunLimits(runtime=m.RuntimeLimit(300_000))))
    return sim, first


class NativeApiTests(unittest.TestCase):
    def test_profile_id_discovery_binary_json_and_errors(self):
        binary_path = FIXTURE / "modulation-profile.bin"
        decoded_path = ROOT / "examples/measurements/single-channel/modulation_profile.json"
        for path in (binary_path, decoded_path):
            ids = m.DPD.profile_ids_from_file(path)
            self.assertEqual([profile_id.value for profile_id in ids], [0, 1, 2, 3])
            self.assertTrue(all(isinstance(profile_id, m.ProfileId) for profile_id in ids))
            copied = m.DPD.profile_ids_from_bytes("wrong.extension", path.read_bytes())
            self.assertEqual([profile_id.value for profile_id in copied], [0, 1, 2, 3])
            for profile_id in ids:
                self.assertEqual(m.DPD.from_file(path, profile_id).profile_id.value, profile_id.value)
            ids.clear()
            self.assertEqual([profile_id.value for profile_id in copied], [0, 1, 2, 3])
            self.assertEqual([value.value for value in m.DPD.profile_ids_from_file(path)], [0, 1, 2, 3])
        collection = ROOT / "examples/workbench/downstream/multi-rxmer-2"
        path = next(collection.glob("ds_ofdm_modulation_profile_*.bin"))
        self.assertEqual([value.value for value in m.DPD.profile_ids_from_file(path)], [0, 3, 4])
        document = json.loads(decoded_path.read_text())
        cases = [b"{", (FIXTURE / "rxmer.bin").read_bytes()]
        for mode in ("empty", "duplicate", "unsupported_id", "count"):
            malformed = json.loads(json.dumps(document))
            if mode == "empty":
                malformed["profiles"] = []
                malformed["num_profiles"] = 0
            elif mode == "duplicate":
                malformed["profiles"][1]["profile_id"] = malformed["profiles"][0]["profile_id"]
            elif mode == "unsupported_id":
                malformed["profiles"][0]["profile_id"] = 16
            else:
                malformed["num_profiles"] = 1
            cases.append(json.dumps(malformed).encode())
        for data in cases:
            with self.assertRaises(m.ApiError) as context:
                m.DPD.profile_ids_from_bytes("bad", data)
            self.assertIsInstance(context.exception.code, m.ErrorCode)
        with self.assertRaises(m.ApiError) as context:
            m.DPD.profile_ids_from_file(collection / "missing.bin")
        self.assertEqual(context.exception.code, m.ErrorCode.Io)

    def test_loaded_dpd_member_retains_catalog_without_source_file(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "profile.bin"
            path.write_bytes((FIXTURE / "modulation-profile.bin").read_bytes())
            loaded = m.DPD.from_file(path, m.ProfileId(2))
            static = m.DPD.profile_ids_from_file(path)
            path.unlink()
        self.assertEqual(loaded.profile_id.value, 2)
        self.assertEqual([value.value for value in loaded.profile_ids], [0, 1, 2, 3])
        self.assertEqual([value.value for value in static], [0, 1, 2, 3])
        copied = loaded.profile_ids
        copied.clear()
        self.assertEqual([value.value for value in loaded.profile_ids], [0, 1, 2, 3])
        with self.assertRaises(AttributeError):
            loaded.profile_ids = []
        created = m.DPD.from_ocd(configured()[0].ocd, m.ProfileId(3), m.DataModulation.Qam256)
        self.assertEqual([value.value for value in created.profile_ids], [3])
        edited = created.with_modulation(
            m.CarrierRange(m.SubcarrierIndex(356), m.SubcarrierIndex(400)),
            m.DataModulation.Qam64)
        self.assertEqual([value.value for value in edited.profile_ids], [3])

    def test_content_detection_typed_values_and_provenance(self):
        binary = m.RxMer.from_file(FIXTURE / "rxmer.bin")
        decoded = m.RxMer.from_file(ROOT / "examples/measurements/single-channel/rxmer.json")
        self.assertEqual([v.value if v else None for v in binary.values],
                         [v.value if v else None for v in decoded.values])
        self.assertNotEqual(binary.provenance["original_sha256"],
                            binary.provenance["decoded_sha256"])
        original = (FIXTURE / "modulation-profile.bin").read_bytes()
        dpd = m.DPD.from_bytes("wrong.json", original, m.ProfileId.PROFILE_A)
        self.assertEqual(dpd.profile_id.value, 0)
        channel = m.ChannelEstimate.from_file(FIXTURE / "ds-channel-estimate.bin")
        self.assertEqual(len(channel.response), channel.grid["sample_count"])
        with self.assertRaises(m.ApiError):
            m.RxMer.from_bytes("wrong.bin", original)
        with self.assertRaises(m.ApiError) as context:
            m.ProfileId(16)
        self.assertEqual(context.exception.code, m.ErrorCode.InvalidInput)
        with self.assertRaises(AttributeError):
            m.ProfileId.PROFILE_A.value = 2

    def test_defaults_owned_requests_scoped_ids_and_precancelled_batch(self):
        sim, first = configured()
        other, _ = configured()
        with self.assertRaises(m.ApiError) as context:
            other.get_rxmer(first)
        self.assertEqual(context.exception.code, m.ErrorCode.InvalidIdentifier)
        request = sim.options
        request.limits.runtime = m.RuntimeLimit(1000)
        self.assertEqual(request.limits.runtime.value, 1000)
        self.assertEqual(sim.options.limits.runtime.value, 300_000)
        sim.set_options(request)
        request.limits.runtime = m.RuntimeLimit(2000)
        self.assertEqual(sim.options.limits.runtime.value, 1000)
        control = m.RunControl()
        control.cancel()
        batch = sim.run_with_control(control)
        self.assertEqual(batch.status, m.BatchStatus.Cancelled)
        self.assertTrue(all(entry.status == m.RunStatus.NotStarted and entry.report is None
                            for entry in batch))

    def test_manual_ocd_profile_creation_and_protected_edits(self):
        sim, _ = configured()
        snapshot = sim.ocd.to_dict()
        request = m.OcdConfiguration.from_json(
            json.dumps(snapshot["key"]), json.dumps(snapshot["layout"]),
            m.OcdOverrides(bandwidth=m.BandwidthHz(192_000_000)))
        ocd = m.OCD.create(m.ProtocolScope.DOCSIS31, request)
        profile = m.DPD.from_ocd(ocd, m.ProfileId(3), m.DataModulation.Qam256)
        edited = profile.with_modulation(
            m.CarrierRange(m.SubcarrierIndex(356), m.SubcarrierIndex(400)),
            m.DataModulation.Qam64)
        self.assertEqual(profile.origin, "Created")
        self.assertEqual(edited.origin, "Edited")
        self.assertEqual(profile.sources, [])
        self.assertIn("grid", ocd.configuration)
        self.assertIn("layout", ocd.configuration)
        for before, after in zip(profile.carriers, edited.carriers):
            if before["carrier_type"] != "data":
                self.assertEqual(before, after)
        created = m.Merlin(ocd)
        created.set_profile(edited)
        created.add_rxmer(m.RxMer.from_file(FIXTURE / "rxmer.bin"))
        created.set_options(m.EvaluationOptions(channel_selection=m.ChannelSelection.unity()))
        control = m.RunControl()
        control.cancel()
        self.assertEqual(created.run_with_control(control).status, m.BatchStatus.Cancelled)

    def test_runtime_partial_results_and_create_new_export(self):
        sim, _ = configured()
        sim.add_rxmer(m.RxMer.from_file(FIXTURE / "rxmer.bin"))
        options = sim.options
        options.limits.runtime = m.RuntimeLimit(1)
        sim.set_options(options)
        batch = sim.run()
        self.assertEqual(len(batch), 2)
        self.assertTrue(all(entry.status == m.RunStatus.RuntimeExceeded for entry in batch))
        for entry in batch:
            self.assertIsNotNone(entry.report)
            self.assertIsNone(entry.report.profile)
        original = batch.to_json()
        copy = batch.to_dict()
        copy["entries"].clear()
        self.assertEqual(batch.to_json(), original)
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "result.json"
            batch.save_json(path)
            with self.assertRaises(m.ApiError) as context:
                batch.save_json(path)
            self.assertEqual(context.exception.code, m.ErrorCode.Io)
            self.assertEqual(path.read_text(), original)

    def test_gil_release_busy_guard_and_cross_thread_cancellation(self):
        sim, _ = configured()
        options = sim.options
        options.observation = m.ObservationDuration(100)
        sim.set_options(options)
        control = m.RunControl()
        outcome = []
        failures = []

        def run():
            """Dispatch native simulation and preserve any test-thread exception."""
            try:
                outcome.append(sim.run_with_control(control))
            except BaseException as failure:
                failures.append(failure)

        worker = threading.Thread(target=run)
        worker.start()
        deadline = time.monotonic() + 10
        busy = False
        while time.monotonic() < deadline and worker.is_alive():
            try:
                sim.options
            except m.ApiError as failure:
                busy = failure.code == m.ErrorCode.Busy
                if busy:
                    break
            time.sleep(0.005)
        control.cancel()
        worker.join(30)
        self.assertFalse(worker.is_alive())
        self.assertTrue(busy, "Python must remain runnable while native simulation owns the controller")
        self.assertFalse(failures)
        self.assertEqual(outcome[0].status, m.BatchStatus.Cancelled)
        self.assertTrue(all(entry.status in (m.RunStatus.Cancelled, m.RunStatus.NotStarted)
                            for entry in outcome[0]))
        self.assertEqual(sim.options.observation.value, 100)

    def test_ctrl_c_returns_owned_partial_batch(self):
        sim, _ = configured()
        options = sim.options
        options.observation = m.ObservationDuration(100)
        sim.set_options(options)
        timer = threading.Timer(0.1, lambda: os.kill(os.getpid(), signal.SIGINT))
        timer.start()
        try:
            batch = sim.run()
        finally:
            timer.cancel()
            timer.join()
        self.assertEqual(batch.status, m.BatchStatus.Cancelled)
        self.assertTrue(any(entry.report is not None for entry in batch))


class DerivedBandwidthTests(unittest.TestCase):
    def test_profile_without_bandwidth_override(self):
        """Derive the fixture width in Rust while preserving explicit origins."""
        profile = m.DPD.from_file(ROOT / "examples/workbench/downstream/single/modulation-profile.bin", m.ProfileId.PROFILE_A)
        derived = m.OCD.from_profile(m.ProtocolScope.DOCSIS31, profile)
        explicit = m.OCD.from_profile(m.ProtocolScope.DOCSIS31, profile,
            m.OcdOverrides(bandwidth=m.BandwidthHz(192_000_000)))
        self.assertEqual(derived.configuration["bandwidth"],
                         {"value": 192_000_000, "origin": "Derived"})
        self.assertEqual(explicit.configuration["bandwidth"],
                         {"value": 192_000_000, "origin": "Explicit"})


class SimulationTests(unittest.TestCase):
    def test_idle_load_reload_and_legacy_alias(self):
        """Empty sessions fail safely and reload preserves captured identifiers."""
        self.assertIs(m.Merlin, m.Simulation)
        sim = m.Simulation()
        self.assertEqual(sim.profile_ids, [])
        with self.assertRaises(m.ApiError):
            sim.run()
        sim.load_profiles(FIXTURE / "modulation-profile.bin")
        rx = sim.add_rxmer_file(FIXTURE / "rxmer.bin")
        channel = sim.add_channel_estimate_file(FIXTURE / "ds-channel-estimate.bin")
        sim.set_options(m.EvaluationOptions(
            rxmer_selection=m.RxMerSelection.individual(rx),
            channel_selection=m.ChannelSelection.explicit(channel)))
        with self.assertRaises(m.ApiError):
            sim.load_profiles(FIXTURE / "rxmer.bin")
        self.assertEqual(len(sim.profile_ids), 4)
        sim.load_profiles(FIXTURE / "modulation-profile.bin")
        self.assertIsNotNone(sim.get_rxmer(rx))
        self.assertIsNotNone(sim.get_channel_estimate(channel))


class LoadedProfilesTests(unittest.TestCase):
    def test_inventory_selection_cancellation_and_single_run_parity(self):
        """Retain profiles and match all-profile outcomes to independent runs."""
        sim = m.Merlin.from_profile_file(FIXTURE / "modulation-profile.bin", m.ProtocolScope.DOCSIS31)
        default_sim = m.Merlin.from_profile_file(FIXTURE / "modulation-profile.bin")
        self.assertEqual([p.value for p in default_sim.profile_ids],
                         [p.value for p in sim.profile_ids])
        ids = sim.profile_ids
        self.assertEqual([item.value for item in ids], [0, 1, 2, 3])
        sim.select_profile(ids[2])
        self.assertEqual(sim.profile_by_id(ids[1]).profile_id.value, 1)
        sim.add_rxmer(m.RxMer.from_file(FIXTURE / "rxmer.bin"))
        sim.set_options(m.EvaluationOptions(
            channel_selection=m.ChannelSelection.unity(),
            observation=m.ObservationDuration(1),
            profile_tolerance=m.AssociationTolerance(90_000),
            limits=m.RunLimits(runtime=m.RuntimeLimit(300_000))))
        token = m.RunControl()
        token.cancel()
        cancelled = sim.run_all_profiles(token)
        self.assertEqual([entry.profile_id.value for entry in cancelled], [0, 1, 2, 3])
        self.assertTrue(all(entry.report is None for entry in cancelled))
        self.assertEqual(sim.profile.profile_id.value, 2)
        all_results = sim.run_all_profiles()
        self.assertEqual([entry.profile_id.value for entry in all_results], [0, 1, 2, 3])
        for entry in all_results:
            sim.select_profile(entry.profile_id)
            single = next(iter(sim.run()))
            self.assertEqual(entry.report.fec.total_codewords.value, single.report.fec.total_codewords.value)
            self.assertEqual(entry.report.fec.uncorrectable_codewords.value, single.report.fec.uncorrectable_codewords.value)
        self.assertTrue(all(entry.report is None for entry in cancelled))


if __name__ == "__main__":
    unittest.main()
