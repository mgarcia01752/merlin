/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent copy-only capacity/error anchors; no scientific acceptance fixture.
use super::*;
use crate::simulation::{
    channel::constellation::ConstellationStatus,
    test_points::{RunPhase, TransmitCellRole},
};

/// Distinct unused source capacities detect accidental clone/collect retention.
const SOURCE_PAIR_SLOTS: usize = 13;
const SOURCE_LLR_SLOTS: usize = 11;
const SOURCE_ROLE_SLOTS: usize = 7;
const SOURCE_ID_SPARE_BYTES: usize = 17;
/// Selection counts exercise literal floor-selected source indices.
const SOURCE_CELL_COUNT: usize = 5;
const COPIED_PAYLOAD_CELLS: usize = 3;
const COPIED_NCP_CELLS: usize = 2;
const COPIED_PROTECTED_CELLS: usize = 3;
const COPIED_DECODER_LLRS: usize = 2;

fn policy() -> FullLoopReceiverArchivePolicy {
    FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput {
        payload_cells: COPIED_PAYLOAD_CELLS,
        ncp_cells: COPIED_NCP_CELLS,
        protected_cells_per_stream: COPIED_PROTECTED_CELLS,
        decoder_llrs: COPIED_DECODER_LLRS,
        ..Default::default()
    })
    .unwrap()
}

fn llrs() -> Vec<f64> {
    let mut values = Vec::with_capacity(SOURCE_LLR_SLOTS);
    values.extend([-1.0, 1.0]); // Two finite copy anchors; no demapping is performed.
    values
}

fn data() -> ConstellationResult {
    let mut pairs = Vec::with_capacity(SOURCE_PAIR_SLOTS);
    let mut ncp_pairs = Vec::with_capacity(SOURCE_PAIR_SLOTS);
    for index in 0..SOURCE_CELL_COUNT {
        pairs.push(ConstellationPair {
            channel_symbol_index: index as u64,
            source_symbol_index: index as u64,
            physical_bin: index as u16,
            frequency_hz: index as u32,
            data_position: index,
            profile_id: 0,       // Independent Profile A identity, no TX/RX run.
            modulation_order: 4, // QPSK label matching the two copied LLR anchors.
            before_equalization: [1.0, 0.0],
            after_equalization: [1.0, 0.0],
            reference_iq: Some([1.0, 0.0]),
            llrs: llrs(),
        });
        ncp_pairs.push(NcpConstellationPair {
            channel_symbol_index: index as u64,
            source_symbol_index: index as u64,
            physical_bin: index as u16,
            frequency_hz: index as u32,
            data_position: index,
            modulation_order: 4,
            before_equalization: [1.0, 0.0],
            after_equalization: [1.0, 0.0],
            reference_iq: Some([1.0, 0.0]),
            llrs: llrs(),
        });
    }
    ConstellationResult {
        schema_version: 1, // Existing report schema identity.
        selection: Default::default(),
        status: ConstellationStatus::Partial,
        collected_pairs: SOURCE_CELL_COUNT,
        reference: "copy-only independent fixture",
        channel_coefficients: "copy-only unity fixture",
        pairs,
        ncp_pairs,
    }
}

fn protected_data() -> ProtectedConstellation {
    let mut original =
        ProtectedConstellation::new(Default::default(), ReceiverMode::SuppliedChannel).unwrap();
    for stream in [
        &mut original.continuous_pilots,
        &mut original.scattered_pilots,
        &mut original.plc_data,
        &mut original.plc_preamble,
    ] {
        stream.pairs = Vec::with_capacity(SOURCE_PAIR_SLOTS);
        for index in 0..SOURCE_CELL_COUNT {
            stream.pairs.push(ProtectedPair {
                physical_bin: index as u16,
                frequency_hz: index as u32,
                channel_symbol_index: index as u64,
                plc_phase: 0,
                before_equalization: [1.0, 0.0],
                after_equalization: [1.0, 0.0],
                reference_iq: None,
                receiver_estimate_ready: false,
                used_in_current_fit: false,
                prior_channel_prediction: None,
                pilot_reference: None,
            });
        }
        stream.collected_pairs = SOURCE_CELL_COUNT;
        stream.status = ConstellationStatus::Partial;
    }
    original
}

