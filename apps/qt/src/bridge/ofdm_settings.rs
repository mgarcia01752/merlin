/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! In-memory transmission editor; the core owns validation and timing.
use crate::shared::presentation::constants::SAMPLES_PER_MEGASAMPLE;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use merlin::downstream::{
    TransmissionConfiguration, TransmissionSettings, MAX_TRANSMISSION_DOCUMENT_BYTES,
};
use serde_json::json;

const REGISTRY: &str = include_str!("../../../../resources/registry/ofdm-settings.json");

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type OfdmSettings = super::OfdmSettingsRust;

        /// Returns bundled field metadata as JSON for presentation.
        #[qinvokable]
        fn registry(self: &OfdmSettings) -> QString;
        /// Returns the accepted in-memory settings as JSON.
        #[qinvokable]
        fn current(self: &OfdmSettings) -> QString;
        /// Returns core application defaults as JSON without modifying state.
        #[qinvokable]
        fn defaults(self: &OfdmSettings) -> QString;
        /// Validates a bounded JSON draft and atomically replaces accepted state.
        /// # Arguments
        /// * `document` - Five-field transmission settings document.
        /// # Returns
        /// Empty on success; a diagnostic on rejection, preserving previous state.
        #[qinvokable]
        fn apply(self: Pin<&mut OfdmSettings>, document: &QString) -> QString;
        /// Returns derived timing or an error as JSON without modifying state.
        /// # Arguments
        /// * `document` - Draft transmission settings document.
        /// # Returns
        /// Display timing values from the core or an error field.
        #[qinvokable]
        fn describe(self: &OfdmSettings, document: &QString) -> QString;
    }
}

/// Accepted transmission configuration, independent of transient dialog drafts.
#[derive(Default)]
pub struct OfdmSettingsRust {
    accepted: TransmissionSettings,
}

fn parse(document: &str) -> Result<TransmissionConfiguration, String> {
    if document.len() > MAX_TRANSMISSION_DOCUMENT_BYTES {
        return Err(format!(
            "Settings document exceeds {MAX_TRANSMISSION_DOCUMENT_BYTES} bytes."
        ));
    }
    TransmissionConfiguration::from_json(document.as_bytes())
        .map_err(|cause| cause.message().to_owned())
}

impl OfdmSettingsRust {
    fn accept(&mut self, document: &str) -> Result<(), String> {
        self.accepted = parse(document)?.settings().clone();
        Ok(())
    }

    fn description(document: &str) -> serde_json::Value {
        let result = parse(document).and_then(|configuration| {
            let timing = configuration.symbol_timing(None)
                .map_err(|cause| cause.message().to_owned())?;
            let clock = configuration.settings().one_second_timing()
                .map_err(|cause| cause.to_string())?;
            let sample_rate_millions = f64::from(clock.sample_rate_hz) / SAMPLES_PER_MEGASAMPLE;
            Ok(json!({"summary": format!(
                "Sample rate: {sample_rate_millions} Msamples/s\nFFT size: {}\nSymbol period including CP: {} µs",
                timing.fft_samples, timing.symbol_us
            )}))
        });
        match result {
            Ok(value) => value,
            Err(error) => json!({"error":error}),
        }
    }
}

