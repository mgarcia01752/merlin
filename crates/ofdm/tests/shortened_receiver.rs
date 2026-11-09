/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Boundary-driven reception, rejection and failed decisions independent of TX metadata.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::{
        modulation::interleaving::{mixed::PayloadBitOrder, shortened::ShortenedBitOrder},
        receiver::payload::{variable::VariablePayloadCollector, SoftDataCell},
    },
    transmission::ncp::NcpDataSpan,
};
use fec_codec::{
    concatenated::ConcatenatedCodec,
    shortening::{ShortenedCodec, Shortening},
};
fn span(start: bool, filler: bool, count: usize) -> NcpDataSpan {
    NcpDataSpan {
        start: 0,
        end_exclusive: count,
        profile_id: if filler || !start { None } else { Some(0) },
        starts_codeword: start,
        zero_loaded: filler,
    }
}
fn cells(bits: &[u8], width: usize) -> Vec<SoftDataCell> {
    bits.chunks(width)
        .enumerate()
        .map(|(position, chunk)| SoftDataCell {
            data_position: position,
            llrs: (0..width)
                .map(|index| {
                    if chunk.get(index).copied().unwrap_or(0) == 0 {
                        20.0
                    } else {
                        -20.0
                    }
                })
                .collect(),
        })
        .collect()
}
#[test]
fn boundary_receiver_handles_full_short_and_new_lengths_without_tx_size_input() {
    let mut collector = VariablePayloadCollector::new(&[7]).unwrap();
    for (sequence, count) in [1032, 6136, 14232].into_iter().enumerate() {
        let payload: Vec<_> = (0..count).map(|index| (index % 2) as u8).collect();
        let bits = if count == 14232 {
            let encoded = ConcatenatedCodec::new().unwrap().encode(&payload).unwrap();
            PayloadBitOrder::new(&[7])
                .unwrap()
                .encode(&encoded.ldpc_codeword)
                .unwrap()
        } else {
            let geometry = Shortening::new(count).unwrap();
            let encoded = ShortenedCodec::new(geometry)
                .unwrap()
                .encode(&payload)
                .unwrap();
            ShortenedBitOrder::new(geometry)
                .unwrap()
                .encode(&encoded.transmitted)
                .unwrap()
        };
        let received = cells(&bits, 7);
        assert!(collector
            .accept_span(
                &span(true, false, received.len()),
                &received,
                10 + sequence as u64 * 2,
                30
            )
            .unwrap()
            .is_none());
        let word = collector
            .accept_span(&span(false, true, 1), &[], 11 + sequence as u64 * 2, 30)
            .unwrap()
            .unwrap();
        assert_eq!(word.payload().unwrap(), payload);
        assert_eq!(word.receive_sequence, sequence as u64);
        assert_eq!(word.last_source_symbol, 10 + sequence as u64 * 2);
        assert!(
            word.decoded.ldpc.success && word.decoded.bch.success && word.known_zeros_consistent
        );
        assert_eq!(collector.pending_bits(), 0);
        let mut malformed = word;
        malformed.decoded.bch.codeword.clear();
        assert!(malformed.payload().is_err());
    }
}
#[test]
fn boundary_receiver_retains_failed_decisions_instead_of_substituting_truth() {
    let mut collector = VariablePayloadCollector::new(&[8]).unwrap();
    let soft: Vec<_> = (0..375)
        .map(|position| SoftDataCell {
            data_position: position,
            llrs: (0..8)
                .map(|bit| {
                    if (position * 17 + bit) % 7 < 3 {
                        20.0
                    } else {
                        -20.0
                    }
                })
                .collect(),
        })
        .collect();
    collector
        .accept_span(&span(true, false, soft.len()), &soft, 0, 1)
        .unwrap();
    let word = collector
        .accept_span(&span(false, true, 1), &[], 1, 1)
        .unwrap()
        .unwrap();
    assert_eq!(word.payload().unwrap().len(), 1032);
    assert!(!word.decoded.ldpc.success);
    assert_eq!(word.decoded.ldpc.iterations, 1);
    assert!(word.payload().unwrap().contains(&1));
}
#[test]
fn malformed_or_unterminated_intervals_never_become_invented_words() {
    let mut collector = VariablePayloadCollector::new(&[8]).unwrap();
    let soft = cells(&[0; 8], 8);
    assert!(collector
        .accept_span(&span(false, false, 1), &soft, 0, 30)
        .is_err());
    assert_eq!(collector.pending_bits(), 0);
    assert!(collector
        .accept_span(&span(true, false, 1), &soft, 0, 0)
        .is_err());
    let mut invalid = soft.clone();
    invalid[0].llrs[0] = f64::NAN;
    assert!(collector
        .accept_span(&span(true, false, 1), &invalid, 0, 30)
        .is_err());
    assert!(collector
        .accept_span(&span(true, false, 1), &soft, 0, 30)
        .unwrap()
        .is_none());
    assert_eq!(collector.pending_bits(), 8);
    assert!(collector
        .accept_span(&span(false, true, 1), &[], 1, 30)
        .is_err());
    assert_eq!(collector.pending_bits(), 8); // Failure does not fabricate an EOF-complete word.
}
