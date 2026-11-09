/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared real PLC FEC/message path with soft inverse, before randomization/waveform.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    simulation::plc::PlcInterleavingStages,
    transmission::{
        plc::{
            block_interleaving::deinterleave_llrs, ExtendedTimestamp, PlcBlockCodec,
            PlcMessageFrame,
        },
        SubcarrierSpacing,
    },
};
#[test]
fn both_full_frames_recover_after_permutation_without_transmitter_bit_ids() {
    let mut codec = PlcBlockCodec::new().unwrap();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let frame =
            PlcMessageFrame::idle(spacing, ExtendedTimestamp::new(123, 19, 0).unwrap()).unwrap();
        let mut recovered = Vec::new();
        for (word, information) in frame.information_blocks().unwrap().iter().enumerate() {
            let encoded = codec.encode(information).unwrap();
            let start = (word * frame.geometry().data_symbols_per_codeword) as u64;
            let stages =
                PlcInterleavingStages::generate(spacing, start, &encoded.transmitted_bits).unwrap();
            let llrs: Vec<_> = stages
                .interleaved()
                .values()
                .iter()
                .flat_map(|label| {
                    (0..4).map(move |bit| if label & (8 >> bit) == 0 { 20.0 } else { -20.0 })
                })
                .collect();
            // Receiver needs spacing and received soft values, not the TX permutation
            // indices, stage snapshot, original information or codeword diagnostics.
            let decoded = codec
                .decode(&deinterleave_llrs(spacing, &llrs).unwrap(), 100)
                .unwrap();
            assert!(decoded.success);
            assert_eq!(decoded.information, *information);
            recovered.extend_from_slice(decoded.information.bytes());
        }
        assert_eq!(recovered, frame.to_bytes());
        assert_eq!(
            PlcMessageFrame::from_bytes(spacing, &recovered).unwrap(),
            frame
        );
    }
}
