/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual public decoder/soft results, original planes and independent transport anchors.
use super::*;
use merlin::upstream::{
    ConfigurationDocument, FecWordRequest, ReceiverObservationPolicy, ReportJson, Simulation,
    SoftCellRequest, SoftWordRequest, MAX_FEC_PAGE_WORDS, MAX_SOFT_WORD_QUERY_LLRS,
};
use serde_json::json;

const SOURCE_50: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");
const SOURCE_25: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-25khz.json");
// Independent original-bit viewport vector; not an application work limit.
const TEST_FIRST_BIT: usize = 7;
const TEST_BIT_COUNT: usize = 11;

fn report(bytes: &[u8], observed: bool) -> BatchReport {
    let document = ConfigurationDocument::from_json(bytes).unwrap();
    let (configuration, options) = document.prepare().unwrap();
    let mut simulation = Simulation::new(options).unwrap();
    simulation.configure(configuration).unwrap();
    if observed {
        simulation
            .set_receiver_observation_policy(
                super::super::observation_policy::instruments().unwrap(),
            )
            .unwrap();
    }
    simulation.run().unwrap()
}

fn input_request(first_bit: usize, maximum_bits: usize) -> FecRequest {
    FecRequest::SoftWord {
        retained_index: 0,
        maximum_llrs: MAX_SOFT_WORD_QUERY_LLRS,
        first_bit,
        maximum_bits,
    }
}

