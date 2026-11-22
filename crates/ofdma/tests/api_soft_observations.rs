/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual soft-plane contracts with independent BPSK/mask and code-dimension anchors.
mod common;
use docsis_ofdma::api::*;

const SOURCE_PRBS15_SEED: u16 = 1;
const PROFILE_ID: u16 = 224;
const DATA_IUC: u8 = 5;
const MASK_IMPULSE_SEED: u32 = 0x02_0000;
const DATA_LOADING: u8 = 4;
const FIRST_50KHZ_PILOT_PATTERN: u8 = 1;
const FIRST_25KHZ_PILOT_PATTERN: u8 = 8;
const GRANT_FRAMES: usize = 2;
const SHORT_GRANT_MINISLOTS: usize = 8;
const MEDIUM_GRANT_MINISLOTS: usize = 24;
const LONG_GRANT_MINISLOTS: usize = 64;
const REFERENCE_VARIANCE: f64 = 0.5;
const POWER_FLOOR: f64 = 1e-12;
const ITERATION_BUDGET: usize = 50;
const SOFT_CELL_BUDGET: usize = 1024;
const SOFT_WORD_BUDGET: usize = 16;
const WORD_INPUT_BUDGET: usize = 262_144;
const CELL_PAGE_COUNT: usize = 128;
const QUERY_SCALAR_BUDGET: usize = 4096;
const NUMERICAL_TOLERANCE: f64 = 1e-10;
const ACTUAL_NOISE_VARIANCE: f64 = 1e-4;

fn configuration(mode: Numerology, slots: usize) -> ExperimentConfig {
    let pattern = match mode {
        Numerology::Khz50 => FIRST_50KHZ_PILOT_PATTERN,
        Numerology::Khz25 => FIRST_25KHZ_PILOT_PATTERN,
    };
    let base = common::configuration(mode, DATA_LOADING, pattern, GRANT_FRAMES, slots);
    let profile = Profile::new(
        PROFILE_ID,
        DATA_IUC,
        MASK_IMPULSE_SEED,
        base.selected_profile().minislots().to_vec(),
    )
    .unwrap();
    let us = UsConfiguration::new(
        base.waveform().clone(),
        vec![profile],
        base.grant().clone(),
        base.excluded().to_vec(),
    )
    .unwrap();
    ExperimentConfig::new(
        us,
        SOURCE_PRBS15_SEED,
        ReceiverConfig::new(
            Estimator::FrameFlatLs,
            REFERENCE_VARIANCE,
            POWER_FLOOR,
            ITERATION_BUDGET,
        )
        .unwrap(),
    )
    .unwrap()
}
fn session(config: ExperimentConfig) -> Simulation {
    let mut session = Simulation::new(Default::default()).unwrap();
    session.configure(config).unwrap();
    session
}
fn soft_policy(cells: usize, words: usize, llrs: usize) -> ReceiverObservationPolicy {
    ReceiverObservationPolicy::default()
        .with_soft_metrics(SoftObservationPolicy::new(cells, words, llrs).unwrap())
}
fn all_cells(report: &BatchReport) -> Vec<SoftCellSnapshot> {
    let mut output = Vec::new();
    let mut first_index = 0;
    loop {
        let page = report
            .soft_cells(SoftCellRequest {
                first_index,
                maximum_cells: CELL_PAGE_COUNT,
                maximum_llrs: QUERY_SCALAR_BUDGET,
            })
            .unwrap();
        output.extend(page.cells);
        let Some(next) = page.next_index else {
            return output;
        };
        first_index = next;
    }
}

