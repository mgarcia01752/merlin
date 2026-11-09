/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent index/delay expectations from PHY I20 Figures55–58, PDF129–132.
mod support;
use docsis_ofdm_fec::{
    signal_processing::interleaving::{
        FrequencyInterleaver, InterleaverDirection, InterleaverSettings, SymbolInterleaver,
        TimeInterleaver,
    },
    transmission::{
        layout::{ncp::PreInterleavingRole, PhysicalCarrierRole, PilotReference},
        SubcarrierSpacing,
    },
};

#[test]
fn frequency_indices_verify_rotation_direction_seed_and_column_bit_reversal() {
    let make = |cells| FrequencyInterleaver::new(SubcarrierSpacing::Khz25, cells).unwrap();
    // Single unrotated column: seven-bit reversal of row labels.
    assert_eq!(
        &make(128).output_to_input()[..8],
        &[0, 64, 32, 96, 16, 80, 48, 112]
    );
    // First column rotates +17. Input row111 moves to output row0; each written
    // row has two inputs, so2*reverse7(111)=246. The last full column never rotates.
    let two = make(256);
    assert_eq!(
        &two.output_to_input()[..8],
        &[246, 14, 142, 78, 206, 46, 174, 110]
    );
    assert_eq!(&two.output_to_input()[128..132], &[1, 129, 65, 193]);
    let partial = make(129);
    assert_eq!(&partial.output_to_input()[..4], &[124, 8, 72, 40]);
    assert_eq!(partial.output_to_input()[128], 1);
    // K3 reads0,2,1; second rotated column uses next register56.
    let three = make(384);
    assert_eq!(
        (
            three.output_to_input()[0],
            three.output_to_input()[128],
            three.output_to_input()[256]
        ),
        (369, 2, 28)
    );
    // K4 reads0,2,1,3 with seed sequence17,56,28; last column unrotated.
    let four = make(512);
    assert_eq!(
        (
            four.output_to_input()[128],
            four.output_to_input()[256],
            four.output_to_input()[384]
        ),
        (78, 37, 3)
    );
    // K5 fallback preserves column3; the one-element final column4 reads second.
    assert_eq!(make(513).output_to_input()[128], 4);
}
#[test]
fn frequency_maps_are_bijective_at_partial_full_and_spacing_limits() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for cells in [
            1,
            2,
            127,
            128,
            129,
            255,
            256,
            257,
            384,
            512,
            513,
            769,
            spacing.max_interleaver_cells(),
        ] {
            let frequency = FrequencyInterleaver::new(spacing, cells).unwrap();
            let input: Vec<_> = (0..cells).collect();
            let output = frequency.interleave(&input).unwrap();
            let mut sorted = output.clone();
            sorted.sort_unstable();
            assert_eq!(sorted, input);
            assert_eq!(frequency.deinterleave(&output).unwrap(), input);
            assert_eq!(frequency.interleave(&input).unwrap(), output); // Per-symbol reset.
            assert!(frequency.interleave(&input[..cells - 1]).is_err());
        }
        assert!(FrequencyInterleaver::new(spacing, 0).is_err());
        assert!(FrequencyInterleaver::new(spacing, spacing.max_interleaver_cells() + 1).is_err());
    }
}
#[test]
fn every_time_depth_preserves_exact_tags_and_dummy_padding() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for depth in 1..=usize::from(spacing.max_time_interleaver_depth()) {
            for cells in [1, 5, 32, 33] {
                let settings = InterleaverSettings::new(spacing, cells, depth).unwrap();
                let mut transmit = TimeInterleaver::new(settings, InterleaverDirection::Transmit);
                let mut receive = TimeInterleaver::new(settings, InterleaverDirection::Receive);
                for symbol in 0..(2 * depth + 4) {
                    let input: Vec<_> = (0..cells).map(|rank| Some((symbol, rank))).collect();
                    let output = transmit.process_symbol(symbol as u64, &input).unwrap();
                    for (rank, cell) in output.iter().enumerate() {
                        assert_eq!(
                            *cell,
                            symbol
                                .checked_sub(rank % depth)
                                .map(|source| (source, rank))
                        );
                    }
                    let restored = receive.process_symbol(symbol as u64, &output).unwrap();
                    for (rank, cell) in restored.iter().enumerate() {
                        assert_eq!(
                            *cell,
                            symbol.checked_sub(depth - 1).map(|source| (source, rank))
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn composed_streams_drain_and_rejected_calls_leave_history_unchanged() {
    let settings = InterleaverSettings::new(SubcarrierSpacing::Khz25, 129, 3).unwrap();
    let mut transmitter = SymbolInterleaver::new(settings, InterleaverDirection::Transmit).unwrap();
    let mut receiver = SymbolInterleaver::new(settings, InterleaverDirection::Receive).unwrap();
    let inputs: Vec<_> = (0..4)
        .map(|symbol| {
            (0..129)
                .map(|rank| Some(symbol * 1000 + rank))
                .collect::<Vec<_>>()
        })
        .collect();
    for symbol in 0..6 {
        let input = inputs.get(symbol).cloned().unwrap_or(vec![None; 129]);
        assert!(transmitter.process_symbol(u64::MAX, &input).is_err());
        assert!(transmitter
            .process_symbol(symbol as u64 + 1, &input)
            .is_err());
        assert!(transmitter
            .process_symbol(symbol as u64, &input[..128])
            .is_err());
        let output = transmitter.process_symbol(symbol as u64, &input).unwrap();
        assert!(receiver
            .process_symbol(symbol as u64, &output[..128])
            .is_err());
        let restored = receiver.process_symbol(symbol as u64, &output).unwrap();
        assert_eq!(
            restored,
            if symbol < 2 {
                vec![None; 129]
            } else {
                inputs[symbol - 2].clone()
            }
        );
    }
    for (spacing, depth) in [
        (SubcarrierSpacing::Khz25, 17),
        (SubcarrierSpacing::Khz50, 33),
        (SubcarrierSpacing::Khz25, 0),
    ] {
        assert!(InterleaverSettings::new(spacing, 129, depth).is_err());
    }
}
#[test]
fn inverse_reference_lands_on_physical_pilots_without_touching_protected_bins() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let layout = support::layout(spacing);
        for depth in [1, 3, usize::from(spacing.max_time_interleaver_depth())] {
            let reference = PilotReference::new(&layout, depth).unwrap();
            let mut stream =
                SymbolInterleaver::new(reference.settings(), InterleaverDirection::Transmit)
                    .unwrap();
            for symbol in 0..(128 + depth) {
                let roles = reference.roles(symbol as u64);
                assert_eq!(roles, reference.roles(symbol as u64 + 128));
                let input: Vec<_> = roles
                    .iter()
                    .map(|role| Some(*role == PreInterleavingRole::ScatteredPilotPlaceholder))
                    .collect();
                let output = stream.process_symbol(symbol as u64, &input).unwrap();
                let physical = layout.symbol_roles(symbol as u64);
                for (&bin, cell) in reference.physical_bins().iter().zip(output) {
                    assert!(matches!(
                        physical[usize::from(bin)],
                        PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot
                    ));
                    if let Some(pilot) = cell {
                        assert_eq!(
                            pilot,
                            physical[usize::from(bin)] == PhysicalCarrierRole::ScatteredPilot
                        );
                    } else {
                        assert!(symbol < depth - 1);
                    }
                }
            }
            let fixed = layout.symbol_roles(0);
            for (bin, role) in fixed.iter().enumerate() {
                if matches!(
                    role,
                    PhysicalCarrierRole::Plc
                        | PhysicalCarrierRole::ContinuousPilot
                        | PhysicalCarrierRole::Excluded
                ) {
                    assert!(!reference.physical_bins().contains(&(bin as u16)));
                }
            }
            assert_eq!(reference.roles(u64::MAX), reference.roles(127));
        }
        assert!(PilotReference::new(&layout, 0).is_err());
    }
}
