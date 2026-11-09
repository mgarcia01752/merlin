/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent test oracles from ETSI EN302769 V1.2.1 Tables4(b)/B.6.
//! Moved intact from normative.rs; no production encoder or matrix is called.
const ADDRESSES: &[&[usize]] = &[
    &[0, 1558, 712, 805],
    &[1, 1450, 873, 1337],
    &[2, 1741, 1129, 1184],
    &[3, 294, 806, 1566],
    &[4, 482, 605, 923],
    &[0, 926, 1578],
    &[1, 777, 1374],
    &[2, 608, 151],
    &[3, 1195, 210],
    &[4, 1484, 692],
    &[0, 427, 488],
    &[1, 828, 1124],
    &[2, 874, 1366],
    &[3, 1500, 835],
    &[4, 1496, 502],
    &[0, 1006, 1701],
    &[1, 1155, 97],
    &[2, 657, 1403],
    &[3, 1453, 624],
    &[4, 429, 1495],
    &[0, 809, 385],
    &[1, 367, 151],
    &[2, 1323, 202],
    &[3, 960, 318],
    &[4, 1451, 1039],
    &[0, 1098, 1722],
    &[1, 1015, 1428],
    &[2, 1261, 1564],
    &[3, 544, 1190],
    &[4, 1472, 1246],
    &[0, 508, 630],
    &[1, 421, 1704],
    &[2, 284, 898],
    &[3, 392, 577],
    &[4, 1155, 556],
    &[0, 631, 1000],
    &[1, 732, 1368],
    &[2, 1328, 329],
    &[3, 1515, 506],
    &[4, 1104, 1172],
];
const FACTORS: &[&[usize]] = &[
    &[0, 1, 3, 5, 14],
    &[0, 6, 8, 11, 14],
    &[0, 1, 2, 6, 9, 10, 14],
    &[0, 4, 7, 8, 10, 12, 14],
    &[0, 2, 4, 6, 8, 9, 11, 13, 14],
    &[0, 3, 7, 8, 9, 13, 14],
    &[0, 2, 5, 6, 7, 10, 11, 13, 14],
    &[0, 5, 8, 9, 10, 11, 14],
    &[0, 1, 2, 3, 9, 10, 14],
    &[0, 3, 6, 9, 11, 12, 14],
    &[0, 4, 11, 12, 14],
    &[0, 1, 2, 3, 5, 6, 7, 8, 10, 13, 14],
];

/// Computes parity with independently transcribed accumulator addresses.
/// # Arguments
/// * `message` -14400 binary LDPC information bits.
/// # Returns
///1800 accumulated parity bits, without calling the production matrix/encoder.
/// # Panics
/// Asserts fixed test-oracle dimensions and binary values.
pub fn ldpc_parity(message: &[u8]) -> Vec<u8> {
    assert_eq!(message.len(), 14400);
    assert!(message.iter().all(|bit| *bit <= 1));
    let mut parity = vec![0u8; 1800];
    for (group, addresses) in ADDRESSES.iter().enumerate() {
        for bit in 0..360 {
            if message[group * 360 + bit] == 1 {
                for address in *addresses {
                    parity[(address + 5 * bit) % 1800] ^= 1;
                }
            }
        }
    }
    for index in 1..1800 {
        parity[index] ^= parity[index - 1];
    }
    parity
}
/// Computes BCH parity by independent binary-polynomial long division.
/// # Arguments
/// * `message` - At most14232 binary bits; implicit leading zeros do not change remainder.
/// # Returns
///168 parity bits from the published polynomial factors, not the production LFSR.
/// # Panics
/// Asserts test-oracle input range/values and the normative generator degree.
pub fn bch_parity(message: &[u8]) -> Vec<u8> {
    assert!(message.len() <= 14232);
    assert!(message.iter().all(|bit| *bit <= 1));
    let mut generator = vec![1u8];
    for factor in FACTORS {
        let mut product = vec![0; generator.len() + 14];
        for (degree, coefficient) in generator.iter().enumerate() {
            for exponent in *factor {
                product[degree + exponent] ^= coefficient;
            }
        }
        generator = product;
    }
    assert_eq!(generator.len(), 169);

    let mut dividend = message.to_vec();
    dividend.extend([0; 168]);
    for offset in 0..message.len() {
        if dividend[offset] == 1 {
            for degree in 0..=168 {
                dividend[offset + degree] ^= generator[168 - degree];
            }
        }
    }
    dividend[message.len()..].to_vec()
}
