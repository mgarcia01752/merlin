#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Pre-copy admission and exact final owners; all measurements come from Rust."""
import gc
import json
import os
from pathlib import Path
import unittest

import merlin.common as common
import merlin.downstream as downstream
from test_downstream_retained_footprint import prepare, quantities
from test_receiver_archive import query

# Deliberately excessive for every supported target-usize, not a PHY domain limit.
OVERFLOW_INTEGER_BITS = 1024
# Small original-cell query page, independent of retained role counts and LLR widths.
QUERY_ROWS = 1
STREAMS = ('payload', 'ncp', 'plc_data', 'plc_preamble',
           'continuous_pilots', 'scattered_pilots')


def views(archive):
    """Query original archived cells through Rust's unchanged scientific owners.

    Args:
        archive: Immutable detached native archive.
    Returns:
        All six actual role views, with explicit missing decoder coverage.
    """
    result = {name: dict(qam=archive.qam(query(stream=name, count=QUERY_ROWS)),
                        decisions=archive.cell_decisions(query(stream=name, count=QUERY_ROWS)),
                        soft=archive.cell_soft_view(query(stream=name, count=QUERY_ROWS)))
              for name in STREAMS}
    try:
        decoder = archive.decoder_soft_view()
    except downstream.ApiError as error:
        if error.code != downstream.ErrorCode.MissingCoverage:
            raise
        decoder = None
    return result, decoder


class BoundedReceiverArchiveTests(unittest.TestCase):
    def test_common_limit_literal_is_immutable_and_checks_native_integer_domain(self):
        value = common.RetainedFootprint(payload_bytes=0)
        self.assertEqual(quantities(value), dict(payload_bytes=0, ordered_entries=0, shared_roots=0))
        explicit = dict(payload_bytes=7, ordered_entries=3, shared_roots=2)
        # Independent arbitrary caller quantities exercise all three fields, not scientific limits.
        value = common.RetainedFootprint(**explicit)
        self.assertEqual(quantities(value), explicit)
        for name in explicit:
            with self.assertRaises(AttributeError):
                setattr(value, name, 0)
            for invalid in (-1, 1 << OVERFLOW_INTEGER_BITS):
                with self.subTest(field=name, invalid=invalid), self.assertRaises(OverflowError):
                    common.RetainedFootprint(**{**explicit, name: invalid})
        with self.assertRaises(TypeError):
            common.RetainedFootprint()

    def test_exact_bound_dimension_rejection_and_legacy_science_are_preserved(self):
        _, request = prepare(observed=True)
        output = request.run()
        bound = output.receiver_archive_retained_bound()
        exact = quantities(bound)
        self.assertEqual(exact['ordered_entries'], 0)
        self.assertEqual(exact['shared_roots'], 1)  # Detached archive's sole Arc edge.
        before = output.report()
        provenance = output.input_summary()
        for changed in ({**exact, 'payload_bytes': exact['payload_bytes'] - 1},
                        {**exact, 'shared_roots': 0}):
            with self.assertRaises(downstream.ApiError) as rejected:
                output.receiver_archive_bounded(common.RetainedFootprint(**changed))
            self.assertEqual(rejected.exception.code, downstream.ErrorCode.ResourceLimit)
            self.assertEqual(output.report(), before)
            self.assertEqual(output.input_summary(), provenance)
        legacy = output.receiver_archive()
        archived = output.receiver_archive_bounded(bound)
        self.assertEqual(quantities(archived.retained_footprint()), exact)
        self.assertEqual(archived.info, legacy.info)
        expected = views(legacy)
        self.assertEqual(views(archived), expected)
        # Extra allowed entries do not introduce entries or change the actual target.
        slack = common.RetainedFootprint(**{**exact, 'ordered_entries': 1})
        self.assertEqual(quantities(output.receiver_archive_bounded(slack).retained_footprint()), exact)
        del output, request, legacy
        gc.collect()
        self.assertEqual(views(archived), expected)
        self.assertEqual(quantities(archived.retained_footprint()), exact)

    def test_legacy_policy_rejection_and_unobserved_cancelled_outputs_remain_explicit(self):
        _, request = prepare()
        control = downstream.FullLoopControl()
        control.cancel()
        output = request.run(control)
        bound = output.receiver_archive_retained_bound()
        archive = output.receiver_archive_bounded(bound)
        self.assertEqual(quantities(archive.retained_footprint()), quantities(bound))
        self.assertIsNone(views(archive)[1])
        too_small = downstream.FullLoopReceiverArchivePolicy.from_json(b'{"max_retained_bytes":1}')
        for operation in (output.receiver_archive_retained_bound,
                          lambda policy: output.receiver_archive_bounded(bound, policy)):
            with self.assertRaises(downstream.ApiError) as rejected:
                operation(too_small)
            self.assertEqual(rejected.exception.code, downstream.ErrorCode.ResourceLimit)

    @unittest.skipUnless(os.environ.get('MERLIN_BOUNDED_ARCHIVE_REFERENCE'),
                         'Separate current Rust-process bounded archive reference required')
    def test_installed_preflight_copies_and_six_scientific_views_match_direct_rust(self):
        reference = json.loads(Path(os.environ['MERLIN_BOUNDED_ARCHIVE_REFERENCE']).read_text())
        grouped = {}
        for case in reference['cases']:
            key = (case['project'], case['observed'], case['cancelled'])
            if key not in grouped:
                _, request = prepare(case['project'], case['observed'])
                control = downstream.FullLoopControl()
                if case['cancelled']:
                    control.cancel()
                grouped[key] = request.run(control)
            output = grouped[key]
            policy = downstream.FullLoopReceiverArchivePolicy.from_json(json.dumps(case['settings']).encode())
            bound = output.receiver_archive_retained_bound(policy)
            archive = output.receiver_archive_bounded(bound, policy)
            self.assertEqual(quantities(bound), case['bound'])
            self.assertEqual(quantities(archive.retained_footprint()), case['actual'])
            self.assertEqual(archive.info, case['info'])
            self.assertEqual(output.receiver_archive(policy).info, case['legacy_info'])
            actual_views, decoder = views(archive)
            self.assertEqual(actual_views, case['views'])
            self.assertEqual(decoder, case['decoder'])
            self.assertEqual(output.report()['status'], case['status'])


if __name__ == '__main__':
    unittest.main()