#[test]
fn exact_decoder_selector_modes_reject_unknown_fields_and_negative_indices() {
    // Independent serialized-contract anchors: 262144 decision bits, eight
    // native cells, 4096 cell LLRs and 32768 word LLRs. These literals verify
    // transport separately from the public constants used by production.
    for value in [
        json!({"mode":"words","grant_index":0,"first_word":0,"maximum_words":1,"max_decision_bits":262144}),
        json!({"mode":"soft_cells","first_index":0,"maximum_cells":8,"maximum_llrs":4096}),
        json!({"mode":"soft_word","retained_index":0,"maximum_llrs":32768,"first_bit":7,"maximum_bits":11}),
    ] {
        let request: FecRequest = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(request).unwrap(), value);
        let mut invalid = value;
        invalid["posterior"] = true.into();
        assert!(serde_json::from_value::<FecRequest>(invalid).is_err());
    }
    assert!(Request::parse("fec",r#"{"mode":"words","grant_index":-1,"first_word":0,"maximum_words":1,"max_decision_bits":262144}"#).is_err());
}

#[test]
fn both_numerologies_preserve_original_ldpc_family_shortening_source_and_outcomes() {
    for source in [SOURCE_50, SOURCE_25] {
        let report = report(source, true);
        let before = report.to_json().unwrap();
        let expected = report.fec_words(FecWordRequest::default()).unwrap();
        let actual = fec::project(&report, FecRequest::default()).unwrap();
        assert_eq!(actual["kind"], "fec");
        assert_eq!(actual["status"], "ready");
        assert_eq!(
            actual["metadata"]["verification"],
            serde_json::to_value(report.verification().unwrap()).unwrap()
        );
        assert_eq!(
            actual["metadata"]["fec_summary"],
            serde_json::to_value(report.fec_summary().unwrap()).unwrap()
        );
        let original = &expected.words[0];
        let word = &actual["data"]["words"][0];
        assert_eq!(word["code_family"], original.code_family);
        assert_eq!(word["shortened_bits"], original.shortened_bits);
        assert_eq!(word["mother_codeword_bits"], original.mother_codeword_bits);
        assert_eq!(
            word["source_start_bit"],
            original.source_start_bit.to_string()
        );
        assert_eq!(word["iterations"], original.word.decoded.iterations);
        assert_eq!(word["parity_valid"], original.word.decoded.parity_valid);
        assert_eq!(word["converged"], original.word.decoded.converged);
        assert_eq!(
            word["information_fill_valid"],
            original.word.information_fill_valid
        );
        assert_eq!(
            actual["data"]["next_word"],
            serde_json::to_value(expected.next_word).unwrap()
        );
        assert_eq!(
            actual["data"]["copied_decision_bits"],
            expected.copied_decision_bits
        );
        assert!(word.get("payload").is_none());
        assert!(word.get("mother_codeword").is_none());
        assert!(actual["table"]["headings"]
            .as_array()
            .unwrap()
            .iter()
            .all(|label| !label.as_str().unwrap().contains("BCH")));
        assert_eq!(
            actual["chart"]["points"][0]["count"],
            original.word.decoded.iterations.to_string()
        );
        assert_eq!(report.to_json().unwrap(), before);
    }
}

#[test]
fn actual_destructive_channel_displays_failed_decoder_flags_separately_from_source_errors() {
    let document = ConfigurationDocument::from_json(SOURCE_50).unwrap();
    let (mut configuration, options) = document.prepare().unwrap();
    // The existing public recovery fixture uses a zero channel, destroying
    // received data without editing decoder flags or the generated source.
    configuration.channel.taps = vec![merlin::upstream::Iq::default()];
    let mut simulation = Simulation::new(options).unwrap();
    simulation.configure(configuration).unwrap();
    let report = simulation.run().unwrap();
    let original = report.fec_words(FecWordRequest::default()).unwrap();
    let verification = report.verification().unwrap();
    assert!(verification.bit_errors > 0);
    assert!(!verification.reference_recovered);
    assert!(!original.words[0].word.decoded.converged);
    assert!(!original.words[0].word.decoded.parity_valid);
    let display = fec::project(&report, FecRequest::default()).unwrap();
    let word = &display["data"]["words"][0];
    assert_eq!(word["converged"], false);
    assert_eq!(word["parity_valid"], false);
    assert_eq!(
        word["iterations"],
        original.words[0].word.decoded.iterations
    );
    assert_eq!(display["table"]["rows"][0][3], "Fail");
    assert_eq!(display["table"]["rows"][0][4], "Fail");
    assert_eq!(display["chart"]["points"][0]["attention"], true);
    assert_eq!(
        display["metadata"]["verification"],
        serde_json::to_value(verification).unwrap()
    );
    assert!(word.get("bit_errors").is_none());
}

#[test]
fn actual_cell_page_keeps_raw_and_derandomized_vectors_and_original_coordinates() {
    let report = report(SOURCE_50, true);
    let request = SoftCellRequest {
        maximum_cells: fec::MAX_CELL_VIEW_CELLS,
        ..Default::default()
    };
    let expected = report.soft_cells(request).unwrap();
    assert!(!expected.cells.is_empty());
    let actual = fec::project(
        &report,
        FecRequest::SoftCells {
            first_index: request.first_index,
            maximum_cells: request.maximum_cells,
            maximum_llrs: request.maximum_llrs,
        },
    )
    .unwrap();
    assert_eq!(
        actual["metadata"]["llr_convention"],
        "natural_log_p0_over_p1;positive_favors_zero"
    );
    assert_eq!(actual["metadata"]["bit_order"], "msb_first");
    assert_eq!(
        actual["data"]["next_index"],
        serde_json::to_value(expected.next_index).unwrap()
    );
    let cells = actual["data"]["cells"].as_array().unwrap();
    let mut row = 0;
    for (original, cell) in expected.cells.iter().zip(cells) {
        assert_eq!(cell["frame"], original.frame.to_string());
        assert_eq!(cell["physical_carrier"], original.physical_carrier);
        assert_eq!(
            cell["demapper_llrs"],
            serde_json::to_value(&original.demapper_llrs).unwrap()
        );
        assert_eq!(
            cell["descrambled_llrs"],
            serde_json::to_value(&original.descrambled_llrs).unwrap()
        );
        for bit in 0..original.demapper_llrs.len() {
            let values = &actual["table"]["rows"][row];
            assert_eq!(values[0], original.retained_index.to_string());
            assert_eq!(values[4], bit.to_string());
            assert_eq!(values[5], original.demapper_llrs[bit].to_string());
            assert_eq!(values[6], original.descrambled_llrs[bit].to_string());
            row += 1;
        }
    }
    assert_eq!(row, actual["table"]["rows"].as_array().unwrap().len());
}

#[test]
fn completed_input_view_keeps_exact_values_sign_plane_and_continuation() {
    let report = report(SOURCE_25, true);
    let expected = report.soft_word(SoftWordRequest::default()).unwrap();
    let actual = fec::project(&report, input_request(TEST_FIRST_BIT, TEST_BIT_COUNT)).unwrap();
    assert_eq!(
        actual["data"]["input_llrs"],
        serde_json::to_value(&expected.input_llrs[TEST_FIRST_BIT..TEST_FIRST_BIT + TEST_BIT_COUNT])
            .unwrap()
    );
    assert_eq!(actual["page"]["next"], TEST_FIRST_BIT + TEST_BIT_COUNT);
    assert_eq!(actual["page"]["total"], expected.input_llrs.len());
    assert_eq!(actual["table"]["rows"][0][0], TEST_FIRST_BIT.to_string());
    assert_eq!(actual["metadata"]["code_family"], expected.code_family);
    assert_eq!(
        actual["metadata"]["word"]["completion_frame"],
        expected.word.completion_frame.to_string()
    );
    assert_eq!(
        actual["metadata"]["word"]["shortened_bits"],
        expected.shortened_bits
    );
    assert_eq!(
        actual["metadata"]["llr_convention"],
        "natural_log_p0_over_p1;positive_favors_zero"
    );
    assert!(actual["metadata"]["reference_plane"]
        .as_str()
        .unwrap()
        .contains("not posterior"));
}

#[test]
fn exact_end_and_invalid_windows_preserve_public_and_native_independent_limits() {
    // Literal copy budgets below are independent public-contract test anchors;
    // only the deliberately invalid dimension changes in each request.
    let report = report(SOURCE_50, true);
    let bits = report
        .soft_word(SoftWordRequest::default())
        .unwrap()
        .input_llrs
        .len();
    let end = fec::project(&report, input_request(bits, TEST_BIT_COUNT)).unwrap();
    assert_eq!(end["table"]["rows"], json!([]));
    assert_eq!(end["page"]["next"], Value::Null);
    for request in [
        input_request(bits + 1, TEST_BIT_COUNT),
        input_request(0, 0),
        input_request(0, fec::MAX_LLR_VIEW_ROWS + 1),
        FecRequest::Words {
            grant_index: 0,
            first_word: 0,
            maximum_words: MAX_FEC_PAGE_WORDS + 1,
            max_decision_bits: 262144,
        },
        FecRequest::SoftCells {
            first_index: 0,
            maximum_cells: fec::MAX_CELL_VIEW_CELLS + 1,
            maximum_llrs: 4096,
        },
        FecRequest::SoftWord {
            retained_index: 0,
            maximum_llrs: MAX_SOFT_WORD_QUERY_LLRS + 1,
            first_bit: 0,
            maximum_bits: TEST_BIT_COUNT,
        },
    ] {
        assert!(fec::project(&report, request).is_err(), "{request:?}");
    }
    let first = report.fec_words(FecWordRequest::default()).unwrap();
    let words = fec::project(
        &report,
        FecRequest::Words {
            grant_index: 0,
            first_word: first.total_completed_words,
            maximum_words: 1,
            max_decision_bits: 262144,
        },
    )
    .unwrap();
    assert_eq!(words["table"]["rows"], json!([]));
    assert_eq!(words["page"]["next"], Value::Null);
}

#[test]
fn disabled_soft_history_is_unavailable_even_when_actual_decoding_completed() {
    // The literal 4096-LLR contract budget is valid; disabled history alone must
    // make this request unavailable, independently of a production default.
    let report = report(SOURCE_50, false);
    assert!(report.fec_summary().unwrap().completed_words > 0);
    for request in [
        input_request(0, TEST_BIT_COUNT),
        FecRequest::SoftCells {
            first_index: 0,
            maximum_cells: 1,
            maximum_llrs: 4096,
        },
    ] {
        let value = fec::project(&report, request).unwrap();
        assert_eq!(value["status"], "unavailable");
        assert_eq!(value["table"]["rows"], json!([]));
    }
}

#[test]
fn native_opt_in_preserves_source_recovery_decisions_and_public_disabled_defaults() {
    let plain = report(SOURCE_50, false);
    let observed = report(SOURCE_50, true);
    assert_eq!(
        plain.verification().unwrap(),
        observed.verification().unwrap()
    );
    assert_eq!(
        plain.fec_summary().unwrap(),
        observed.fec_summary().unwrap()
    );
    assert_eq!(
        plain.fec_words(FecWordRequest::default()).unwrap().words[0].word,
        observed.fec_words(FecWordRequest::default()).unwrap().words[0].word
    );
    assert_eq!(plain.soft_observations().retained_cells, 0);
    assert!(observed.soft_observations().retained_cells > 0);
    assert!(observed.soft_observations().retained_words > 0);
    assert!(ReceiverObservationPolicy::default()
        .info()
        .soft_metrics
        .is_none());
}

#[test]
fn wide_decoder_identity_transport_has_independent_lossless_literal_anchors() {
    let actual = fec::identity_value(
        &json!({"run":u64::MAX,"generation":9_007_199_254_740_993_u64,"grant":u64::MAX-1}),
    )
    .unwrap();
    assert_eq!(
        actual,
        json!({"run":"18446744073709551615","generation":"9007199254740993","grant":"18446744073709551614"})
    );
}