#[test]
fn actual_cells_preserve_independent_bpsk_scale_mask_positions_and_exact_ldpc_input_both_modes() {
    // Figure-7 impulse seed 0x020000 gives these independently derived mask-one positions.
    const MASK_PREFIX_BITS: usize = 64;
    const MASK_ONES: &[usize] = &[0, 5, 18, 28, 36, 41, 46, 51, 54];
    // Unit BPSK at complex variance 0.5 gives LLR=4*I/0.5=8*I before clipping.
    const EXPECTED_BPSK_LLR_PER_REAL_UNIT: f64 = 8.0;
    const EXPECTED_GRANT_CAPACITY: usize = 2820;
    // Four 25-kHz minislots retain the same 64-carrier width as eight 50-kHz minislots.
    // Per frame: 364 ordinary cells and ten complementary cells give 1466 bits.
    const FULL_CAPTURE_25KHZ_MINISLOTS: usize = 4;
    const EXPECTED_25KHZ_GRANT_CAPACITY: usize = 2932;
    const EXPECTED_25KHZ_COMPUTED_CELLS: usize = 748;
    const EXPECTED_WORD_STARTS: &[usize] = &[0, 1120, 1820];
    const EXPECTED_WORD_LENGTHS: &[usize] = &[1120, 700, 1000];
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let slots = match mode {
            Numerology::Khz50 => SHORT_GRANT_MINISLOTS,
            Numerology::Khz25 => FULL_CAPTURE_25KHZ_MINISLOTS,
        };
        let mut session = session(configuration(mode, slots));
        session
            .set_receiver_observation_policy(soft_policy(
                SOFT_CELL_BUDGET,
                SOFT_WORD_BUDGET,
                WORD_INPUT_BUDGET,
            ))
            .unwrap();
        let report = session.run().unwrap();
        assert!(report.reference_recovered);
        assert_eq!(report.receiver_observations().retained_frames, 0);
        let cells = all_cells(&report);
        let mut raw = Vec::new();
        let mut restored = Vec::new();
        for cell in &cells {
            assert_eq!(cell.coded_offset, raw.len());
            assert_eq!(cell.demapper_llrs.len(), usize::from(cell.loading));
            assert_eq!(cell.descrambled_llrs.len(), usize::from(cell.loading));
            assert_eq!(cell.reference_variance, REFERENCE_VARIANCE);
            if cell.loading == 1 {
                let expected = cell.corrected.unwrap().re * EXPECTED_BPSK_LLR_PER_REAL_UNIT;
                assert!((cell.demapper_llrs[0] - expected).abs() < NUMERICAL_TOLERANCE);
            }
            raw.extend_from_slice(&cell.demapper_llrs);
            restored.extend_from_slice(&cell.descrambled_llrs);
        }
        let summary = report.soft_observations();
        match mode {
            Numerology::Khz50 => {
                assert_eq!(raw.len(), EXPECTED_GRANT_CAPACITY);
                assert_eq!(summary.omitted_cells, 0);
                assert_eq!(summary.retained_words, EXPECTED_WORD_LENGTHS.len());
            }
            Numerology::Khz25 => {
                assert_eq!(summary.computed_cells, EXPECTED_25KHZ_COMPUTED_CELLS);
                assert_eq!(summary.retained_cells, EXPECTED_25KHZ_COMPUTED_CELLS);
                assert_eq!(summary.omitted_cells, 0);
                assert_eq!(raw.len(), EXPECTED_25KHZ_GRANT_CAPACITY);
            }
        }
        for index in 0..MASK_PREFIX_BITS {
            let expected = if MASK_ONES.contains(&index) {
                -raw[index]
            } else {
                raw[index]
            };
            assert_eq!(restored[index], expected);
        }
        assert_eq!(summary.retained_cells, cells.len());
        let mut fully_compared_words = 0;
        for (index, entry) in summary.words.iter().enumerate() {
            let word = report
                .soft_word(SoftWordRequest {
                    retained_index: index,
                    ..Default::default()
                })
                .unwrap();
            assert_eq!(&word.word, entry);
            if mode == Numerology::Khz50 {
                assert_eq!(word.word.coded_offset, EXPECTED_WORD_STARTS[index]);
                assert_eq!(word.input_llrs.len(), EXPECTED_WORD_LENGTHS[index]);
            }
            if entry.coded_offset + word.transmitted_bits <= restored.len() {
                assert_eq!(
                    word.input_llrs,
                    restored[entry.coded_offset..entry.coded_offset + word.transmitted_bits]
                );
                fully_compared_words += 1;
            }
            let outcome = &report.grants[0].words[entry.word_ordinal];
            assert_eq!(word.iterations, outcome.decoded.iterations);
            assert_eq!(word.parity_valid, outcome.decoded.parity_valid);
            assert_eq!(word.converged, outcome.decoded.converged);
            assert_eq!(word.information_fill_valid, outcome.information_fill_valid);
        }
        assert_eq!(fully_compared_words, summary.words.len());
    }
}

