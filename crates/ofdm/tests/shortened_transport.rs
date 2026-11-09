/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent length admission and actual live-PLC waveform reception of shortened words.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::{
        payload::shortened::run_shortened_waveform,
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::{
        packing::shortening::{discover_payload_length, validate_shortened_transport},
        SubcarrierSpacing,
    },
};
use fec_codec::shortening::Shortening;
#[test]
fn received_lengths_follow_odd_byte_rule_not_transmitter_metadata() {
    for payload_bits in [8, 24, 1032, 6136, 14216] {
        let geometry = Shortening::new(payload_bits).unwrap();
        validate_shortened_transport(geometry).unwrap();
        for extra in 0..14 {
            assert_eq!(
                discover_payload_length(geometry.transmitted_bits() + extra).unwrap(),
                geometry
            );
        }
    }
    for payload_bits in [1, 7, 16, 6137, 14232] {
        assert!(validate_shortened_transport(Shortening::new(payload_bits).unwrap()).is_err());
    }
    for bits in [0, 1968, 16214, usize::MAX] {
        assert!(discover_payload_length(bits).is_err());
    }
    assert_eq!(
        discover_payload_length(16213).unwrap().payload_bits(),
        14232
    );
}
#[test]
fn square_cross_and_mixed_shortened_words_recover_through_waveform_at_both_spacings() {
    let payloads: Vec<Vec<u8>> = [24, 1032, 6136]
        .iter()
        .map(|count| {
            (0..*count)
                .map(|index| u8::from((index * 17 + index / 11) % 19 < 9))
                .collect()
        })
        .collect();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for bits in [7, 14, 0] {
            let (mut settings, layout) = acquisition_support::acquisition_setup(spacing, false);
            settings.time_interleaver_depth = 4;
            let mut loading = vec![bits; spacing.fft_samples() as usize];
            if bits == 0 {
                for (index, value) in loading.iter_mut().enumerate() {
                    *value = match (index / 160) % 5 {
                        0 => 0,
                        1 => 4,
                        2 => 7,
                        3 => 8,
                        _ => 14,
                    };
                }
            }
            let result = run_shortened_waveform(
                &settings,
                &layout,
                &loading,
                &payloads,
                &mut TestPointProbe::disabled(),
                &mut NoopObserver,
            )
            .unwrap();
            assert_eq!(result.words.len(), 3);
            assert_eq!(result.pending_payload_bits, 0);
            assert_eq!(result.link.plc_bit_errors, 0);
            assert!(result.link.maximum_bin_error < 1e-11);
            for (word, original) in result.words.iter().zip(&payloads) {
                assert_eq!(word.payload_bits, original.len());
                assert_eq!(word.bit_errors, 0, "spacing={spacing:?},bits={bits}");
                assert!(word.ldpc_success && word.bch_success && word.known_zeros_consistent);
            }
        }
    }
}
