#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Current input and idle-preview ownership; storage calculations stay in Rust."""
import gc
import json
import os
from pathlib import Path
import unittest

import merlin
import merlin.common as common
import merlin.downstream as downstream

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'examples/measurements/single-channel'
# Exact original finite bytes and independent recurrence seed match Rust fixture.
FILE_BYTES = bytes([0x81, 0x00, 0x5a])
PREVIEW_SEED = 7
# Original encoded Profile A identity, independent of collection position.
FIXTURE_PROFILE_ID = 0
# Small original sample prefix, independent of PSD/retention and source lengths.
SAMPLE_PREFIX_COUNT = 3


def quantities(value):
    """Read three native quantities without estimating Python object storage.

    Args:
        value: Rust-produced immutable metadata from an existing owner.
    Returns:
        The original named quantities for direct Rust-process comparison.
    """
    return {name: getattr(value, name)
            for name in ('payload_bytes', 'ordered_entries', 'shared_roots')}


def owners():
    """Admit identical original fixture bytes and generate one actual idle preview.

    Returns:
        Existing native input, design and preview owners; no PHY session is run.
    """
    capture = downstream.ProfileCapture.from_bytes(
        'profile', (FIXTURES / 'modulation_profile.json').read_bytes())
    source = downstream.SourceInput.prbs15(downstream.PrbsSeed(PREVIEW_SEED))
    request = downstream.TransmitterPreviewRequest.from_capture(
        downstream.TransmissionConfiguration(), capture, FIXTURE_PROFILE_ID, source)
    return dict(
        source=source,
        file=downstream.SourceInput.from_bytes('file', FILE_BYTES),
        capture=capture,
        response=downstream.ChannelEstimate.from_bytes(
            'channel', (FIXTURES / 'channel_estimation.json').read_bytes()),
        request=request,
        preview=request.generate(),
        saved=downstream.RxMerDesign.from_bytes(
            'design', (ROOT / 'crates/pnm-input/tests/fixtures/designer-flat.json').read_bytes()),
        imported=downstream.RxMerDesign.imported_capture(
            capture, FIXTURE_PROFILE_ID, 'baseline', (FIXTURES / 'rxmer.json').read_bytes()))


class InputRetainedFootprintTests(unittest.TestCase):
    def test_native_metadata_legacy_identity_passivity_and_detached_values(self):
        values = owners()
        before = {name: quantities(owner.retained_footprint()) for name, owner in values.items()}
        original = values['saved'].original_bytes
        provenance = values['capture'].provenance
        samples = values['preview'].sample_view(count=SAMPLE_PREFIX_COUNT)
        metadata = {}
        for name, owner in values.items():
            self.assertIs(getattr(merlin, type(owner).__name__), type(owner))
            value = owner.retained_footprint()
            self.assertIs(type(value), common.RetainedFootprint)
            self.assertGreater(value.payload_bytes, 0)
            with self.assertRaises(AttributeError):
                value.shared_roots = 0
            metadata[name] = value
        self.assertEqual({name: quantities(owner.retained_footprint())
                          for name, owner in values.items()}, before)
        self.assertEqual(values['saved'].original_bytes, original)
        self.assertEqual(values['capture'].provenance, provenance)
        self.assertEqual(values['preview'].sample_view(count=SAMPLE_PREFIX_COUNT), samples)
        self.assertEqual(values['file'].original_bytes, FILE_BYTES)
        preview = values['preview']
        del values, owner
        gc.collect()
        self.assertEqual(preview.sample_view(count=SAMPLE_PREFIX_COUNT), samples)
        self.assertEqual({name: quantities(value) for name, value in metadata.items()}, before)

    def test_rejected_design_and_precancelled_preview_preserve_accepted_owners(self):
        values = owners()
        before = {name: quantities(owner.retained_footprint()) for name, owner in values.items()}
        with self.assertRaises(downstream.ApiError):
            values['saved'].with_recipe(b'{"unrecognized":1}')
        control = downstream.TransmitterPreviewControl()
        control.cancel()
        with self.assertRaises(downstream.ApiError):
            values['request'].generate(control)
        self.assertEqual({name: quantities(owner.retained_footprint())
                          for name, owner in values.items()}, before)

    @unittest.skipUnless(os.environ.get('MERLIN_INPUT_RETAINED_REFERENCE'),
                         'Separate current Rust-process metadata reference required')
    def test_installed_owners_and_original_scientific_views_match_direct_rust(self):
        reference = json.loads(Path(os.environ['MERLIN_INPUT_RETAINED_REFERENCE']).read_text())
        values = owners()
        self.assertEqual({name: quantities(owner.retained_footprint())
                          for name, owner in values.items()}, reference['owners'])
        self.assertEqual(values['preview'].info(), reference['preview_info'])
        self.assertEqual(values['preview'].sample_view(count=SAMPLE_PREFIX_COUNT), reference['preview_samples'])
        self.assertEqual(values['capture'].provenance, reference['capture_provenance'])
        self.assertEqual(values['response'].frame(), reference['response_frame'])
        self.assertEqual(values['saved'].original_bytes, bytes(reference['saved_original']))
        self.assertEqual(values['imported'].original_bytes, bytes(reference['imported_original']))
        # These Rust owners have argument/dictionary representations in Python.
        self.assertEqual(set(reference['rust_only']), {'named', 'provenance'})
        self.assertFalse(hasattr(downstream, 'NamedCapture'))
        self.assertFalse(hasattr(downstream, 'CaptureProvenance'))


if __name__ == '__main__':
    unittest.main()
