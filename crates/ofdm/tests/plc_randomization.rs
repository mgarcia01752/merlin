/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.8, PDF157/Figure68: independent polynomial oracle and cadence.
use docsis_ofdm_fec::{
    signal_processing::modulation::randomization::{CellRandomizer, RandomizerState},
    transmission::{
        plc::{InterleavedPlcBlock, PlcFrameGeometry, PlcRandomizer},
        SubcarrierSpacing,
    },
};

// Independent carryless polynomial remainder: materialize alpha^11*D1, then
// divide by alpha^12+alpha^6+alpha^4+alpha+1. Production clocks use shift feedback.
fn oracle_clock(state: RandomizerState) -> RandomizerState {
    let mut polynomial = u32::from(state.register_d1) << 11;
    for degree in (12..=22).rev() {
        if polynomial & (1 << degree) != 0 {
            polynomial ^= 0x1053 << (degree - 12);
        }
    }
    RandomizerState {
        register_d0: polynomial as u16,
        register_d1: state.register_d0 ^ state.register_d1,
    }
}
fn initial() -> RandomizerState {
    RandomizerState {
        register_d0: 0x4a7,
        register_d1: 0xb4c,
    }
}

#[test]
fn both_grids_match_polynomial_oracle_across_three_frames() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let geometry = PlcFrameGeometry::new(spacing);
        let labels: Vec<_> = (0..96).map(|index| (index % 16) as u8).collect();
        let block = InterleavedPlcBlock::from_values(spacing, &labels).unwrap();
        let mut stream = PlcRandomizer::new(spacing);
        let mut inverse = PlcRandomizer::new(spacing);
        for frame in 0..3 {
            let mut expected = initial();
            for word in 0..geometry.codewords_per_frame {
                let index = (frame * 128 + word * geometry.data_symbols_per_codeword) as u64;
                assert_eq!(stream.next_symbol_index(), index);
                let output = stream.apply(&block).unwrap();
                assert_eq!(output.first_symbol_index, index);
                assert_eq!(output.state_before, expected);
                for (input, actual) in labels.iter().zip(output.values()) {
                    // reverse_bits on the four coefficients independently establishes
                    // that D0 alpha^0 is the label's highest-order bit.
                    let mask = ((expected.register_d0 & 15).reverse_bits() >> 12) as u8;
                    assert_eq!(*actual, input ^ mask);
                    expected = oracle_clock(expected);
                }
                assert_eq!(output.state_after, expected);
                assert_eq!(stream.state(), expected);
                assert_eq!(block.values(), labels);
                let received = InterleavedPlcBlock::from_values(spacing, output.values()).unwrap();
                assert_eq!(inverse.apply(&received).unwrap().values(), labels);
            }
            assert_eq!(stream.next_symbol_index(), ((frame + 1) * 128) as u64);
            // Last data-cell state survives the eight untouched preamble slots;
            // the next operation's state_before must be the reset seeds instead.
            assert_eq!(stream.state(), expected);
        }
    }
}

#[test]
fn published_first_mask_is_label_e_and_soft_inverse_preserves_magnitudes() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let block = InterleavedPlcBlock::from_values(spacing, &[0; 96]).unwrap();
        let mut tx = PlcRandomizer::new(spacing);
        let mut rx = PlcRandomizer::new(spacing);
        let geometry = PlcFrameGeometry::new(spacing);
        for word in 0..=geometry.codewords_per_frame {
            let encoded = tx.apply(&block).unwrap();
            if word == 0 || word == geometry.codewords_per_frame {
                assert_eq!(encoded.values()[0], 0xe);
                assert_eq!(encoded.state_before, initial());
            }
            let llrs: Vec<_> = encoded
                .values()
                .iter()
                .flat_map(|label| {
                    (0..4).map(move |bit| {
                        let magnitude = (bit + 1) as f64 / 8.0;
                        if label & (8 >> bit) == 0 {
                            magnitude
                        } else {
                            -magnitude
                        }
                    })
                })
                .collect();
            let saved = llrs.clone();
            let decoded = rx.descramble_llrs(&llrs).unwrap();
            for (index, actual) in decoded.llrs().iter().enumerate() {
                assert_eq!(*actual, (index % 4 + 1) as f64 / 8.0);
            }
            assert_eq!(decoded.state_before, encoded.state_before);
            assert_eq!(decoded.state_after, encoded.state_after);
            assert_eq!(decoded.first_symbol_index, encoded.first_symbol_index);
            assert_eq!(llrs, saved);
        }
    }
}

#[test]
fn rejected_inputs_preserve_state_before_and_after_frame_boundary() {
    let block = InterleavedPlcBlock::from_values(SubcarrierSpacing::Khz25, &[0; 96]).unwrap();
    let wrong = InterleavedPlcBlock::from_values(SubcarrierSpacing::Khz50, &[0; 96]).unwrap();
    let mut stream = PlcRandomizer::new(SubcarrierSpacing::Khz25);
    for completed in 0..=20 {
        let before = stream.state();
        let next = stream.next_symbol_index();
        assert!(stream.apply(&wrong).is_err());
        for invalid in [
            vec![],
            vec![0.0; 383],
            vec![0.0; 385],
            vec![f64::NAN; 384],
            vec![f64::INFINITY; 384],
        ] {
            assert!(stream.descramble_llrs(&invalid).is_err());
            assert_eq!(stream.state(), before);
            assert_eq!(stream.next_symbol_index(), next);
        }
        if completed < 20 {
            stream.apply(&block).unwrap();
        }
    }
    assert_eq!(stream.next_symbol_index(), 128);
    assert_eq!(stream.apply(&block).unwrap().state_before, initial());
}

#[test]
fn plc_does_not_read_or_advance_data_ncp_state() {
    let mut data = CellRandomizer::new();
    data.begin_symbol(0).unwrap();
    let before = data.state();
    let mut plc = PlcRandomizer::new(SubcarrierSpacing::Khz25);
    let block = InterleavedPlcBlock::from_values(SubcarrierSpacing::Khz25, &[0; 96]).unwrap();
    let expected = plc.clone().apply(&block).unwrap();
    // Advance the data/NCP stream between PLC calls; it must not affect PLC.
    data.zero_loaded_bpsk().unwrap();
    assert_ne!(data.state(), before);
    let data_after = data.state();
    assert_eq!(plc.apply(&block).unwrap(), expected);
    assert_eq!(data.state(), data_after);
    assert_eq!(
        PlcRandomizer::new(SubcarrierSpacing::Khz25).state(),
        initial()
    );
}
