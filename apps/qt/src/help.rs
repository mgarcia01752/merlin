/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded offline help; numerical context comes from validated core timing.
use docsis_ofdm_fec::{
    simulation::channel::ChannelRunSettings, transmission::DurationTiming, SimulationError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::OnceLock;

const CATALOG: &str = include_str!("../../../resources/help/diagnostics.json");
const FAQ: &str = include_str!("../../../docs/user/faq.md");
const PLAYBACK_CODE: &str = "PlaybackObservationTooShort";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    schema_version: u32,
    entries: Vec<Entry>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    code: String,
    title: String,
    faq_heading: String,
    faq_anchor: String,
    summary: String,
    context: String,
    suggestion: String,
    over_limit_suggestion: String,
    note: String,
}
/// Transient presentation context, never part of a saved workbench.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Diagnostic {
    code: String,
    title: String,
    tooltip: String,
    faq_anchor: String,
    current_duration_ms: u64,
    final_transition_symbol: u64,
    final_transition_ms: f64,
    minimum_duration_ms: u64,
    maximum_duration_ms: u64,
}
/// Admission failure preserving original wording and optional stable-code help.
#[derive(Debug)]
pub(crate) struct AdmissionError {
    pub message: String,
    pub diagnostic: Option<Box<Diagnostic>>,
}
impl AdmissionError {
    /// Preserves a stable core code for static help without numeric context.
    /// # Arguments
    /// * `error` - Original scientific validation failure.
    /// # Returns
    /// Same message and known catalog guidance, or no help for an unknown code.
    pub(crate) fn classified(error: SimulationError) -> Self {
        Self {
            message: error.to_string(),
            diagnostic: catalog_diagnostic(error.code).map(Box::new),
        }
    }
    /// Attaches help only to a recognized core diagnostic, without revalidating captures.
    /// # Arguments
    /// * `error` - Original core failure; its stable code selects the help entry.
    /// * `settings` - Frozen run settings used by the failed admission.
    /// * `span` - Last transition symbol/milliseconds returned by the core playback plan.
    /// # Returns
    /// Original message with optional computed help; unavailable help never alters admission.
    pub(crate) fn from_run(
        error: SimulationError,
        settings: &ChannelRunSettings,
        span: (u64, f64),
    ) -> Self {
        let diagnostic = if error.code == PLAYBACK_CODE {
            settings
                .transmission
                .duration_timing(settings.duration)
                .ok()
                .and_then(|timing| {
                    let current_ms = u64::try_from(settings.duration.as_millis()).ok()?;
                    playback_help(timing, span, current_ms)
                })
        } else {
            catalog_diagnostic(error.code)
        };
        Self {
            message: error.to_string(),
            diagnostic: diagnostic.map(Box::new),
        }
    }
    /// Adds profile identity without discarding a known diagnostic or its timing context.
    /// # Arguments
    /// * `profile` - Captured profile ID that failed sweep admission.
    /// # Returns
    /// Same failure with a profile-prefixed original message.
    pub(crate) fn with_profile(mut self, profile: u8) -> Self {
        self.message = format!("Profile {profile}: {}", self.message);
        self
    }
}
impl From<String> for AdmissionError {
    /// Preserves an unclassified message without guessing a help entry.
    /// # Arguments
    /// * `message` - Original admission failure.
    /// # Returns
    /// Failure without contextual help.
    fn from(message: String) -> Self {
        Self {
            message,
            diagnostic: None,
        }
    }
}
impl From<&str> for AdmissionError {
    /// Copies an unclassified admission failure.
    /// # Arguments
    /// * `message` - Original failure text.
    /// # Returns
    /// Owned failure without contextual help.
    fn from(message: &str) -> Self {
        message.to_owned().into()
    }
}
impl std::fmt::Display for AdmissionError {
    /// Writes the preserved admission message.
    /// # Arguments
    /// * `formatter` - Destination formatter.
    /// # Returns
    /// Result of writing the original message.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}
fn catalog() -> Option<&'static Catalog> {
    static PARSED: OnceLock<Option<Catalog>> = OnceLock::new();
    PARSED
        .get_or_init(|| {
            if CATALOG.len() > 65536 {
                return None;
            }
            let parsed: Catalog = serde_json::from_str(CATALOG).ok()?;
            (parsed.schema_version == 1 && parsed.entries.len() <= 128).then_some(parsed)
        })
        .as_ref()
}
fn entry(code: &str) -> Option<&'static Entry> {
    if code.len() > 96 {
        return None;
    }
    catalog()?.entries.iter().find(|entry| entry.code == code)
}
fn playback_help(timing: DurationTiming, span: (u64, f64), current_ms: u64) -> Option<Diagnostic> {
    let entry = entry(PLAYBACK_CODE)?;
    if span.0 == 0 || !span.1.is_finite() || span.0 < timing.complete_symbols {
        return None;
    }
    // The engine requires complete_symbols > last_transition, not merely equal elapsed time.
    let ticks = (u128::from(span.0) + 1) * u128::from(timing.symbol_period_samples);
    let minimum_ms =
        u64::try_from((ticks * 1000).div_ceil(u128::from(timing.sample_rate_hz))).ok()?;
    let maximum_ms = crate::simulation::closed_loop::MAX_DURATION_MS;
    let values = [
        ("{final_transition_ms}", format!("{:.3}", span.1)),
        ("{final_transition_symbol}", span.0.to_string()),
        ("{current_duration_ms}", current_ms.to_string()),
        ("{minimum_duration_ms}", minimum_ms.to_string()),
        ("{maximum_duration_ms}", maximum_ms.to_string()),
    ];
    let render = |template: &str| {
        values
            .iter()
            .fold(template.to_owned(), |text, (key, value)| {
                text.replace(key, value)
            })
    };
    let suggestion = if minimum_ms <= maximum_ms {
        &entry.suggestion
    } else {
        &entry.over_limit_suggestion
    };
    Some(Diagnostic {
        code: entry.code.clone(),
        title: entry.title.clone(),
        faq_anchor: entry.faq_anchor.clone(),
        tooltip: format!(
            "{}\n{}\n{}\n{}",
            entry.summary,
            render(&entry.context),
            render(suggestion),
            entry.note
        ),
        current_duration_ms: current_ms,
        final_transition_symbol: span.0,
        final_transition_ms: span.1,
        minimum_duration_ms: minimum_ms,
        maximum_duration_ms: maximum_ms,
    })
}
/// Returns a whitelisted bundled FAQ section or the complete offline index.
/// # Arguments
/// * `code` - Empty for the index, or a recognized diagnostic code; never a file path.
/// # Returns
/// Supplies static catalog help for a known state without inventing numeric context.
/// # Arguments
/// * `code` - Exact registered diagnostic code.
/// # Returns
/// Bounded tooltip/link metadata, or null for an unknown state.
pub(crate) fn static_diagnostic(code: &str) -> Value {
    match entry(code) {
        Some(entry) => json!({"code":entry.code,"title":entry.title,"faq_anchor":entry.faq_anchor,
            "tooltip":format!("{}\n{}\n{}",entry.summary,entry.suggestion,entry.note)}),
        None => Value::Null,
    }
}
/// Builds static known-error guidance without inventing measurement context.
pub(crate) fn catalog_diagnostic(code: &str) -> Option<Diagnostic> {
    let entry = entry(code)?;
    Some(Diagnostic {
        code: entry.code.clone(),
        title: entry.title.clone(),
        faq_anchor: entry.faq_anchor.clone(),
        tooltip: format!("{}\n{}\n{}", entry.summary, entry.suggestion, entry.note),
        current_duration_ms: 0,
        final_transition_symbol: 0,
        final_transition_ms: 0.0,
        minimum_duration_ms: 0,
        maximum_duration_ms: 0,
    })
}
/// Bounded title/Markdown JSON. Unknown keys report unavailable help without I/O.
pub(crate) fn faq_document(code: &str) -> Value {
    if FAQ.len() <= 65536 {
        if code.is_empty() {
            return json!({"title":"MERLIN FAQ", "text":FAQ});
        }
        if let Some(entry) = entry(code) {
            let heading = format!("## {}\n", entry.faq_heading);
            if let Some((_, tail)) = FAQ.split_once(&heading) {
                let section = tail.split_once("\n## ").map_or(tail, |(body, _)| body);
                return json!({"title":entry.title,"text":section.trim(),"anchor":entry.faq_anchor});
            }
        }
    }
    json!({"title":"FAQ Unavailable", "text":"No bundled FAQ entry is available for this message.", "error":true})
}

