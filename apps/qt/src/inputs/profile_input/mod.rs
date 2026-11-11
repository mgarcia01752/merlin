/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Profile draft/accepted state and independent project restoration.
pub(crate) mod document;
pub(crate) mod plot;
use docsis_ofdm_fec::transmission::TransmissionSettings;
use document::Capture;
#[cfg(test)]
use serde_json::json;
use serde_json::Value;
/// Profile editor state retains original bytes independently of Qt.
#[derive(Default)]
pub struct ProfileModel {
    pub(crate) draft: Option<Capture>,
    pub(crate) accepted: Option<(Capture, u8)>,
}

/// Parses and validates a bounded OCD document for profile admission.
/// # Arguments
/// * `document` - Accepted OCD JSON, limited to4096 bytes.
/// # Returns
/// Validated transmission settings.
/// # Errors
/// Rejects oversized, malformed or unsupported settings.
pub(crate) fn settings(document: &str) -> Result<TransmissionSettings, String> {
    if document.len() > 4096 {
        return Err("OCD request exceeds 4096 bytes.".into());
    }
    let settings: TransmissionSettings =
        serde_json::from_str(document).map_err(|error| error.to_string())?;
    settings.validate().map_err(|error| error.to_string())?;
    Ok(settings)
}

impl ProfileModel {
    /// Reconstructs profile editor state from an embedded original capture.
    /// # Arguments
    /// * `value` - Saved file and selected profile ID, or null.
    /// # Returns
    /// Independent draft and accepted snapshots.
    /// # Errors
    /// Rejects malformed saved data, capture bytes or unavailable profile IDs.
    pub(crate) fn from_saved(value: &Value) -> Result<Self, String> {
        if value.is_null() {
            return Ok(Self::default());
        }
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Saved {
            file: Value,
            profile_id: u8,
        }
        let saved: Saved = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        let (name, bytes) = crate::workbench::project_file::decode(&saved.file)?;
        let capture = Capture::decode(name, bytes)?;
        capture.profile(saved.profile_id)?;
        Ok(Self {
            draft: Some(capture.clone()),
            accepted: Some((capture, saved.profile_id)),
        })
    }

    /// Decodes an original capture before replacing the current draft.
    /// # Arguments
    /// * `name` - Display basename retained with the original bytes.
    /// * `bytes` - Bounded original binary PNM or supported JSON.
    /// # Returns
    /// Compact preview for profile0 when present, otherwise the first profile.
    /// # Errors
    /// Preserves the draft when decoding or preview admission fails.
    pub(crate) fn stage_bytes(&mut self, name: String, bytes: Vec<u8>) -> Result<Value, String> {
        let capture = Capture::decode(name, bytes)?;
        let id = capture
            .profiles
            .iter()
            .find(|profile| profile.profile_id == 0)
            .map_or(capture.profiles[0].profile_id, |profile| profile.profile_id);
        let preview = capture.preview(id)?;
        self.draft = Some(capture);
        Ok(preview)
    }
    /// Accepts a compatible draft profile or clears an absent draft.
    /// # Arguments
    /// * `id` - Selected profile identifier.
    /// * `document` - Accepted OCD JSON.
    /// # Returns
    /// Unit after updating the accepted snapshot.
    /// # Errors
    /// Preserves the accepted profile on invalid selection or OCD mismatch.
    pub(crate) fn accept(&mut self, id: i32, document: &str) -> Result<(), String> {
        if let Some(capture) = &self.draft {
            let id = u8::try_from(id).map_err(|_| "Choose a valid profile ID.")?;
            capture.compatible(id, &settings(document)?)?;
            self.accepted = Some((capture.clone(), id));
        } else {
            self.accepted = None;
        }
        Ok(())
    }
}

/// Decodes an embedded original profile without mutating current state.
/// # Arguments
/// * `value` - Original bytes and selected profile ID.
/// * `_settings` - Current numerology; cross-input errors remain readiness diagnostics.
/// # Returns
/// Typed profile, or None when unconfigured.
/// # Errors
/// Reports malformed bytes, digest or unavailable profile IDs.
pub(crate) fn restore_profile(
    value: &Value,
    _settings: &TransmissionSettings,
) -> Result<Option<docsis_ofdm_fec::profile::candidate::CapturedProfile>, String> {
    let model = ProfileModel::from_saved(value)?;
    model
        .accepted
        .map(|(capture, id)| capture.profile(id).cloned())
        .transpose()
}
#[cfg(test)]
mod tests {
    use super::*;
    const BINARY: &[u8] = include_bytes!(
        "../../../../../examples/measurements/single-channel/modulation_profile.bin"
    );
    const JSON: &[u8] = include_bytes!(
        "../../../../../examples/measurements/single-channel/modulation_profile.json"
    );

