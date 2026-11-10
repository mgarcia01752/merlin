/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded presentation coordinates for independent recovered-data comparisons.
use docsis_ofdm_fec::simulation::channel::ChannelJobResult;
use serde_json::{json, Value};

fn bars(entries: Vec<(String, u64, bool)>) -> Value {
    let maximum = entries.iter().map(|entry| entry.1).max().unwrap_or(0);
    let scale = maximum.max(1) as f64;
    let points: Vec<_> = entries
        .into_iter()
        .map(|(label, count, attention)| {
            json!({"label":label,"count":count.to_string(),"fraction":count as f64/scale,"attention":attention})
        })
        .collect();
    json!({"points":points,"maximum":maximum.to_string()})
}

/// Formats recovery totals and actual per-word errors without deriving a time history.
///
/// # Arguments
/// * `job` - Immutable completed or partial job with bounded retained word details.
///
/// # Returns
/// Two normalized count charts, at most 64 word bars and an explicit omitted count.
/// Zero bars remain zero; unavailable comparisons are marked rather than fabricated.
pub(super) fn charts(job: &ChannelJobResult) -> Value {
    let recovery = bars(vec![
        ("Recovered".into(), job.summary.recovered_payloads, false),
        (
            "Unrecovered".into(),
            job.unrecovered_generated_payloads,
            true,
        ),
    ]);
    let words = bars(
        job.retained_words
            .iter()
            .take(64)
            .filter(|word| word.comparison.bits_tested > 0)
            .map(|word| {
                (
                    word.receive_sequence.to_string(),
                    word.comparison.bit_errors,
                    word.comparison.bit_errors > 0,
                )
            })
            .collect(),
    );
    json!({"recovery":recovery,"word_errors":words,
        "compared":job.summary.comparison.bits_tested>0,
        "uncompared_retained":job.retained_words.iter().take(64).filter(|word| word.comparison.bits_tested==0).count(),
        "omitted":job.omitted_word_details.saturating_add(job.retained_words.len().saturating_sub(64) as u64)})
}

#[cfg(test)]
mod tests {
    use super::bars;

    #[test]
    fn zero_counts_and_mixed_outcomes_keep_honest_coordinates() {
        let zero = bars(vec![("Recovered".into(), 0, false)]);
        assert_eq!(zero["points"][0]["fraction"], 0.0);
        assert_eq!(zero["maximum"], "0");
        let outcomes = bars(vec![
            ("Recovered".into(), 12, false),
            ("Unrecovered".into(), 3, true),
        ]);
        assert_eq!(outcomes["points"][0]["fraction"], 1.0);
        assert_eq!(outcomes["points"][1]["fraction"], 0.25);
        assert_eq!(outcomes["points"][1]["count"], "3");
        assert_eq!(outcomes["points"][1]["attention"], true);
    }
}