fn event() -> FullLoopStageSnapshot {
    let mut context = FullLoopStageContext::new("rún", "configuration", RunPhase::Drain).unwrap();
    context.run_id.reserve(SOURCE_ID_SPARE_BYTES);
    context.configuration_id.reserve(SOURCE_ID_SPARE_BYTES);
    context.waveform_grid = Some(FullLoopWaveformGrid {
        geometry: WaveformGeometry::new(&crate::transmission::TransmissionSettings::default())
            .unwrap(),
        sample_rate_hz: crate::transmission::settings::SAMPLE_RATE_HZ,
    });
    let mut roles = Vec::with_capacity(SOURCE_ROLE_SLOTS);
    roles.extend([TransmitCellRole::Data, TransmitCellRole::Ncp]);
    context.transmit_cell_roles = Some(roles);
    let mut decoder_values = llrs();
    decoder_values.extend([0.0, 2.0]); // Additional source values must stay outside the prefix.
    FullLoopStageSnapshot {
        stage: FullLoopStage::PayloadLdpcInput,
        retained_index: 7, // Original diagnostic index, deliberately unrelated to copied count.
        direction: SignalDirection::Receive,
        context,
        domain: SignalDomain::CodewordBit,
        units: ProbeUnits::LogLikelihood,
        element_start: 0,
        total_elements: SOURCE_LLR_SLOTS,
        data: FullLoopStageData::Llrs(decoder_values),
    }
}

#[test]
fn exact_target_matches_all_nested_capacities_and_historical_selection_statistic() {
    let data = data();
    let protected = protected_data();
    let event = event();
    let source = serde_json::to_value((&data, &protected, &event)).unwrap();
    let (bound, _) = preflight(Some(&data), Some(&protected), Some(&event), &policy()).unwrap();
    let archive = detach(
        Some(&data),
        Some(&protected),
        ReceiverMode::SuppliedChannel,
        Some(&event),
        policy(),
        bound,
    )
    .unwrap();
    let legacy = copy::detach(
        Some(&data),
        Some(&protected),
        ReceiverMode::SuppliedChannel,
        Some(&event),
        policy(),
    )
    .unwrap();
    assert_eq!(archive.retained_footprint().unwrap(), bound);
    assert_eq!(
        bound.payload_bytes,
        archive.info().retained_bytes + size_of::<FullLoopReceiverArchive>()
    );
    assert_eq!(bound.ordered_entries, 0);
    assert_eq!(bound.shared_roots, 1);
    assert_eq!(
        serde_json::to_value(archive.info()).unwrap(),
        serde_json::to_value(legacy.info()).unwrap()
    );
    let copied = archive.0.data.as_ref().unwrap();
    assert_eq!(
        copied
            .pairs
            .iter()
            .map(|pair| pair.physical_bin)
            .collect::<Vec<_>>(),
        vec![0, 1, 3]
    );
    assert_eq!(
        copied
            .ncp_pairs
            .iter()
            .map(|pair| pair.physical_bin)
            .collect::<Vec<_>>(),
        vec![0, 2]
    );
    assert_eq!(copied.pairs.capacity(), copied.pairs.len());
    assert_eq!(copied.ncp_pairs.capacity(), copied.ncp_pairs.len());
    for pair in &copied.pairs {
        assert_eq!(pair.llrs.capacity(), pair.llrs.len());
    }
    for pair in &copied.ncp_pairs {
        assert_eq!(pair.llrs.capacity(), pair.llrs.len());
    }
    let copied_protected = archive.0.protected.as_ref().unwrap();
    for stream in [
        &copied_protected.continuous_pilots,
        &copied_protected.scattered_pilots,
        &copied_protected.plc_data,
        &copied_protected.plc_preamble,
    ] {
        assert_eq!(stream.pairs.capacity(), stream.pairs.len());
    }
    let decoder = archive.0.decoder.as_ref().unwrap();
    assert_eq!(
        decoder.context.run_id.capacity(),
        decoder.context.run_id.len()
    );
    assert_eq!(
        decoder.context.configuration_id.capacity(),
        decoder.context.configuration_id.len()
    );
    let roles = decoder.context.transmit_cell_roles.as_ref().unwrap();
    assert_eq!(roles.capacity(), roles.len());
    let FullLoopStageData::Llrs(values) = &decoder.data else {
        panic!("Archived decoder must preserve LLR representation");
    };
    assert_eq!(values.capacity(), values.len());
    assert_eq!(decoder.context.run_id, "rún");
    assert_eq!(
        serde_json::to_value((&data, &protected, &event)).unwrap(),
        source
    );
}

#[test]
fn every_dimension_admits_exact_and_rejects_one_extra_required_unit() {
    // Independent admission predicate anchors; actual archives always have zero ordered entries.
    let exact = RetainedFootprint {
        payload_bytes: 1,
        ordered_entries: 2,
        shared_roots: 3,
    };
    admit(exact, exact).unwrap();
    for required in [
        RetainedFootprint {
            payload_bytes: exact.payload_bytes + 1,
            ..exact
        },
        RetainedFootprint {
            ordered_entries: exact.ordered_entries + 1,
            ..exact
        },
        RetainedFootprint {
            shared_roots: exact.shared_roots + 1,
            ..exact
        },
    ] {
        let Err(error) = admit(required, exact) else {
            panic!("One extra required unit must reject");
        };
        assert_eq!(error.code(), ErrorCode::ResourceLimit);
    }
    admit(RetainedFootprint::default(), RetainedFootprint::default()).unwrap();
}

