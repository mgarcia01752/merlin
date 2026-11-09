/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Minimal current-decoder shapes; unrelated derived presentation fields are ignored.
use crate::{
    error::invalid,
    scenario::{FrequencyGrid, ScenarioKey},
    types::{modulation::CarrierType, IqPair},
    LabError,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Header {
    pub file_type: String,
    pub file_type_version: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub capture_time: u32,
}
impl Header {
    /// Checks the supported current decoded capture identity.
    /// # Arguments
    /// * `kind` - Expected PNM type suffix.
    /// # Returns
    /// Unit for supported prefix, type and1.0 file format.
    /// # Errors
    /// Rejects unsupported identities or revisions without guessing fields.
    pub(super) fn check(&self, kind: u8) -> Result<(), LabError> {
        if !["PNN", "PNM"].contains(&self.file_type.as_str())
            || self.file_type_version != kind
            || self.major_version != 1
            || self.minor_version != 0
        {
            return Err(invalid("Unsupported decoded PNM capture type or format."));
        }
        Ok(())
    }
}
#[derive(Deserialize)]
pub(super) struct Grid {
    mac_address: String,
    channel_id: u8,
    subcarrier_zero_frequency: u32,
    first_active_subcarrier_index: u16,
    subcarrier_spacing: u32,
}
impl Grid {
    /// Converts decoded header coordinates without modifying them.
    /// # Arguments
    /// * `count` - Independently decoded sample or expanded-assignment count.
    /// # Returns
    /// Owned exact device/channel/grid identity.
    /// # Errors
    /// Rejects malformed MAC octets, count, spacing or frequency overflow.
    pub(super) fn key(&self, count: usize) -> Result<ScenarioKey, LabError> {
        let octets = self
            .mac_address
            .split(':')
            .map(|octet| {
                if octet.len() != 2 {
                    return Err(invalid("MAC must have six two-digit octets."));
                }
                u8::from_str_radix(octet, 16).map_err(|_| invalid("Invalid MAC octet."))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mac_address = octets
            .try_into()
            .map_err(|_| invalid("Invalid MAC length."))?;
        let grid = FrequencyGrid {
            zero_frequency_hz: self.subcarrier_zero_frequency,
            first_subcarrier_index: self.first_active_subcarrier_index,
            spacing_hz: self.subcarrier_spacing,
            sample_count: count,
        };
        grid.validate()?;
        Ok(ScenarioKey {
            mac_address,
            channel_id: self.channel_id,
            grid,
        })
    }
}
#[derive(Deserialize)]
pub(super) struct Rxmer {
    #[serde(rename = "pnm_header")]
    pub header: Header,
    #[serde(flatten)]
    pub grid: Grid,
    pub data_length: usize,
    pub value_units: String,
    pub values: Vec<Option<f64>>,
}
#[derive(Deserialize)]
pub(super) struct Channel {
    #[serde(rename = "pnm_header")]
    pub header: Header,
    #[serde(flatten)]
    pub grid: Grid,
    pub data_length: usize,
    pub iq: Vec<Option<IqPair>>,
}
#[derive(Deserialize)]
pub(super) struct ProfileDocument {
    #[serde(rename = "pnm_header")]
    pub header: Header,
    #[serde(flatten)]
    pub grid: Grid,
    pub num_profiles: usize,
    pub profiles: Vec<Profile>,
}
#[derive(Deserialize)]
pub(super) struct Profile {
    pub profile_id: u8,
    pub schemes: Vec<Scheme>,
}
#[derive(Deserialize)]
pub(super) struct Scheme {
    pub schema_type: String,
    pub num_subcarriers: usize,
    pub modulation_order_code: u8,
    pub modulation_order: String,
    pub bit_loading: Option<u8>,
    pub carrier_type: CarrierType,
}
