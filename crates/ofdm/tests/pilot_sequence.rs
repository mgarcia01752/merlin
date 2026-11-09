/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Figure84 register diagram and the two published pilot examples.
use docsis_ofdm_fec::{signal_processing::pilots::PilotSequence, transmission::SubcarrierSpacing};
#[test]
fn published_bits_and_independent_thirteen_stage_diagram_match() {
    let sequence = PilotSequence::new(SubcarrierSpacing::Khz25);
    assert_eq!(sequence.bits().len(), 8192);
    assert_eq!(sequence.bit_at(1000), Some(0));
    assert_eq!(sequence.bit_at(2999), Some(1));
    let mut stages = [1u8; 13];
    let mut seen = std::collections::HashSet::new();
    for (index, actual) in sequence.bits().iter().enumerate() {
        if index < 8191 {
            assert!(seen.insert(stages));
        } else {
            assert_eq!(stages, [1; 13]);
        }
        let output = stages[12];
        assert_eq!(*actual, output);
        let previous = stages;
        stages[0] = output;
        stages[1..].copy_from_slice(&previous[..12]);
        for stage in [8, 11, 12] {
            stages[stage] ^= output;
        }
    }
    assert_eq!(sequence.bits()[0], sequence.bits()[8191]);
    let short = PilotSequence::new(SubcarrierSpacing::Khz50);
    assert_eq!(short.bits(), &sequence.bits()[..4096]);
    assert_eq!(short.bit_at(4096), None);
    assert_eq!(sequence.bit_at(8192), None);
}
