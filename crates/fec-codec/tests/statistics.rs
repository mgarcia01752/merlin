/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use fec_codec::{
    concatenated::ConcatenatedCodec,
    ldpc::LdpcCodec,
    parameters::{BCH_INPUT_BITS, LDPC_CODEWORD_BITS},
    statistics::{FecCounters, FecOutcome},
};
#[test]
fn decoder_outcomes_partition_words_without_double_counting_or_using_truth() {
    let mut counters = FecCounters::new();
    for (ldpc, bch, changes, bits, truth, outcome) in [
        (true, true, 0, 0, false, FecOutcome::Clean),
        (true, true, 1, 0, false, FecOutcome::Corrected),
        (true, true, 0, 2, false, FecOutcome::Corrected),
        (true, true, 3, 2, false, FecOutcome::Corrected),
        (false, false, 3, 2, true, FecOutcome::Uncorrectable),
        (true, false, 0, 0, true, FecOutcome::Uncorrectable),
        (true, true, 0, 0, true, FecOutcome::Clean),
    ] {
        let actual = FecOutcome::from_decoder(ldpc, bch, changes, bits);
        assert_eq!(actual, outcome);
        counters.record(actual, truth);
    }
    assert_eq!(counters.total_codewords(), 7);
    assert_eq!(counters.corrected_codewords(), 3);
    assert_eq!(counters.uncorrectable_codewords(), 2);
    assert_eq!(counters.clean_codewords(), 2);
    assert_eq!(counters.undetected_errors(), 1);
    let snapshot = counters.snapshot();
    counters.record(FecOutcome::Clean, false);
    assert_eq!(snapshot.total_codewords(), 7);
    assert_eq!(
        counters.total_codewords(),
        counters.clean_codewords()
            + counters.corrected_codewords()
            + counters.uncorrectable_codewords()
    );
}
#[test]
fn real_ldpc_only_correction_counts_one_word() {
    let mut codec = ConcatenatedCodec::new().unwrap();
    let encoded = codec.encode(&vec![0; BCH_INPUT_BITS]).unwrap();
    let mut llrs = vec![8.0; LDPC_CODEWORD_BITS];
    llrs[17] = -0.1;
    let decoded = codec.decode(&llrs, 50).unwrap();
    assert!(decoded.ldpc.success && decoded.bch.success);
    assert_eq!(decoded.ldpc.codeword, encoded.ldpc_codeword);
    assert_eq!(decoded.ldpc.hard_decision_changes, 1);
    assert_eq!(decoded.bch.corrected_bits, 0);
    assert_eq!(
        FecOutcome::from_decoder(true, true, decoded.ldpc.hard_decision_changes, 0),
        FecOutcome::Corrected
    );
}
#[test]
fn bch_only_and_combined_corrections_are_real_and_counted_once() {
    let codec = ConcatenatedCodec::new().unwrap();
    let mut bch = codec.encode(&vec![0; BCH_INPUT_BITS]).unwrap().bch_codeword;
    bch[37] ^= 1;
    let ldpc = LdpcCodec::new().unwrap();
    let encoded = ldpc.encode(&bch).unwrap();
    let mut receiver = ConcatenatedCodec::new().unwrap();
    for disturb in [false, true] {
        let mut llrs = encoded
            .iter()
            .map(|bit| if *bit == 0 { 8.0 } else { -8.0 })
            .collect::<Vec<_>>();
        if disturb {
            llrs[23] = -0.1;
        }
        let decoded = receiver.decode(&llrs, 50).unwrap();
        assert!(decoded.ldpc.success && decoded.bch.success);
        assert_eq!(decoded.bch.corrected_bits, 1);
        assert_eq!(decoded.ldpc.hard_decision_changes, usize::from(disturb));
        let mut counts = FecCounters::new();
        counts.record(
            FecOutcome::from_decoder(
                true,
                true,
                decoded.ldpc.hard_decision_changes,
                decoded.bch.corrected_bits,
            ),
            false,
        );
        assert_eq!(counts.corrected_codewords(), 1);
        assert_eq!(counts.total_codewords(), 1);
    }
}
