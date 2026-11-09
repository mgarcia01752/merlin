/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent frame dimensions from PHY I20 §7.5.13.7 and MULPI I25 Table74.
use docsis_ofdm_fec::transmission::{
    plc::{geometry::*, PlcFrameGeometry},
    SubcarrierSpacing,
};
#[test]
fn frame_capacities_follow_both_fft_sizes() {
    for (spacing, carriers, symbols, words, payload, encoded) in [
        (SubcarrierSpacing::Khz25, 16, 6, 20, 720, 960),
        (SubcarrierSpacing::Khz50, 8, 12, 10, 360, 480),
    ] {
        let geometry = PlcFrameGeometry::new(spacing);
        assert_eq!(geometry.plc_subcarriers, carriers);
        assert_eq!(geometry.data_symbols_per_codeword, symbols);
        assert_eq!(geometry.codewords_per_frame, words);
        assert_eq!(geometry.payload_bytes, payload);
        assert_eq!(geometry.encoded_bytes, encoded);
        assert_eq!(words * symbols + PLC_PREAMBLE_SYMBOLS, 128);
        assert_eq!(payload * 8, words * 288);
        assert_eq!(encoded * 8, words * 384);
        assert_eq!(PLC_INFORMATION_BITS, 288);
    }
}
