/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Generic source configuration schema, independent from result/FEC schemas.
use crate::api::{
    ApiResult, Estimator, ExperimentConfig, ExperimentOptions, PhyError, ReceiverConfig,
    SyntheticGrant,
};
use crate::configuration::ucd::{CmAssignment, MapControl, OfdmaChannel, ProbeControl};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, sync::Arc};

/// One generic named preset; a vendor identifier never defines its semantics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationPreset {
    /// Stable local preset name, 1..128 UTF-8 bytes.
    pub id: String,
    /// Human-readable generic description, at most 1024 bytes.
    pub description: String,
    /// Optional vendor template/catalog number, retained only as provenance.
    pub vendor_template_id: Option<u32>,
    /// Source-derived UCD fields, distinct from source/channel truth.
    pub channel: OfdmaChannel,
    /// Frozen one-CM assignment context.
    pub assignment: CmAssignment,
    /// Optional represented CAT0 source controls; raw MAP execution is unavailable.
    pub map: Option<MapControl>,
    /// Optional represented CAT1 source controls; probe waveform execution is unavailable.
    pub probe_map: Option<ProbeControl>,
}
/// Explicit synthetic experiment settings separate from normative channel fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunSettings {
    /// scheduled_data, initial_ranging, fine_ranging, probe or profile_testing.
    pub mode: String,
    /// Explicit selected normative burst IUC; never inferred from template IDs.
    pub selected_iuc: u8,
    /// Independent nonzero continuous PRBS15 payload seed.
    pub source_seed: u16,
    /// First synthetic grant identity.
    pub grant_id: u64,
    /// Explicit first-prefix origin in 102.4 MHz sample ticks.
    pub epoch_sample: u64,
    /// First synthetic physical frame identity.
    pub first_frame: u64,
    /// Complete physical frames per grant.
    pub frames: usize,
    /// Selected physical slot indices, no automatic Cartesian capture/trial product.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 237>")]
    pub minislots: Vec<usize>,
    /// Explicit grant repetitions, 1..256.
    pub grants: usize,
    /// Aggregate received sample budget at most 16,777,216.
    pub maximum_samples: usize,
    /// Bounded transport chunk, independent of frame/codeword boundaries.
    pub chunk_samples: usize,
    /// Monotonic active execution timeout, excludes acknowledged pauses.
    pub timeout_ms: u64,
    /// Retained passive points across selected taps, 0..65536.
    pub retained_points: usize,
    /// Explicit tap names, maximum 9, no silently retained unselected history.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 9>")]
    pub retained_taps: Vec<String>,
    /// frame_flat_ls or frame_pilot_linear, both received-pilot policies.
    pub estimator: String,
    /// Required frequency interpolation gap limit for linear estimator, 1..4096.
    pub maximum_pilot_gap: usize,
    /// Finite positive complex-domain reference demapper variance, never RxMER truth.
    pub reference_variance: f64,
    /// Finite positive zero-forcing minimum effective-response power.
    pub equalizer_power_floor: f64,
    /// Bounded LDPC iterations, 1..100.
    pub ldpc_iterations: usize,
    /// Omission/null selects exact unity H, disabled sample noise/timing and default seed.
    pub channel: Option<crate::api::ChannelConfig>,
    /// Omission/null selects zero synthetic observation guards.
    pub guards: Option<crate::api::GuardIntervals>,
    /// Separate explicit synthetic targets; unsupported binary types are not accepted here.
    pub synthetic_rxmer: Option<super::RxmerSelectionDocument>,
    /// Separate complete banks and explicitly based synthetic updates.
    pub synthetic_pre_eq: Option<super::PreEqDocument>,
}
/// Accepted schema-1 generic document; parsing does not configure or run a session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationDocument {
    /// Exactly merlin.upstream.configuration, distinct from experiment/FEC reports.
    pub format: String,
    /// Exactly 1; unsupported versions require an explicit future migration.
    pub schema_version: u32,
    /// Complete generic source inventory, 1..256 entries.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 256>")]
    pub presets: Vec<ConfigurationPreset>,
    /// Exactly one inventory ID; loading never iterates all presets implicitly.
    pub selected_preset: String,
    /// Explicit synthetic execution settings; represented MAC controls stay separate.
    pub simulation: RunSettings,
    #[serde(skip)]
    original: Option<Arc<[u8]>>,
}
fn invalid(field: &'static str) -> PhyError {
    PhyError::InvalidInput { field }
}
impl ConfigurationDocument {
    /// Parses a borrowed UTF-8 JSON document, bounded to 16 MiB before decoding.
    ///
    /// Returns an owned source-valid inventory; unknown typed fields, versions,
    /// reserved/conditional values and excessive arrays reject without session effects.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > 16 * 1024 * 1024 {
            return Err(PhyError::ResourceLimit {
                field: "configuration.document_bytes",
            }
            .into());
        }
        let mut document: Self = serde_json::from_slice(bytes)?;
        document.original = Some(Arc::from(bytes));
        document.validate()?;
        Ok(document)
    }
    /// Validates complete retained inventory and selected identity without running.
    ///
    /// Returns unit; unsupported waveform modes remain representable with a known
    /// name, but `prepare` rejects execution before publishing any session settings.
    pub fn validate(&self) -> ApiResult<()> {
        self.validate_inventory()?;
        self.simulation.receiver()?;
        self.simulation.options()?;
        crate::source::Prbs15::new(self.simulation.source_seed)?;
        if let Some(guards) = self.simulation.guards {
            guards.validate(self.simulation.epoch_sample)?;
        }
        if !self.simulation.reference_variance.is_finite()
            || !self.simulation.equalizer_power_floor.is_finite()
        {
            return Err(invalid("configuration.nonfinite_number").into());
        }
        if let Some(channel) = &self.simulation.channel {
            channel.validate()?;
        }
        super::synthetic::validate_finite(
            self.simulation.synthetic_pre_eq.as_ref(),
            self.simulation.synthetic_rxmer.as_ref(),
        )?;
        let selected = self
            .presets
            .iter()
            .find(|preset| preset.id == self.selected_preset)
            .ok_or_else(|| invalid("configuration.selection"))?;
        if self.simulation.mode == "scheduled_data"
            && selected.map.is_none()
            && selected.probe_map.is_none()
        {
            self.prepare()?;
        }
        Ok(())
    }
    fn validate_inventory(&self) -> ApiResult<()> {
        if self.format != "merlin.upstream.configuration" || self.schema_version != 1 {
            return Err(PhyError::Unsupported {
                field: "configuration.format_version",
            }
            .into());
        }
        if self.presets.is_empty() || self.presets.len() > 256 {
            return Err(invalid("configuration.presets").into());
        }
        let mut ids = BTreeSet::new();
        for preset in &self.presets {
            if preset.id.is_empty()
                || preset.id.len() > 128
                || preset.description.len() > 1024
                || !ids.insert(preset.id.as_str())
            {
                return Err(invalid("configuration.preset_identity").into());
            }
            preset.channel.validate()?;
            preset.assignment.validate(&preset.channel)?;
            if let Some(map) = &preset.map {
                map.validate(&preset.channel)?;
            }
            if let Some(probe) = &preset.probe_map {
                probe.validate(&preset.channel, &preset.assignment)?;
            }
        }
        if !ids.contains(self.selected_preset.as_str()) {
            return Err(invalid("configuration.selection").into());
        }
        if !matches!(
            self.simulation.mode.as_str(),
            "scheduled_data" | "initial_ranging" | "fine_ranging" | "probe" | "profile_testing"
        ) {
            return Err(invalid("configuration.mode").into());
        }
        Ok(())
    }
    /// Borrows the exact original JSON bytes without normalizing whitespace or nulls.
    ///
    /// Returns None only for direct typed/serde construction outside `from_json`.
    /// Later edits/export do not rewrite this immutable input provenance.
    pub fn original_bytes(&self) -> Option<&[u8]> {
        self.original.as_deref()
    }
    /// Computes SHA-256 over exact retained original bytes, never resolved output.
    ///
    /// Returns lowercase hexadecimal or None when no original byte input exists.
    pub fn original_sha256(&self) -> Option<String> {
        self.original_bytes()
            .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
    }
    /// Returns every retained preset identity in original order without selection changes.
    ///
    /// No hardcoded profile catalog, source clock or trials are created by this query.
    pub fn inventory(&self) -> Vec<&str> {
        self.presets
            .iter()
            .map(|preset| preset.id.as_str())
            .collect()
    }
    /// Prepares the selected scheduled-data configuration and explicit run budgets.
    ///
    /// Returns owned validated settings, never executes work. Recognized unsupported
    /// modes/MAP execution reject; raw source controls are not silently discarded.
    pub fn prepare(&self) -> ApiResult<(ExperimentConfig, ExperimentOptions)> {
        self.validate_inventory()?;
        let preset = self
            .presets
            .iter()
            .find(|preset| preset.id == self.selected_preset)
            .ok_or_else(|| invalid("configuration.selection"))?;
        let run = &self.simulation;
        if run.mode != "scheduled_data" || preset.map.is_some() || preset.probe_map.is_some() {
            return Err(PhyError::Unsupported {
                field: "configuration.execution_mode",
            }
            .into());
        }
        if !preset.assignment.permits(run.selected_iuc) {
            return Err(invalid("configuration.assigned_iuc").into());
        }
        let grant = SyntheticGrant {
            id: run.grant_id,
            profile_id: u16::from(run.selected_iuc),
            epoch_sample: run.epoch_sample,
            first_frame: run.first_frame,
            frames: run.frames,
            minislots: run.minislots.clone(),
        };
        let us = preset.channel.project(grant)?;
        let receiver = run.receiver()?;
        let options = run.options()?;
        let mut config = ExperimentConfig::new(us, run.source_seed, receiver)?;
        if let Some(channel) = &run.channel {
            config.channel = channel.clone();
        }
        if let Some(guards) = run.guards {
            config.guards = guards;
        }
        if let Some(pre_eq) = &run.synthetic_pre_eq {
            let (initial, playback) = pre_eq.materialize(&config.us_configuration)?;
            config.pre_eq = initial;
            config.pre_eq_updates = Some(playback);
        }
        if let Some(rxmer) = &run.synthetic_rxmer {
            config.rxmer = Some(rxmer.materialize(&config.us_configuration)?);
        }

        crate::simulation::preview(&config, &options)?;
        Ok((config, options))
    }
    /// Exports all represented fields in the same independent versioned schema.
    ///
    /// Returns compact JSON capped at 16 MiB; validates first and never writes a file.
    pub fn to_json(&self) -> ApiResult<String> {
        self.validate()?;
        crate::api::bounded_json(self, 16 * 1024 * 1024)
    }
}