#[cfg(test)]
mod tests {
    use super::*;
    use docsis_ofdm_fec::transmission::{SubcarrierSpacing, TransmissionSettings};
    use std::time::Duration;

    fn help(last: u64, spacing: SubcarrierSpacing, prefix: u16) -> Diagnostic {
        let transmission = TransmissionSettings {
            subcarrier_spacing: spacing,
            cyclic_prefix_samples: prefix,
            ..Default::default()
        };
        let timing = transmission
            .duration_timing(Duration::from_millis(3))
            .unwrap();
        let milliseconds = last as f64 * f64::from(timing.symbol_period_samples) * 1000.0
            / f64::from(timing.sample_rate_hz);
        playback_help(timing, (last, milliseconds), 3).unwrap()
    }
    #[test]
    fn unavailable_qam_help_is_registered_and_unknown_states_stay_unclassified() {
        let diagnostic = static_diagnostic("QamRunUnavailable");
        assert_eq!(diagnostic["code"], "QamRunUnavailable");
        assert!(diagnostic["tooltip"]
            .as_str()
            .unwrap()
            .contains("Configure WorkBench"));
        assert!(faq_document("QamRunUnavailable")["text"]
            .as_str()
            .unwrap()
            .contains("Run Profile"));
        assert!(static_diagnostic("UnknownQamState").is_null());
    }
    #[test]
    fn catalog_targets_are_unique_and_bundled() {
        let mut codes = std::collections::BTreeSet::new();
        let mut anchors = std::collections::BTreeSet::new();
        for entry in &catalog().unwrap().entries {
            assert!(codes.insert(&entry.code));
            assert!(anchors.insert(&entry.faq_anchor));
            assert!(FAQ.contains(&format!("## {}\n", entry.faq_heading)));
            assert_eq!(
                entry.faq_heading.to_lowercase().replace(' ', "-"),
                entry.faq_anchor
            );
            assert!(faq_document(&entry.code)["error"].is_null());
        }
        assert!(faq_document("")["text"].as_str().unwrap().contains("## "));
        assert_eq!(faq_document("/etc/passwd")["error"], true);
    }
    #[test]
    fn minimum_duration_obeys_whole_symbol_boundaries() {
        for (last, spacing, prefix, expected) in [
            (1000, SubcarrierSpacing::Khz25, 512, 43),
            (400, SubcarrierSpacing::Khz25, 512, 18),
            (2000, SubcarrierSpacing::Khz25, 512, 86),
            (1000, SubcarrierSpacing::Khz50, 512, 23),
            (1000, SubcarrierSpacing::Khz25, 1024, 46),
        ] {
            let diagnostic = help(last, spacing, prefix);
            assert_eq!(diagnostic.minimum_duration_ms, expected);
            assert!(!diagnostic.tooltip.contains('{'));
            let transmission = TransmissionSettings {
                subcarrier_spacing: spacing,
                cyclic_prefix_samples: prefix,
                ..Default::default()
            };
            assert!(
                transmission
                    .duration_timing(Duration::from_millis(expected))
                    .unwrap()
                    .complete_symbols
                    > last
            );
            assert!(
                transmission
                    .duration_timing(Duration::from_millis(expected - 1))
                    .unwrap()
                    .complete_symbols
                    <= last
            );
        }
    }
    #[test]
    fn over_limit_help_never_recommends_an_invalid_ui_duration() {
        let diagnostic = help(100000, SubcarrierSpacing::Khz25, 512);
        assert_eq!(diagnostic.minimum_duration_ms, 4251);
        assert!(diagnostic.tooltip.contains("above the 1000 ms"));
        assert!(!diagnostic.tooltip.contains("Set Configure"));
        assert!(help(u64::MAX, SubcarrierSpacing::Khz25, 512).minimum_duration_ms > 1000);
    }
    #[test]
    fn unknown_errors_keep_original_text_and_no_guessed_help() {
        let transmission = TransmissionSettings::default();
        let layout = docsis_ofdm_fec::transmission::layout::CarrierLayoutSettings {
            zero_frequency_hz: 0,
            first_active_bin: 296,
            active_end_exclusive: 7896,
            plc_first_bin: 1000,
            excluded_bins: vec![],
            additional_continuous_pilots: vec![],
        };
        let settings = ChannelRunSettings::new(transmission, layout);
        let error = AdmissionError::from_run(
            SimulationError::new(
                "Unknown",
                "Playback final transition exceeds the observation window.",
            ),
            &settings,
            (1000, 42.5),
        );
        assert_eq!(
            error.message,
            "Playback final transition exceeds the observation window."
        );
        assert!(error.diagnostic.is_none());
    }
}
