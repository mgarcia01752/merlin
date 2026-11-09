/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Table21 parity and all288 basis vectors, not only round-trip tests.
#[path = "support/plc.rs"]
mod support;
use fec_codec::plc::PlcFecCodec;
#[test]
fn table21_mother_parity_matches_all_information_basis_vectors() {
    let codec = PlcFecCodec::new().unwrap();
    let mut cases = vec![
        [0; 288],
        [1; 288],
        std::array::from_fn(|index| u8::from((index * 13 + index / 7) % 11 < 5)),
    ];
    for index in 0..288 {
        let mut information = [0; 288];
        information[index] = 1;
        cases.push(information);
    }
    for information in cases {
        let mother = codec.encode_mother(&information).unwrap();
        assert_eq!(mother, support::oracle(&information));
        assert_eq!(&mother[..288], information);
        assert!(codec.valid_syndrome(&mother));
        let expected: Vec<_> = mother[..48]
            .iter()
            .chain(&mother[96..384])
            .chain(&mother[432..])
            .copied()
            .collect();
        assert_eq!(codec.encode(&information).unwrap().as_slice(), expected);
    }
}
#[test]
fn encoding_rejects_nonbinary_information_and_wrong_dimensions() {
    let codec = PlcFecCodec::new().unwrap();
    assert!(codec.encode(&[0; 287]).is_err());
    assert!(codec.encode_mother(&[0; 289]).is_err());
    let mut information = [0; 288];
    information[48] = 2; // Even information later punctured must be validated.
    assert!(codec.encode(&information).is_err());
    assert!(!codec.valid_syndrome(&[0; 384]));
    assert!(!codec.valid_syndrome(&[2; 480]));
    let mut mother = [0; 480];
    mother[0] = 1;
    assert!(!codec.valid_syndrome(&mother));
}
