/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Normative data orders, optional capabilities and implemented-mode boundaries.
use docsis_ofdm_fec::{
    math::bit_loading::{is_data_bit_loading, BitLoadingModel},
    profile::{
        document::ScenarioDocument,
        evaluation::{evaluate, generate_candidate, EvaluationSettings},
        model::ProtocolScope,
    },
    types::ofdm_modulation::{
        DeviceSupportRequirement, ImplementationState, OfdmModulation, OFDM_DATA_MODULATIONS,
    },
};
use std::sync::atomic::AtomicBool;
#[test]
fn data_registry_covers_optional_16k_without_claiming_simulation() {
    // PHY I20 section7.5.5.1, PDF120; I07 section7.3.5.5, PDF134.
    let expected = [16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];
    assert_eq!(OFDM_DATA_MODULATIONS.map(|entry| entry.order), expected);
    for entry in OFDM_DATA_MODULATIONS {
        assert_eq!(entry.order, 1u16 << entry.bits_per_symbol);
        assert!(is_data_bit_loading(entry.bits_per_symbol));
        assert_eq!(
            entry.device_support,
            if entry.bits_per_symbol >= 13 {
                DeviceSupportRequirement::Optional
            } else {
                DeviceSupportRequirement::Required
            }
        );
        assert_eq!(
            entry.coding_simulation,
            if entry.bits_per_symbol == 4 {
                ImplementationState::ImplementedExperimental
            } else {
                ImplementationState::Pending
            }
        );
    }
    for bits in [0, 1, 2, 3, 5, 15, 255] {
        assert!(!is_data_bit_loading(bits));
        assert!(OfdmModulation::from_bits(bits).is_none());
    }
    assert_eq!(
        BitLoadingModel::OssiCandidateDefaults.target_db(14),
        Some(52.0)
    );
}
#[test]
fn both_protocol_scopes_allow_16k_analysis_with_an_explicit_device_cap() {
    let mut scenario: ScenarioDocument =
        serde_json::from_str(include_str!("../../../examples/scenario.json")).unwrap();
    for frame in &mut scenario.frames {
        frame.mer_db[0] = Some(55.0);
    }
    let settings = EvaluationSettings::default();
    assert_eq!(settings.max_bit_loading, 14);
    let frames: Vec<_> = scenario.frames.iter().collect();
    for protocol in [ProtocolScope::Docsis31, ProtocolScope::Docsis40Fdd] {
        let candidate = generate_candidate(
            &frames,
            Some(&scenario.profiles[0]),
            &settings,
            protocol,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(candidate.assignments[0].bit_loading, Some(14));
        let report = evaluate(&candidate, &frames, &settings, &AtomicBool::new(false)).unwrap();
        assert_eq!(report.estimated_gross_rate_bps, Some(350_000));
        let capped = EvaluationSettings {
            max_bit_loading: 12,
            ..settings.clone()
        };
        let candidate = generate_candidate(
            &frames,
            Some(&scenario.profiles[0]),
            &capped,
            protocol,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(candidate.assignments[0].bit_loading, Some(12));
    }
}
