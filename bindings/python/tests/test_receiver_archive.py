#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Detached public histories preserve actual roles, ownership and live-query compatibility."""
import gc
import json
import os
from pathlib import Path
import unittest
import merlin
import merlin.downstream as m
from test_full_loop import request


def query(**values):
    """Admit one strict actual-cell selector through the existing public Rust owner."""
    return m.FullLoopCellQuery.from_json(json.dumps(values).encode())


class ReceiverArchiveTests(unittest.TestCase):
    def test_policy_domains_exports_and_dictionary_isolation(self):
        self.assertIs(merlin.FullLoopReceiverArchivePolicy, m.FullLoopReceiverArchivePolicy)
        self.assertIs(merlin.FullLoopReceiverArchive, m.FullLoopReceiverArchive)
        policy = m.FullLoopReceiverArchivePolicy()
        # Independent existing history anchors; equal NCP/protected caps have separate meanings.
        self.assertEqual(policy.settings, dict(payload_cells=8192, ncp_cells=1024,
            protected_cells_per_stream=1024, decoder_llrs=16200, max_retained_bytes=8388608))
        edited = policy.settings
        edited['payload_cells'] = 0
        self.assertEqual(policy.settings['payload_cells'], 8192)
        self.assertEqual(m.FullLoopReceiverArchivePolicy.from_json(policy.to_json().encode()).settings, policy.settings)
        for bad in [dict(payload_cells=8193), dict(ncp_cells=1025),
                    dict(protected_cells_per_stream=1025), dict(decoder_llrs=16201),
                    dict(max_retained_bytes=0), dict(max_retained_bytes=8388609),
                    dict(payload_cells=-1), dict(unexpected=True)]:
            with self.subTest(bad=bad), self.assertRaises(m.ApiError):
                m.FullLoopReceiverArchivePolicy.from_json(json.dumps(bad).encode())
        with self.assertRaises(m.ApiError) as error:
            m.FullLoopReceiverArchivePolicy.from_json(b' ' * 4097)
        self.assertEqual(error.exception.code, m.ErrorCode.ResourceLimit)

    def test_actual_roles_reduction_and_lifetime_are_independent_of_live_results(self):
        prepared = request().with_observations(m.FullLoopObservationPolicy())
        live = prepared.run()
        archive = live.receiver_archive()
        expected = {}
        for stream in ['payload', 'ncp', 'plc_data', 'plc_preamble', 'continuous_pilots', 'scattered_pilots']:
            original = live.qam(query(stream=stream))
            population = original['selection']['retained_cells']
            cap = 8192 if stream == 'payload' else 1024
            copied = min(population, cap)
            last_original = 0 if copied == 0 else (copied-1)*population//copied
            # The independent index equation is the contract oracle, not a Python simulation.
            expected[stream] = live.qam(query(stream=stream, start=last_original, count=1))['cells']
            view = archive.qam(query(stream=stream, start=max(0,copied-1), count=1))
            self.assertEqual(view['cells'], expected[stream])
            self.assertEqual(view['selection']['retained_cells'], copied)
            if copied:
                self.assertEqual(archive.cell_decisions(query(stream=stream, count=1))['rows'][0]['cell'],
                                 archive.qam(query(stream=stream, count=1))['cells'][0])
                soft = archive.cell_soft_view(query(stream=stream, count=1))
                self.assertEqual(soft['source_plane'], 'post_descrambling_cell_llrs')
                if stream not in ['payload', 'ncp']:
                    self.assertEqual(soft['view']['statistics']['count'], 0)
        info = archive.info
        info['payload_cells'] = -1
        self.assertGreater(archive.info['payload_cells'], 0)
        self.assertLessEqual(archive.info['retained_bytes'], 8388608)
        decoder = archive.decoder_soft_view()
        self.assertEqual(decoder['view']['statistics']['count'], 16200)
        self.assertEqual(decoder['context'], live.decoder_soft_view()['context'])
        del prepared, live
        gc.collect()
        for stream, cells in expected.items():
            copied = archive.qam(query(stream=stream))['selection']['retained_cells']
            self.assertEqual(archive.qam(query(stream=stream, start=max(0,copied-1), count=1))['cells'], cells)
        original = archive.qam(query(count=1))['cells'][0]['after_equalization'][:]
        edited = archive.qam(query(count=1))
        edited['cells'][0]['after_equalization'] = [99,99]
        self.assertEqual(archive.qam(query(count=1))['cells'][0]['after_equalization'], original)

    def test_omitted_decoder_zero_roles_byte_rejection_and_invalid_queries_stay_explicit(self):
        live = request().run()
        archive = live.receiver_archive()
        self.assertIsNone(archive.info['decoder_snapshot_index'])
        with self.assertRaises(m.ApiError) as missing:
            archive.decoder_soft_view()
        self.assertEqual(missing.exception.code, m.ErrorCode.MissingCoverage)
        empty_policy = m.FullLoopReceiverArchivePolicy.from_json(json.dumps(dict(payload_cells=0,
            ncp_cells=0,protected_cells_per_stream=0,decoder_llrs=0)).encode())
        empty = live.receiver_archive(empty_policy)
        self.assertEqual(empty.qam()['cells'], [])
        self.assertEqual(empty.cell_soft_view()['view']['statistics']['count'], 0)
        tight = m.FullLoopReceiverArchivePolicy.from_json(b'{"max_retained_bytes":1}')
        with self.assertRaises(m.ApiError) as budget:
            live.receiver_archive(tight)
        self.assertEqual(budget.exception.code,m.ErrorCode.ResourceLimit)
        with self.assertRaises(m.ApiError):
            archive.cell_decisions(query(count=129))
        with self.assertRaises(m.ApiError):
            archive.qam(query(start=999999))
        self.assertGreater(live.report()['summary']['comparison']['bits_tested'],0)

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_fresh_direct_rust_process_archive_fields_and_scientific_views_match(self):
        expected = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['archive']
        live = request().with_observations(m.FullLoopObservationPolicy()).run()
        archive = live.receiver_archive()
        actual = dict(info=archive.info,qam=archive.qam(query(count=5)),
            decisions=archive.cell_decisions(query(count=3)),
            cell_soft=archive.cell_soft_view(query(count=3)),decoder_soft=archive.decoder_soft_view())
        self.assertEqual(actual,expected)
