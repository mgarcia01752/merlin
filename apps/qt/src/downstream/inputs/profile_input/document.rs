/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded input decoding and compact tree views; no filesystem work in QML.
use crate::{
    downstream::common::constants::{
        PROFILE_TREE_CHILD_DEPTH, PROFILE_TREE_DETAIL_DEPTH, PROFILE_TREE_ROOT_DEPTH,
        UTF8_JSON_BYTE_ORDER_MARK,
    },
    shared::presentation::constants::{HERTZ_PER_KILOHERTZ, HERTZ_PER_MEGAHERTZ},
};
use merlin::downstream::{
    inputs::CaptureLoader, CaptureKind, NamedCapture, ProfileCapture, SourceProfileFrame,
    TransmissionConfiguration, TransmissionSettings, MAX_CAPTURE_BYTES, MAX_PROFILE_SCHEME_ROWS,
};
use serde_json::{json, Value};
use std::sync::Arc;

/// Chooser byte bound comes from the authoritative public capture-admission owner.
pub(crate) const MAX_INPUT_BYTES: usize = MAX_CAPTURE_BYTES;

#[derive(Clone)]
pub(crate) struct Capture {
    pub name: String,
    pub format: &'static str,
    pub original: NamedCapture,
    pub sha256: String,
    pub catalog: ProfileCapture,
    pub document: Arc<Value>,
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
        let original =
            NamedCapture::new(&name, bytes).map_err(|cause| cause.message().to_owned())?;
        let catalog = CaptureLoader::profile_capture(&original)
            .map_err(|cause| cause.message().to_owned())?;

