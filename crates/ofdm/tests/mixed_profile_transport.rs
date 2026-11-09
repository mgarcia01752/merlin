/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::{
        modulation::interleaving::mixed::PayloadBitOrder,
        receiver::channel_estimation::ReceiverMode,
    },
    simulation::{
        channel::{
            constellation::ConstellationSelection, ChannelPhase, ChannelRun, ChannelRunSettings,
        },
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::SubcarrierSpacing,
};
#[test]
fn mixed_twist_matches_independent_transcribed_matrix_and_bijection() {
    let order = PayloadBitOrder::new(&[4, 8]).unwrap();
    let shifts = [
        0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 7, 9, 9, 9, 10, 10, 10, 10, 10, 11,
    ];
    let mut matrix = vec![vec![0; 675]; 24];
    for column in 0..24 {
        for source_row in 0..675 {
            let position = column * 675 + source_row;
            let original = if position < 14400 {
                position
            } else {
                let parity = position - 14400;
                14400 + 5 * (parity % 360) + parity / 360
            };
            matrix[column][(source_row + shifts[column]) % 675] = original;
        }
    }
    let expected: Vec<_> = (0..675)
        .flat_map(|row| matrix.iter().map(move |column| column[row]))
        .collect();
    assert_eq!(order.permutation(), expected);
    let mut unique = expected;
    unique.sort_unstable();
    assert_eq!(unique, (0..16200).collect::<Vec<_>>());
    assert!(PayloadBitOrder::new(&[0, 0]).is_err());
    assert!(PayloadBitOrder::new(&[5, 8]).is_err());
    assert!(PayloadBitOrder::new(&[6]).is_err()); // never substitute mixed rules for uniform64
}
#[test]
fn waveform_supports_uniform16_and_mixed_with_zero_loading_and_16k() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for (mixed, mode) in [
            (false, ReceiverMode::SuppliedChannel),
            (true, ReceiverMode::SuppliedChannel),
            (true, ReceiverMode::PilotEstimated),
        ] {
            let (transmission, raw) = acquisition_support::acquisition_setup(spacing, false);
            let mut loading = vec![4; spacing.fft_samples() as usize];
            if mixed {
                loading[usize::from(raw.first_active_bin)..usize::from(raw.first_active_bin) + 32]
                    .fill(8);
                loading[usize::from(raw.active_end_exclusive) - 8
                    ..usize::from(raw.active_end_exclusive)]
                    .fill(14);
                loading[usize::from(raw.first_active_bin) + 48
                    ..usize::from(raw.first_active_bin) + 56]
                    .fill(0);
            }
            let mut settings = ChannelRunSettings::new(transmission, raw);
            settings.duration = std::time::Duration::from_micros(100);
            settings.data_bit_loading = Some(loading);
            settings.receiver_mode = mode;
            settings.constellation = Some(ConstellationSelection {
                requested_pairs: 128,
                ..Default::default()
            });
            let mut run = ChannelRun::new(&settings).unwrap();
            while run
                .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
                .unwrap()
                .is_some()
            {}
            assert_eq!(run.summary().phase, ChannelPhase::Finished);
            assert_eq!(
                run.summary().comparison.bit_errors,
                0,
                "mixed={mixed} spacing={spacing:?} mode={mode:?}"
            );
            assert!(run.summary().comparison.bits_tested > 0);
            for pair in &run.constellation().unwrap().pairs {
                let bits =
                    settings.data_bit_loading.as_ref().unwrap()[usize::from(pair.physical_bin)];
                assert_eq!(
                    pair.modulation_order,
                    1u16 << bits,
                    "Physical profile permutation disagrees"
                );
            }

            assert!(run
                .constellation()
                .unwrap()
                .pairs
                .iter()
                .all(|pair| matches!(pair.modulation_order, 16 | 256 | 16384)));
        }
    }
}
