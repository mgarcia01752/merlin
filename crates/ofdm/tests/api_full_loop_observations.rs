/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual passive stage ownership, original coordinate anchors and finite copy/query budgets.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::*;
use serde_json::{json, Value};
fn request() -> FullLoopRequest {
    request_with_receiver(ReceiverMode::SuppliedChannel)
}
fn request_with_receiver(receiver_mode: ReceiverMode) -> FullLoopRequest {
    let profile = Dpd::from_bytes(
        &NamedCapture::new(
            "profile",
            include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json")
                .to_vec(),
        )
        .unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap();
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let mut capture: Value = serde_json::from_slice(include_bytes!(
        "../../../examples/measurements/single-channel/rxmer.json"
    ))
    .unwrap();
    capture["values"] = json!(vec![80.0; 7480]);
    capture["pnm_header"]["capture_time"] = json!(1466967);
    let measured = RxMer::from_bytes(
        &NamedCapture::new("clean", serde_json::to_vec(&capture).unwrap()).unwrap(),
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
    Simulation::prepare_full_loop(
        FullLoopInputs {
            transmission,
            profile,
            measurements,
            source: SourceInput::prbs15(PrbsSeed::new(32767).unwrap()),
            channel: ChannelConfiguration::new(ChannelConfigurationInput::default()).unwrap(),
            response: None,
        },
        FullLoopOptions {
            receiver_mode,
            duration_ms: 1,
            max_iterations: 10,
            max_runtime_ms: 60000,
            ..Default::default()
        },
    )
    .unwrap()
}

#[test]
fn actual_plc_message_query_uses_crc_validated_original_bytes_and_typed_descriptor_owners() {
    use docsis_ofdm_fec::transmission::plc::{descriptors::parse, mac_packets::ManagementPacket};
    let original = request();
    let disabled = original.run(&FullLoopControl::new()).unwrap();
    assert_eq!(
        disabled.plc_messages().err().unwrap().code(),
        ErrorCode::MissingCoverage
    );
    let result = original
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let report = result.plc_messages().unwrap();
    assert_eq!(report.reference_plane, "tx_plc_message_construction");
    assert!(report.retained_frames > 0 && report.retained_frames <= 64);
    assert_eq!(report.processed_frames, report.retained_frames);
    assert!(report.messages.iter().any(|m| m.message_type == 49));
    assert!(report
        .messages
        .iter()
        .any(|m| m.descriptor.profile_id == Some(0)));
    assert!(report
        .messages
        .iter()
        .any(|m| m.descriptor.profile_id == Some(255)));
    assert!(report.messages.len() <= 32);
    assert!(report.issue.is_none());
    for message in &report.messages {
        let packet = ManagementPacket::from_bytes(&message.packet_bytes).unwrap();
        let parsed = parse(&packet).unwrap();
        assert_eq!(packet.message_type(), message.message_type);
        assert_eq!(packet.source_address(), message.source_address);
        assert_eq!(parsed.channel_id, message.descriptor.channel_id);
        assert_eq!(parsed.profile_id, message.descriptor.profile_id);
        assert_eq!(parsed.tlvs.len(), message.descriptor.fields.len());
        for (tlv, field) in parsed.tlvs.iter().zip(&message.descriptor.fields) {
            assert_eq!(tlv.value, field.value);
            assert_eq!(message.packet_bytes[field.offset], field.kind);
            assert_eq!(
                &message.packet_bytes[field.offset + 2..field.offset + 2 + field.value.len()],
                field.value
            );
        }
        let symbol = message.last_frame_context.channel_symbol_index.unwrap();
        assert_eq!(message.last_frame_end_sample, (symbol + 128) * 8704);
        assert_eq!(
            message.completely_transmitted,
            result.report().summary.total_transmitted_samples >= message.last_frame_end_sample
        );
    }
    let truncated = original
        .with_observations(
            FullLoopObservationPolicy::new(FullLoopObservationInput {
                max_elements_per_event: 16,
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let report = truncated.plc_messages().unwrap();
    assert!(report.messages.is_empty());
    assert_eq!(report.processed_frames, 0);
    assert_eq!(
        report.issue.unwrap().error.code(),
        ErrorCode::MissingCoverage
    );
    assert_eq!(truncated.report().summary.comparison.bit_errors, 0);
    assert_eq!(
        serde_json::to_value(result.plc_messages().unwrap()).unwrap(),
        serde_json::to_value(result.plc_messages().unwrap()).unwrap()
    );
}

#[test]
fn receiver_cell_queries_preserve_actual_pairings_all_selected_metrics_and_diagnostic_soft_values()
{
    use docsis_ofdm_fec::signal_processing::modulation::{mer::MerCalculator, slice_symbol};
    let result = request().run(&FullLoopControl::new()).unwrap();
    let view = result
        .qam(FullLoopCellQuery {
            count: 5,
            ..Default::default()
        })
        .unwrap();
    let original = result.report().constellation.as_ref().unwrap();
    assert!(view.selection.selected_cells > 2048);
    assert_eq!(view.selection.retained_cells, original.pairs.len());
    assert_eq!(view.cells.len(), 5);
    for (cell, pair) in view.cells.iter().zip(&original.pairs) {
        assert_eq!(cell.physical_bin, pair.physical_bin);
        assert_eq!(cell.frequency_hz, pair.frequency_hz);
        assert_eq!(cell.channel_symbol_index, pair.channel_symbol_index);
        assert_eq!(cell.source_symbol_index, Some(pair.source_symbol_index));
        assert_eq!(cell.before_equalization, pair.before_equalization);
        assert_eq!(cell.after_equalization, pair.after_equalization);
        assert_eq!(cell.reference_iq, pair.reference_iq);
        assert_eq!(cell.llrs, pair.llrs);
    }
    let order = view.selection.selected_order.unwrap();
    let owner = MerCalculator::new(order, 1.0).unwrap();
    let expected = owner.calculate(
        original
            .pairs
            .iter()
            .filter(|pair| pair.modulation_order == order)
            .map(|pair| pair.before_equalization),
    );
    assert_eq!(view.raw_nearest_mer, expected);
    let later = result
        .qam(FullLoopCellQuery {
            start: 5,
            count: 3,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(later.raw_nearest_mer, view.raw_nearest_mer);
    let decisions = result
        .cell_decisions(FullLoopCellQuery {
            count: 3,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(decisions.rows.len(), 3);
    assert_eq!(decisions.llr_first_selected_cell, 0);
    assert_eq!(decisions.llr_selected_cells, 2048);
    assert!(decisions.llr_statistics.count > 0);
    for row in &decisions.rows {
        assert_eq!(
            row.decision.normalized,
            slice_symbol(row.cell.modulation_order, row.cell.after_equalization).unwrap()
        );
        assert_eq!(
            row.cell.llrs,
            original
                .pairs
                .iter()
                .find(|pair| pair.physical_bin == row.cell.physical_bin
                    && pair.channel_symbol_index == row.cell.channel_symbol_index)
                .unwrap()
                .llrs
        );
    }
    for stream in [
        FullLoopCellStream::Ncp,
        FullLoopCellStream::PlcData,
        FullLoopCellStream::PlcPreamble,
        FullLoopCellStream::ContinuousPilots,
        FullLoopCellStream::ScatteredPilots,
    ] {
        let view = result
            .qam(FullLoopCellQuery {
                stream,
                count: 3,
                ..Default::default()
            })
            .unwrap();
        assert!(!view.cells.is_empty());
        let decisions = result
            .cell_decisions(FullLoopCellQuery {
                stream,
                count: 3,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(decisions.rows.len(), 3);
        for row in &decisions.rows {
            let amplitude = row.cell.reference_amplitude;
            assert_eq!(
                row.decision.normalized_iq,
                row.cell.after_equalization.map(|value| value / amplitude)
            );
            if matches!(
                stream,
                FullLoopCellStream::ContinuousPilots | FullLoopCellStream::ScatteredPilots
            ) {
                assert_eq!(amplitude, 2.0);
                assert_eq!(row.cell.modulation_order, 2);
            }
            if !matches!(stream, FullLoopCellStream::Ncp) {
                assert!(row.cell.llrs.is_empty());
            }
        }
    }
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
}

#[test]
fn empty_and_invalid_cell_views_preserve_retained_availability_and_original_ownership() {
    let result = request().run(&FullLoopControl::new()).unwrap();
    let empty = result
        .qam(FullLoopCellQuery {
            profile_id: Some(255),
            ..Default::default()
        })
        .unwrap();
    assert!(empty.selection.retained_cells > 0);
    assert_eq!(empty.selection.selected_cells, 0);
    assert!(empty.cells.is_empty());
    assert!(empty.raw_tx_reference_mer.is_none());
    assert!(result
        .qam(FullLoopCellQuery {
            count: 2049,
            ..Default::default()
        })
        .is_err());
    assert!(result
        .cell_decisions(FullLoopCellQuery {
            count: 129,
            ..Default::default()
        })
        .is_err());
    assert!(result
        .qam(FullLoopCellQuery {
            start: usize::MAX,
            ..Default::default()
        })
        .is_err());
    let mut view = result
        .qam(FullLoopCellQuery {
            count: 1,
            ..Default::default()
        })
        .unwrap();
    let before = view.cells[0].after_equalization;
    view.cells[0].after_equalization = [99.0, 99.0];
    assert_eq!(
        result
            .qam(FullLoopCellQuery {
                count: 1,
                ..Default::default()
            })
            .unwrap()
            .cells[0]
            .after_equalization,
        before
    );
}

#[test]
fn passive_spectrum_matches_original_samples_and_existing_normalized_welch_owner() {
    use docsis_ofdm_fec::signal_processing::waveform::spectrum::estimate;
    let result = request()
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let query = FullLoopSpectrumQuery::default();
    let view = result.spectrum(query).unwrap();
    assert_eq!(view.stage, FullLoopStage::TxOverlap);
    assert_eq!(view.power.reference_plane, "tx_post_overlap_before_channel");
    assert_eq!(
        view.power.density_units,
        "normalized_sample_units_squared_per_hz"
    );
    assert_eq!(view.power.sample_rate_hz, 204800000);
    assert_eq!(view.power.zero_frequency_hz, 631100000);
    assert_eq!(view.power.spacing_hz, 25000);
    assert_eq!(view.power.fft_samples, 8192);
    assert_eq!(view.snapshots_used, 4);
    assert_eq!(view.available_samples, 4 * 8704);
    assert_eq!(view.used_samples, 8192 + (view.power.averages - 1) * 4096);
    assert_eq!(view.power.averages, 7);
    assert_eq!(view.power.buckets.len(), 2048);
    let mut samples = Vec::new();
    for snapshot in 0..view.snapshots_used {
        for start in [0, 4096, 8192] {
            let point = result
                .stage(
                    FullLoopStage::TxOverlap,
                    snapshot,
                    FullLoopStageViewport { start, count: 0 },
                )
                .unwrap();
            if let FullLoopStageData::Iq(values) = point.data {
                samples.extend(values);
            } else {
                panic!("Actual time samples must be retained I/Q");
            }
        }
    }
    let original = estimate(SubcarrierSpacing::Khz25, 631100000, &samples).unwrap();
    assert!((view.power.equivalent_noise_bandwidth_hz - 37500.0).abs() < 1e-8);
    for (actual, expected) in view
        .power
        .buckets
        .iter()
        .zip(original.buckets(0, 8192).unwrap())
    {
        assert_eq!(actual.first_bin, expected.first_bin);
        assert_eq!(actual.last_bin, expected.last_bin);
        assert_eq!(actual.minimum_db, expected.minimum_db);
        assert_eq!(actual.maximum_db, expected.maximum_db);
    }
    let selected = result
        .spectrum(FullLoopSpectrumQuery {
            max_averages: 1,
            start: 123,
            count: 7,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(selected.snapshots_used, 1);
    assert_eq!(selected.available_samples, 8192);
    assert_eq!(selected.used_samples, 8192);
    assert_eq!(selected.power.buckets[0].first_bin, 123);
    assert_eq!(selected.power.buckets.last().unwrap().last_bin, 129);
    let channel = result
        .spectrum(FullLoopSpectrumQuery {
            stage: FullLoopStage::ChannelOutput,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(
        channel.power.reference_plane,
        "channel_output_receiver_input"
    );
    assert!(channel.snapshots_used > 1);
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
}

#[test]
fn response_and_llr_queries_preserve_original_masks_planes_and_actual_coding_values() {
    use docsis_ofdm_fec::signal_processing::receiver::diagnostics::llr_statistics;
    let result = request()
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let response = result.response(FullLoopResponseQuery::default()).unwrap();
    assert_eq!(
        response.reference_plane,
        "configured_channel_transfer_before_receiver"
    );
    assert_eq!(response.zero_frequency_hz, 631100000);
    assert_eq!(response.spacing_hz, 25000);
    assert_eq!(response.channel_symbol_index, None);
    assert_eq!(response.diagnostics.real.len(), 8192);
    assert_eq!(response.diagnostics.real[0], None);
    assert!(response
        .diagnostics
        .real
        .iter()
        .flatten()
        .all(|value| *value == 1.0));
    assert!(response
        .diagnostics
        .gain_db
        .iter()
        .flatten()
        .all(|value| *value == 0.0));
    assert_eq!(response.diagnostics.peak_delay_microseconds, 0.0);
    assert_eq!(response.diagnostics.delay_microseconds[4096], 0.0);
    assert_eq!(response.diagnostics.available_bins, 7480);
    let soft = result.llr_statistics(0).unwrap();
    assert_eq!(soft.sign_convention, "log_p_zero_over_p_one");
    assert_eq!(soft.element_start, 0);
    assert_eq!(soft.total_elements, 16200);
    let incomplete_estimate = request_with_receiver(ReceiverMode::PilotEstimated)
        .with_observations(
            FullLoopObservationPolicy::new(FullLoopObservationInput {
                max_elements_per_event: 4096,
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    assert_eq!(
        incomplete_estimate
            .response(FullLoopResponseQuery {
                source: FullLoopResponseSource::ReceiverEstimate { snapshot: 0 },
                ..Default::default()
            })
            .err()
            .unwrap()
            .code(),
        ErrorCode::MissingCoverage
    );
    assert_eq!(soft.statistics.count, 16200);
    let mut llrs = Vec::new();
    for start in [0, 4096, 8192, 12288] {
        let actual = result
            .stage(
                FullLoopStage::PayloadLdpcInput,
                0,
                FullLoopStageViewport { start, count: 0 },
            )
            .unwrap();
        if let FullLoopStageData::Llrs(values) = actual.data {
            llrs.extend(values);
        } else {
            panic!("Decoder input must retain actual soft values");
        }
    }
    assert_eq!(
        serde_json::to_value(&soft.statistics).unwrap(),
        serde_json::to_value(llr_statistics(&llrs).unwrap()).unwrap()
    );
    assert_eq!(soft.statistics.histogram.iter().sum::<u64>(), 16200);
    assert_eq!(
        result
            .response(FullLoopResponseQuery {
                source: FullLoopResponseSource::ReceiverEstimate { snapshot: 0 },
                ..Default::default()
            })
            .err()
            .unwrap()
            .code(),
        ErrorCode::MissingCoverage
    );
    let estimated = request_with_receiver(ReceiverMode::PilotEstimated)
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let response = estimated
        .response(FullLoopResponseQuery {
            source: FullLoopResponseSource::ReceiverEstimate { snapshot: 0 },
            ..Default::default()
        })
        .unwrap();
    assert_eq!(
        response.reference_plane,
        "receiver_estimated_channel_transfer"
    );
    assert!(response.channel_symbol_index.is_some());
    assert_eq!(response.diagnostics.available_bins, 7480);
    assert_eq!(response.diagnostics.real[0], None);
}

#[test]
fn numerical_query_limits_gaps_missing_and_truncated_observations_remain_explicit() {
    let request = request();
    let disabled = request.run(&FullLoopControl::new()).unwrap();
    assert_eq!(
        disabled.spectrum(Default::default()).err().unwrap().code(),
        ErrorCode::MissingCoverage
    );
    assert_eq!(
        disabled.llr_statistics(0).err().unwrap().code(),
        ErrorCode::MissingCoverage
    );
    let truncated = request
        .with_observations(
            FullLoopObservationPolicy::new(FullLoopObservationInput {
                max_elements_per_event: 4096,
                ..Default::default()
            })
            .unwrap(),
        )
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    // Each TX event's second half is absent: adjacent retained events must not hide that gap.
    assert!(truncated.spectrum(Default::default()).is_err());
    let soft = truncated.llr_statistics(0).unwrap();
    assert_eq!(soft.statistics.count, 4096);
    assert_eq!(soft.total_elements, 16200);
    let result = request
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    for query in [
        FullLoopSpectrumQuery {
            max_averages: 0,
            ..Default::default()
        },
        FullLoopSpectrumQuery {
            max_averages: 33,
            ..Default::default()
        },
        FullLoopSpectrumQuery {
            start: usize::MAX,
            ..Default::default()
        },
        FullLoopSpectrumQuery {
            count: usize::MAX,
            ..Default::default()
        },
        FullLoopSpectrumQuery {
            snapshot: usize::MAX,
            ..Default::default()
        },
        FullLoopSpectrumQuery {
            stage: FullLoopStage::RxFft,
            ..Default::default()
        },
    ] {
        assert!(result.spectrum(query).is_err());
    }
    for velocity_factor in [0.0, 1.01, f64::NAN, f64::INFINITY] {
        assert!(result
            .response(FullLoopResponseQuery {
                velocity_factor,
                ..Default::default()
            })
            .is_err());
    }
    assert!(result.llr_statistics(usize::MAX).is_err());
    let before = serde_json::to_value(result.observation_stats()).unwrap();
    let a = result.spectrum(Default::default()).unwrap();
    let b = result.spectrum(Default::default()).unwrap();
    assert_eq!(
        serde_json::to_value(a).unwrap(),
        serde_json::to_value(b).unwrap()
    );
    assert_eq!(
        before,
        serde_json::to_value(result.observation_stats()).unwrap()
    );
}
fn policy() -> FullLoopObservationPolicy {
    FullLoopObservationPolicy::new(FullLoopObservationInput::default()).unwrap()
}

#[test]
fn actual_typed_retention_preserves_science_and_literal_source_grid_pilot_and_phase_anchors() {
    let original = request();
    let selected = original.with_observations(policy()).unwrap();
    assert!(original.observation_policy().is_none());
    assert!(selected.observation_policy().is_some());
    let disabled = original.run(&FullLoopControl::new()).unwrap();
    let retained = selected.run(&FullLoopControl::new()).unwrap();
    assert_eq!(
        disabled.report().summary.comparison,
        retained.report().summary.comparison
    );
    assert_eq!(
        disabled.report().retained_words,
        retained.report().retained_words
    );
    assert_eq!(disabled.report().probe_stats.dispatched_events, 0);
    assert!(disabled.stage_inventory().is_empty());
    assert!(retained.report().probe_stats.dispatched_events > 0);
    assert!(retained.observation_stats().retained_events > 0);
    assert!(!retained.observation_stats().byte_budget_exhausted);
    let source = retained
        .stage(
            FullLoopStage::SourcePayload,
            0,
            FullLoopStageViewport {
                start: 0,
                count: 32,
            },
        )
        .unwrap();
    assert_eq!(source.total_elements, 14232);
    assert_eq!(source.element_start, 0);
    assert_eq!(source.context.phase, FullLoopStagePhase::Observation);
    if let FullLoopStageData::Bits(bits) = &source.data {
        assert_eq!(
            *bits,
            b"11111111111111101010101010101001"
                .iter()
                .map(|digit| digit - b'0')
                .collect::<Vec<_>>()
        );
    } else {
        panic!("Source must retain actual binary values");
    }
    // Frame assembly defines excluded FFT bins as known zero, not unavailable history.
    let physical = retained
        .stage(
            FullLoopStage::PhysicalBins,
            0,
            FullLoopStageViewport {
                start: 0,
                count: 16,
            },
        )
        .unwrap();
    assert_eq!(physical.total_elements, 8192);
    assert_eq!(
        physical.context.physical_grid.unwrap().zero_frequency_hz,
        631100000
    );
    assert!(
        matches!(physical.data,FullLoopStageData::OptionalIq(ref points) if points.iter().all(|point| *point == Some([0.0, 0.0])))
    );
    let pilot = retained
        .stage(
            FullLoopStage::ContinuousPilots,
            0,
            FullLoopStageViewport::default(),
        )
        .unwrap();
    assert_eq!(
        pilot.context.reference,
        FullLoopStageReference::OtherSubcarrierRms { rms: 1.0 }
    );
    if let FullLoopStageData::Pilots(points) = pilot.data {
        assert!(!points.is_empty());
        for point in points {
            let bin = retained
                .stage(
                    FullLoopStage::PhysicalBins,
                    0,
                    FullLoopStageViewport {
                        start: usize::from(point.bin),
                        count: 1,
                    },
                )
                .unwrap();
            assert_eq!(
                bin.context.channel_symbol_index,
                pilot.context.channel_symbol_index
            );
            assert!(
                matches!(bin.data,FullLoopStageData::OptionalIq(ref samples) if samples[0]==Some(point.iq))
            );
            assert!((point.iq[0] * point.iq[0] + point.iq[1] * point.iq[1] - 4.0).abs() < 1e-12);
        }
    } else {
        panic!("Pilot records must keep their original bins and I/Q");
    }
    let plc = retained
        .stage(
            FullLoopStage::PlcMessage,
            0,
            FullLoopStageViewport::default(),
        )
        .unwrap();
    assert_eq!(plc.context.phase, FullLoopStagePhase::Warmup);
    assert!(matches!(plc.data, FullLoopStageData::Bytes(_)));
    let waveform = retained
        .stage(
            FullLoopStage::TxOverlap,
            0,
            FullLoopStageViewport::default(),
        )
        .unwrap();
    assert_eq!(
        waveform
            .context
            .waveform_grid
            .as_ref()
            .unwrap()
            .sample_rate_hz,
        204800000
    );
    assert!(waveform.data.len() <= 4096);
    drop(retained);
    drop(selected);
    assert_eq!(source.data.len(), 32);
}

#[test]
fn byte_exhaustion_and_event_caps_never_change_completed_science() {
    let input = FullLoopObservationInput {
        max_retained_bytes: 1,
        ..Default::default()
    };
    let request = request();
    let baseline = request.run(&FullLoopControl::new()).unwrap();
    let result = request
        .with_observations(FullLoopObservationPolicy::new(input).unwrap())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Complete);
    assert_eq!(
        result.report().retained_words,
        baseline.report().retained_words
    );
    let stats = result.observation_stats();
    assert_eq!(stats.retained_bytes, 0);
    assert_eq!(stats.retained_events, 0);
    assert!(stats.dropped_events > 0);
    assert!(stats.byte_budget_exhausted);
    assert!(result.stage_inventory().is_empty());
    let limited = FullLoopObservationInput {
        stages: vec![FullLoopStageRetention {
            stage: FullLoopStage::TxOverlap,
            max_events: 1,
            observation_only: true,
        }],
        max_elements_per_event: 16,
        max_total_elements: 16,
        max_events: 1,
        ..Default::default()
    };
    let result = request
        .with_observations(FullLoopObservationPolicy::new(limited).unwrap())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    assert_eq!(result.stage_inventory()[0].snapshots, 1);
    assert_eq!(result.observation_stats().retained_elements, 16);
    assert!(result.observation_stats().retained_bytes <= MAX_STAGE_RETENTION_BYTES);
    assert_eq!(
        result.report().summary.comparison,
        baseline.report().summary.comparison
    );
}

#[test]
fn policy_and_query_bounds_reject_without_mutating_retained_originals() {
    for bytes in [b"{".as_slice(), b"{\"unknown\":1}".as_slice()] {
        assert!(FullLoopObservationPolicy::from_json(bytes).is_err());
    }
    assert!(FullLoopObservationPolicy::from_json(&vec![b' '; 65537]).is_err());
    assert!(FullLoopObservationPolicy::new(FullLoopObservationInput {
        stages: vec![],
        ..Default::default()
    })
    .is_err());
    let mut duplicate = FullLoopObservationInput::default();
    duplicate.stages.push(duplicate.stages[0].clone());
    assert!(FullLoopObservationPolicy::new(duplicate).is_err());
    assert_eq!(
        FullLoopObservationPolicy::new(FullLoopObservationInput {
            stages: vec![FullLoopStageRetention {
                stage: FullLoopStage::LegacyNoiseOutput,
                max_events: 1,
                observation_only: true
            }],
            ..Default::default()
        })
        .err()
        .unwrap()
        .code(),
        ErrorCode::UnsupportedCapability
    );
    for invalid in [
        FullLoopObservationInput {
            max_events: 0,
            ..Default::default()
        },
        FullLoopObservationInput {
            max_elements_per_event: 16385,
            ..Default::default()
        },
        FullLoopObservationInput {
            max_retained_bytes: MAX_STAGE_RETENTION_BYTES + 1,
            ..Default::default()
        },
    ] {
        assert!(FullLoopObservationPolicy::new(invalid).is_err());
    }
    let result = request()
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let before = result
        .stage(
            FullLoopStage::TxOverlap,
            0,
            FullLoopStageViewport { start: 7, count: 5 },
        )
        .unwrap();
    assert_eq!(before.element_start, 7);
    assert_eq!(before.data.len(), 5);
    for view in [
        FullLoopStageViewport {
            start: 0,
            count: 4097,
        },
        FullLoopStageViewport {
            start: usize::MAX,
            count: 0,
        },
        FullLoopStageViewport {
            start: 8191,
            count: 1000,
        },
    ] {
        assert!(result.stage(FullLoopStage::TxOverlap, 0, view).is_err());
    }
    assert_eq!(
        result
            .stage(
                FullLoopStage::TxOverlap,
                usize::MAX,
                FullLoopStageViewport::default()
            )
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidIdentifier
    );
    assert_eq!(
        result
            .stage(
                FullLoopStage::LegacyNoiseOutput,
                0,
                FullLoopStageViewport::default()
            )
            .err()
            .unwrap()
            .code(),
        ErrorCode::MissingCoverage
    );
    let after = result
        .stage(
            FullLoopStage::TxOverlap,
            0,
            FullLoopStageViewport { start: 7, count: 5 },
        )
        .unwrap();
    assert_eq!(
        serde_json::to_value(before).unwrap(),
        serde_json::to_value(after).unwrap()
    );
}