        // This read-only JSON view preserves original presentation text after scientific
        // admission. UTF-8 transport handling does not reinterpret profile assignments.
        let decoded = catalog.provenance().decoded_bytes();
        let document = serde_json::from_slice(
            decoded
                .strip_prefix(UTF8_JSON_BYTE_ORDER_MARK)
                .unwrap_or(decoded),
        )
        .map_err(|cause| cause.to_string())?;
        let format = match catalog.metadata().original_kind {
            CaptureKind::BinaryPnm => "Binary PNM",
            CaptureKind::DecodedJson => "JSON text",
        };
        Ok(Self {
            name,
            format,
            sha256: catalog.provenance().original_sha256().to_owned(),
            original,
            catalog,
            document: Arc::new(document),
        })
    }

    /// Borrows a decoded profile by its captured identity.
    /// # Arguments
    /// * `profile_id` - ID present in the capture.
    /// # Returns
    /// The unchanged decoded profile.
    /// # Errors
    /// Rejects an unavailable profile ID.
    pub(crate) fn profile(&self, profile_id: u8) -> Result<&SourceProfileFrame, String> {
        self.catalog
            .frame(profile_id)
            .map_err(|_| "Choose an available profile.".into())
    }

    /// Copies original frames in sorted UI order for the existing owned session snapshot.
    /// # Returns
    /// Independent profile DTOs; original catalog/source-order metadata remain immutable.
    /// # Errors
    /// Preserves an unavailable identity error before any session publication.
    pub(crate) fn owned_profiles(&self) -> Result<Vec<SourceProfileFrame>, String> {
        self.catalog
            .profile_ids()
            .iter()
            .map(|id| self.profile(*id).cloned())
            .collect()
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
        TransmissionConfiguration::new(settings.clone())
            .and_then(|configuration| {
                configuration.check_profile_capture(&self.catalog, profile_id)
            })
            .map_err(|cause| cause.message().to_owned())
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
        let metadata = self.catalog.metadata();
        let mut rows = Vec::new();
        rows.push(branch(
            "file",
            "File information",
            "",
            PROFILE_TREE_ROOT_DEPTH,
            &[],
        ));
        for (key, label, value) in [
            (
                "capture_type",
                "Capture type",
                format!(
                    "{}{} · Format {}.{}",
                    metadata.header.file_type,
                    metadata.header.file_type_version,
                    metadata.header.major_version,
                    metadata.header.minor_version
                ),
            ),
            ("channel", "Channel ID", metadata.key.channel_id.to_string()),
            (
                "mac",
                "CM MAC address",
                self.document["mac_address"]
                    .as_str()
                    .unwrap_or("")
                    .to_owned(),
            ),
        ] {
            rows.push(leaf(
                key,
                label,
                &value,
                PROFILE_TREE_CHILD_DEPTH,
                &["file"],
                "Original capture metadata. Input bytes remain unchanged.",
            ));
        }
        rows.push(branch(
            "channel_info",
            "Channel information",
            "",
            PROFILE_TREE_ROOT_DEPTH,
            &[],
        ));
        let grid = &metadata.key.grid;
        for (key, label, value) in [
            (
                "zero",
                "Subcarrier zero frequency",
                format!(
                    "{} MHz",
                    f64::from(grid.zero_frequency_hz) / HERTZ_PER_MEGAHERTZ
                ),
            ),
            (
                "first",
                "First active subcarrier index",
                grid.first_subcarrier_index.to_string(),
            ),
            (
                "spacing",
                "Subcarrier spacing",
                format!("{} kHz", f64::from(grid.spacing_hz) / HERTZ_PER_KILOHERTZ),
            ),
        ] {
            rows.push(leaf(
                key,
                label,
                &value,
                PROFILE_TREE_CHILD_DEPTH,
                &["channel_info"],
                "Exact coordinates from the PNM capture, separate from DPD MAC message fields.",
            ));
        }
        rows.push(branch(
            "profiles",
            "Profiles",
            &format!("{} available", self.catalog.profile_ids().len()),
            PROFILE_TREE_ROOT_DEPTH,
            &[],
        ));
        for &id in self.catalog.profile_ids() {
            let profile = self.profile(id)?;
            let key = format!("profile_{id}");
            let mut node = branch(
                &key,
                &format!("Profile {id}"),
                &format!("{} assignments", profile.assignments.len()),
                PROFILE_TREE_CHILD_DEPTH,
                &["profiles"],
            );
            node["profile_id"] = json!(id);
            rows.push(node);
            if id != selected {
                continue;
            }
            let window = self
                .catalog
                .schemes(id, 0, MAX_PROFILE_SCHEME_ROWS)
                .map_err(|cause| cause.message().to_owned())?;
            rows.push(leaf(
                "schemes",
                "Assignment schemes",
                &window.total.to_string(),
                PROFILE_TREE_DETAIL_DEPTH,
                &["profiles", &key],
                "Compact range schemes from the capture. These are not invented DPD TLVs.",
            ));
            for scheme in &window.schemes {
                let index = scheme.source_index;
                let label = format!("Range {}", index + 1);
                let order = scheme.modulation_order;
                let value = format!("{} · {} subcarriers", scheme.display_label, scheme.count);
                let description = format!("{label}: {order}, code {}, bit loading {}, role {}. CM-OSSI 3.1 I27 §D.2.1.7.3.1. Range numbering follows capture order; it is not a physical carrier index.", scheme.modulation_order_code, json!(scheme.bit_loading), json!(scheme.carrier_type));
                rows.push(leaf(
                    &format!("range_{index}"),
                    &label,
                    &value,
                    PROFILE_TREE_DETAIL_DEPTH,
                    &["profiles", &key],
                    &description,
                ));
            }
            if window.total > MAX_PROFILE_SCHEME_ROWS {
                rows.push(leaf(
                    "limit",
                    "Display limit",
                    &format!(
                        "Showing {MAX_PROFILE_SCHEME_ROWS} of {} schemes",
                        window.total
                    ),
                    PROFILE_TREE_DETAIL_DEPTH,
                    &["profiles", &key],
                    "Only the displayed rows are limited; the full capture is validated.",
                ));
            }
        }
        rows.push(branch(
            "provenance",
            "Input provenance",
            "",
            PROFILE_TREE_ROOT_DEPTH,
            &[],
        ));
        for (key, label, value) in [
            ("format", "Detected format", self.format.into()),
            (
                "bytes",
                "Original input size",
                format!("{} bytes", self.original.bytes().len()),
            ),
            (
                "epoch",
                "Capture time (Unix seconds)",
                metadata.header.capture_time.to_string(),
            ),
            ("hash", "Original input SHA-256", self.sha256.clone()),
        ] {
            rows.push(leaf(
                key,
                label,
                &value,
                PROFILE_TREE_CHILD_DEPTH,
                &["provenance"],
                "Exact original-input provenance, separate from decoded-JSON provenance.",
            ));
        }
        Ok(
            json!({"source_name": self.name, "format": self.format, "profile_ids": self.catalog.profile_ids(), "selected_profile_id": selected, "rows": rows}),
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
