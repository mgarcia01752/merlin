/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent roles/signs/energy across physical pilot phases, without frame insertion.
mod support;
use docsis_ofdm_fec::{
    signal_processing::pilots::{
        generate_continuous, generate_scattered, PilotPower, PilotSequence,
    },
    transmission::{layout::PhysicalCarrierRole, SubcarrierSpacing},
};
#[test]
fn continuous_and_scattered_values_follow_physical_bins_over_full_cycle() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let layout = support::layout(spacing);
        let sequence = PilotSequence::new(spacing);
        let power = PilotPower::new(0.5).unwrap();
        let continuous = generate_continuous(&layout, &sequence, power).unwrap();
        assert_eq!(continuous.len(), 16); // Eight predefined plus eight fixture pilots.
        let initial = layout.symbol_roles(0);
        for phase in 0..128 {
            let roles = layout.symbol_roles(phase);
            let scattered = generate_scattered(&layout, &sequence, power, phase).unwrap();
            assert_eq!(
                scattered.len(),
                roles
                    .iter()
                    .filter(|role| **role == PhysicalCarrierRole::ScatteredPilot)
                    .count()
            );
            for (values, role) in [
                (&continuous, PhysicalCarrierRole::ContinuousPilot),
                (&scattered, PhysicalCarrierRole::ScatteredPilot),
            ] {
                assert!(values.windows(2).all(|pair| pair[0].bin < pair[1].bin));
                for value in values {
                    assert_eq!(roles[usize::from(value.bin)], role);
                    assert_eq!(value.sequence_bit, sequence.bit_at(value.bin).unwrap());
                    let sign = if value.sequence_bit == 0 { 1.0 } else { -1.0 };
                    assert_eq!(value.iq, [sign, 0.0]); // RMS0.5 => boosted amplitude1.
                    assert_eq!(value.iq[0] * value.iq[0] + value.iq[1] * value.iq[1], 1.0);
                }
            }
            assert!(scattered
                .iter()
                .all(|value| !continuous.iter().any(|fixed| fixed.bin == value.bin)));
        }
        assert_eq!(layout.symbol_roles(0), initial);
        assert_eq!(
            generate_scattered(&layout, &sequence, power, 0).unwrap(),
            generate_scattered(&layout, &sequence, power, 128).unwrap()
        );
        assert_eq!(
            generate_scattered(&layout, &sequence, power, u64::MAX).unwrap(),
            generate_scattered(&layout, &sequence, power, 127).unwrap()
        );
    }
}
#[test]
fn different_fft_numerologies_cannot_be_mixed() {
    let layout = support::layout(SubcarrierSpacing::Khz25);
    let sequence = PilotSequence::new(SubcarrierSpacing::Khz50);
    let power = PilotPower::new(1.0).unwrap();
    assert!(generate_continuous(&layout, &sequence, power).is_err());
    assert!(generate_scattered(&layout, &sequence, power, 0).is_err());
}