#[test]
fn empty_archive_still_requires_one_root_and_rejection_preserves_source_and_aliases() {
    let (bound, _) = preflight(None, None, None, &policy()).unwrap();
    assert_eq!(
        bound.payload_bytes,
        size_of::<FullLoopReceiverArchive>() + size_of::<Retained>()
    );
    let Err(error) = detach(
        None,
        None,
        ReceiverMode::SuppliedChannel,
        None,
        policy(),
        RetainedFootprint {
            shared_roots: 0,
            ..bound
        },
    ) else {
        panic!("Even an empty archive needs its one shared root");
    };
    assert_eq!(error.code(), ErrorCode::ResourceLimit);
    let data = data();
    let original = serde_json::to_value(&data).unwrap();
    let (bound, _) = preflight(Some(&data), None, None, &policy()).unwrap();
    let archive = detach(
        Some(&data),
        None,
        ReceiverMode::SuppliedChannel,
        None,
        policy(),
        bound,
    )
    .unwrap();
    let alias = archive.clone();
    let Err(error) = detach(
        Some(&data),
        None,
        ReceiverMode::SuppliedChannel,
        None,
        policy(),
        RetainedFootprint {
            payload_bytes: bound.payload_bytes - 1,
            ..bound
        },
    ) else {
        panic!("One missing target byte must reject");
    };
    assert_eq!(error.code(), ErrorCode::ResourceLimit);
    drop(archive);
    assert_eq!(alias.retained_footprint().unwrap(), bound);
    assert_eq!(serde_json::to_value(&data).unwrap(), original);
}

#[test]
fn omitted_roles_and_decoder_preserve_some_headers_with_zero_buffer_capacity() {
    let data = data();
    let event = event();
    let omitted = || {
        FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput {
            payload_cells: 0,
            ncp_cells: 0,
            protected_cells_per_stream: 0,
            decoder_llrs: 0,
            ..Default::default()
        })
        .unwrap()
    };
    let (bound, _) = preflight(Some(&data), None, Some(&event), &omitted()).unwrap();
    let archive = detach(
        Some(&data),
        None,
        ReceiverMode::SuppliedChannel,
        Some(&event),
        omitted(),
        bound,
    )
    .unwrap();
    assert_eq!(archive.retained_footprint().unwrap(), bound);
    let copied = archive.0.data.as_ref().unwrap();
    assert_eq!(copied.status, data.status);
    assert_eq!(copied.collected_pairs, data.collected_pairs);
    assert_eq!(copied.pairs.capacity(), 0);
    assert_eq!(copied.ncp_pairs.capacity(), 0);
    assert!(archive.0.decoder.is_none());
}

#[test]
fn preflight_preserves_protected_and_decoder_representation_errors() {
    let mut protected = protected_data();
    protected.selection.pairs_per_stream = 0; // Existing invalid selection boundary.
    let Err(old) = copy::detach(
        None,
        Some(&protected),
        ReceiverMode::SuppliedChannel,
        None,
        policy(),
    ) else {
        panic!("Legacy copy must reject invalid protected selection");
    };
    let Err(new) = preflight(None, Some(&protected), None, &policy()) else {
        panic!("Bound must preserve protected rejection");
    };
    assert_eq!(new.code(), old.code());
    assert_eq!(new.engine_code(), old.engine_code());
    assert_eq!(new.message(), old.message());
    let mut event = event();
    event.data = FullLoopStageData::Bits(vec![0]); // Existing wrong decoder representation.
    let Err(old) = copy::detach(
        None,
        None,
        ReceiverMode::SuppliedChannel,
        Some(&event),
        policy(),
    ) else {
        panic!("Legacy copy must reject non-LLR decoder representation");
    };
    let Err(new) = preflight(None, None, Some(&event), &policy()) else {
        panic!("Bound must preserve decoder rejection");
    };
    assert_eq!(new.code(), old.code());
    assert_eq!(new.message(), old.message());
}

#[test]
fn retained_handle_addition_overflow_is_a_resource_limit_before_copy() {
    let Err(error) = bound_from_legacy(usize::MAX) else {
        panic!("Fixed handle addition must reject overflow");
    };
    assert_eq!(error.code(), ErrorCode::ResourceLimit);
}

#[test]
fn decoder_selection_skips_empty_and_non_llr_events_without_relabeling_original() {
    let mut empty = event();
    empty.data = FullLoopStageData::Llrs(Vec::new());
    let mut other = event();
    other.data = FullLoopStageData::Bits(vec![0]);
    let original = event();
    let events = [empty, other, original];
    let before = serde_json::to_value(&events).unwrap();
    let selected = first_nonempty_decoder(&events).unwrap();
    assert!(std::ptr::eq(selected, &events[2])); // Third original event, no copied context.
    assert_eq!(selected.retained_index, events[2].retained_index);
    assert_eq!(serde_json::to_value(&events).unwrap(), before);
}
