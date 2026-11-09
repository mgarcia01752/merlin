/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::simulation::source::file::{
    FilePayload, FileSource, FileVerifier, PrbsFill, MAX_FILE_BITS, MAX_FILE_BYTES,
};
use fec_codec::parameters::BCH_INPUT_BITS;

#[test]
fn exact_byte_order_preserves_binary_and_text_and_ignores_padding_errors() {
    let bytes = [0xef, 0xbb, 0xbf, b'A', b'\r', b'\n', 0, 0xff, 0x81];
    let input = FilePayload::new(&bytes, None).unwrap();
    let mut source = FileSource::new(input.clone()).unwrap();
    let mut verifier = FileVerifier::new(input).unwrap();
    let mut bits = source.next_payload().unwrap().unwrap();
    assert_eq!(&bits[..8], &[1, 1, 1, 0, 1, 1, 1, 1]);
    assert_eq!(&bits[64..72], &[1, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(source.position(), 72);
    assert_eq!(source.padding_bits(), BCH_INPUT_BITS as u64 - 72);
    bits[72..].fill(1); // Padding decisions never count as user data.
    let comparison = verifier.compare_payload(&bits).unwrap();
    assert_eq!(comparison.bits_tested, 72);
    assert_eq!(comparison.bit_errors, 0);
    assert_eq!(verifier.recovered_file().bytes(), bytes);
    assert!(verifier.recovered_file().complete);
    assert!(verifier.compare_payload(&bits).is_err());
    assert!(source.next_payload().unwrap().is_none());
    assert!(source.next_payload().unwrap().is_none());
    let summary = verifier.summary(source.position(), source.padding_bits());
    assert!(summary.fully_compared && summary.fully_generated);
    assert_eq!(summary.file_ber, Some(0.0));
    assert_eq!(summary.fill_ber, None);
}

#[test]
fn finite_fill_continues_across_words_with_independent_recurrence_and_segment_errors() {
    let bytes = vec![0x53; 1800]; // Original crosses a14232-bit payload boundary.
    let target = BCH_INPUT_BITS as u64 * 3 + 5;
    let input = FilePayload::new(
        &bytes,
        Some(PrbsFill {
            target_bits: target,
            seed: 1,
        }),
    )
    .unwrap();
    let mut source = FileSource::new(input.clone()).unwrap();
    let mut verifier = FileVerifier::new(input.clone()).unwrap();
    let mut original_bits: Vec<u8> = bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |shift| (byte >> shift) & 1))
        .collect();
    // Independent recurrence vector, with initial s[0..15] from seed1 LSB-first.
    let fill_length = target as usize - original_bits.len();
    let mut reference = vec![0; fill_length.max(15)];
    reference[0] = 1;
    for index in 15..reference.len() {
        reference[index] = reference[index - 1] ^ reference[index - 15];
    }
    original_bits.extend_from_slice(&reference[..fill_length]);
    let mut emitted = Vec::new();
    let mut word_count = 0;
    while let Some(mut word) = source.next_payload().unwrap() {
        emitted.extend_from_slice(&word);
        if word_count == 0 {
            word[0] ^= 1;
        }
        if word_count == 1 {
            word[200] ^= 1;
        } // File ended168bits into second word.
        verifier.compare_payload(&word).unwrap();
        word_count += 1;
    }
    assert_eq!(&emitted[..target as usize], original_bits);
    assert_eq!(word_count, 4);
    assert_eq!(source.padding_bits(), BCH_INPUT_BITS as u64 - 5);
    let summary = verifier.summary(source.position(), source.padding_bits());
    assert_eq!(summary.file.bit_errors, 1);
    assert_eq!(summary.fill.bit_errors, 1);
    assert_eq!(summary.file.first_error_bit, Some(0));
    assert_eq!(summary.fill.first_error_bit, Some(32));
    assert_eq!(summary.fill.bits_tested, target - bytes.len() as u64 * 8);
    assert_eq!(verifier.totals().bit_errors, 2);
    let recovered = verifier.recovered_file();
    assert_eq!(recovered.bytes().len(), bytes.len());
    assert_eq!(recovered.bytes()[0], bytes[0] ^ 0x80);
    assert!(summary.fully_compared);
    let serialized = serde_json::to_value(&input).unwrap();
    assert!(serialized.get("bytes").is_none());
    assert!(serde_json::to_value(recovered)
        .unwrap()
        .get("bytes")
        .is_none());
}

#[test]
fn validation_empty_exact_boundary_and_rejected_calls_preserve_state() {
    assert!(FilePayload::new(&vec![0; MAX_FILE_BYTES + 1], None).is_err());
    assert!(FilePayload::new(
        &[1],
        Some(PrbsFill {
            target_bits: 7,
            seed: 1
        })
    )
    .is_err());
    assert!(FilePayload::new(
        &[],
        Some(PrbsFill {
            target_bits: MAX_FILE_BITS + 1,
            seed: 1
        })
    )
    .is_err());
    for seed in [0, 0x8000] {
        assert!(FilePayload::new(
            &[],
            Some(PrbsFill {
                target_bits: 8,
                seed
            })
        )
        .is_err());
    }
    let empty = FilePayload::new(&[], None).unwrap();
    assert!(FileSource::new(empty.clone())
        .unwrap()
        .next_payload()
        .unwrap()
        .is_none());
    let summary = FileVerifier::new(empty).unwrap().summary(0, 0);
    assert!(summary.fully_compared && summary.fully_generated);
    assert_eq!(summary.file_ber, None);
    let input = FilePayload::new(&vec![0x55; BCH_INPUT_BITS / 8], None).unwrap();
    let mut source = FileSource::new(input.clone()).unwrap();
    let mut verifier = FileVerifier::new(input).unwrap();
    assert!(verifier.compare_payload(&[0]).is_err());
    let mut malformed = vec![0; BCH_INPUT_BITS];
    malformed[500] = 2;
    assert!(verifier.compare_payload(&malformed).is_err());
    assert_eq!(verifier.totals().bits_tested, 0);
    let bits = source.next_payload().unwrap().unwrap();
    assert_eq!(source.padding_bits(), 0);
    assert_eq!(verifier.compare_payload(&bits).unwrap().bit_errors, 0);
    assert!(source.next_payload().unwrap().is_none());
}

#[test]
fn empty_file_can_supply_finite_prbs_fill_and_equal_target_needs_no_append() {
    for bytes in [&[][..], &[0xa5][..]] {
        let input = FilePayload::new(
            bytes,
            Some(PrbsFill {
                target_bits: 8,
                seed: 1,
            }),
        )
        .unwrap();
        let mut source = FileSource::new(input.clone()).unwrap();
        let mut verifier = FileVerifier::new(input).unwrap();
        let bits = source.next_payload().unwrap().unwrap();
        if bytes.is_empty() {
            assert_eq!(&bits[..8], &[1, 0, 0, 0, 0, 0, 0, 0]);
        } else {
            assert_eq!(&bits[..8], &[1, 0, 1, 0, 0, 1, 0, 1]);
        }
        assert_eq!(verifier.compare_payload(&bits).unwrap().bit_errors, 0);
        let summary = verifier.summary(source.position(), source.padding_bits());
        assert_eq!(
            summary.fill.bits_tested,
            if bytes.is_empty() { 8 } else { 0 }
        );
        assert_eq!(verifier.recovered_file().bytes(), bytes);
    }
}
