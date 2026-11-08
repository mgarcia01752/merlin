/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent temporal/grid/role checks for the laboratory.
use docsis_ofdm_fec::{
    profile::{association::match_time, evaluation::*, model::*},
    types::modulation::CarrierType,
};
use std::sync::atomic::AtomicBool;
fn frame(epoch: u32, values: Vec<Option<f64>>) -> RxMerFrame {
    RxMerFrame {
        key: ScenarioKey {
            mac_address: [0, 202, 0, 0, 0, 1],
            channel_id: 1,
            grid: FrequencyGrid {
                zero_frequency_hz: 600_000_000,
                first_subcarrier_index: 100,
                spacing_hz: 25_000,
                sample_count: values.len(),
            },
        },
        source: SourceProvenance {
            name: format!("frame-{epoch}"),
            sha256: format!("{epoch:064x}"),
            byte_length: 32,
            capture_time: Some(epoch),
            file_kind: "RxMer".into(),
            format_major: 1,
            format_minor: 0,
        },
        capture_time: epoch,
        mer_db: values,
    }
}
#[test]
fn exact_grids_and_explicit_time_matches() {
    let first = frame(100, vec![Some(30.)]);
    let mut other = first.key.clone();
    other.channel_id = 2;
    assert!(
        match_time(&first.key, 100, &[(&other, 100), (&first.key, 103)], 0)
            .unwrap()
            .is_none()
    );
    let matched = match_time(&first.key, 100, &[(&other, 100), (&first.key, 103)], 3)
        .unwrap()
        .unwrap();
    assert_eq!(matched.source_index, 1);
    assert_eq!(matched.delta_seconds, 3);
    assert!(!matched.simultaneous);
    assert!(match_time(&first.key, 100, &[(&first.key, 97), (&first.key, 103)], 3).is_err());
    other.grid.spacing_hz = 0;
    assert!(other.grid.validate().is_err());
}
#[test]
fn individual_intervals_expose_impairments_and_missing_readings() {
    let good = frame(100, vec![Some(40.), Some(40.)]);
    let bad = frame(101, vec![Some(10.), None]);
    let settings = EvaluationSettings {
        assume_unknown_data_roles: true,
        ..Default::default()
    };
    let mut candidate = generate_candidate(
        &[&good],
        None,
        &settings,
        ProtocolScope::Docsis31,
        &AtomicBool::new(false),
    )
    .unwrap();
    candidate.edit_range(0, 2, Some(4)).unwrap();
    let report = evaluate(
        &candidate,
        &[&good, &bad],
        &settings,
        &AtomicBool::new(false),
    )
    .unwrap();
    assert_eq!(report.passing_frame_count, 1);
    assert_eq!(report.complete_frame_count, 1);
    assert_eq!(report.fraction_all_frames_meeting_target, 0.5);
    assert_eq!(report.frames[1].vulnerable_carriers, 1);
    assert_eq!(report.frames[1].missing_data_carriers, 1);
    assert_eq!(report.worst_interval_epoch, Some(101));
    assert_eq!(report.estimated_gross_rate_bps, Some(200_000));
    assert!((equivalent_noise_mer(&[10., 20.]).unwrap() - 12.59637310505756).abs() < 1e-10);
}
#[test]
fn edits_preserve_special_and_unknown_carriers() {
    let source = frame(100, vec![Some(40.); 4]);
    let roles = [
        CarrierType::Data,
        CarrierType::Plc,
        CarrierType::Exclusion,
        CarrierType::ContinuousPilot,
    ];
    let template = CapturedProfile {
        key: source.key.clone(),
        source: source.source.clone(),
        capture_time: 100,
        profile_id: 3,
        assignments: roles
            .into_iter()
            .map(|carrier_type| CarrierAssignment {
                carrier_type,
                bit_loading: match carrier_type {
                    CarrierType::Exclusion => None,
                    CarrierType::ContinuousPilot => Some(1),
                    _ => Some(4),
                },
                origin: AssignmentOrigin::Captured,
            })
            .collect(),
    };
    let settings = EvaluationSettings::default();
    let mut candidate = generate_candidate(
        &[&source],
        Some(&template),
        &settings,
        ProtocolScope::Docsis31,
        &AtomicBool::new(false),
    )
    .unwrap();
    assert_eq!(candidate.edit_range(0, 4, Some(8)).unwrap(), 1);
    assert_eq!(&candidate.assignments[1..], &template.assignments[1..]);
    assert!(candidate.edit_range(3, 2, Some(8)).is_err());
    assert!(generate_candidate(
        &[&source],
        None,
        &settings,
        ProtocolScope::Docsis31,
        &AtomicBool::new(true)
    )
    .is_err());
}
#[cfg(feature = "simulation")]
mod coding {
    use super::*;
    use docsis_ofdm_fec::profile::simulation::*;
    #[test]
    fn normative_qam_labels_energy_and_soft_bits() {
        // PHY I20 Annex A Figure102, page198. Hex rows from top left to right.
        let labels = [[8, 10, 2, 0], [9, 11, 3, 1], [13, 15, 7, 5], [12, 14, 6, 4]];
        let mut energy = 0.;
        for (row, labels) in labels.iter().enumerate() {
            for (column, label) in labels.iter().enumerate() {
                let point = map_qam16(*label).unwrap();
                assert!((point[0] - (-3. + 2. * column as f64) / 10f64.sqrt()).abs() < 1e-10);
                assert!((point[1] - (3. - 2. * row as f64) / 10f64.sqrt()).abs() < 1e-10);
                energy += point[0] * point[0] + point[1] * point[1];
                for (bit, llr) in qam16_llrs(point, 0.1).unwrap().iter().enumerate() {
                    assert_eq!(*llr < 0., (label >> (3 - bit)) & 1 == 1);
                }
            }
        }
        assert!((energy / 16. - 1.).abs() < 1e-10);
        let point = map_qam16(0).unwrap();
        let first = qam16_llrs(point, 1.).unwrap();
        let second = qam16_llrs(point, 2.).unwrap();
        for bit in 0..4 {
            assert!((first[bit] - 2. * second[bit]).abs() < 1e-10);
        }
        let order = transmit_bit_order();
        let mut sorted = order.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..16200).collect::<Vec<_>>());
        assert_ne!(order, sorted);
    }
    #[test]
    fn deterministic_noiseless_noise_failure_and_cancellation() {
        let cancel = AtomicBool::new(false);
        let settings = SimulationSettings {
            codewords: 2,
            noiseless: true,
            max_runtime_ms: 30_000,
            ..Default::default()
        };
        let result = simulate(&settings, None, &cancel, |_| {}).unwrap();
        assert_eq!(result.completed_codewords, 2);
        assert_eq!(result.post_bch_payload_bit_errors, 0);
        assert_eq!(result.pre_ldpc_bit_errors, 0);
        assert_eq!(result.ldpc_detected_failures, 0);
        assert!(
            (result.zero_error_one_sided_upper_95.unwrap() - (1. - 0.05f64.sqrt())).abs() < 1e-12
        );
        let noisy = SimulationSettings {
            codewords: 1,
            noiseless: false,
            awgn_mer_db: 0.,
            max_iterations: 2,
            ..settings.clone()
        };
        let first = simulate(&noisy, None, &cancel, |_| {}).unwrap();
        let second = simulate(&noisy, None, &cancel, |_| {}).unwrap();
        assert_eq!(first.pre_ldpc_bit_errors, second.pre_ldpc_bit_errors);
        assert!(first.pre_ldpc_bit_errors > 1000);
        assert!(first.post_bch_payload_bit_errors > 0);
        assert_eq!(first.ldpc_detected_failures, 1);
        let partial = simulate(&settings, None, &cancel, |_| {
            cancel.store(true, std::sync::atomic::Ordering::Relaxed)
        })
        .unwrap();
        assert_eq!(partial.completed_codewords, 1);
        assert_eq!(partial.status, "cancelled");
    }
}

#[cfg(feature = "simulation")]
#[test]
fn uncoded_awgn_agrees_with_gray_16qam_reference() {
    use docsis_ofdm_fec::profile::simulation::{simulate, SimulationSettings};
    // Unit Es, Es/N0=10dB. Exact Gray16-QAM bit error probability is
    // 3/4 Q(sqrt(Es/(5N0))) + 1/2 Q(3sqrt(Es/(5N0))) - 1/4 Q(5sqrt(Es/(5N0))).
    // Independently evaluated: 0.0589927. Tolerance0.004 comfortably covers
    // finite-sample fluctuations in129600 transmitted bits; not a coded BER target.
    let settings = SimulationSettings {
        awgn_mer_db: 10.,
        codewords: 8,
        max_iterations: 1,
        max_runtime_ms: 60_000,
        ..Default::default()
    };
    let result = simulate(&settings, None, &AtomicBool::new(false), |_| {}).unwrap();
    assert_eq!(result.completed_codewords, 8);
    assert!((result.pre_ldpc_ber.unwrap() - 0.0589927).abs() < 0.004);
}