#[test]
fn actual_soft_words_retain_long_medium_short_dimensions_shortening_and_failed_outcomes() {
    for (slots, expected_family, expected_mother_bits) in [
        (SHORT_GRANT_MINISLOTS, "upstream_short", 1120),
        (MEDIUM_GRANT_MINISLOTS, "upstream_medium", 5940),
        (LONG_GRANT_MINISLOTS, "upstream_long", 16200),
    ] {
        let mut session = session(configuration(Numerology::Khz50, slots));
        session
            .set_receiver_observation_policy(soft_policy(0, SOFT_WORD_BUDGET, WORD_INPUT_BUDGET))
            .unwrap();
        let report = session.run().unwrap();
        let first = report.soft_word(Default::default()).unwrap();
        assert_eq!(first.code_family, expected_family);
        assert_eq!(first.mother_codeword_bits, expected_mother_bits);
        assert_eq!(first.input_llrs.len(), first.transmitted_bits);
        assert_eq!(
            first.shortened_bits,
            first.mother_information_bits - first.retained_information_bits
        );
        assert_eq!(
            first.input_llrs.len(),
            first.mother_codeword_bits - first.shortened_bits
        );
        assert_eq!(report.soft_observations().computed_cells, 0);
    }
    let mut config = configuration(Numerology::Khz50, SHORT_GRANT_MINISLOTS);
    config.channel.taps = vec![Iq::default()];
    let mut session = session(config);
    session
        .set_receiver_observation_policy(soft_policy(
            SOFT_CELL_BUDGET,
            SOFT_WORD_BUDGET,
            WORD_INPUT_BUDGET,
        ))
        .unwrap();
    let report = session.run().unwrap();
    assert!(!report.reference_recovered);
    let cells = all_cells(&report);
    assert!(cells.iter().all(|cell| cell.corrected.is_none()
        && cell.demapper_llrs.iter().all(|value| *value == 0.0)
        && cell.descrambled_llrs.iter().all(|value| *value == 0.0)));
    for entry in report.soft_observations().words {
        let word = report
            .soft_word(SoftWordRequest {
                retained_index: entry.retained_index,
                ..Default::default()
            })
            .unwrap();
        let actual = &report.grants[0].words[entry.word_ordinal];
        assert_eq!(word.converged, actual.decoded.converged);
        assert_eq!(word.parity_valid, actual.decoded.parity_valid);
        assert_eq!(
            word.shortened_zeros_valid,
            actual.decoded.shortened_zeros_valid
        );
        assert!(word.input_llrs.iter().all(|value| *value == 0.0));
    }
}