    #[test]
    fn binary_and_json_use_the_same_profile_validation() {
        let binary = Capture::decode("wrong.json".into(), BINARY.to_vec()).unwrap();
        let json = Capture::decode("wrong.bin".into(), JSON.to_vec()).unwrap();
        assert_eq!(binary.format, "Binary PNM");
        assert_eq!(json.format, "JSON text");
        for (left, right) in binary.profiles.iter().zip(json.profiles.iter()) {
            assert_eq!(left.profile_id, right.profile_id);
            assert_eq!(left.key, right.key);
            assert_eq!(left.assignments, right.assignments);
        }
        assert_eq!(binary.profiles.len(), 4);
        assert_eq!(binary.profiles[0].assignments.len(), 7480);
    }
    #[test]
    fn profile_plot_preserves_source_grid_roles_and_bounds() {
        let binary = Capture::decode("original.bin".into(), BINARY.to_vec()).unwrap();
        let decoded = Capture::decode("decoded.json".into(), JSON.to_vec()).unwrap();
        let full = plot::viewport(&binary, 0, 0, 0).unwrap();
        assert_eq!(full, plot::viewport(&decoded, 0, 0, 0).unwrap());
        assert!(full["points"].as_array().unwrap().len() <= 512 * 7);
        assert!(full["roles"]
            .as_array()
            .unwrap()
            .iter()
            .any(|role| role["kind"] == "continuous_pilot"));
        let zoomed = plot::viewport(&binary, 0, 20, 4).unwrap();
        let profile = binary.profile(0).unwrap();
        for point in zoomed["points"].as_array().unwrap() {
            let index = point["bin"].as_u64().unwrap() as usize
                - usize::from(profile.key.grid.first_subcarrier_index);
            assert!((20..24).contains(&index));
            assert_eq!(
                point["frequency_hz"],
                profile.key.grid.frequency(index).unwrap()
            );
            assert_eq!(point["bits"], json!(profile.assignments[index].bit_loading));
        }
        assert_eq!(zoomed["points"][0]["x"], 0.0);
        assert_eq!(zoomed["points"][3]["x"], 1.0);
        assert!(plot::viewport(&binary, 255, 0, 0).is_err());
        assert!(plot::viewport(&binary, 0, usize::MAX, 2).is_err());
        assert!(plot::viewport(&binary, 0, 7479, 2).is_err());
        let rows = binary.preview(0).unwrap();
        assert!(!rows["rows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["key"] == "source"));
        assert_eq!(binary.name, "original.bin");
        let mut missing = binary.clone();
        let mut profile = binary.profile(0).unwrap().clone();
        profile.assignments[0].carrier_type =
            docsis_ofdm_fec::types::modulation::CarrierType::Exclusion;
        profile.assignments[0].bit_loading = None;
        missing.profiles = std::sync::Arc::new(vec![profile]);
        let marker = plot::viewport(&missing, 0, 0, 2).unwrap();
        assert!(marker["points"][0]["bits"].is_null());
        assert_eq!(marker["points"][0]["role"], "exclusion");
        assert_eq!(marker["role_markers"], true);
    }
    #[test]
    fn staged_failures_removal_and_profile_rejection_preserve_accepted_input() {
        let mut model = ProfileModel::default();
        model
            .stage_bytes("demo.bin".into(), BINARY.to_vec())
            .unwrap();
        let config = json!(TransmissionSettings::default()).to_string();
        model.accept(0, &config).unwrap();
        assert!(model
            .stage_bytes("bad.bin".into(), b"not JSON".to_vec())
            .is_err());
        assert_eq!(model.accepted.as_ref().unwrap().1, 0);
        assert!(model.accept(99, &config).is_err());
        let mut mismatched = json!(TransmissionSettings::default());
        mismatched["subcarrier_spacing"] = json!("50000");
        assert!(model.accept(0, &mismatched.to_string()).is_err());
        assert_eq!(model.accepted.as_ref().unwrap().1, 0);
        model.draft = None;
        assert!(model.accepted.is_some());
        model.accept(-1, &config).unwrap();
        assert!(model.accepted.is_none());
    }
}
