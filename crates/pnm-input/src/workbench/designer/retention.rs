/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive Designer ownership audit; this child retains access to private admission fields.
use super::{Baseline, RecipeInput, RxMerDesign};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};

impl RetainedPayload for RxMerDesign {
    /// Counts the exact original, shared typed record and shared cached observation.
    ///
    /// # Returns
    /// Complete dynamic-child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            original,
            record,
            capture,
        } = self;
        merlin_api::retention_support::sum([
            original.retained_children(),
            record.retained_children(),
            capture.retained_children(),
        ])
    }
}

impl RetainedPayload for RecipeInput {
    /// Visits every baseline/recipe child beyond the Arc-owned fixed record.
    ///
    /// # Returns
    /// Complete dynamic-child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            merlin_rxmer_designer,
            baseline,
            recipe,
            capture_index,
            capture_count,
        } = self;
        merlin_api::retention_support::sum([
            merlin_rxmer_designer.retained_children(),
            baseline.retained_children(),
            recipe.retained_children(),
            capture_index.retained_children(),
            capture_count.retained_children(),
        ])
    }
}

impl RetainedPayload for Baseline {
    /// Visits flat source-grid/mask ownership or the exact imported saved input.
    ///
    /// # Returns
    /// Complete selected-variant children, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::Flat {
                key,
                anchor_epoch,
                valid,
                baseline_db,
            } => merlin_api::retention_support::sum([
                key.retained_children(),
                anchor_epoch.retained_children(),
                valid.retained_children(),
                baseline_db.retained_children(),
            ]),
            Self::Imported { original } => original.retained_children(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use docsis_ofdm_fec::{
        api::{Dpd, NamedCapture, ProfileId, RxMer},
        signal_processing::rxmer_designer::{RxmerRecipe, ValueEdit},
    };
    use merlin_api::retention_support::retained_footprint;
    use std::{mem::size_of, sync::Arc};

    // Original public Designer fixture; spare-capacity tests preserve these values/bytes.
    const SAVED: &[u8] = include_bytes!("../../../tests/fixtures/designer-flat.json");
    const PROFILE: &[u8] = include_bytes!(
        "../../../../../examples/measurements/single-channel/modulation_profile.json"
    );
    const RXMER: &[u8] =
        include_bytes!("../../../../../examples/measurements/single-channel/rxmer.json");
    // Independent extra element slots; these are test capacities, not scientific limits.
    const SMALL_SPARE_SLOTS: usize = 7;
    const LARGE_SPARE_SLOTS: usize = 29;
    // Independent extra UTF-8 bytes for requested String capacity.
    const SMALL_SPARE_BYTES: usize = 11;
    const LARGE_SPARE_BYTES: usize = 43;
    // Original encoded profile identity, independent of collection position.
    const FIXTURE_PROFILE_ID: u8 = 0;
    // First one-based baseline position and complete collection size.
    const ORIGINAL_CAPTURE_INDEX: usize = 1;
    const ORIGINAL_CAPTURE_COUNT: usize = 1;

    fn design() -> RxMerDesign {
        RxMerDesign::from_bytes(&NamedCapture::new("retained design", SAVED.to_vec()).unwrap())
            .unwrap()
    }

    fn imported_design() -> RxMerDesign {
        let profile = Dpd::from_bytes(
            &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
            ProfileId::new(FIXTURE_PROFILE_ID).unwrap(),
        )
        .unwrap();
        RxMerDesign::imported(
            &profile,
            &NamedCapture::new("ordinary original", RXMER.to_vec()).unwrap(),
            RxmerRecipe::default(),
            super::super::RxMerDesignPosition::new(ORIGINAL_CAPTURE_INDEX, ORIGINAL_CAPTURE_COUNT)
                .unwrap(),
        )
        .unwrap()
    }

    fn reserved_string(value: &str, extra: usize) -> String {
        let mut string = String::with_capacity(value.len() + extra);
        string.push_str(value);
        string
    }

    fn assert_only_payload_delta(
        small: RetainedFootprint,
        large: RetainedFootprint,
        expected_bytes: usize,
    ) {
        assert_eq!(large.payload_bytes - small.payload_bytes, expected_bytes);
        assert_eq!(large.ordered_entries, small.ordered_entries);
        assert_eq!(large.shared_roots, small.shared_roots);
    }

    #[test]
    fn flat_valid_mask_requested_spare_capacity_is_retained() {
        let original = design();
        let with_capacity = |extra| {
            let mut changed = original.clone();
            let record = Arc::make_mut(&mut changed.record);
            let Baseline::Flat { valid, .. } = &mut record.baseline else {
                panic!("flat fixture changed variant");
            };
            let mut values = Vec::with_capacity(valid.len() + extra);
            values.extend_from_slice(valid);
            let capacity = values.capacity();
            *valid = values;
            (changed, capacity)
        };
        let (small, small_capacity) = with_capacity(SMALL_SPARE_SLOTS);
        let (large, large_capacity) = with_capacity(LARGE_SPARE_SLOTS);
        assert_only_payload_delta(
            small.retained_footprint().unwrap(),
            large.retained_footprint().unwrap(),
            (large_capacity - small_capacity) * size_of::<bool>(),
        );
        assert_eq!(small.original_bytes(), SAVED);
        assert_eq!(large.original_bytes(), SAVED);
        assert_eq!(small.materialize().values(), large.materialize().values());
    }

    #[test]
    fn recipe_edit_spare_slots_beyond_initialized_values_are_retained() {
        let original = design();
        let with_capacity = |extra| {
            let mut changed = original.clone();
            let record = Arc::make_mut(&mut changed.record);
            let mut edits = Vec::with_capacity(record.recipe.edits.len() + extra);
            edits.extend_from_slice(&record.recipe.edits);
            let capacity = edits.capacity();
            record.recipe.edits = edits;
            (changed, capacity)
        };
        let (small, small_capacity) = with_capacity(SMALL_SPARE_SLOTS);
        let (large, large_capacity) = with_capacity(LARGE_SPARE_SLOTS);
        assert_only_payload_delta(
            small.retained_footprint().unwrap(),
            large.retained_footprint().unwrap(),
            (large_capacity - small_capacity) * size_of::<ValueEdit>(),
        );
        assert_eq!(small.recipe(), large.recipe());
        assert_eq!(small.original_bytes(), large.original_bytes());
    }

    #[test]
    fn original_named_capture_name_capacity_is_counted_without_getter_reconstruction() {
        let original = design();
        let with_capacity = |extra| {
            let mut changed = original.clone();
            let name = reserved_string(original.original.name(), extra);
            let capacity = name.capacity();
            changed.original = NamedCapture::new(name, original.original_bytes().to_vec()).unwrap();
            (changed, capacity)
        };
        let (small, small_capacity) = with_capacity(SMALL_SPARE_BYTES);
        let (large, large_capacity) = with_capacity(LARGE_SPARE_BYTES);
        assert_only_payload_delta(
            small.retained_footprint().unwrap(),
            large.retained_footprint().unwrap(),
            large_capacity - small_capacity,
        );
        assert_eq!(small.original_bytes(), large.original_bytes());
        assert_eq!(small.materialize().values(), large.materialize().values());
    }

    #[test]
    fn imported_stored_strings_count_all_requested_capacities() {
        let original = imported_design();
        let with_capacity = |extra| {
            let mut changed = original.clone();
            let record = Arc::make_mut(&mut changed.record);
            let Baseline::Imported { original: stored } = &mut record.baseline else {
                panic!("imported fixture changed variant");
            };
            stored.name = reserved_string(&stored.name, extra);
            stored.sha256 = reserved_string(&stored.sha256, extra);
            stored.base64 = reserved_string(&stored.base64, extra);
            let capacities =
                stored.name.capacity() + stored.sha256.capacity() + stored.base64.capacity();
            (changed, capacities)
        };
        let (small, small_capacity) = with_capacity(SMALL_SPARE_BYTES);
        let (large, large_capacity) = with_capacity(LARGE_SPARE_BYTES);
        assert_only_payload_delta(
            small.retained_footprint().unwrap(),
            large.retained_footprint().unwrap(),
            large_capacity - small_capacity,
        );
        assert_eq!(small.original_bytes(), large.original_bytes());
        assert_eq!(small.capture.values(), large.capture.values());
    }

    #[test]
    fn cached_materialized_rxmer_counts_sample_spare_capacity() {
        let original = design();
        let with_capacity = |extra| {
            let mut changed = original.clone();
            let mut frame = original.capture.frame().clone();
            let mut values = Vec::with_capacity(frame.mer_db.len() + extra);
            values.extend_from_slice(&frame.mer_db);
            let capacity = values.capacity();
            frame.mer_db = values;
            changed.capture = Arc::new(RxMer::from_frame(&changed.original, frame).unwrap());
            (changed, capacity)
        };
        let (small, small_capacity) = with_capacity(SMALL_SPARE_SLOTS);
        let (large, large_capacity) = with_capacity(LARGE_SPARE_SLOTS);
        assert_only_payload_delta(
            small.retained_footprint().unwrap(),
            large.retained_footprint().unwrap(),
            (large_capacity - small_capacity) * size_of::<Option<f64>>(),
        );
        assert_eq!(small.capture.values(), large.capture.values());
        assert_eq!(
            small.capture.provenance().decoded_bytes(),
            large.capture.provenance().decoded_bytes()
        );
    }

    #[test]
    fn cached_source_provenance_string_capacities_are_retained() {
        let original = design();
        let with_capacity = |extra| {
            let mut changed = original.clone();
            let mut frame = original.capture.frame().clone();
            frame.source.name = reserved_string(&frame.source.name, extra);
            frame.source.sha256 = reserved_string(&frame.source.sha256, extra);
            frame.source.file_kind = reserved_string(&frame.source.file_kind, extra);
            let capacity = frame.source.name.capacity()
                + frame.source.sha256.capacity()
                + frame.source.file_kind.capacity();
            changed.capture = Arc::new(RxMer::from_frame(&changed.original, frame).unwrap());
            (changed, capacity)
        };
        let (small, small_capacity) = with_capacity(SMALL_SPARE_BYTES);
        let (large, large_capacity) = with_capacity(LARGE_SPARE_BYTES);
        assert_only_payload_delta(
            small.retained_footprint().unwrap(),
            large.retained_footprint().unwrap(),
            large_capacity - small_capacity,
        );
        assert_eq!(
            small.capture.provenance().decoded_bytes(),
            large.capture.provenance().decoded_bytes()
        );
        assert_eq!(small.capture.frame().source, large.capture.frame().source);
    }

    #[test]
    fn original_record_cache_and_nested_provenance_edges_are_counted_on_aliases() {
        let original = design();
        let alias = original.clone();
        assert!(Arc::ptr_eq(&original.record, &alias.record));
        assert!(Arc::ptr_eq(&original.capture, &alias.capture));
        let footprint = original.retained_footprint().unwrap();
        // Original bytes, record, cache and cached original/decoded provenance: five edges.
        assert_eq!(footprint.shared_roots, 5);
        assert_eq!(footprint.ordered_entries, 0);
        assert_eq!(alias.retained_footprint().unwrap(), footprint);
        let roots = retained_footprint(&(original.clone(), alias.clone())).unwrap();
        assert_eq!(roots.payload_bytes, footprint.payload_bytes * 2);
        assert_eq!(roots.shared_roots, 10);
        drop(original);
        assert_eq!(alias.retained_footprint().unwrap(), footprint);
    }
}
