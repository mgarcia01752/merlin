/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual recovery, independent grant arithmetic and passive bounded report projections.
use docsis_ofdma::api::*;
use std::{
    thread,
    time::{Duration, Instant},
};

const PREFIX_SAMPLES: usize = 192;
const ROLLOFF_SAMPLES: usize = 64;
const FRAME_SYMBOLS: usize = 6;
const BANDWIDTH_HZ: u32 = 20_000_000;
const PROFILE_ID: u16 = 224;
const DATA_IUC: u8 = 5;
const PROFILE_RANDOMIZER_SEED: u32 = 0x7f_ffff;
const SOURCE_PRBS15_SEED: u16 = 1;
const FIRST_ACTIVE_CARRIER: usize = 128;
const DATA_LOADING: u8 = 4;
const GRANT_ID: u64 = 7;
const GRANT_EPOCH_SAMPLE: u64 = 73;
const FIRST_FRAME_ID: u64 = 3;
const GRANT_FRAMES: usize = 2;
const ESTIMATOR_VARIANCE: f64 = 0.02;
const ESTIMATOR_POWER_FLOOR: f64 = 1e-12;
const LDPC_ITERATION_BUDGET: usize = 50;
const DEFAULT_MINISLOTS: usize = 8;
const STOP_TEST_GRANTS: usize = 128;
const STOP_CHUNK_SAMPLES: usize = 313;
const STOP_DEADLINE_SECONDS: u64 = 10;
const FIRST_50KHZ_PILOT_PATTERN: u8 = 1;
const FIRST_25KHZ_PILOT_PATTERN: u8 = 8;

fn configuration(mode: Numerology, minislots: usize) -> ExperimentConfig {
    let width = mode.carriers_per_minislot();
    let pattern = match mode {
        Numerology::Khz50 => FIRST_50KHZ_PILOT_PATTERN,
        Numerology::Khz25 => FIRST_25KHZ_PILOT_PATTERN,
    };
    let profile = Profile::new(
        PROFILE_ID,
        DATA_IUC,
        PROFILE_RANDOMIZER_SEED,
        (0..minislots)
            .map(|index| Minislot {
                first_carrier: FIRST_ACTIVE_CARRIER + index * width,
                loading: DATA_LOADING,
                pilot_pattern: pattern,
            })
            .collect(),
    )
    .unwrap();
    let grant = SyntheticGrant {
        id: GRANT_ID,
        profile_id: PROFILE_ID,
        epoch_sample: GRANT_EPOCH_SAMPLE,
        first_frame: FIRST_FRAME_ID,
        frames: GRANT_FRAMES,
        minislots: (0..minislots).collect(),
    };
    let waveform = Waveform::new(
        mode,
        PREFIX_SAMPLES,
        ROLLOFF_SAMPLES,
        FRAME_SYMBOLS,
        BANDWIDTH_HZ,
    )
    .unwrap();
    let us = UsConfiguration::new(waveform, vec![profile], grant, vec![0, 1]).unwrap();
    ExperimentConfig::new(
        us,
        SOURCE_PRBS15_SEED,
        ReceiverConfig::new(
            Estimator::FrameFlatLs,
            ESTIMATOR_VARIANCE,
            ESTIMATOR_POWER_FLOOR,
            LDPC_ITERATION_BUDGET,
        )
        .unwrap(),
    )
    .unwrap()
}
fn session(mode: Numerology, minislots: usize, options: ExperimentOptions) -> Simulation {
    let mut simulation = Simulation::new(options).unwrap();
    simulation
        .configure(configuration(mode, minislots))
        .unwrap();
    simulation
}

