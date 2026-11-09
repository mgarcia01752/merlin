/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Production state owners replace the prior test-local orchestration.
#![cfg(feature = "simulation")]
mod acquisition_support;
mod payload_support;
use docsis_ofdm_fec::{
    simulation::{
        payload::{PayloadReceiver, PayloadTransmitter},
        source::SeededSource,
    },
    transmission::{
        packing::NcpModulation, plc::receiver_configuration::ReceiverConfigurationState,
        SubcarrierSpacing,
    },
};
use fec_codec::concatenated::compare_payload;
#[test]
fn both_spacings_and_all_ncp_orders_recover_real_words_across_reset_and_drain() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for ncp in [
            NcpModulation::Qpsk,
            NcpModulation::Qam16,
            NcpModulation::Qam64,
        ] {
            let (mut settings, raw) = acquisition_support::acquisition_setup(spacing, false);
            settings.time_interleaver_depth = spacing.max_time_interleaver_depth();
            let state = payload_support::ready(&settings, &raw, ncp);
            let mut tx = PayloadTransmitter::new(&settings, &raw, ncp, 256).unwrap();
            let mut rx = PayloadReceiver::new(&state, 256).unwrap();
            let mut source = SeededSource::new(123);
            let payloads: Vec<_> = (0..4).map(|_| source.next_payload()).collect();
            let scheduled = [0, 61, 126, 140];
            let mut recovered = 0;
            let mut warmup = 0;
            let mut filler = 0;
            let mut spans_reset = false;
            for local in 0..160 {
                if let Some(index) = scheduled.iter().position(|time| *time == local) {
                    tx.enqueue(&payloads[index]).unwrap();
                }
                let symbol = tx.transmit().unwrap();
                filler += symbol.channel.unavailable_data_ranks.len();
                let result = rx
                    .receive(
                        &state,
                        symbol.channel.channel_symbol_index,
                        &symbol.channel.reduced,
                        0.01,
                        50,
                    )
                    .unwrap();
                if let Some(decoded) = result.decoded {
                    assert!(decoded.error.is_none(), "{:?}", decoded.error);
                    for word in decoded.recovered {
                        assert_eq!(word.receive_sequence, recovered as u64);
                        assert!(word.decoded.ldpc.success && word.decoded.bch.success);
                        assert_eq!(
                            compare_payload(&payloads[recovered], &word.decoded)
                                .unwrap()
                                .bit_errors,
                            0
                        );
                        spans_reset |=
                            word.first_source_symbol < 384 && word.last_source_symbol >= 384;
                        recovered += 1;
                    }
                } else {
                    warmup += 1;
                }
            }
            assert_eq!(tx.queued_codewords(), 0);
            let mut drain = 0;
            while let Some(symbol) = tx.drain().unwrap() {
                drain += 1;
                let result = rx
                    .receive(
                        &state,
                        symbol.channel_symbol_index,
                        &symbol.reduced,
                        0.01,
                        50,
                    )
                    .unwrap();
                let decoded = result.decoded.unwrap();
                assert!(decoded.error.is_none(), "{:?}", decoded.error);
                for word in decoded.recovered {
                    assert_eq!(word.receive_sequence, recovered as u64);
                    assert!(word.decoded.ldpc.success && word.decoded.bch.success);
                    assert_eq!(
                        compare_payload(&payloads[recovered], &word.decoded)
                            .unwrap()
                            .bit_errors,
                        0
                    );
                    spans_reset |= word.first_source_symbol < 384 && word.last_source_symbol >= 384;
                    recovered += 1;
                }
            }
            assert_eq!(warmup, usize::from(settings.time_interleaver_depth) - 1);
            assert_eq!(drain, warmup);
            assert!(filler > 0);
            assert_eq!(recovered, 4);
            assert!(spans_reset);
            assert_eq!(rx.pending_bits(), 0);
            assert!(tx.enqueue(&payloads[0]).is_err());
            assert!(tx.transmit().is_err());
        }
    }
}
#[test]
fn readiness_sequence_and_shape_rejections_do_not_consume_the_next_channel_symbol() {
    let spacing = SubcarrierSpacing::Khz50;
    let (settings, raw) = acquisition_support::acquisition_setup(spacing, false);
    let mut state = payload_support::ready(&settings, &raw, NcpModulation::Qpsk);
    assert!(PayloadReceiver::new(&ReceiverConfigurationState::new(spacing), 0).is_err());
    assert!(PayloadReceiver::new(&state, 1).is_err());
    let mut tx = PayloadTransmitter::new(&settings, &raw, NcpModulation::Qpsk, 128).unwrap();
    let mut rx = PayloadReceiver::new(&state, 128).unwrap();
    let symbol = tx.transmit().unwrap();
    let changed = payload_support::ready(&settings, &raw, NcpModulation::Qam16);
    assert!(rx
        .receive(&changed, 128, &symbol.channel.reduced, 0.01, 50)
        .is_err());
    assert!(rx
        .receive(&state, 129, &symbol.channel.reduced, 0.01, 50)
        .is_err());
    assert!(rx
        .receive(&state, 128, &symbol.channel.reduced[1..], 0.01, 50)
        .is_err());
    assert!(rx
        .receive(&state, 128, &symbol.channel.reduced, 0.0, 50)
        .is_err());
    assert!(state.accept_bytes(&[]).is_err());
    assert!(rx
        .receive(&state, 128, &symbol.channel.reduced, 0.01, 50)
        .is_err());
    state = payload_support::ready(&settings, &raw, NcpModulation::Qpsk);
    assert!(rx
        .receive(&state, 128, &symbol.channel.reduced, 0.01, 50)
        .is_ok());
}
#[test]
fn queue_and_drain_bounds_preserve_pending_source_words() {
    let (settings, raw) = acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    let mut tx = PayloadTransmitter::new(&settings, &raw, NcpModulation::Qpsk, 0).unwrap();
    let mut untouched = PayloadTransmitter::new(&settings, &raw, NcpModulation::Qpsk, 0).unwrap();
    assert!(untouched.drain().unwrap().is_none());
    let payload = vec![0; 14232];
    assert!(tx.enqueue(&payload[1..]).is_err());
    assert_eq!(tx.queued_codewords(), 0);
    for _ in 0..16 {
        tx.enqueue(&payload).unwrap();
    }
    assert!(tx.enqueue(&payload).is_err());
    assert_eq!(tx.queued_codewords(), 16);
    assert!(tx.drain().is_err());
    assert_eq!(tx.queued_codewords(), 16);
}