impl bridge::OfdmSettings {
    /// Returns the immutable bundled registry for the five editable fields.
    /// # Returns
    /// Owned presentation metadata; this does not validate or execute a simulation.
    pub fn registry(&self) -> QString {
        QString::from(REGISTRY)
    }
    /// Serializes the accepted configuration; no filesystem access occurs.
    /// # Returns
    /// Owned unchanged five-field JSON from the last accepted public configuration.
    pub fn current(&self) -> QString {
        QString::from(json!(self.rust().accepted).to_string())
    }
    /// Serializes core defaults without changing the accepted configuration.
    /// # Returns
    /// Owned existing downstream defaults; no source, receiver or live state changes.
    pub fn defaults(&self) -> QString {
        QString::from(json!(TransmissionSettings::default()).to_string())
    }
    /// Applies only valid complete settings; rejection preserves accepted state.
    /// # Arguments
    /// * `document` - Bounded JSON configuration from the draft editor.
    /// # Returns
    /// Empty on success or a human-readable diagnostic.
    pub fn apply(mut self: core::pin::Pin<&mut Self>, document: &QString) -> QString {
        match self.as_mut().rust_mut().accept(&document.to_string()) {
            Ok(()) => QString::from(""),
            Err(error) => QString::from(error),
        }
    }
    /// Adapts core timing results to presentation units without changing settings.
    /// # Arguments
    /// * `document` - Draft configuration to validate and describe.
    /// # Returns
    /// JSON with a summary, or an error when the draft is invalid.
    pub fn describe(&self, document: &QString) -> QString {
        QString::from(OfdmSettingsRust::description(&document.to_string()).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejected_edits_preserve_accepted_settings() {
        let mut model = OfdmSettingsRust::default();
        let original = model.accepted.clone();
        // Independent invalid fixtures: RP equals the default CP, 32 branches
        // exceed the 25 kHz maximum, and 25 MHz is not a 6 MHz bandwidth step.
        for change in [
            json!({"roll_off_samples":512}),
            json!({"time_interleaver_depth":32}),
            json!({"occupied_bandwidth_hz":25000000}),
        ] {
            let mut draft = json!(original);
            for (key, value) in change.as_object().unwrap() {
                draft[key] = value.clone();
            }
            assert!(model.accept(&draft.to_string()).is_err());
            assert_eq!(model.accepted, original);
        }
        assert!(model.accept("{}").is_err());
        // Independent admission anchor: one byte beyond the existing 4096-byte limit.
        let oversized_draft = " ".repeat(4097);
        assert_eq!(
            model.accept(&oversized_draft),
            Err("Settings document exceeds 4096 bytes.".to_owned())
        );
        assert_eq!(model.accepted, original);
        let mut draft = json!(original);
        // At 50 kHz, the historical editor admits 32 interleaver branches.
        draft["subcarrier_spacing"] = json!("50000");
        draft["time_interleaver_depth"] = json!(32);
        model.accept(&draft.to_string()).unwrap();
        // A 50 kHz carrier grid at the downstream clock has 4096 FFT samples.
        assert_eq!(model.accepted.subcarrier_spacing.fft_samples(), 4096);
        assert_eq!(model.accepted.time_interleaver_depth, 32);
    }

    #[test]
    fn registry_defaults_and_choices_match_core() {
        let registry: serde_json::Value = serde_json::from_str(REGISTRY).unwrap();
        let defaults = json!(TransmissionSettings::default());
        // The persisted editor contract contains exactly five settings fields.
        assert_eq!(registry["fields"].as_array().unwrap().len(), 5);
        for field in registry["fields"].as_array().unwrap() {
            let id = field["id"].as_str().unwrap();
            assert_eq!(field["application_default"], defaults[id]);
            for choice in field["choices"].as_array().unwrap() {
                let mut draft = defaults.clone();
                // The 50 kHz fixture admits every registered depth choice.
                draft["subcarrier_spacing"] = json!("50000");
                draft[id] = choice["value"].clone();
                assert!(parse(&draft.to_string()).is_ok(), "{id}: {choice}");
            }
        }
    }

    #[test]
    fn summaries_preserve_both_numerologies_without_accepting_drafts() {
        let model = OfdmSettingsRust::default();
        let original = model.accepted.clone();
        // Independent timing anchors: 512 CP samples add 2.5 us to either FFT.
        // The fixture fixes 16 interleaver branches and 128 roll-off samples.
        for (spacing, expected) in [
            ("25000", "Sample rate: 204.8 Msamples/s\nFFT size: 8192\nSymbol period including CP: 42.5 µs"),
            ("50000", "Sample rate: 204.8 Msamples/s\nFFT size: 4096\nSymbol period including CP: 22.5 µs"),
        ] {
            let draft = json!({
                "occupied_bandwidth_hz": 192_000_000,
                "subcarrier_spacing": spacing,
                "cyclic_prefix_samples": 512,
                "roll_off_samples": 128,
                "time_interleaver_depth": 16,
            });
            assert_eq!(OfdmSettingsRust::description(&draft.to_string()), json!({"summary": expected}));
            assert_eq!(model.accepted, original);
        }
        assert!(OfdmSettingsRust::description("{}")["error"].is_string());
        assert_eq!(model.accepted, original);
    }
}
