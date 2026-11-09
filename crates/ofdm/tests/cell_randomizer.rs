/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 page124 published states, mask ordering and noiseless soft inverse.
use docsis_ofdm_fec::signal_processing::modulation::{
    map_symbol, qam_llrs, randomization::CellRandomizer,
};

fn packed_state(stream: &CellRandomizer) -> u32 {
    let state = stream.state();
    (u32::from(state.register_d1) << 12) | u32::from(state.register_d0)
}

#[test]
fn published_data_pilot_zero_sequence_matches_every_register_state() {
    let mut stream = CellRandomizer::new();
    stream.begin_symbol(0).unwrap();
    // PHY I20 section7.5.5.3, PDF124: {A0,SP,A1,Z,A2} states before each cell.
    assert_eq!(packed_state(&stream), 0xaaa555);
    assert_eq!(stream.apply_data(256, 0).unwrap(), 0xaa);
    assert_eq!(packed_state(&stream), 0xfffadf);
    stream.skip_scattered_pilot().unwrap();
    assert_eq!(packed_state(&stream), 0xfffadf);
    assert_eq!(stream.apply_data(256, 0).unwrap(), 0xfb);
    assert_eq!(packed_state(&stream), 0x520799);
    let label = stream.zero_loaded_bpsk().unwrap();
    assert_eq!(label, 1);
    assert_eq!(map_symbol(2, label).unwrap(), [-1.0, 0.0]);
    assert_eq!(packed_state(&stream), 0x2b9828);
    assert_eq!(stream.apply_data(256, 0).unwrap(), 0x14);
}

#[test]
fn lsb_first_register_bits_become_msb_first_labels_including_8k_and_16k() {
    // Independent bit strings from D0=555 and D1=AAA, Figure52/PDF123–124.
    for (order, mask) in [
        (16, 0b1010),
        (64, 0b101010),
        (256, 0b10101010),
        (4096, 0b101010101010),
        (8192, 0b1010101010100),
        (16384, 0b10101010101001),
    ] {
        let mut stream = CellRandomizer::new();
        stream.begin_symbol(0).unwrap();
        assert_eq!(stream.apply_data(order, 0).unwrap(), mask);
    }
    for (order, mask) in [(4, 0b10), (16, 0b1010), (64, 0b101010)] {
        let mut stream = CellRandomizer::new();
        stream.begin_symbol(0).unwrap();
        assert_eq!(stream.apply_ncp(order, 0).unwrap(), mask);
    }
}

#[test]
fn consecutive_symbols_preserve_state_until_128_boundary_even_with_first_pilot() {
    let mut stream = CellRandomizer::new();
    stream.begin_symbol(0).unwrap();
    stream.apply_data(256, 0).unwrap();
    let advanced = stream.state();
    for symbol in 1..128 {
        stream.begin_symbol(symbol).unwrap();
        assert_eq!(stream.state(), advanced);
    }
    stream.begin_symbol(128).unwrap();
    assert_eq!(packed_state(&stream), 0xaaa555);
    stream.skip_scattered_pilot().unwrap();
    assert_eq!(packed_state(&stream), 0xaaa555);
    assert_eq!(stream.apply_data(256, 0).unwrap(), 0xaa);
    assert!(stream.begin_symbol(128).is_err());
    assert!(stream.begin_symbol(130).is_err());
    stream.begin_symbol(129).unwrap();
    assert_eq!(stream.state(), advanced);
}

#[test]
fn all_256_labels_recover_original_bits_after_mapping_and_soft_descrambling() {
    let mut transmit = CellRandomizer::new();
    let mut hard_receive = CellRandomizer::new();
    let mut soft_receive = CellRandomizer::new();
    for stream in [&mut transmit, &mut hard_receive, &mut soft_receive] {
        stream.begin_symbol(0).unwrap();
    }
    for original in 0..256 {
        let randomized = transmit.apply_data(256, original).unwrap();
        assert_eq!(hard_receive.apply_data(256, randomized).unwrap(), original);
        let llrs = qam_llrs(256, map_symbol(256, randomized).unwrap(), 0.01).unwrap();
        let recovered = soft_receive.descramble_data_llrs(256, &llrs).unwrap();
        for (bit, (before, after)) in llrs.iter().zip(&recovered).enumerate() {
            assert_eq!(before.abs(), after.abs());
            assert_eq!(u16::from(*after < 0.0), (original >> (7 - bit)) & 1);
        }
    }
    assert_eq!(transmit.state(), hard_receive.state());
    assert_eq!(transmit.state(), soft_receive.state());
}

#[test]
fn mixed_orders_ncp_and_skipped_cells_keep_transmitter_and_receiver_synchronized() {
    let mut transmit = CellRandomizer::new();
    let mut receive = CellRandomizer::new();
    transmit.begin_symbol(0).unwrap();
    receive.begin_symbol(0).unwrap();
    for order in [16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
        for original in [0, 1, order - 1] {
            let label = transmit.apply_data(order, original).unwrap();
            let soft = qam_llrs(order, map_symbol(order, label).unwrap(), 0.01).unwrap();
            let recovered = receive.descramble_data_llrs(order, &soft).unwrap();
            for (bit, llr) in recovered.iter().enumerate() {
                assert_eq!(
                    u16::from(*llr < 0.0),
                    (original >> (recovered.len() - 1 - bit)) & 1
                );
            }
            transmit.skip_scattered_pilot().unwrap();
            receive.skip_scattered_pilot().unwrap();
            assert_eq!(
                transmit.zero_loaded_bpsk().unwrap(),
                receive.zero_loaded_bpsk().unwrap()
            );
        }
    }
    for order in [4, 16, 64] {
        let label = transmit.apply_ncp(order, 1).unwrap();
        let soft = qam_llrs(order, map_symbol(order, label).unwrap(), 0.01).unwrap();
        let recovered = receive.descramble_ncp_llrs(order, &soft).unwrap();
        let hard = recovered
            .iter()
            .fold(0u16, |word, llr| (word << 1) | u16::from(*llr < 0.0));
        assert_eq!(hard, 1);
    }
    assert_eq!(transmit.state(), receive.state());
}

#[test]
fn rejected_inputs_never_advance_registers_or_symbol_sequence() {
    let mut stream = CellRandomizer::new();
    let seed = stream.state();
    assert!(stream.apply_data(256, 0).is_err());
    assert!(stream.zero_loaded_bpsk().is_err());
    assert!(stream.skip_scattered_pilot().is_err());
    assert!(stream.begin_symbol(1).is_err());
    assert_eq!(stream.state(), seed);
    stream.begin_symbol(0).unwrap();
    for order in [0, 2, 4, 8, 32, 65535] {
        assert!(stream.apply_data(order, 0).is_err());
    }
    assert!(stream.apply_ncp(256, 0).is_err());
    assert!(stream.apply_data(256, 256).is_err());
    assert!(stream.apply_ncp(4, 4).is_err());
    assert!(stream.descramble_data_llrs(256, &[0.0; 7]).is_err());
    assert!(stream.descramble_data_llrs(256, &[f64::NAN; 8]).is_err());
    assert!(stream.descramble_ncp_llrs(4, &[f64::INFINITY; 2]).is_err());
    assert!(stream.begin_symbol(0).is_err());
    assert_eq!(stream.state(), seed);
    stream.begin_symbol(1).unwrap();
    let llrs = [1.0; 8];
    let original = llrs;
    stream.descramble_data_llrs(256, &llrs).unwrap();
    assert_eq!(llrs, original);
}
