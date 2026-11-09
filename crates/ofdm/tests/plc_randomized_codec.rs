/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Complete known-phase PLC soft recovery; no waveform/acquisition claim.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::transmission::{
    plc::{
        block_interleaving::{deinterleave_llrs, interleave},
        ExtendedTimestamp, PlcBlockCodec, PlcCodewordNibbles, PlcMessageFrame, PlcRandomizer,
    },
    SubcarrierSpacing,
};
#[test]
fn fec_interleaver_randomizer_soft_inverse_recovers_two_frames_both_spacings() {
    let mut codec = PlcBlockCodec::new().unwrap();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let mut transmitter = PlcRandomizer::new(spacing);
        let mut receiver = PlcRandomizer::new(spacing);
        for tick in 0..2 {
            let frame =
                PlcMessageFrame::idle(spacing, ExtendedTimestamp::new(123 + tick, 19, 0).unwrap())
                    .unwrap();
            let mut received = Vec::new();
            for information in frame.information_blocks().unwrap() {
                let encoded = codec.encode(&information).unwrap();
                let coded = PlcCodewordNibbles::from_bits(&encoded.transmitted_bits).unwrap();
                let output = transmitter.apply(&interleave(spacing, &coded)).unwrap();
                let llrs: Vec<_> = output
                    .values()
                    .iter()
                    .flat_map(|label| {
                        (0..4).map(move |bit| if label & (8 >> bit) == 0 { 20.0 } else { -20.0 })
                    })
                    .collect();
                // RX gets only received soft bits and independently configured spacing
                // and frame alignment. No TX masks, registers or truth feed decoding.
                let soft = receiver.descramble_llrs(&llrs).unwrap();
                let decoded = codec
                    .decode(&deinterleave_llrs(spacing, soft.llrs()).unwrap(), 100)
                    .unwrap();
                assert!(decoded.success);
                assert_eq!(decoded.information, information);
                received.extend_from_slice(decoded.information.bytes());
            }
            assert_eq!(received, frame.to_bytes());
            assert_eq!(
                PlcMessageFrame::from_bytes(spacing, &received).unwrap(),
                frame
            );
        }
    }
}
