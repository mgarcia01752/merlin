/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! In-memory transmission editor; the core owns validation and timing.
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use docsis_ofdm_fec::transmission::TransmissionSettings;
use serde_json::json;

const REGISTRY: &str = include_str!("../../../resources/registry/ofdm-settings.json");

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

fn parse(document: &str) -> Result<TransmissionSettings, String> {
    if document.len() > 4096 {
        return Err("Settings document exceeds 4096 bytes.".into());
    }
    let settings: TransmissionSettings =
        serde_json::from_str(document).map_err(|error| error.to_string())?;
    settings.validate().map_err(|error| error.to_string())?;
    Ok(settings)
}

impl OfdmSettingsRust {
    fn accept(&mut self, document: &str) -> Result<(), String> {
        self.accepted = parse(document)?;
        Ok(())
    }
}

impl bridge::OfdmSettings {
    /// Returns the immutable bundled registry for the five editable fields.
    pub fn registry(&self) -> QString {
        QString::from(REGISTRY)
    }
    /// Serializes the accepted configuration; no filesystem access occurs.
    pub fn current(&self) -> QString {
        QString::from(json!(self.rust().accepted).to_string())
    }
    /// Serializes core defaults without changing the accepted configuration.
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
        let result = parse(&document.to_string()).and_then(|settings| {
            let timing = settings
                .one_second_timing()
                .map_err(|error| error.to_string())?;
            let period_us =
                f64::from(timing.symbol_period_samples) * 1e6 / f64::from(timing.sample_rate_hz);
            Ok(json!({"summary": format!(
                "Sample rate: 204.8 Msamples/s\nFFT size: {}\nSymbol period including CP: {} µs",
                timing.fft_samples, period_us
            )}))
        });
        QString::from(match result {
            Ok(value) => value.to_string(),
            Err(error) => json!({"error": error}).to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejected_edits_preserve_accepted_settings() {
        let mut model = OfdmSettingsRust::default();
        let original = model.accepted.clone();
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
        assert!(model.accept(&" ".repeat(4097)).is_err());
        assert_eq!(model.accepted, original);
        let mut draft = json!(original);
        draft["subcarrier_spacing"] = json!("50000");
        draft["time_interleaver_depth"] = json!(32);
        model.accept(&draft.to_string()).unwrap();
        assert_eq!(model.accepted.subcarrier_spacing.fft_samples(), 4096);
        assert_eq!(model.accepted.time_interleaver_depth, 32);
    }

    #[test]
    fn registry_defaults_and_choices_match_core() {
        let registry: serde_json::Value = serde_json::from_str(REGISTRY).unwrap();
        let defaults = json!(TransmissionSettings::default());
        assert_eq!(registry["fields"].as_array().unwrap().len(), 5);
        for field in registry["fields"].as_array().unwrap() {
            let id = field["id"].as_str().unwrap();
            assert_eq!(field["application_default"], defaults[id]);
            for choice in field["choices"].as_array().unwrap() {
                let mut draft = defaults.clone();
                draft["subcarrier_spacing"] = json!("50000");
                draft[id] = choice["value"].clone();
                assert!(parse(&draft.to_string()).is_ok(), "{id}: {choice}");
            }
        }
    }
}
