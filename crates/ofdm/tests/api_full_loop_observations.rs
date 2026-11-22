/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual passive stage ownership, original coordinate anchors and finite copy/query budgets.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::api::*;
use serde_json::{json, Value};
fn request() -> FullLoopRequest {
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
            duration_ms: 1,
            max_iterations: 10,
            max_runtime_ms: 60000,
            ..Default::default()
        },
    )
    .unwrap()
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
