/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded input decoding and compact tree views; no filesystem work in QML.
use docsis_ofdm_fec::{
    input_format::{detect, json_bytes, InputFormat},
    profile::candidate::CapturedProfile,
    scenario::pnm_json::{import_profiles, JsonCapture},
    transmission::TransmissionSettings,
};
use pnm::pnm::parser::{
    cm_ds_ofdm_modulation_profile::CmDsOfdmModulationProfile, parse_limits::ParseLimits,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::Arc;

pub(crate) const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_VISIBLE_SCHEMES: usize = 256;

#[derive(Clone)]
pub(crate) struct Capture {
    pub name: String,
    pub format: &'static str,
    pub original: Arc<Vec<u8>>,
    pub sha256: String,
    pub document: Arc<Value>,
    pub profiles: Arc<Vec<CapturedProfile>>,
}

impl Capture {
    /// Decodes a bounded binary capture or supported decoder JSON without writes.
    /// # Arguments
    /// * `name` - Display identity, never used to detect the representation.
    /// * `bytes` - Original file bytes retained for provenance.
    /// # Returns
    /// Owned capture and compact metadata backed by the shared Rust validator.
    /// # Errors
    /// Rejects size, format, decoder or profile-contract failures.
    pub(crate) fn decode(name: String, bytes: Vec<u8>) -> Result<Self, String> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err("Input exceeds the 16 MiB file limit.".into());
        }
        let (format, decoded) = match detect(&bytes) {
            InputFormat::BinaryPnm => {
                let capture = CmDsOfdmModulationProfile::parse(&bytes, ParseLimits::default())
                    .map_err(|error| format!("Binary PNM validation failed: {error}"))?;
                (
                    "Binary PNM",
                    capture
                        .to_json(MAX_INPUT_BYTES)
                        .map_err(|error| error.to_string())?
                        .into_bytes(),
                )
            }
            InputFormat::JsonText => ("JSON text", json_bytes(&bytes).to_vec()),
            InputFormat::Empty => return Err("The selected file is empty.".into()),
            InputFormat::UnsupportedText => {
                return Err("Unsupported text file. Choose decoded modulation-profile JSON.".into())
            }
            InputFormat::UnsupportedBinary => {
                return Err("Unsupported binary file: no recognized PNM header.".into())
            }
        };
        let mut profiles = import_profiles(JsonCapture {
            name: &name,
            bytes: &decoded,
        })
        .map_err(|error| format!("{format}: {error}"))?;
        profiles.sort_by_key(|profile| profile.profile_id);
        let document: Value =
            serde_json::from_slice(&decoded).map_err(|error| error.to_string())?;
        Ok(Self {
            name,
            format,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            original: Arc::new(bytes),
            document: Arc::new(document),
            profiles: Arc::new(profiles),
        })
    }

    /// Borrows a decoded profile by its captured identity.
    /// # Arguments
    /// * `profile_id` - ID present in the capture.
    /// # Returns
    /// The unchanged decoded profile.
    /// # Errors
    /// Rejects an unavailable profile ID.
    pub(crate) fn profile(&self, profile_id: u8) -> Result<&CapturedProfile, String> {
        self.profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
            .ok_or_else(|| "Choose an available profile.".into())
    }

    /// Checks profile spacing and carrier coverage against accepted numerology.
    /// # Arguments
    /// * `profile_id` - Captured profile selection.
    /// * `settings` - Accepted transmission configuration.
    /// # Returns
    /// Unit when the capture fits the supported channel.
    /// # Errors
    /// Rejects invalid settings, unavailable profiles or incompatible grids.
    pub(crate) fn compatible(
        &self,
        profile_id: u8,
        settings: &TransmissionSettings,
    ) -> Result<(), String> {
        settings.validate().map_err(|error| error.to_string())?;
        let profile = self.profile(profile_id)?;
        let grid = &profile.key.grid;
        if grid.spacing_hz != settings.subcarrier_spacing.frequency_hz() {
            return Err("Capture spacing differs from the accepted OCD settings. Channel-spacing editing is pending; use a compatible capture.".into());
        }
        let end = usize::from(grid.first_subcarrier_index)
            .checked_add(grid.sample_count)
            .ok_or("Profile carrier span overflow.")?;
        if end > settings.subcarrier_spacing.fft_samples() as usize
            || (grid.sample_count as u64) * u64::from(grid.spacing_hz)
                > u64::from(settings.occupied_bandwidth_hz)
        {
            return Err("Profile carrier span does not fit the accepted OCD channel.".into());
        }
        Ok(())
    }

    /// Builds bounded compact tree rows for one selected profile.
    /// # Arguments
    /// * `selected` - Available captured profile ID.
    /// # Returns
    /// Metadata, profile options and at most 256 selected-profile scheme rows.
    /// # Errors
    /// Rejects missing profile IDs or unavailable source scheme metadata.
    pub(crate) fn preview(&self, selected: u8) -> Result<Value, String> {
        self.profile(selected)?;
        let doc = &self.document;
        let mut rows = Vec::new();
        rows.push(branch("file", "File information", "", 0, &[]));
        for (key, label, value) in [
            (
                "capture_type",
                "Capture type",
                format!(
                    "{}{} · Format {}.{}",
                    doc["pnm_header"]["file_type"].as_str().unwrap_or(""),
                    doc["pnm_header"]["file_type_version"],
                    doc["pnm_header"]["major_version"],
                    doc["pnm_header"]["minor_version"]
                ),
            ),
            ("channel", "Channel ID", doc["channel_id"].to_string()),
            (
                "mac",
                "CM MAC address",
                doc["mac_address"].as_str().unwrap_or("").into(),
            ),
        ] {
            rows.push(leaf(
                key,
                label,
                &value,
                1,
                &["file"],
                "Original capture metadata. Input bytes remain unchanged.",
            ));
        }
        rows.push(branch("channel_info", "Channel information", "", 0, &[]));
        let grid = &self.profiles[0].key.grid;
        for (key, label, value) in [
            (
                "zero",
                "Subcarrier zero frequency",
                format!("{} MHz", f64::from(grid.zero_frequency_hz) / 1e6),
            ),
            (
                "first",
                "First active subcarrier index",
                grid.first_subcarrier_index.to_string(),
            ),
            (
                "spacing",
                "Subcarrier spacing",
                format!("{} kHz", grid.spacing_hz / 1000),
            ),
        ] {
            rows.push(leaf(
                key,
                label,
                &value,
                1,
                &["channel_info"],
                "Exact coordinates from the PNM capture, separate from DPD MAC message fields.",
            ));
        }
        rows.push(branch(
            "profiles",
            "Profiles",
            &format!("{} available", self.profiles.len()),
            0,
            &[],
        ));
        for profile in self.profiles.iter() {
            let id = profile.profile_id;
            let key = format!("profile_{id}");
            let mut node = branch(
                &key,
                &format!("Profile {id}"),
                &format!("{} assignments", profile.assignments.len()),
                1,
                &["profiles"],
            );
            node["profile_id"] = json!(id);
            rows.push(node);
            if id != selected {
                continue;
            }
            let raw = doc["profiles"]
                .as_array()
                .and_then(|profiles| {
                    profiles
                        .iter()
                        .find(|profile| profile["profile_id"].as_u64() == Some(u64::from(id)))
                })
                .ok_or("Profile source metadata is unavailable.")?;
            let schemes = raw["schemes"]
                .as_array()
                .ok_or("Profile schemes are unavailable.")?;
            rows.push(leaf(
                "schemes",
                "Assignment schemes",
                &schemes.len().to_string(),
                2,
                &["profiles", &key],
                "Compact range schemes from the capture. These are not invented DPD TLVs.",
            ));
            for (index, scheme) in schemes.iter().take(MAX_VISIBLE_SCHEMES).enumerate() {
                let label = format!("Range {}", index + 1);
                let order = scheme["modulation_order"].as_str().unwrap_or("unknown");
                let assignment =
                    docsis_ofdm_fec::types::modulation::ModulationAssignment::from_code(
                        scheme["modulation_order_code"].as_u64().unwrap_or(255) as u8,
                    );
                let value = format!(
                    "{} · {} subcarriers",
                    assignment.display_label, scheme["num_subcarriers"]
                );
                let description = format!("{label}: {order}, code {}, bit loading {}, role {}. CM-OSSI 3.1 I27 §D.2.1.7.3.1. Range numbering follows capture order; it is not a physical carrier index.", scheme["modulation_order_code"], scheme["bit_loading"], scheme["carrier_type"]);
                rows.push(leaf(
                    &format!("range_{index}"),
                    &label,
                    &value,
                    2,
                    &["profiles", &key],
                    &description,
                ));
            }
            if schemes.len() > MAX_VISIBLE_SCHEMES {
                rows.push(leaf(
                    "limit",
                    "Display limit",
                    &format!("Showing {MAX_VISIBLE_SCHEMES} of {} schemes", schemes.len()),
                    2,
                    &["profiles", &key],
                    "Only the displayed rows are limited; the full capture is validated.",
                ));
            }
        }
        rows.push(branch("provenance", "Input provenance", "", 0, &[]));
        for (key, label, value) in [
            ("format", "Detected format", self.format.into()),
            (
                "bytes",
                "Original input size",
                format!("{} bytes", self.original.len()),
            ),
            (
                "epoch",
                "Capture time (Unix seconds)",
                doc["pnm_header"]["capture_time"].to_string(),
            ),
            ("hash", "Original input SHA-256", self.sha256.clone()),
        ] {
            rows.push(leaf(
                key,
                label,
                &value,
                1,
                &["provenance"],
                "Exact original-input provenance, separate from decoded-JSON provenance.",
            ));
        }
        Ok(
            json!({"source_name": self.name, "format": self.format, "profile_ids": self.profiles.iter().map(|profile| profile.profile_id).collect::<Vec<_>>(), "selected_profile_id": selected, "rows": rows}),
        )
    }
}

fn branch(key: &str, label: &str, value: &str, depth: u8, ancestors: &[&str]) -> Value {
    json!({"key": key, "label": label, "value": value, "depth": depth, "ancestors": ancestors, "branch": true, "description": "Choose a profile for the next run. Imported capture bytes remain unchanged."})
}
fn leaf(
    key: &str,
    label: &str,
    value: &str,
    depth: u8,
    ancestors: &[&str],
    description: &str,
) -> Value {
    json!({"key": key, "label": label, "value": value, "depth": depth, "ancestors": ancestors, "branch": false, "description": description})
}