#[test]
fn independent_budgets_pages_and_owned_results_preserve_original_identities() {
    const RETAINED_CELL_LIMIT: usize = 3;
    const RETAINED_WORD_LIMIT: usize = 1;
    const EXACT_FIRST_WORD_LLRS: usize = 1120;
    const OVER_CELL_PAGE_LIMIT: usize = 129;
    const OVER_CELL_SCALAR_LIMIT: usize = 4097;
    const OVER_WORD_QUERY_LIMIT: usize = 32769;
    let mut session = session(configuration(Numerology::Khz50, SHORT_GRANT_MINISLOTS));
    session
        .set_receiver_observation_policy(soft_policy(
            RETAINED_CELL_LIMIT,
            RETAINED_WORD_LIMIT,
            EXACT_FIRST_WORD_LLRS,
        ))
        .unwrap();
    let report = session.run().unwrap();
    let summary = report.soft_observations();
    assert_eq!(summary.retained_cells, RETAINED_CELL_LIMIT);
    assert!(summary.omitted_cells > 0);
    assert_eq!(summary.retained_words, RETAINED_WORD_LIMIT);
    assert_eq!(summary.retained_word_llrs, EXACT_FIRST_WORD_LLRS);
    assert!(summary.omitted_words > 0);
    let original = report.soft_cells(Default::default()).unwrap();
    let word = report.soft_word(Default::default()).unwrap();
    assert_eq!(word.word.identity, report.grants[0].identity);
    assert!(report
        .soft_cells(SoftCellRequest {
            first_index: RETAINED_CELL_LIMIT,
            ..Default::default()
        })
        .unwrap()
        .cells
        .is_empty());
    for request in [
        SoftCellRequest {
            maximum_cells: 0,
            ..Default::default()
        },
        SoftCellRequest {
            maximum_cells: OVER_CELL_PAGE_LIMIT,
            ..Default::default()
        },
        SoftCellRequest {
            maximum_llrs: OVER_CELL_SCALAR_LIMIT,
            ..Default::default()
        },
        SoftCellRequest {
            maximum_llrs: 1,
            ..Default::default()
        },
        SoftCellRequest {
            first_index: usize::MAX,
            ..Default::default()
        },
    ] {
        assert!(report.soft_cells(request).is_err());
    }
    for request in [
        SoftWordRequest {
            maximum_llrs: 0,
            ..Default::default()
        },
        SoftWordRequest {
            maximum_llrs: OVER_WORD_QUERY_LIMIT,
            ..Default::default()
        },
        SoftWordRequest {
            maximum_llrs: 1,
            ..Default::default()
        },
        SoftWordRequest {
            retained_index: usize::MAX,
            ..Default::default()
        },
    ] {
        assert!(report.soft_word(request).is_err());
    }
    assert_eq!(report.soft_cells(Default::default()).unwrap(), original);
    drop(session);
    drop(report);
    assert_eq!(word.input_llrs.len(), EXACT_FIRST_WORD_LLRS);
}

#[test]
fn optional_soft_policy_preserves_defaults_estimate_budgets_and_actual_noisy_science() {
    const ESTIMATE_FRAME_BUDGET: usize = 1;
    const ESTIMATE_SCALAR_BUDGET: usize = 4096;
    const EXHAUSTED_CELL_BUDGET: usize = 1;
    const EXHAUSTED_WORD_BUDGET: usize = 1;
    const INSUFFICIENT_COMPLETE_WORD_LLRS: usize = 1;
    let mut config = configuration(Numerology::Khz50, SHORT_GRANT_MINISLOTS);
    config.channel.noise_variance = ACTUAL_NOISE_VARIANCE;
    let mut session = session(config);
    let original = session.run().unwrap();
    let policy =
        ReceiverObservationPolicy::new(ESTIMATE_FRAME_BUDGET, ESTIMATE_SCALAR_BUDGET).unwrap();
    let info = serde_json::to_value(policy.info()).unwrap();
    assert!(info.get("soft_metrics").is_none());
    assert!(original.soft_cells(Default::default()).is_err());
    assert!(original.soft_word(Default::default()).is_err());
    assert_eq!(original.soft_observations().completed_words, 0);
    for soft in [
        SoftObservationPolicy::new(SOFT_CELL_BUDGET, SOFT_WORD_BUDGET, WORD_INPUT_BUDGET).unwrap(),
        SoftObservationPolicy::new(
            EXHAUSTED_CELL_BUDGET,
            EXHAUSTED_WORD_BUDGET,
            INSUFFICIENT_COMPLETE_WORD_LLRS,
        )
        .unwrap(),
    ] {
        session
            .set_receiver_observation_policy(policy.with_soft_metrics(soft))
            .unwrap();
        let observed = session.run().unwrap();
        assert_eq!(
            observed.receiver_observations().retained_frames,
            ESTIMATE_FRAME_BUDGET
        );
        assert_eq!(observed.grants[0].words, original.grants[0].words);
        assert_eq!(
            observed.grants[0].matched_data_mer,
            original.grants[0].matched_data_mer
        );
        assert_eq!(observed.bit_errors, original.bit_errors);
        assert_eq!(observed.ber, original.ber);
    }
    let disabled = policy.with_soft_metrics(SoftObservationPolicy::default());
    assert_eq!(disabled, policy);
}

