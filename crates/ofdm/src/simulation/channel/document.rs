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
    SimulationError,
};
use serde::Deserialize;
use std::time::Duration;
/// Version1 fixed-profile reference/impaired channel request; unknown fields reject.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelDocument {
    #[serde(default)]
    pub source: crate::simulation::source::file::SourceSelection,
    pub data_bit_loading: Option<Vec<u8>>,
    #[serde(default)]
    pub impairment_timeline: Vec<super::timeline::ImpairmentEpoch>,
    pub protected_constellation: Option<super::protected_constellation::ProtectedCaptureSelection>,
    #[serde(default)]
    pub receiver_mode: crate::signal_processing::receiver::channel_estimation::ReceiverMode,
    pub constellation: Option<super::constellation::ConstellationSelection>,
    #[serde(default)]
    pub impairments: crate::signal_processing::channel::impairments::ChannelImpairments,
    pub measurement: Option<super::measurement::MeasurementSelection>,
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
    pub fn settings(&self) -> Result<ChannelRunSettings, SimulationError> {
        self.settings_with_scenario(None)
    }
    /// Resolves optional explicit measurement selection through the same scientific validation.
    /// # Arguments
    /// * `scenario` - Required exactly when request.measurement is present.
    /// # Returns
    /// Owned settings with selected full-resolution measurements, no file dependencies.
    /// # Errors
    /// Rejects missing/unrequested scenarios, incompatible profile/epochs or bad geometry.
    pub fn settings_with_scenario(
        &self,
        scenario: Option<&crate::scenario::document::ScenarioDocument>,
    ) -> Result<ChannelRunSettings, SimulationError> {
        self.settings_with_inputs(scenario, None)
    }
    /// Resolves measurement selection and exact file bytes through shared Rust validation.
    /// # Arguments
    /// * `scenario` - Required exactly when measurement selection is requested.
    /// * `payload_bytes` - Required exactly for source.kind=file; no paths enter core APIs.
    /// # Returns
    /// Frozen validated settings with shared immutable bytes and bounded fill policy.
    /// # Errors
    /// Rejects mismatched inputs, invalid targets/seeds, schema or channel settings.
    pub fn settings_with_inputs(
        &self,
        scenario: Option<&crate::scenario::document::ScenarioDocument>,
        payload_bytes: Option<&[u8]>,
    ) -> Result<ChannelRunSettings, SimulationError> {
        if self.schema_version != 1 {
            return Err(invalid("Unsupported channel request schema version."));
        }
        use crate::simulation::source::{
            file::{FilePayload, SourceSelection},
            PayloadSourceConfig,
        };
        let source = match (&self.source, payload_bytes) {
            (SourceSelection::Prbs15, None) => PayloadSourceConfig::Prbs15,
            (SourceSelection::File { fill }, Some(bytes)) => PayloadSourceConfig::File {
                payload: FilePayload::new(bytes, *fill)?,
            },
            _ => {
                return Err(invalid(
                    "Supply payload bytes exactly when source.kind is file.",
                ))
            }
        };
        let mut settings = ChannelRunSettings {
            source,
            data_bit_loading: self.data_bit_loading.clone(),
            impairment_timeline: self.impairment_timeline.clone(),
            receiver_mode: self.receiver_mode,
            protected_constellation: self.protected_constellation.clone(),
            constellation: self.constellation.clone(),
            impairments: self.impairments.clone(),
            transmission: self.transmission.clone(),
            layout: self.layout.clone(),
            duration: Duration::from_millis(self.duration_ms),
            prbs_seed: self.prbs_seed,
            ncp_modulation: self.ncp_modulation,
            llr_variance: self.llr_variance,
            max_iterations: self.max_iterations,
            receive_block_samples: self.receive_block_samples,
        };
        match (&self.measurement, scenario) {
            (Some(selection), Some(scenario)) => selection.apply(scenario, &mut settings)?,
            (None, None) => {}
            _ => {
                return Err(invalid(
                    "Supply both request.measurement and a scenario, or neither.",
                ))
            }
        }
        self.limits.validate()?;
        settings.validate()?;
        Ok(settings)
    }
}