impl RunSettings {
    fn receiver(&self) -> ApiResult<ReceiverConfig> {
        if !(1..=4096).contains(&self.maximum_pilot_gap) {
            return Err(invalid("configuration.maximum_pilot_gap").into());
        }
        let estimator = match self.estimator.as_str() {
            "frame_flat_ls" => Estimator::FrameFlatLs,
            "frame_pilot_linear" => Estimator::FramePilotLinear {
                maximum_gap: self.maximum_pilot_gap,
            },
            _ => {
                return Err(PhyError::Unsupported {
                    field: "configuration.estimator",
                }
                .into())
            }
        };
        let receiver = ReceiverConfig::new(
            estimator,
            self.reference_variance,
            self.equalizer_power_floor,
            self.ldpc_iterations,
        )?;
        Ok(receiver)
    }
    fn options(&self) -> ApiResult<ExperimentOptions> {
        if self.retained_taps.len() > 9
            || self.minislots.is_empty()
            || self.minislots.len() > 237
            || !(1..=256).contains(&self.frames)
        {
            return Err(invalid("configuration.run_counts").into());
        }
        let taps = self
            .retained_taps
            .iter()
            .map(|name| match name.as_str() {
                "TxGrid" => Ok(crate::api::Tap::TxGrid),
                "TxPreEq" => Ok(crate::api::Tap::TxPreEq),
                "TxUseful" => Ok(crate::api::Tap::TxUseful),
                "TxOutput" => Ok(crate::api::Tap::TxOutput),
                "ChannelInput" => Ok(crate::api::Tap::ChannelInput),
                "ChannelOutput" => Ok(crate::api::Tap::ChannelOutput),
                "RxInput" => Ok(crate::api::Tap::RxInput),
                "RxFft" => Ok(crate::api::Tap::RxFft),
                "RxEqualized" => Ok(crate::api::Tap::RxEqualized),
                _ => Err(invalid("configuration.tap")),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let options = ExperimentOptions {
            grants: self.grants,
            maximum_samples: self.maximum_samples,
            chunk_samples: self.chunk_samples,
            timeout_ms: self.timeout_ms,
            retained_points: self.retained_points,
            retained_taps: taps,
        };
        options.validate()?;
        Ok(options)
    }
}
