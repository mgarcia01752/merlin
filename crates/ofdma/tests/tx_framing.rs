/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Meaningful/fill/pad roles and transactional transmitter admission.
mod common;
use docsis_ofdma::{
    configuration::Numerology,
    grant::select,
    signal_processing::PreEqBank,
    source::Prbs15,
    transmitter::{encode_grant, Transmitter},
    types::Identity,
};
use std::sync::Arc;

#[test]
fn codeword_information_fill_and_grant_pad_have_distinct_values() {
    for (capacity, information_lengths, fill, pad) in [
        (700, vec![420], 4, 0),
        (1400, vec![840], 0, 280),
        (1401, vec![420, 421], 1, 0),
        (3421, vec![2521], 1, 0),
        (11881, vec![10081], 1, 0),
    ] {
        let plan = select(capacity).unwrap();
        let encoded = encode_grant(&plan, &vec![0; plan.source_bits]).unwrap();
        let mut information = Vec::new();
        for (word, count) in plan.codewords.iter().zip(information_lengths) {
            information.extend_from_slice(&encoded[word.coded_offset..word.coded_offset + count]);
        }
        assert_eq!(
            &information[..information.len() - fill],
            vec![0; information.len() - fill]
        );
        assert_eq!(&information[information.len() - fill..], vec![1; fill]);
        assert_eq!(&encoded[capacity - pad..], vec![0; pad]);
        let mut changed = plan.clone();
        changed.source_bits += 1;
        assert!(encode_grant(&changed, &vec![0; changed.source_bits]).is_err());
    }
}

#[test]
fn transmitter_rejection_preserves_source_and_emission_stops_at_declared_support() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let pattern = if mode == Numerology::Khz50 { 1 } else { 8 };
        let config = Arc::new(common::configuration(mode, 4, pattern, 2, 8));
        let id = Identity {
            run: 1,
            generation: 1,
            grant: 7,
        };
        let mut source = Prbs15::new(1).unwrap();
        assert!(Transmitter::new(
            config.clone(),
            Identity { grant: 8, ..id },
            &mut source,
            PreEqBank::unity(&config)
        )
        .is_err());
        assert_eq!(source.position(), 0);
        let mut transmitter =
            Transmitter::new(config.clone(), id, &mut source, PreEqBank::unity(&config)).unwrap();
        let count = source.position();
        assert_eq!(count, transmitter.plan().source_bits as u64);
        let first = transmitter.next_frame().unwrap().unwrap();
        let second = transmitter.next_frame().unwrap().unwrap();
        assert_eq!(first.block.start, config.grant().epoch_sample);
        assert_eq!(
            second.block.start,
            first.block.start + first.block.samples.len() as u64
        );
        let expected = 2 * config.waveform().frame_symbols() * config.waveform().stride()
            + config.waveform().rolloff();
        assert_eq!(
            first.block.samples.len() + second.block.samples.len(),
            expected
        );
        assert!(transmitter.next_frame().unwrap().is_none());
        assert_eq!(source.position(), count);
    }
}
