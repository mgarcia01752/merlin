/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Versioned channel request shared by CLI and future adapters; all math stays in Rust.
use super::{ChannelJobLimits, ChannelRunSettings};
use crate::{
    error::invalid,
    simulation::{source::PrbsSeed, test_points::ProbeConfig},
    transmission::{layout::CarrierLayoutSettings, packing::NcpModulation, TransmissionSettings},
    LabError,
};
use serde::Deserialize;
use std::time::Duration;
/// Version1 fixed-profile noiseless channel request; unknown fields are rejected.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelDocument {
    pub schema_version: u16,
    pub transmission: TransmissionSettings,
    pub layout: CarrierLayoutSettings,
    #[serde(default = "default_duration_ms")]
    pub duration_ms: u64,
    #[serde(default = "default_seed")]
    pub prbs_seed: PrbsSeed,
    #[serde(default = "default_ncp")]
    pub ncp_modulation: NcpModulation,
    #[serde(default = "default_variance")]
    pub llr_variance: f64,
    #[serde(default = "default_iterations")]
    pub max_iterations: usize,
    #[serde(default = "default_block")]
    pub receive_block_samples: usize,
    #[serde(default)]
    pub limits: ChannelJobLimits,
    pub probes: Option<ProbeConfig>,
}
fn default_duration_ms() -> u64 {
    1000
}
fn default_seed() -> PrbsSeed {
    0x7fff
}
fn default_ncp() -> NcpModulation {
    NcpModulation::Qpsk
}
fn default_variance() -> f64 {
    0.01
}
fn default_iterations() -> usize {
    50
}
fn default_block() -> usize {
    4096
}
impl ChannelDocument {
    /// Validates a request and returns independently owned scientific settings.
    /// # Returns
    /// A settings snapshot using the shared timing/configuration checks.
    /// # Errors
    /// Rejects unsupported schema version or invalid channel settings.
    pub fn settings(&self) -> Result<ChannelRunSettings, LabError> {
        if self.schema_version != 1 {
            return Err(invalid("Unsupported channel request schema version."));
        }
        let settings = ChannelRunSettings {
            transmission: self.transmission.clone(),
            layout: self.layout.clone(),
            duration: Duration::from_millis(self.duration_ms),
            prbs_seed: self.prbs_seed,
            ncp_modulation: self.ncp_modulation,
            llr_variance: self.llr_variance,
            max_iterations: self.max_iterations,
            receive_block_samples: self.receive_block_samples,
        };
        self.limits.validate()?;
        settings.validate()?;
        Ok(settings)
    }
}