#[test]
fn invalid_retention_budgets_and_never_started_runs_preserve_unavailable_states() {
    const OVER_RETAINED_CELL_LIMIT: usize = 1025;
    const OVER_RETAINED_WORD_LIMIT: usize = 33;
    const OVER_RETAINED_WORD_LLRS: usize = 262_145;
    for (cells, words, llrs) in [
        (OVER_RETAINED_CELL_LIMIT, 0, 0),
        (0, OVER_RETAINED_WORD_LIMIT, WORD_INPUT_BUDGET),
        (0, 1, OVER_RETAINED_WORD_LLRS),
        (0, 0, 1),
        (0, 1, 0),
    ] {
        assert!(SoftObservationPolicy::new(cells, words, llrs).is_err());
    }
    let mut session = session(configuration(Numerology::Khz50, SHORT_GRANT_MINISLOTS));
    session
        .set_receiver_observation_policy(soft_policy(
            SOFT_CELL_BUDGET,
            SOFT_WORD_BUDGET,
            WORD_INPUT_BUDGET,
        ))
        .unwrap();
    let control = RunControl::new();
    control.cancel();
    let report = session.run_with_control(&control).unwrap();
    assert_eq!(report.status, RunStatus::Cancelled);
    let summary = report.soft_observations();
    assert_eq!(summary.computed_cells, 0);
    assert_eq!(summary.completed_words, 0);
    assert!(summary.words.is_empty());
    assert!(report.soft_word(Default::default()).is_err());
    assert!(report
        .soft_cells(Default::default())
        .unwrap()
        .cells
        .is_empty());
}

#[test]
fn a_later_complete_word_can_fit_without_renumbering_original_coded_positions() {
    const RETAINED_WORD_LIMIT: usize = 2;
    const SMALLER_SECOND_WORD_LLRS: usize = 700;
    const ORIGINAL_SECOND_WORD_ORDINAL: usize = 1;
    const ORIGINAL_SECOND_WORD_CODED_OFFSET: usize = 1120;
    let mut session = session(configuration(Numerology::Khz50, SHORT_GRANT_MINISLOTS));
    session
        .set_receiver_observation_policy(soft_policy(
            0,
            RETAINED_WORD_LIMIT,
            SMALLER_SECOND_WORD_LLRS,
        ))
        .unwrap();
    let report = session.run().unwrap();
    assert!(report.reference_recovered);
    let summary = report.soft_observations();
    assert_eq!(summary.retained_words, 1);
    let word = report.soft_word(Default::default()).unwrap();
    assert_eq!(word.word.retained_index, 0);
    assert_eq!(word.word.word_ordinal, ORIGINAL_SECOND_WORD_ORDINAL);
    assert_eq!(word.word.coded_offset, ORIGINAL_SECOND_WORD_CODED_OFFSET);
    assert_eq!(word.input_llrs.len(), SMALLER_SECOND_WORD_LLRS);
    assert_eq!(
        word.parity_valid,
        report.grants[0].words[ORIGINAL_SECOND_WORD_ORDINAL]
            .decoded
            .parity_valid
    );
}
