/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent history vectors; expected counts/coordinates stay literal to detect policy drift.
use super::*;
use crate::simulation::channel::constellation::ConstellationStatus;
use serde_json::{json, Value};

fn policy() -> FullLoopReceiverArchivePolicy {
    FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput::default()).unwrap()
}
fn payload(count: usize) -> ConstellationResult {
    ConstellationResult {
        schema_version: 1,
        selection: Default::default(),
        status: ConstellationStatus::Partial,
        collected_pairs: count,
        reference: "independent matched fixture",
        channel_coefficients: "independent unity fixture",
        pairs: (0..count)
            .map(|index| ConstellationPair {
                channel_symbol_index: index as u64,
                source_symbol_index: index as u64 + 100,
                physical_bin: 320 + (index % 32) as u16,
                frequency_hz: 100_000_000 + (index % 32) as u32 * 25000,
                data_position: index,
                profile_id: 0,
                modulation_order: 2,
                before_equalization: [1.0, if index < 2048 { 0.0 } else { 0.1 }],
                after_equalization: [1.0, 0.0],
                reference_iq: Some([1.0, 0.0]),
                llrs: vec![-100.0, -99.0, -1.0, 0.0, 1.0, 99.0, 100.0],
            })
            .collect(),
        ncp_pairs: vec![],
    }
}
fn archive(
    data: Option<&ConstellationResult>,
    protected: Option<&ProtectedConstellation>,
    decoder: Option<&FullLoopStageSnapshot>,
    policy: FullLoopReceiverArchivePolicy,
) -> FullLoopReceiverArchive {
    copy::detach(
        data,
        protected,
        ReceiverMode::SuppliedChannel,
        decoder,
        policy,
    )
    .unwrap()
}
fn json<T: serde::Serialize>(value: T) -> Value {
    serde_json::to_value(value).unwrap()
}
#[test]
fn uniform_reduction_keeps_original_coordinates_and_exact_default_caps() {
    let source = payload(10003);
    let history = archive(Some(&source), None, None, policy());
    let info = history.info();
    assert_eq!(info.payload_cells, 8192);
    let last = history
        .qam(FullLoopCellQuery {
            start: 8191,
            count: 1,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(last.cells[0].channel_symbol_index, 10001);
    assert_eq!(last.cells[0].source_symbol_index, Some(10101));
    assert_eq!(history.0.data.as_ref().unwrap().collected_pairs, 10003);
    assert_eq!(
        history.0.data.as_ref().unwrap().status,
        ConstellationStatus::Partial
    );
    let short = payload(10);
    let reduced = archive(
        Some(&short),
        None,
        None,
        FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput {
            payload_cells: 4,
            ..Default::default()
        })
        .unwrap(),
    );
    assert_eq!(
        reduced
            .qam(Default::default())
            .unwrap()
            .cells
            .iter()
            .map(|cell| cell.channel_symbol_index)
            .collect::<Vec<_>>(),
        vec![0, 2, 5, 7]
    );
    assert!(info.retained_bytes <= 8 * 1024 * 1024);
}
#[test]
fn all_selected_mer_uses_errors_beyond_the_display_page() {
    let source = payload(4096);
    let history = archive(Some(&source), None, None, policy());
    let view = history.qam(Default::default()).unwrap();
    assert_eq!(view.cells.len(), 2048);
    assert_eq!(view.selection.selected_cells, 4096);
    // Independent 1/(0.1^2/2) and 1/(0.1^2) power-ratio anchors in dB.
    assert!(
        (view.raw_tx_reference_mer.unwrap().mer_db.unwrap() - 23.010299956639813).abs() < 1e-10
    );
    let tail = history
        .qam(FullLoopCellQuery {
            first_symbol: Some(2048),
            ..Default::default()
        })
        .unwrap();
    assert!((tail.raw_tx_reference_mer.unwrap().mer_db.unwrap() - 20.0).abs() < 1e-10);
    let expected = cells::queries::qam_from_reports(
        Some(&source),
        None,
        Default::default(),
        ReceiverMode::SuppliedChannel,
    )
    .unwrap();
    assert_eq!(json(view), json(expected));
}
#[test]
fn protected_and_ncp_roles_keep_separate_coordinates_amplitudes_and_soft_planes() {
    let mut source = payload(3);
    source.ncp_pairs = source
        .pairs
        .iter()
        .map(|pair| NcpConstellationPair {
            channel_symbol_index: pair.channel_symbol_index,
            source_symbol_index: pair.source_symbol_index,
            physical_bin: pair.physical_bin,
            frequency_hz: pair.frequency_hz,
            data_position: pair.data_position,
            modulation_order: 4,
            before_equalization: pair.before_equalization,
            after_equalization: pair.after_equalization,
            reference_iq: pair.reference_iq,
            llrs: pair.llrs.clone(),
        })
        .collect();
    let mut protected =
        ProtectedConstellation::new(Default::default(), ReceiverMode::SuppliedChannel).unwrap();
    for stream in [
        &mut protected.plc_data,
        &mut protected.plc_preamble,
        &mut protected.continuous_pilots,
        &mut protected.scattered_pilots,
    ] {
        stream.status = ConstellationStatus::Partial;
        stream.collected_pairs = 3;
        stream.pairs = (0..3)
            .map(|index| ProtectedPair {
                physical_bin: 500 + index,
                frequency_hz: 120_000_000 + u32::from(index) * 25000,
                channel_symbol_index: 20 + u64::from(index),
                plc_phase: 3,
                before_equalization: [2.0, 0.0],
                after_equalization: [2.0, 0.0],
                reference_iq: Some([2.0, 0.0]),
                receiver_estimate_ready: true,
                used_in_current_fit: false,
                prior_channel_prediction: None,
                pilot_reference: Some([2.0, 0.0]),
            })
            .collect();
    }
    let history = archive(Some(&source), Some(&protected), None, policy());
    for (stream, amplitude, order) in [
        (FullLoopCellStream::Payload, 1.0, 2),
        (FullLoopCellStream::Ncp, 1.0, 4),
        (FullLoopCellStream::PlcData, 1.0, 16),
        (FullLoopCellStream::PlcPreamble, 1.0, 2),
        (FullLoopCellStream::ContinuousPilots, 2.0, 2),
        (FullLoopCellStream::ScatteredPilots, 2.0, 2),
    ] {
        let query = FullLoopCellQuery {
            stream,
            count: 1,
            ..Default::default()
        };
        let view = history.qam(query.clone()).unwrap();
        assert_eq!(view.cells[0].reference_amplitude, amplitude);
        assert_eq!(view.cells[0].modulation_order, order);
        let expected = cells::queries::qam_from_reports(
            Some(&source),
            Some(&protected),
            query.clone(),
            ReceiverMode::SuppliedChannel,
        )
        .unwrap();
        assert_eq!(json(view), json(expected));
        assert_eq!(
            json(history.cell_decisions(query.clone()).unwrap()),
            json(
                cells::queries::decisions_from_reports(
                    Some(&source),
                    Some(&protected),
                    query.clone()
                )
                .unwrap()
            )
        );
        let soft = history.cell_soft_view(query).unwrap();
        if matches!(
            stream,
            FullLoopCellStream::Payload | FullLoopCellStream::Ncp
        ) {
            assert_eq!(soft.view.statistics.count, 7);
            assert_eq!(soft.view.full.shown_values, 5);
            assert_eq!(soft.view.near_zero.shown_values, 3);
            assert_eq!(soft.view.negative_limit_count, 1);
            assert_eq!(soft.view.positive_limit_count, 1);
        } else {
            assert_eq!(soft.view.statistics.count, 0);
        }
    }
}
#[test]
fn decoder_prefix_preserves_original_context_offset_and_total() {
    let mut context = FullLoopStageContext::new(
        "independent-run",
        "independent-config",
        FullLoopStagePhase::Observation,
    )
    .unwrap();
    context.codeword_index = Some(77);
    let mut values = vec![0.0; 17000];
    values[15] = -100.0;
    values[16] = 100.0;
    let event = FullLoopStageSnapshot {
        stage: FullLoopStage::PayloadLdpcInput,
        retained_index: 3,
        direction: SignalDirection::Receive,
        context,
        domain: SignalDomain::CodewordBit,
        units: ProbeUnits::LogLikelihood,
        element_start: 9,
        total_elements: 20000,
        data: FullLoopStageData::Llrs(values),
    };
    let history = archive(None, None, Some(&event), policy());
    let info = history.info();
    assert_eq!(info.decoder_llrs, 16200);
    assert_eq!(info.decoder_snapshot_index, Some(3));
    assert_eq!(
        info.decoder_context.as_ref().unwrap().codeword_index,
        Some(77)
    );
    assert_eq!(info.decoder_element_start, Some(9));
    assert_eq!(info.decoder_total_elements, Some(20000));
    let view = history.decoder_soft_view().unwrap();
    assert_eq!(view.context, event.context);
    assert_eq!(view.element_start, 9);
    assert_eq!(view.total_elements, 20000);
    assert_eq!(view.view.statistics.count, 16200);
    assert_eq!(view.view.sequence[0].first, 9);
    assert!(view
        .view
        .sequence
        .iter()
        .any(|group| group.minimum == -100.0));
    assert!(view
        .view
        .sequence
        .iter()
        .any(|group| group.maximum == 100.0));
    let omitted = FullLoopReceiverArchiveInput {
        decoder_llrs: 0,
        ..Default::default()
    };
    let omitted = archive(
        None,
        None,
        Some(&event),
        FullLoopReceiverArchivePolicy::new(omitted).unwrap(),
    );
    assert_eq!(omitted.info().decoder_snapshot_index, None);
    assert_eq!(
        omitted.decoder_soft_view().err().unwrap().code(),
        ErrorCode::MissingCoverage
    );
}
#[test]
fn empty_roles_zero_caps_policy_errors_and_byte_rejection_remain_explicit() {
    let source = payload(10);
    let input = FullLoopReceiverArchiveInput {
        payload_cells: 0,
        ncp_cells: 0,
        protected_cells_per_stream: 0,
        decoder_llrs: 0,
        ..Default::default()
    };
    let empty = archive(
        Some(&source),
        None,
        None,
        FullLoopReceiverArchivePolicy::new(input).unwrap(),
    );
    assert_eq!(empty.info().payload_cells, 0);
    assert!(empty.qam(Default::default()).unwrap().cells.is_empty());
    assert_eq!(
        empty.decoder_soft_view().err().unwrap().code(),
        ErrorCode::MissingCoverage
    );
    for document in [
        json!({"payload_cells":8193}),
        json!({"ncp_cells":1025}),
        json!({"protected_cells_per_stream":1025}),
        json!({"decoder_llrs":16201}),
        json!({"max_retained_bytes":0}),
        json!({"max_retained_bytes":8388609}),
        json!({"unexpected":true}),
        json!({"payload_cells":-1}),
    ] {
        assert!(
            FullLoopReceiverArchivePolicy::from_json(&serde_json::to_vec(&document).unwrap())
                .is_err()
        );
    }
    let tight = FullLoopReceiverArchivePolicy::new(FullLoopReceiverArchiveInput {
        max_retained_bytes: 1,
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        copy::detach(
            Some(&source),
            None,
            ReceiverMode::SuppliedChannel,
            None,
            tight
        )
        .err()
        .unwrap()
        .code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(source.pairs.len(), 10);
    let serialized = json(policy().settings());
    assert_eq!(serialized["payload_cells"], 8192);
    assert_eq!(serialized["ncp_cells"], 1024);
    assert_eq!(serialized["protected_cells_per_stream"], 1024);
    assert_eq!(serialized["decoder_llrs"], 16200);
}

#[test]
fn actual_archive_releases_job_stage_store_and_original_profile_bytes() {
    let profile = Dpd::from_bytes(
        &NamedCapture::new(
            "profile",
            include_bytes!(
                "../../../../../../examples/measurements/single-channel/modulation_profile.json"
            )
            .to_vec(),
        )
        .unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap();
    let source_bytes = Arc::downgrade(&profile.sources[0].original);
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let mut original: Value = serde_json::from_slice(include_bytes!(
        "../../../../../../examples/measurements/single-channel/rxmer.json"
    ))
    .unwrap();
    // Clean deterministic one-millisecond physical/FEC fixture; not application defaults.
    original["values"] = json!(vec![80.0; 7480]);
    original["pnm_header"]["capture_time"] = json!(1466967);
    let measured = RxMer::from_bytes(
        &NamedCapture::new("clean", serde_json::to_vec(&original).unwrap()).unwrap(),
    )
    .unwrap();
    let measurements = MeasurementSequence::new(
        &profile,
        &transmission,
        vec![measured],
        MeasurementSequenceOptions {
            mode: MeasurementMode::Single,
            interval_symbols: 1000,
            tolerance: AssociationTolerance::ZERO,
        },
    )
    .unwrap();
    let request = Simulation::prepare_full_loop(
        FullLoopInputs {
            transmission,
            profile,
            source: SourceInput::prbs15(PrbsSeed::new(32767).unwrap()),
            measurements,
            channel: ChannelConfiguration::new(Default::default()).unwrap(),
            response: None,
        },
        FullLoopOptions {
            duration_ms: 1,
            max_iterations: 10,
            max_runtime_ms: 60000,
            ..Default::default()
        },
    )
    .unwrap()
    .with_observations(FullLoopObservationPolicy::new(Default::default()).unwrap())
    .unwrap();
    let mut result = request.run(&FullLoopControl::new()).unwrap();
    let original_event = result
        .stages
        .events(FullLoopStage::PayloadLdpcInput)
        .unwrap()[0]
        .clone();
    let FullLoopStageData::Llrs(values) = &original_event.data else {
        panic!("Actual payload decoder event must carry LLRs");
    };
    // Independent one-MiB fixture budget admits an empty event followed by this real word.
    use crate::simulation::test_points::{ProbeData, ProbeEvent, ProbeObserver};
    let mut retained = observations::RetainedStages::new(1_000_000);
    for observed in [&[][..], values.as_slice()] {
        retained
            .observe(&ProbeEvent {
                stage: FullLoopStage::PayloadLdpcInput,
                direction: original_event.direction,
                context: &original_event.context,
                domain: original_event.domain,
                units: original_event.units,
                element_start: original_event.element_start,
                total_elements: original_event.total_elements,
                shape: [observed.len()],
                data: ProbeData::Llrs(observed),
            })
            .unwrap();
    }
    result.stages = Arc::new(retained);
    let job = Arc::downgrade(&result.job);
    let stages = Arc::downgrade(&result.stages);
    let archive = result.receiver_archive(policy()).unwrap();
    assert_eq!(archive.info().decoder_snapshot_index, Some(1));
    let duplicate = archive.clone();
    let selected = archive
        .qam(FullLoopCellQuery {
            count: 3,
            ..Default::default()
        })
        .unwrap();
    assert!(!selected.cells.is_empty());
    drop(result);
    drop(request);
    drop(archive);
    assert!(job.upgrade().is_none());
    assert!(stages.upgrade().is_none());
    assert!(source_bytes.upgrade().is_none());
    assert_eq!(
        json(
            duplicate
                .qam(FullLoopCellQuery {
                    count: 3,
                    ..Default::default()
                })
                .unwrap()
        ),
        json(selected)
    );
    assert_eq!(
        duplicate.decoder_soft_view().unwrap().view.statistics.count,
        16200
    );
}