#[test]
fn independent_short_medium_long_selection_offsets_and_original_decisions_are_owned_and_bounded() {
    // Independent capacity arithmetic for 50-kHz pattern 1 with two six-symbol frames:
    // 356*m - 28 coded bits for m adjacent loaded minislots. Selection/borrowing
    // anchors below are independent of the production ResourcePlan/selector.
    for (minislots, capacity, source_bits, families, retained_information, transmitted) in [
        (
            8,
            2820,
            1976,
            vec!["upstream_short", "upstream_short", "upstream_short"],
            vec![840, 420, 720],
            vec![1120, 700, 1000],
        ),
        (
            24,
            8516,
            6776,
            vec![
                "upstream_medium",
                "upstream_short",
                "upstream_short",
                "upstream_short",
            ],
            vec![5040, 840, 420, 476],
            vec![5940, 1120, 700, 756],
        ),
        (
            64,
            22756,
            19776,
            vec!["upstream_long", "upstream_medium", "upstream_short"],
            vec![14400, 4620, 756],
            vec![16200, 5520, 1036],
        ),
    ] {
        let mut simulation = session(Numerology::Khz50, minislots, ExperimentOptions::default());
        let report = simulation.run().unwrap();
        let original = report.to_json().unwrap();
        assert!(report.reference_recovered);
        assert_eq!(report.planned.grant_plan.capacity_bits, capacity);
        let verification = report.verification().unwrap();
        assert_eq!(verification.generated_source_bits, source_bits);
        assert_eq!(verification.compared_source_bits, source_bits);
        assert_eq!(verification.bit_errors, 0);
        assert_eq!(verification.ber, Some(0.0));
        assert_eq!(verification.source_seed, SOURCE_PRBS15_SEED);
        assert!(verification.reference_recovered);
        let summary = report.fec_summary().unwrap();
        assert_eq!(summary.completed_words, families.len());
        assert_eq!(summary.pending_words, 0);
        assert_eq!(summary.converged_words, summary.completed_words);
        assert_eq!(summary.nonconverged_words, 0);
        let mut coded_offset = 0;
        let mut information_offset = 0;
        for ordinal in 0..families.len() {
            let page = report
                .fec_words(FecWordRequest {
                    first_word: ordinal,
                    ..Default::default()
                })
                .unwrap();
            let actual = &page.words[0];
            assert_eq!(actual.code_family, families[ordinal]);
            assert_eq!(
                actual.retained_information_bits,
                retained_information[ordinal]
            );
            assert_eq!(actual.transmitted_bits, transmitted[ordinal]);
            assert_eq!(actual.coded_offset, coded_offset);
            assert_eq!(actual.source_start_bit, information_offset);
            assert_eq!(actual.word, report.grants[0].words[ordinal]);
            assert_eq!(actual.identity, report.grants[0].identity);
            assert_eq!(page.total_completed_words, families.len());
            assert_eq!(page.profile_id, PROFILE_ID);
            assert_eq!(page.iuc, DATA_IUC);
            assert_eq!(
                actual.shortened_bits,
                actual.mother_information_bits - retained_information[ordinal]
            );
            coded_offset += transmitted[ordinal];
            information_offset += retained_information[ordinal] as u64;
        }
        let page = report.fec_words(FecWordRequest::default()).unwrap();
        let retained = page.clone();
        let failed_budget = FecWordRequest {
            max_decision_bits: page.copied_decision_bits - 1,
            ..Default::default()
        };
        assert!(report.fec_words(failed_budget).is_err());
        assert_eq!(report.to_json().unwrap(), original);
        drop(report);
        drop(simulation);
        assert_eq!(page, retained);
    }
}

