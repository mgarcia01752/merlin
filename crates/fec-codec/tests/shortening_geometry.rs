/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independently checked Figure46/47 coordinates and Table23 dimensions.
use fec_codec::shortening::{decoding::SHORTENED_ZERO_LLR_MAGNITUDE, Shortening};
#[test]
fn all_payload_lengths_retain_both_parities_and_match_published_burst_example() {
    for payload_bits in 1..=14232 {
        let geometry = Shortening::new(payload_bits).unwrap();
        assert_eq!(geometry.payload_bits(), payload_bits);
        assert_eq!(geometry.omitted_bits() + payload_bits, 14232);
        assert_eq!(geometry.bch_bits() - payload_bits, 168);
        assert_eq!(geometry.transmitted_bits() - geometry.bch_bits(), 1800);
        assert_eq!(geometry.transmitted_bits() + geometry.omitted_bits(), 16200);
    }
    let geometry = Shortening::new(6136).unwrap();
    assert_eq!(geometry.omitted_bits(), 8096);
    assert_eq!(geometry.bch_bits(), 6304);
    assert_eq!(geometry.transmitted_bits(), 8104);
    assert!((geometry.ldpc_rate() - 6304.0 / 8104.0).abs() < 1e-15);
    assert!((geometry.payload_rate() - 6136.0 / 8104.0).abs() < 1e-15);
    for unsupported in [0, 14233, 64800, usize::MAX] {
        assert!(Shortening::new(unsupported).is_err());
    }
}
#[test]
fn figure47_zero_regions_and_soft_restoration_are_exact_and_do_not_erase_parity() {
    for payload_bits in [1, 7, 64, 6136, 14231, 14232] {
        let geometry = Shortening::new(payload_bits).unwrap();
        let payload: Vec<_> = (0..payload_bits).map(|index| (index % 2) as u8).collect();
        let padded = geometry.prepend_bch_zeros(&payload).unwrap();
        assert_eq!(padded.len(), 14232);
        assert!(padded[..14232 - payload_bits].iter().all(|bit| *bit == 0));
        assert_eq!(&padded[14232 - payload_bits..], payload);
        let mut bch_mother = padded;
        bch_mother.extend(vec![1; 168]); // Tagged parity tests coordinates, not validity.
        let bch = geometry.remove_bch_zeros(&bch_mother).unwrap();
        assert_eq!(&bch[payload_bits..], vec![1; 168]);
        let mut ldpc = geometry.append_ldpc_zeros(&bch).unwrap();
        assert_eq!(&ldpc[..payload_bits + 168], bch);
        assert!(ldpc[payload_bits + 168..].iter().all(|bit| *bit == 0));
        ldpc.extend((0..1800).map(|index| (index % 2) as u8));
        let wire = geometry.remove_ldpc_zeros(&ldpc).unwrap();
        assert_eq!(&wire[payload_bits + 168..], &ldpc[14400..]);
        assert_eq!(geometry.restore_bch_input(&ldpc).unwrap(), bch_mother);
        let llrs: Vec<_> = (0..payload_bits + 1968)
            .map(|index| (index % 199) as f64 - 99.0)
            .collect();
        let restored = geometry.restore_ldpc_llrs(&llrs).unwrap();
        assert_eq!(restored.len(), 16200);
        assert_eq!(&restored[..payload_bits + 168], &llrs[..payload_bits + 168]);
        assert!(restored[payload_bits + 168..14400]
            .iter()
            .all(|value| *value == SHORTENED_ZERO_LLR_MAGNITUDE));
        assert_eq!(&restored[14400..], &llrs[payload_bits + 168..]);
        assert_eq!(llrs[0], -99.0); // Input was borrowed, never overwritten.
    }
}
#[test]
fn malformed_stage_inputs_reject_and_known_zero_consistency_is_separate() {
    let geometry = Shortening::new(6136).unwrap();
    assert!(geometry.prepend_bch_zeros(&[]).is_err());
    assert!(geometry.prepend_bch_zeros(&vec![2; 6136]).is_err());
    assert!(geometry.remove_bch_zeros(&[0; 12]).is_err());
    let mut bch = vec![0; 14400];
    bch[0] = 1;
    assert!(geometry.remove_bch_zeros(&bch).is_err());
    assert!(geometry.append_ldpc_zeros(&vec![2; 6304]).is_err());
    assert!(geometry.remove_ldpc_zeros(&[]).is_err());
    let mut ldpc = vec![0; 16200];
    ldpc[6304] = 1;
    assert!(geometry.remove_ldpc_zeros(&ldpc).is_err());
    bch[0] = 0;
    assert!(!geometry.known_zeros_consistent(&ldpc, &bch));
    ldpc[6304] = 0;
    bch[8095] = 1;
    assert!(!geometry.known_zeros_consistent(&ldpc, &bch));
    bch[8095] = 0;
    assert!(geometry.known_zeros_consistent(&ldpc, &bch));
    assert!(!geometry.known_zeros_consistent(&[], &bch));
    assert!(geometry.restore_bch_input(&[0; 2]).is_err());
    assert!(geometry.restore_ldpc_llrs(&[0.0; 2]).is_err());
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut llrs = vec![1.0; 8104];
        llrs[300] = invalid;
        assert!(geometry.restore_ldpc_llrs(&llrs).is_err());
    }
    let mut llrs = vec![1.0; 8104];
    llrs[0] = f64::MAX;
    llrs[6304] = -f64::MAX;
    let restored = geometry.restore_ldpc_llrs(&llrs).unwrap();
    assert_eq!(restored[0], 100.0);
    assert_eq!(restored[14400], -100.0);
}
