/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 AnnexE CRC vectors and independent Table19 triangular parity oracle.
use fec_codec::{
    crc::crc24d,
    ncp::{NcpFecCodec, NCP_TRANSMITTED_COORDINATES},
};

// Independent sequential parity solution from Table19, PDF91: each row block
// introduces one new shifted16-bit parity group. This does not call the encoder.
fn oracle(payload: [u8; 3]) -> Vec<u8> {
    let mut word = [0; 160];
    for index in 0..24 {
        word[index] = (payload[index / 8] >> (7 - index % 8)) & 1;
    }
    let rows = [
        [1, 11, 10, 12, 7, 9, -1, -1, -1, -1],
        [2, 1, 14, 15, 14, 14, 12, -1, -1, -1],
        [0, 9, 3, 2, -1, -1, 11, 7, -1, -1],
        [6, 8, -1, 10, 3, -1, -1, 10, 4, -1],
        [12, 13, 11, -1, 0, -1, -1, -1, 5, 2],
    ];
    for (group, shifts) in rows.iter().enumerate() {
        for row in 0..16 {
            let mut parity = 0;
            for (column, shift) in shifts[..5 + group].iter().enumerate() {
                if *shift >= 0 {
                    parity ^= word[column * 16 + (row + *shift as usize) % 16];
                }
            }
            let target = (5 + group) * 16 + (row + shifts[5 + group] as usize) % 16;
            word[target] = parity;
        }
    }
    for shifts in rows {
        for row in 0..16 {
            let parity = shifts
                .iter()
                .enumerate()
                .filter(|(_, shift)| **shift >= 0)
                .fold(0, |sum, (column, shift)| {
                    sum ^ word[column * 16 + (row + *shift as usize) % 16]
                });
            assert_eq!(parity, 0);
        }
    }
    NCP_TRANSMITTED_COORDINATES
        .map(|coordinate| word[coordinate])
        .to_vec()
}
fn soft(bits: &[u8]) -> Vec<f64> {
    bits.iter()
        .map(|bit| if *bit == 0 { 20.0 } else { -20.0 })
        .collect()
}

#[test]
fn published_crc24d_vectors_and_zero_residue() {
    // CM-SP-PHYv3.1-I20-230419 AnnexE, PDF210; external fixed expected values.
    assert_eq!(crc24d(&[1, 2, 3, 4, 5, 6, 7]), [0xcd, 0xef, 0x27]);
    let mut frame: Vec<u8> = (1..=255).collect();
    assert_eq!(crc24d(&frame), [0x2c, 0xa8, 0x8b]);
    frame.extend([0x2c, 0xa8, 0x8b]);
    assert_eq!(crc24d(&frame), [0; 3]);
    assert_eq!(crc24d(&[]), [0; 3]);
}
#[test]
fn table19_basis_parity_and_noiseless_decoding() {
    let mut codec = NcpFecCodec::new().unwrap();
    let mut basis = vec![[0; 3], [255; 3], [0x35, 0xa2, 0xc7]];
    for index in 0..24 {
        let mut payload = [0; 3];
        payload[index / 8] = 1 << (7 - index % 8);
        basis.push(payload);
    }
    // Hand-solved a0 impulse: retained set {a0,b106,b128,b129,b140,b142}.
    let first = codec.encode(&[0x80, 0, 0]);
    assert_eq!(
        first
            .iter()
            .enumerate()
            .filter(|(_, bit)| **bit == 1)
            .map(|(index, _)| index)
            .collect::<Vec<_>>(),
        [0, 26, 32, 33, 44, 46]
    );
    for payload in basis {
        let encoded = codec.encode(&payload);
        assert_eq!(encoded.as_slice(), oracle(payload));
        let decoded = codec.decode(&soft(&encoded), 100).unwrap();
        assert!(
            decoded.success,
            "Payload {payload:?}, iterations {}",
            decoded.iterations
        );
        assert_eq!(decoded.payload, payload);
        assert!(decoded.iterations <= 100);
    }
}
#[test]
fn controlled_error_correction_and_bounded_input_validation() {
    let mut codec = NcpFecCodec::new().unwrap();
    let payload = [0x31, 0x00, 0x09];
    let mut llrs = soft(&codec.encode(&payload));
    // One weak wrong sign, a specified demonstration rather than a code-distance claim.
    llrs[0] = -llrs[0].signum() * 0.25;
    let decoded = codec.decode(&llrs, 100).unwrap();
    assert!(decoded.success);
    assert_eq!(decoded.payload, payload);
    assert!(decoded.iterations > 0);
    let contradictory: Vec<_> = (0..48)
        .map(|index| {
            if (index * 17 + index / 3) % 7 < 3 {
                20.0
            } else {
                -20.0
            }
        })
        .collect();
    assert!(!codec.decode(&contradictory, 1).unwrap().success);
    assert!(codec.decode(&llrs, 0).is_err());
    assert!(codec.decode(&llrs, 101).is_err());
    assert!(codec.decode(&llrs[..47], 30).is_err());
    llrs[0] = f64::NAN;
    assert!(codec.decode(&llrs, 30).is_err());
    llrs[0] = f64::INFINITY;
    assert!(codec.decode(&llrs, 30).is_err());
}