#[test]
fn both_numerologies_preserve_scalars_across_pages_continuations_empty_end_and_invalid_queries() {
    const EXPECTED_WORD_CEILING: usize = 32;
    const OVER_WORD_CEILING: usize = 33;
    const EXPECTED_DECISION_CEILING: usize = 262_144;
    const OVER_DECISION_CEILING: usize = 262_145;
    assert_eq!(MAX_FEC_PAGE_WORDS, EXPECTED_WORD_CEILING);
    assert_eq!(MAX_FEC_PAGE_DECISION_BITS, EXPECTED_DECISION_CEILING);
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let report = session(mode, DEFAULT_MINISLOTS, ExperimentOptions::default())
            .run()
            .unwrap();
        let summary = report.fec_summary().unwrap();
        let verification = report.verification().unwrap();
        let all = report
            .fec_words(FecWordRequest {
                maximum_words: EXPECTED_WORD_CEILING,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(all.words.len(), summary.completed_words);
        let end = report
            .fec_words(FecWordRequest {
                first_word: all.words.len(),
                ..Default::default()
            })
            .unwrap();
        assert!(end.words.is_empty() && end.next_word.is_none());
        assert_eq!(end.copied_decision_bits, 0);
        let first = report.fec_words(FecWordRequest::default()).unwrap();
        if all.words.len() > 1 {
            assert_eq!(first.next_word, Some(1));
        } else {
            assert_eq!(first.next_word, None);
        }
        for request in [
            FecWordRequest {
                grant_index: 1,
                ..Default::default()
            },
            FecWordRequest {
                first_word: usize::MAX,
                ..Default::default()
            },
            FecWordRequest {
                maximum_words: 0,
                ..Default::default()
            },
            FecWordRequest {
                maximum_words: OVER_WORD_CEILING,
                ..Default::default()
            },
            FecWordRequest {
                max_decision_bits: 0,
                ..Default::default()
            },
            FecWordRequest {
                max_decision_bits: OVER_DECISION_CEILING,
                ..Default::default()
            },
            FecWordRequest {
                max_decision_bits: 1,
                ..Default::default()
            },
        ] {
            assert!(report.fec_words(request).is_err());
        }
        assert!(serde_json::from_str::<FecWordRequest>(r#"{"unknown":1}"#).is_err());
        assert_eq!(report.fec_summary().unwrap(), summary);
        assert_eq!(report.verification().unwrap(), verification);
    }
}

#[test]
fn pre_cancel_and_no_transmission_preserve_empty_actual_outcomes_and_absent_metrics() {
    let mut simulation = session(
        Numerology::Khz50,
        DEFAULT_MINISLOTS,
        ExperimentOptions::default(),
    );
    let control = RunControl::new();
    control.cancel();
    let report = simulation.run_with_control(&control).unwrap();
    let verification = report.verification().unwrap();
    let summary = report.fec_summary().unwrap();
    assert_eq!(verification.status, RunStatus::Cancelled);
    assert_eq!(verification.started_grants, 0);
    assert_eq!(verification.not_started_grants, 1);
    assert_eq!(verification.compared_source_bits, 0);
    assert_eq!(verification.ber, None);
    assert!(!verification.reference_recovered);
    assert_eq!(summary.completed_words, 0);
    assert_eq!(summary.maximum_iterations, None);
    assert_eq!(summary.pending_words, summary.planned_words);
    assert!(report.fec_words(Default::default()).is_err());
    const BELOW_TRANSMISSION_MINISLOTS: usize = 1;
    let report = session(
        Numerology::Khz50,
        BELOW_TRANSMISSION_MINISLOTS,
        ExperimentOptions::default(),
    )
    .run()
    .unwrap();
    assert!(!report.planned.grant_plan.transmit);
    assert_eq!(report.verification().unwrap().ber, None);
    assert!(!report.verification().unwrap().reference_recovered);
    assert_eq!(report.fec_summary().unwrap().planned_words, 0);
    assert!(report
        .fec_words(Default::default())
        .unwrap()
        .words
        .is_empty());
}

#[test]
fn actual_cancellation_retains_completed_decoder_words_and_unverified_source_denominators() {
    let mut simulation = session(
        Numerology::Khz50,
        DEFAULT_MINISLOTS,
        ExperimentOptions {
            grants: STOP_TEST_GRANTS,
            chunk_samples: STOP_CHUNK_SAMPLES,
            ..Default::default()
        },
    );
    let control = RunControl::new();
    let worker_control = control.clone();
    let worker = thread::spawn(move || simulation.run_with_control(&worker_control).unwrap());
    let deadline = Instant::now() + Duration::from_secs(STOP_DEADLINE_SECONDS);
    while control.progress().compared_bits == 0 {
        assert!(Instant::now() < deadline);
        thread::yield_now();
    }
    control.cancel();
    let report = worker.join().unwrap();
    let verification = report.verification().unwrap();
    assert_eq!(verification.status, RunStatus::Cancelled);
    assert!(verification.compared_source_bits > 0);
    assert!(verification.compared_source_bits < verification.planned_source_bits);
    assert_eq!(
        verification.unverified_source_bits,
        verification.generated_source_bits - verification.compared_source_bits
    );
    assert_eq!(verification.bit_errors, 0);
    assert!(!verification.reference_recovered);
    let summary = report.fec_summary().unwrap();
    assert!(summary.completed_words > 0 && summary.pending_words > 0);
    assert_eq!(summary.nonconverged_words, 0);
    let page = report.fec_words(Default::default()).unwrap();
    assert_eq!(page.words[0].word, report.grants[0].words[0]);
}

#[test]
fn projection_fixtures_keep_flags_independent_and_reject_inconsistent_dimensions_without_panics() {
    // Deliberately edited flags test projection semantics, not a claimed decoder experiment.
    const PROJECTION_ITERATIONS: usize = 7;
    const PROJECTION_HARD_CHANGES: usize = 3;
    let mut report = session(
        Numerology::Khz50,
        DEFAULT_MINISLOTS,
        ExperimentOptions::default(),
    )
    .run()
    .unwrap();
    let word = &mut report.grants[0].words[0];
    word.decoded.converged = false;
    word.decoded.parity_valid = true;
    word.decoded.shortened_zeros_valid = false;
    word.information_fill_valid = false;
    word.decoded.iterations = PROJECTION_ITERATIONS;
    word.decoded.hard_decision_changes = PROJECTION_HARD_CHANGES;
    report.reference_recovered = false;
    let summary = report.fec_summary().unwrap();
    assert_eq!(summary.nonconverged_words, 1);
    assert_eq!(summary.parity_invalid_words, 0);
    assert_eq!(summary.shortening_invalid_words, 1);
    assert_eq!(summary.fill_invalid_words, 1);
    assert_eq!(summary.maximum_iterations, Some(PROJECTION_ITERATIONS));
    assert_eq!(summary.hard_decision_changes, PROJECTION_HARD_CHANGES);
    assert!(!report.verification().unwrap().reference_recovered);
    let original = report.clone();
    report.grants[0].words[0].decoded.information_bits.pop();
    assert!(report.fec_words(Default::default()).is_err());
    assert!(report.fec_summary().is_err());
    report = original.clone();
    report.grants[0].words[0].ordinal = usize::MAX;
    assert!(report.fec_words(Default::default()).is_err());
    report = original.clone();
    report.planned.grant_plan.codewords[0].information_bits = usize::MAX;
    assert!(report.fec_words(Default::default()).is_err());
    report = original;
    report.unverified_source_bits = usize::MAX;
    assert!(report.verification().is_err());
}

#[test]
fn actual_destructive_channel_preserves_real_failed_outcomes_and_source_errors() {
    let mut configuration = configuration(Numerology::Khz50, DEFAULT_MINISLOTS);
    configuration.channel.taps = vec![Iq::default()];
    let mut simulation = Simulation::new(ExperimentOptions::default()).unwrap();
    simulation.configure(configuration).unwrap();
    let report = simulation.run().unwrap();
    let verification = report.verification().unwrap();
    let summary = report.fec_summary().unwrap();
    assert_eq!(verification.status, RunStatus::Completed);
    assert!(verification.bit_errors > 0);
    assert_eq!(
        verification.generated_source_bits,
        verification.compared_source_bits
    );
    assert!(!verification.reference_recovered);
    assert!(summary.completed_words > 0);
    assert_eq!(summary.pending_words, 0);
    assert!(summary.parity_invalid_words > 0);
    assert!(summary.nonconverged_words > 0);
    let page = report.fec_words(Default::default()).unwrap();
    assert_eq!(page.words[0].word, report.grants[0].words[0]);
}

#[test]
fn original_grant_ids_and_continuous_source_positions_survive_paging_across_grants() {
    const SEQUENTIAL_GRANTS: usize = 2;
    const EXPECTED_BITS_PER_GRANT: usize = 1976;
    const EXPECTED_LAST_WORD_START: u64 = 3236;
    const LAST_SHORT_WORD_ORDINAL: usize = 2;
    const EXPECTED_FIRST_WORD_COPY_BITS: usize = 2800;
    let report = session(
        Numerology::Khz50,
        DEFAULT_MINISLOTS,
        ExperimentOptions {
            grants: SEQUENTIAL_GRANTS,
            ..Default::default()
        },
    )
    .run()
    .unwrap();
    let page = report
        .fec_words(FecWordRequest {
            grant_index: 1,
            max_decision_bits: EXPECTED_FIRST_WORD_COPY_BITS,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(page.identity.grant, GRANT_ID + 1);
    assert_eq!(
        page.words[0].source_start_bit,
        EXPECTED_BITS_PER_GRANT as u64
    );
    assert_eq!(page.copied_decision_bits, EXPECTED_FIRST_WORD_COPY_BITS);
    let last = report
        .fec_words(FecWordRequest {
            grant_index: 1,
            first_word: LAST_SHORT_WORD_ORDINAL,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(last.words[0].source_start_bit, EXPECTED_LAST_WORD_START);
    assert_eq!(last.words[0].word.ordinal, LAST_SHORT_WORD_ORDINAL);
    assert_eq!(
        report.verification().unwrap().compared_source_bits,
        EXPECTED_BITS_PER_GRANT * SEQUENTIAL_GRANTS
    );
}
