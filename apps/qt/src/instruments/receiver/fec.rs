/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded FEC summaries and graph coordinates over immutable simulation results.
use crate::simulation::closed_loop::ResultData;
use serde_json::{json, Value};

fn allocation(entries: &[(&str, &str, u64)], total: u64) -> Value {
    json!({"total":total,"segments":entries.iter().map(|(key,label,count)|json!({
        "key":key,"label":label,"count":count,
        "fraction":if total==0 {0.0}else{*count as f64 / total as f64}
    })).collect::<Vec<_>>()})
}

fn iterations(counts: &[u64]) -> Value {
    let Some(first) = counts.iter().position(|count| *count > 0) else {
        return json!({"points":[],"maximum":0});
    };
    let last = counts.iter().rposition(|count| *count > 0).unwrap();
    let maximum = *counts[first..=last].iter().max().unwrap();
    json!({"maximum":maximum,"first":first,"last":last,
        "points":counts[first..=last].iter().enumerate().map(|(offset,count)|json!({
            "iteration":first+offset,"count":count,"fraction":*count as f64 / maximum as f64
        })).collect::<Vec<_>>()})
}

fn recovery(
    generated: u64,
    fec: &docsis_ofdm_fec::simulation::channel::results::FecCounters,
    pending: u64,
) -> Value {
    let mismatched = fec.undetected_errors();
    let recovered = fec
        .total_codewords()
        .saturating_sub(fec.uncorrectable_codewords())
        .saturating_sub(mismatched);
    allocation(
        &[
            ("clean", "Recovered", recovered),
            ("failed", "Rejected", fec.uncorrectable_codewords()),
            ("mismatch", "Mismatch", mismatched),
            ("pending", "Unrecovered", pending),
        ],
        generated,
    )
}

/// Builds a bounded FEC snapshot for one completed or interrupted profile run.
/// # Arguments
/// * `result` - Immutable result with actual decoder counters and retained records.
/// # Returns
/// Observation outcomes, whole-run recovery/iterations and at most64 word records.
/// Empty denominators yield zero graph widths, not synthetic successful outcomes.
pub(crate) fn snapshot(result: &ResultData) -> Value {
    let summary = &result.job.summary;
    let observation = summary.window.as_ref().map(|window| window.fec.snapshot());
    let outcomes = observation.map(|fec| {
        allocation(
            &[
                ("clean", "Clean", fec.clean_codewords()),
                ("corrected", "Corrected", fec.corrected_codewords()),
                ("failed", "Uncorrectable", fec.uncorrectable_codewords()),
            ],
            fec.total_codewords(),
        )
    });
    let recovery = recovery(
        summary.generated_payloads,
        &summary.fec,
        result.job.unrecovered_generated_payloads,
    );
    let recovered = recovery["segments"][0]["count"].as_u64().unwrap();
    json!({
        "available":true,"profile_id":result.job.settings.playback.as_ref().map(|p|p.profile.profile_id),
        "status":result.job.status,"fec":observation,"outcomes":outcomes,
        "recovery":recovery,"recovered":recovered,
        "iterations":iterations(&summary.payload_ldpc_iterations),
        "mean_iterations":summary.payload_mean_ldpc_iterations,
        "mean_iterations_text":summary.payload_mean_ldpc_iterations.map(|mean|format!("{mean:.3}")),
        "payloads":summary.recovered_payloads,"generated":summary.generated_payloads,
        "ldpc_failures":summary.payload_ldpc_failures,"bch_failures":summary.payload_bch_failures,
        "bch_corrected_bits":summary.bch_corrected_bits,"codeword_errors":summary.payload_codeword_errors,
        "ncp_fields":summary.accepted_ncp_fields,"ncp_iterations":summary.ncp_ldpc_iterations,
        "plc_frames":summary.plc_frames,"plc_iterations":summary.plc_ldpc_iterations,
        "plc_bit_errors":summary.plc_bit_errors,"compared_bits":summary.comparison.bits_tested,
        "bit_errors":summary.comparison.bit_errors,"ber":summary.payload_ber,
        "unrecovered":result.job.unrecovered_generated_payloads,"words":result.job.retained_words,
        "omitted_words":result.job.omitted_word_details,"error":result.job.error,
        "verification_charts":super::verification::charts(&result.job)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_preserves_internal_gaps_and_trims_unused_range() {
        let chart = iterations(&[0, 121, 0, 4, 0, 0]);
        assert_eq!(chart["first"], 1);
        assert_eq!(chart["last"], 3);
        assert_eq!(chart["points"].as_array().unwrap().len(), 3);
        assert_eq!(chart["points"][1]["count"], 0);
        assert_eq!(chart["points"][2]["count"], 4);
        assert_eq!(chart["points"][0]["fraction"], 1.0);
        assert_eq!(iterations(&[0; 101])["points"], json!([]));
        assert_eq!(iterations(&[3])["points"][0]["iteration"], 0);
    }

    #[test]
    fn recovery_never_counts_rejected_or_source_mismatched_words_as_success() {
        use docsis_ofdm_fec::simulation::channel::results::{FecCounters, FecOutcome};
        let mut fec = FecCounters::new();
        fec.record(FecOutcome::Corrected, false);
        fec.record(FecOutcome::Uncorrectable, true);
        fec.record(FecOutcome::Clean, true);
        let chart = recovery(5, &fec, 2);
        let counts: Vec<u64> = chart["segments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|segment| segment["count"].as_u64().unwrap())
            .collect();
        assert_eq!(counts, vec![1, 1, 1, 2]);
        assert_eq!(counts.iter().sum::<u64>(), 5);
    }

    #[test]
    fn allocations_keep_failures_distinct_and_empty_denominators_finite() {
        let chart = allocation(&[("clean", "Recovered", 7), ("failed", "Rejected", 3)], 10);
        assert_eq!(chart["segments"][0]["fraction"], 0.7);
        assert_eq!(chart["segments"][1]["fraction"], 0.3);
        assert_eq!(
            allocation(&[("clean", "Recovered", 0)], 0)["segments"][0]["fraction"],
            0.0
        );
    }
}
