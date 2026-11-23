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
    request_with_channel(
        receiver_mode,
        ChannelConfiguration::new(ChannelConfigurationInput::default()).unwrap(),
    )
}
fn request_with_channel(
    receiver_mode: ReceiverMode,
    channel: ChannelConfiguration,
) -> FullLoopRequest {
    request_with_transmission(receiver_mode, channel, TransmissionSettings::default())
}
fn request_with_transmission(
    receiver_mode: ReceiverMode,
    channel: ChannelConfiguration,
    settings: TransmissionSettings,
) -> FullLoopRequest {
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
    let transmission = TransmissionConfiguration::new(settings).unwrap();
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
            channel,
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
fn sample_budget_capacity_preserves_the_actual_long_cp_legacy_spectrum_prefix() {
    let request = request_with_transmission(
        ReceiverMode::SuppliedChannel,
        ChannelConfiguration::new(ChannelConfigurationInput::default()).unwrap(),
        TransmissionSettings {
            cyclic_prefix_samples: 1024,
            ..Default::default()
        },
    );
    let request = request
        .with_observations(
            FullLoopObservationPolicy::new(FullLoopObservationInput::default()).unwrap(),
        )
        .unwrap();
    let result = request.run(&FullLoopControl::new()).unwrap();
    let before = serde_json::to_value(result.report()).unwrap();
    let full = result
        .calibrated_spectrum(FullLoopSpectrumQuery::default())
        .unwrap();
    // Independent original geometry: four (8192+1024)-sample blocks are 36864
    // samples/eight Welch windows. The legacy 32768 prefix holds seven windows.
    assert_eq!(full.available_samples, 36864);
    assert_eq!(full.used_samples, 36864);
    assert_eq!(full.power.averages, 8);
    let averages = result
        .inputs()
        .transmission
        .spectrum_average_capacity(32768)
        .unwrap();
    assert_eq!(averages, 7);
    let limited = result
        .calibrated_spectrum(FullLoopSpectrumQuery {
            max_averages: averages,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(limited.available_samples, 32768);
    assert_eq!(limited.used_samples, 32768);
    assert_eq!(limited.power.averages, 7);
    assert_eq!(limited.first_sample, full.first_sample);
    assert_eq!(limited.context, full.context);
    assert_eq!(limited.source_reference_plane, full.source_reference_plane);
    assert_eq!(serde_json::to_value(result.report()).unwrap(), before);
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
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
fn static_reference_and_actual_soft_views_reuse_owned_science_without_advancing_inputs() {
    use docsis_ofdm_fec::signal_processing::receiver::diagnostics::llr_statistics;
    let reference = Simulation::constellation_reference(2, 2.0, 0, 0).unwrap();
    assert_eq!(reference.points[0].iq, [2.0, 0.0]);
    assert_eq!(reference.points[1].iq, [-2.0, 0.0]);
    let region = Simulation::decision_region(2, 0, 1.0).unwrap();
    assert_eq!(region.center_iq, [1.0, 0.0]);
    assert_eq!(region.viewport_half_extent, 5.0);
    assert_eq!(
        Simulation::constellation_reference(8, 1.0, 0, 0)
            .err()
            .unwrap()
            .code(),
        ErrorCode::UnsupportedCapability
    );
    assert_eq!(
        Simulation::constellation_reference(16384, 1.0, 0, 4097)
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(
        Simulation::decision_region(16, 16, 1.0)
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidIdentifier
    );
    assert!(Simulation::decision_region(16, 0, f64::NAN).is_err());
    let original = request();
    let missing = original.run(&FullLoopControl::new()).unwrap();
    assert_eq!(
        missing.decoder_soft_view(0).err().unwrap().code(),
        ErrorCode::MissingCoverage
    );
    let result = original
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let cells = result
        .qam(FullLoopCellQuery {
            count: 3,
            ..Default::default()
        })
        .unwrap();
    let values: Vec<_> = cells
        .cells
        .iter()
        .flat_map(|cell| cell.llrs.iter().copied())
        .collect();
    let view = result
        .cell_soft_view(FullLoopCellQuery {
            count: 3,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(view.source_plane, "post_descrambling_cell_llrs");
    assert_eq!(view.selection.displayed_cells, 3);
    assert_eq!(
        serde_json::to_value(&view.view.statistics).unwrap(),
        serde_json::to_value(llr_statistics(&values).unwrap()).unwrap()
    );
    assert_eq!(
        view.view.full.shown_values
            + view.view.negative_limit_count
            + view.view.positive_limit_count,
        values.len()
    );
    let decoder = result.decoder_soft_view(0).unwrap();
    assert_eq!(decoder.total_elements, 16200);
    assert_eq!(decoder.element_start, 0);
    assert_eq!(decoder.view.statistics.count, 16200);
    assert!(decoder.view.sequence.len() <= 1024);
    assert_eq!(decoder.view.sequence.last().unwrap().last, 16199);
    assert_eq!(
        serde_json::to_value(&decoder.view.statistics).unwrap(),
        serde_json::to_value(result.llr_statistics(0).unwrap().statistics).unwrap()
    );
    assert!(result.decoder_soft_view(usize::MAX).is_err());
    let pilot = result
        .cell_soft_view(FullLoopCellQuery {
            stream: FullLoopCellStream::ContinuousPilots,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(pilot.view.statistics.count, 0);
    assert!(pilot.view.sequence.is_empty());
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
    assert_eq!(
        serde_json::to_value(result.decoder_soft_view(0).unwrap()).unwrap(),
        serde_json::to_value(result.decoder_soft_view(0).unwrap()).unwrap()
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
fn calibrated_channel_taps_preserve_actual_sources_full_band_power_and_distinct_amplifier_planes() {
    const AMPLIFIER_GAIN_DB: f64 = 20.0;
    const CPA_ATTENUATION_DB: f64 = 6.0;
    const INPUT_REFERENCE_DBMV: f64 = 0.0;
    const EXPECTED_AMPLITUDE: f64 = 10.0;
    const VIEW_START_BIN: usize = 350;
    const VIEW_BIN_COUNT: usize = 7;
    const POWER_RELATIVE_TOLERANCE: f64 = 1e-10;
    const DB_ABSOLUTE_TOLERANCE: f64 = 1e-9;
    let channel = ChannelConfiguration::new(ChannelConfigurationInput {
        power: PowerConditioning {
            amplifier_gain_db: AMPLIFIER_GAIN_DB,
            cpa_attenuation_db: CPA_ATTENUATION_DB,
            input_data_level_dbmv_per_6mhz: Some(INPUT_REFERENCE_DBMV),
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
    let result = request_with_channel(ReceiverMode::SuppliedChannel, channel)
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let stats = serde_json::to_value(result.observation_stats()).unwrap();
    let original =
        serde_json::to_value(result.spectrum(FullLoopSpectrumQuery::default()).unwrap()).unwrap();
    let query = FullLoopSpectrumQuery {
        start: VIEW_START_BIN,
        count: VIEW_BIN_COUNT,
        ..Default::default()
    };
    let input = result
        .channel_spectrum(FullLoopChannelTap::BeforeAmplifier, query.clone())
        .unwrap();
    let amplified = result
        .channel_spectrum(FullLoopChannelTap::AfterAmplifier, query.clone())
        .unwrap();
    let actual = result.calibrated_spectrum(query.clone()).unwrap();
    assert_eq!(
        actual.power.normalized_power_db,
        input.power.normalized_power_db
    );
    assert_eq!(input.derived_amplitude, 1.0);
    assert_eq!(amplified.derived_amplitude, EXPECTED_AMPLITUDE);
    assert_eq!(
        amplified.source_reference_plane,
        input.source_reference_plane
    );
    assert_eq!(amplified.first_sample, input.first_sample);
    assert_eq!(amplified.available_samples, input.available_samples);
    assert_eq!(input.power.bucket_units, "dbmv_per_hz");
    assert_eq!(input.power.density_units, "millivolts_squared_per_hz");
    assert_eq!(
        input.power.channel_power.dbmv,
        actual.power.channel_power.dbmv
    );
    assert!(
        (amplified.power.channel_power.normalized_power
            / input.power.channel_power.normalized_power
            - EXPECTED_AMPLITUDE * EXPECTED_AMPLITUDE)
            .abs()
            < POWER_RELATIVE_TOLERANCE
    );
    assert!(
        (amplified.power.channel_power.dbmv.unwrap()
            - input.power.channel_power.dbmv.unwrap()
            - AMPLIFIER_GAIN_DB)
            .abs()
            < DB_ABSOLUTE_TOLERANCE
    );
    let full = result
        .calibrated_spectrum(FullLoopSpectrumQuery::default())
        .unwrap();
    assert_eq!(
        full.power.channel_power.normalized_power,
        actual.power.channel_power.normalized_power
    );
    assert_eq!(
        full.power.normalized_power_db,
        actual.power.normalized_power_db
    );
    assert_eq!(
        full.power.channel_power.dbmv,
        actual.power.channel_power.dbmv
    );
    for (after, before) in amplified.power.buckets.iter().zip(&input.power.buckets) {
        assert!(
            (after.maximum_db.unwrap() - before.maximum_db.unwrap() - AMPLIFIER_GAIN_DB).abs()
                < DB_ABSOLUTE_TOLERANCE
        );
    }
    let output_query = FullLoopSpectrumQuery {
        stage: FullLoopStage::ChannelOutput,
        ..query.clone()
    };
    let received = result
        .channel_spectrum(FullLoopChannelTap::ReceiverInput, output_query.clone())
        .unwrap();
    let actual_received = result.calibrated_spectrum(output_query).unwrap();
    assert_eq!(received.derived_amplitude, 1.0);
    assert_eq!(
        received.power.reference_plane,
        "channel_output_receiver_input"
    );
    assert_eq!(
        serde_json::to_value(&received).unwrap(),
        serde_json::to_value(actual_received).unwrap()
    );
    let before = result
        .amplifier_bins(AmplifierBoundary::Before, 0, VIEW_START_BIN, VIEW_BIN_COUNT)
        .unwrap();
    let after = result
        .amplifier_bins(AmplifierBoundary::After, 0, VIEW_START_BIN, VIEW_BIN_COUNT)
        .unwrap();
    assert_eq!(after.amplitude, EXPECTED_AMPLITUDE);
    assert_eq!(
        serde_json::to_value(&before.observation.layout).unwrap(),
        serde_json::to_value(&after.observation.layout).unwrap()
    );
    assert_eq!(before.observation.context, after.observation.context);
    for (before, after) in before.observation.bins.iter().zip(&after.observation.bins) {
        assert_eq!(
            after.iq,
            before
                .iq
                .map(|pair| [pair[0] * EXPECTED_AMPLITUDE, pair[1] * EXPECTED_AMPLITUDE])
        );
        match (before.linear_power, after.linear_power) {
            (Some(before), Some(after)) => {
                assert!(
                    (after - before * EXPECTED_AMPLITUDE * EXPECTED_AMPLITUDE).abs()
                        < POWER_RELATIVE_TOLERANCE * after.max(1.0)
                );
            }
            (None, None) => {}
            _ => panic!("Missing history changed"),
        }
    }
    assert!(result
        .channel_spectrum(FullLoopChannelTap::ReceiverInput, query)
        .is_err());
    assert!(result
        .calibrated_spectrum(FullLoopSpectrumQuery {
            stage: FullLoopStage::PhysicalBins,
            ..Default::default()
        })
        .is_err());
    assert!(result
        .amplifier_bins(AmplifierBoundary::After, 0, 0, usize::MAX)
        .is_err());
    assert_eq!(
        serde_json::to_value(result.spectrum(FullLoopSpectrumQuery::default()).unwrap()).unwrap(),
        original
    );
    assert_eq!(
        serde_json::to_value(result.observation_stats()).unwrap(),
        stats
    );
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
}

#[test]
fn relative_channel_spectra_and_missing_taps_remain_explicit_without_hidden_execution() {
    let original = request();
    let disabled = original.run(&FullLoopControl::new()).unwrap();
    assert_eq!(
        disabled
            .calibrated_spectrum(FullLoopSpectrumQuery::default())
            .err()
            .unwrap()
            .code(),
        ErrorCode::MissingCoverage
    );
    assert_eq!(
        disabled
            .amplifier_bins(AmplifierBoundary::Before, 0, 0, 0)
            .err()
            .unwrap()
            .code(),
        ErrorCode::MissingCoverage
    );
    assert_eq!(
        disabled
            .calibrated_spectrum_sequence(FullLoopSpectrumQuery {
                stage: FullLoopStage::TxIfft,
                ..Default::default()
            })
            .err()
            .unwrap()
            .code(),
        ErrorCode::MissingCoverage
    );
    let observed = original
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let relative = observed
        .calibrated_spectrum(FullLoopSpectrumQuery::default())
        .unwrap();
    assert_eq!(
        relative.power.density_units,
        "normalized_sample_units_squared_per_hz"
    );
    assert!(relative.power.channel_power.dbmv.is_none());
    assert!(relative.power.millivolts_squared_per_unit.is_none());
    let normalized = observed.spectrum(FullLoopSpectrumQuery::default()).unwrap();
    assert_eq!(
        relative.power.buckets[0].minimum_db,
        normalized.power.buckets[0].minimum_db
    );
}

#[test]
fn ordered_frame_spectra_preserve_original_segments_and_keep_contiguous_queries_distinct() {
    // Fixture policies are independent even where their chosen values coincide.
    const RETAINED_FRAMES_PER_STAGE: usize = 3;
    const WELCH_AVERAGES_PER_QUERY: usize = 3;
    const EXPECTED_FFT_SAMPLES: usize = 8192;
    const EXPECTED_TWO_FRAME_SAMPLES: usize = 16384;
    const FIRST_VIEW_BIN: usize = 123;
    const VIEW_BIN_COUNT: usize = 7;
    use docsis_ofdm_fec::signal_processing::waveform::spectrum::estimate;
    let mut input = FullLoopObservationInput::default();
    let stages = [
        FullLoopStage::TxIfft,
        FullLoopStage::TxPrefix,
        FullLoopStage::TxWindow,
        FullLoopStage::RxUseful,
    ];
    for retention in &mut input.stages {
        if stages.contains(&retention.stage) {
            retention.max_events = RETAINED_FRAMES_PER_STAGE;
        }
    }
    let result = request()
        .with_observations(FullLoopObservationPolicy::new(input).unwrap())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let before = serde_json::to_value(result.observation_stats()).unwrap();
    for stage in stages {
        let query = FullLoopSpectrumQuery {
            stage,
            max_averages: WELCH_AVERAGES_PER_QUERY,
            start: FIRST_VIEW_BIN,
            count: VIEW_BIN_COUNT,
            ..Default::default()
        };
        let sequence = result.spectrum_sequence(query.clone()).unwrap();
        assert_eq!(sequence.sampling_basis, "ordered_frame_stage_samples");
        assert_eq!(sequence.available_samples, EXPECTED_TWO_FRAME_SAMPLES);
        assert_eq!(sequence.used_samples, EXPECTED_TWO_FRAME_SAMPLES);
        assert_eq!(sequence.power.averages, WELCH_AVERAGES_PER_QUERY);
        assert_eq!(sequence.segments.len(), 2);
        assert_eq!(sequence.stop, FullLoopSpectrumStop::WorkLimit);
        assert_eq!(
            sequence.physically_contiguous,
            stage == FullLoopStage::TxPrefix
        );
        assert_eq!(
            sequence
                .segments
                .iter()
                .map(|s| s.analyzed_samples)
                .sum::<usize>(),
            sequence.used_samples
        );
        let original = result.spectrum(query).unwrap();
        assert_eq!(original.snapshots_used, 1);
        assert_eq!(original.power.averages, 1);
        let mut samples = Vec::new();
        for segment in &sequence.segments {
            let first = result
                .stage(
                    stage,
                    segment.retained_index,
                    FullLoopStageViewport { start: 0, count: 1 },
                )
                .unwrap();
            assert_eq!(
                segment.original_sample_start,
                first.context.sample_start_index.unwrap()
            );
            assert_eq!(segment.total_elements, first.total_elements);
            assert_eq!(
                segment.channel_symbol_index,
                first.context.channel_symbol_index.unwrap()
            );
            for start in (0..segment.sample_count).step_by(MAX_STAGE_QUERY_ELEMENTS) {
                let raw = result
                    .stage(
                        stage,
                        segment.retained_index,
                        FullLoopStageViewport {
                            start,
                            count: MAX_STAGE_QUERY_ELEMENTS.min(segment.sample_count - start),
                        },
                    )
                    .unwrap();
                let FullLoopStageData::Iq(values) = raw.data else {
                    unreachable!()
                };
                samples.extend(values);
            }
        }
        let grid = sequence.context.physical_grid.unwrap();
        let direct = estimate(grid.spacing, grid.zero_frequency_hz, &samples).unwrap();
        for (actual, expected) in sequence
            .power
            .buckets
            .iter()
            .zip(direct.buckets(FIRST_VIEW_BIN, VIEW_BIN_COUNT).unwrap())
        {
            assert_eq!(actual.minimum_db, expected.minimum_db);
            assert_eq!(actual.maximum_db, expected.maximum_db);
        }
    }
    for stage in [
        FullLoopStage::PhysicalBins,
        FullLoopStage::TxOverlap,
        FullLoopStage::RxFft,
    ] {
        assert_eq!(
            result
                .spectrum_sequence(FullLoopSpectrumQuery {
                    stage,
                    ..Default::default()
                })
                .err()
                .unwrap()
                .code(),
            ErrorCode::UnsupportedCapability
        );
    }
    for max_averages in [0, 33, usize::MAX] {
        assert!(result
            .spectrum_sequence(FullLoopSpectrumQuery {
                stage: FullLoopStage::TxIfft,
                max_averages,
                ..Default::default()
            })
            .is_err());
    }
    assert!(result
        .spectrum_sequence(FullLoopSpectrumQuery {
            stage: FullLoopStage::TxIfft,
            start: EXPECTED_FFT_SAMPLES,
            ..Default::default()
        })
        .is_err());
    assert_eq!(
        serde_json::to_value(result.observation_stats()).unwrap(),
        before
    );
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
    let mut truncated = FullLoopObservationInput {
        max_elements_per_event: EXPECTED_FFT_SAMPLES,
        ..Default::default()
    };
    truncated
        .stages
        .iter_mut()
        .find(|entry| entry.stage == FullLoopStage::TxPrefix)
        .unwrap()
        .max_events = RETAINED_FRAMES_PER_STAGE;
    let partial = request()
        .with_observations(FullLoopObservationPolicy::new(truncated).unwrap())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    let view = partial
        .spectrum_sequence(FullLoopSpectrumQuery {
            stage: FullLoopStage::TxPrefix,
            max_averages: WELCH_AVERAGES_PER_QUERY,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(view.stop, FullLoopSpectrumStop::IncompleteFrame);
    assert_eq!(view.segments.len(), 1);
    assert_eq!(view.available_samples, EXPECTED_FFT_SAMPLES);
    assert_eq!(view.power.averages, 1);
    assert!(view.segments[0].retained_elements < view.segments[0].total_elements);
    let calibrated = partial
        .calibrated_spectrum_sequence(FullLoopSpectrumQuery {
            stage: FullLoopStage::TxPrefix,
            max_averages: WELCH_AVERAGES_PER_QUERY,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(
        serde_json::to_value(&calibrated.sequence).unwrap(),
        serde_json::to_value(&view).unwrap()
    );
    assert_eq!(calibrated.power.averages, 1);
}

#[test]
fn calibrated_ordered_frames_preserve_legacy_sequence_units_power_and_error_contracts() {
    // Independent legacy 16K prefix and three-average policy, separate from event retention.
    const SAMPLE_BUDGET: usize = 16384;
    const RETAINED_FRAMES: usize = 3;
    // Zero dBmV/6 MHz reference at the existing 204.8 MS/s clock: 34.133... mV²/unit.
    const EXPECTED_CALIBRATION: f64 = 204_800_000.0 / 6_000_000.0;
    const DB_TOLERANCE: f64 = 1e-10;
    let stages = [
        FullLoopStage::TxIfft,
        FullLoopStage::TxPrefix,
        FullLoopStage::TxWindow,
        FullLoopStage::RxUseful,
    ];
    for electrical in [false, true] {
        let channel = ChannelConfiguration::new(ChannelConfigurationInput {
            power: PowerConditioning {
                input_data_level_dbmv_per_6mhz: electrical.then_some(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap();
        let mut input = FullLoopObservationInput::default();
        for entry in &mut input.stages {
            if stages.contains(&entry.stage) {
                entry.max_events = RETAINED_FRAMES;
            }
        }
        let result = request_with_channel(ReceiverMode::SuppliedChannel, channel)
            .with_observations(FullLoopObservationPolicy::new(input).unwrap())
            .unwrap()
            .run(&FullLoopControl::new())
            .unwrap();
        let before = serde_json::to_value(result.report()).unwrap();
        let statistics = serde_json::to_value(result.observation_stats()).unwrap();
        for stage in stages {
            let query = FullLoopSpectrumQuery {
                stage,
                max_averages: result
                    .inputs()
                    .transmission
                    .spectrum_average_capacity(SAMPLE_BUDGET)
                    .unwrap(),
                start: 123,
                count: 7,
                ..Default::default()
            };
            let normalized = result.spectrum_sequence(query.clone()).unwrap();
            let view = result.calibrated_spectrum_sequence(query.clone()).unwrap();
            assert_eq!(
                serde_json::to_value(&view.sequence).unwrap(),
                serde_json::to_value(&normalized).unwrap()
            );
            assert_eq!(view.sequence.sampling_basis, "ordered_frame_stage_samples");
            assert_eq!(view.sequence.available_samples, 16384);
            assert_eq!(view.sequence.used_samples, 16384);
            assert_eq!(view.power.averages, 3);
            assert_eq!(
                view.sequence.physically_contiguous,
                stage == FullLoopStage::TxPrefix
            );
            assert_eq!(view.power.reference_plane, normalized.power.reference_plane);
            assert_eq!(
                view.power.millivolts_squared_per_unit,
                electrical.then_some(EXPECTED_CALIBRATION)
            );
            assert_eq!(view.power.channel_power.dbmv.is_some(), electrical);
            let full = result
                .calibrated_spectrum_sequence(FullLoopSpectrumQuery {
                    start: 0,
                    count: 0,
                    ..query
                })
                .unwrap();
            assert_eq!(
                serde_json::to_value(&view.power.channel_power).unwrap(),
                serde_json::to_value(&full.power.channel_power).unwrap()
            );
            for (actual, original) in view.power.buckets.iter().zip(&normalized.power.buckets) {
                assert_eq!(actual.first_bin, original.first_bin);
                assert_eq!(actual.last_bin, original.last_bin);
                let shift = match electrical {
                    true => 10.0 * EXPECTED_CALIBRATION.log10(),
                    false => 0.0,
                };
                for (actual, original) in [
                    (actual.minimum_db, original.minimum_db),
                    (actual.maximum_db, original.maximum_db),
                ] {
                    match (actual, original) {
                        (Some(actual), Some(original)) => {
                            assert!((actual - original - shift).abs() < DB_TOLERANCE)
                        }
                        (None, None) => {}
                        _ => panic!("Missing spectrum density changed during calibration"),
                    }
                }
            }
        }
        for query in [
            FullLoopSpectrumQuery::default(),
            FullLoopSpectrumQuery {
                stage: FullLoopStage::TxIfft,
                max_averages: 0,
                ..Default::default()
            },
            FullLoopSpectrumQuery {
                stage: FullLoopStage::TxIfft,
                max_averages: 33,
                ..Default::default()
            },
            FullLoopSpectrumQuery {
                stage: FullLoopStage::TxIfft,
                start: 8192,
                ..Default::default()
            },
            FullLoopSpectrumQuery {
                stage: FullLoopStage::TxIfft,
                snapshot: usize::MAX,
                ..Default::default()
            },
        ] {
            let old = result.spectrum_sequence(query.clone()).err().unwrap();
            let new = result.calibrated_spectrum_sequence(query).err().unwrap();
            assert_eq!(new.code(), old.code());
            assert_eq!(new.message(), old.message());
            assert_eq!(new.engine_code(), old.engine_code());
        }
        assert_eq!(serde_json::to_value(result.report()).unwrap(), before);
        assert_eq!(
            serde_json::to_value(result.observation_stats()).unwrap(),
            statistics
        );
        assert_eq!(result.report().summary.comparison.bit_errors, 0);
    }
}

#[test]
fn configured_physical_layout_preserves_source_catalog_identity_and_original_fft_grid() {
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let bytes =
        include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
    let profile = Dpd::from_bytes(
        &NamedCapture::new("source", bytes.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap();
    let preview = transmission.physical_layout(&profile, 1, 7, 5).unwrap();
    assert_eq!(preview.reference_plane, "configured_physical_layout");
    assert_eq!(preview.layout.fft_bins, 8192);
    assert_eq!(preview.layout.first_bin, 7);
    assert_eq!(preview.layout.carriers[0].physical_bin, 7);
    assert_eq!(preview.key, *profile.key());
    assert_eq!(preview.key.grid.sample_count, 7480);
    let mut encoded: Value = serde_json::from_slice(bytes).unwrap();
    encoded["profiles"][0]["profile_id"] = json!(200);
    let capture = ProfileCapture::from_bytes(
        &NamedCapture::new("catalog", serde_json::to_vec(&encoded).unwrap()).unwrap(),
    )
    .unwrap();
    let high_id = transmission
        .physical_layout_capture(&capture, 200, u64::MAX, 0, 0)
        .unwrap();
    assert_eq!(high_id.profile_id, 200);
    assert_eq!(high_id.layout.carriers.len(), 8192);
    assert_eq!(high_id.key, preview.key);
    assert!(ProfileId::new(200).is_err());
    for (start, count) in [(8192, 0), (8191, 2), (usize::MAX, 1), (0, 8193)] {
        assert!(transmission
            .physical_layout(&profile, 0, start, count)
            .is_err());
    }
    assert!(transmission
        .physical_layout_capture(&capture, 255, 0, 0, 0)
        .is_err());
}

#[test]
fn actual_fft_bins_and_transmit_cells_preserve_values_power_roles_offsets_and_missing_history() {
    let result = request()
        .with_observations(policy())
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    for stage in [FullLoopStage::PhysicalBins, FullLoopStage::RxFft] {
        let raw = result
            .stage(
                stage,
                0,
                FullLoopStageViewport {
                    start: 7,
                    count: 13,
                },
            )
            .unwrap();
        let bins = result.physical_bins(stage, 0, 7, 13).unwrap();
        assert_eq!(bins.element_start, 7);
        assert_eq!(bins.context, raw.context);
        assert_eq!(bins.total_elements, 8192);
        assert_eq!(bins.power_units, "normalized_bin_power");
        let originals = match raw.data {
            FullLoopStageData::OptionalIq(values) => values,
            FullLoopStageData::Iq(values) => values.into_iter().map(Some).collect(),
            _ => unreachable!(),
        };
        for (bin, original) in bins.bins.iter().zip(originals) {
            assert_eq!(bin.iq, original);
            assert_eq!(bin.linear_power, original.map(|[i, q]| i * i + q * q));
        }
        assert_eq!(bins.bins[0].physical_bin, 7);
        assert_eq!(bins.layout.carriers[0].physical_bin, 7);
    }
    let full = result
        .physical_bins(FullLoopStage::PhysicalBins, 0, 0, 0)
        .unwrap();
    assert_eq!(full.bins.len(), 8192);
    assert_eq!(full.bins[0].linear_power, Some(0.0));
    assert_eq!(full.bins[0].power_db, None);
    for bin in full.bins.iter().filter(|bin| bin.iq.is_none()) {
        assert_eq!(bin.linear_power, None);
        assert_eq!(bin.power_db, None);
    }
    for stage in [
        FullLoopStage::PayloadMapped,
        FullLoopStage::PayloadInterleaved,
    ] {
        let raw = result
            .stage(
                stage,
                0,
                FullLoopStageViewport {
                    start: 7,
                    count: 13,
                },
            )
            .unwrap();
        let cells = result.transmit_cells(stage, 0, 7, 13).unwrap();
        let FullLoopStageData::TransportCells(originals) = raw.data else {
            unreachable!()
        };
        assert_eq!(cells.context, raw.context);
        assert_eq!(cells.cells[0].cell_index, 7);
        for (cell, value) in cells.cells.iter().zip(originals) {
            assert_eq!(cell.value, value);
            if stage == FullLoopStage::PayloadMapped {
                assert_eq!(
                    cell.role,
                    Some(cells.context.transmit_cell_roles.as_ref().unwrap()[cell.cell_index])
                );
            }
        }
        for (start, count) in [(usize::MAX, 1), (0, 8193), (0, usize::MAX)] {
            assert!(result.transmit_cells(stage, 0, start, count).is_err());
        }
    }
    for (start, count) in [(usize::MAX, 1), (8191, 2), (0, 8193)] {
        assert!(result
            .physical_bins(FullLoopStage::PhysicalBins, 0, start, count)
            .is_err());
    }
    assert!(result
        .physical_bins(FullLoopStage::TxIfft, 0, 0, 0)
        .is_err());
    assert!(result
        .transmit_cells(FullLoopStage::PhysicalBins, 0, 0, 0)
        .is_err());
    assert!(result
        .physical_bins(FullLoopStage::PhysicalBins, usize::MAX, 0, 1)
        .is_err());
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
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
