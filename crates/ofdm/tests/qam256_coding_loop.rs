/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Complete independent256-QAM cell-loop trials, with actual payload truth.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::simulation::{coding_loop::NoiselessQam256Codec, source::SeededSource};

#[test]
fn noiseless_256qam_chain_recovers_every_payload_and_reports_decoder_iterations() {
    let mut codec = NoiselessQam256Codec::new().unwrap();
    let mut source = SeededSource::new(41);
    let mut edge = vec![0; 14232];
    edge[14231] = 1;
    for payload in [vec![0; 14232], vec![1; 14232], edge, source.next_payload()] {
        let result = codec.run_block(&payload, 30).unwrap();
        assert_eq!(result.data_cells, 2025);
        assert_eq!(result.coded_bits_tested, 16200);
        assert_eq!(result.payload_bits_tested, 14232);
        assert_eq!(result.pre_ldpc_bit_errors, 0);
        assert_eq!(result.post_ldpc_bits_tested, 14400);
        assert_eq!(result.post_ldpc_bit_errors, 0);
        assert_eq!(result.payload_bit_errors, 0);
        assert!(result.ldpc_success && result.bch_success);
        assert_eq!(result.bch_corrected_bits, 0);
        // A valid initial syndrome can legitimately require zero iterations.
        assert!(result.ldpc_iterations <= 30);
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["ldpc_iterations"], result.ldpc_iterations);
        assert_eq!(json["payload_bit_errors"], 0);
        assert_eq!(
            json["coding_mode"],
            "ds_short_8_9_256qam_noiseless_cell_loop"
        );
    }
}

#[test]
fn bounded_input_failures_do_not_poison_a_reused_codec() {
    let mut codec = NoiselessQam256Codec::new().unwrap();
    assert!(codec.run_block(&[], 30).is_err());
    let mut payload = vec![0; 14232];
    payload[42] = 2;
    assert!(codec.run_block(&payload, 30).is_err());
    assert_eq!(payload[42], 2);
    payload[42] = 0;
    for limit in [0, 101, usize::MAX] {
        assert!(codec.run_block(&payload, limit).is_err());
    }
    let result = codec.run_block(&payload, 1).unwrap();
    assert_eq!(result.payload_bit_errors, 0);
    assert!(result.ldpc_success && result.bch_success);
    assert!(result.ldpc_iterations <= 1);
}
