/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Explicit source-preserving measured association and model projection.
use super::{
    bounded_json, constants::MAX_CAPTURE_SOURCE_BYTES, reassemble_ucd, PreEqCapture,
    PreEqCaptureKind, RxmerCapture,
};
use docsis_ofdma::api::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, sync::Arc};

/// One original file and local immutable identity, independent of trial count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureSource {
    /// Nonzero unique source identity within its input family.
    pub id: u64,
    /// Exact original source bytes, never canonicalized.
    #[serde(
        deserialize_with = "docsis_ofdma::configuration::serde_limits::bounded_vec::<_, _, MAX_CAPTURE_SOURCE_BYTES>"
    )]
    pub bytes: Vec<u8>,
}
/// External association facts missing from one or more capture formats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureAssociation {
    /// Exact CCAP ID header octets, including all 256 bytes and padding.
    #[serde(
        default,
        deserialize_with = "docsis_ofdma::configuration::serde_limits::optional_vec::<_, _, 256>"
    )]
    pub ccap_id_raw: Option<Vec<u8>>,
    /// Explicit upstream CCAP interface mapping to selected UCD UCID.
    pub if_index: Option<u32>,
    /// Exact RPD identity, zeros for integrated CCAP.
    pub rpd_id: Option<[u8; 6]>,
    /// Exact RPD port associated with that CCAP interface.
    pub rpd_port: Option<u8>,
    /// Downstream management MAC receiving MAP/UCD, explicitly supplied.
    pub cmts_mac: [u8; 6],
    /// Inclusive source-time admission interval, Unix epoch seconds.
    pub earliest_epoch_seconds: u32,
    /// Inclusive upper bound; never converted to simulation symbol offsets.
    pub latest_epoch_seconds: u32,
    /// Source/exporter evidence for the selected MS-octet-first/Boolean0/1 profile.
    pub adapter_evidence: String,
}
/// Captured update activation; control semantics are explicit external facts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureEvent {
    /// Original applied-bank or last-update file and effective bank identity.
    pub source: CaptureSource,
    /// Strictly increasing physical-symbol activation after initial symbol zero.
    pub symbol: usize,
    /// snapshot, initialize or adjust; source kind must agree.
    pub operation: String,
    /// Required for updates, absent for snapshots; corresponding probe bank.
    pub base_bank: Option<u64>,
    /// Required for updates, absent for snapshots; explicit corresponding probe.
    pub probe: Option<u64>,
    /// External CM-OSSI application status; only success(2) admits replay.
    pub application_status: u8,
}
/// A corresponding probe with immutable bank, source time and activation coordinate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureProbe {
    /// Nonzero identity referenced by the update's control context.
    pub id: u64,
    /// Effective bank used at this probe, not the latest bank at update reception.
    pub bank_id: u64,
    /// Physical-symbol coordinate strictly before its corresponding update.
    pub symbol: usize,
    /// Source epoch of the observed probe, not synthetic activation time.
    pub epoch_seconds: u32,
}
/// Complete immutable measured candidate; original JSON and file bytes survive runs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasuredExperiment {
    /// Exactly merlin.upstream.measured-experiment, independent of all input/report formats.
    pub format: String,
    /// Exactly one; unknown typed fields/versions reject without implicit migration.
    pub schema_version: u32,
    /// Complete generic source context including explicit CM/grant/bandwidth.
    pub configuration: ConfigurationDocument,
    /// Ordered complete multipart set, or empty for explicitly supplied JSON channel.
    #[serde(
        deserialize_with = "docsis_ofdma::configuration::serde_limits::bounded_vec::<_, _, 16>"
    )]
    pub ucd_fragments: Vec<CaptureSource>,
    /// External mapping and explicit source interpretation evidence.
    pub association: CaptureAssociation,
    /// Exact source inventory, independent of the number of simulation grants.
    #[serde(
        deserialize_with = "docsis_ofdma::configuration::serde_limits::bounded_vec::<_, _, 256>"
    )]
    pub rxmer: Vec<CaptureSource>,
    /// single, average, hold or interpolate when sources exist; none otherwise.
    pub rxmer_policy: String,
    /// Explicit physical-symbol offsets for hold/interpolate only.
    #[serde(
        deserialize_with = "docsis_ofdma::configuration::serde_limits::optional_vec::<_, _, 256>"
    )]
    pub rxmer_offsets: Option<Vec<usize>>,
    /// Applied major1 bank, or explicit normative unity reset when absent.
    pub pre_eq_initial: Option<CaptureSource>,
    /// Immutable reset-bank identity, required even for default unity.
    pub pre_eq_initial_id: u64,
    /// Snapshot or externally controlled update activations.
    #[serde(
        deserialize_with = "docsis_ofdma::configuration::serde_limits::bounded_vec::<_, _, 255>"
    )]
    pub pre_eq_events: Vec<CaptureEvent>,
    /// Explicit corresponding probe lineage; no inference from file ordering.
    #[serde(
        deserialize_with = "docsis_ofdma::configuration::serde_limits::bounded_vec::<_, _, 256>"
    )]
    pub probes: Vec<CaptureProbe>,
    #[serde(skip)]
    original: Option<Arc<[u8]>>,
}
fn invalid(field: &'static str) -> ApiError {
    PhyError::Association { field }.into()
}
impl MeasuredExperiment {
    /// Parses and fully preflights a finite measured plan, without starting work.
    ///
    /// `bytes` is schema1 JSON capped at16MiB before allocation. Returns owned
    /// original provenance; bad source, mapping, lineage or composition rejects.
    ///
    /// # Arguments
    /// * `bytes` - Complete schema1 plan JSON, at most 16 MiB.
    ///
    /// # Returns
    /// An owned fully preflighted plan with exact original JSON retained.
    ///
    /// # Errors
    /// Rejects unsupported schema, invalid associations, future schedules, incompatible
    /// composition, resource limits or serialization failure before source consumption.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > crate::common::constants::MAX_MEASURED_EXPERIMENT_BYTES {
            return Err(PhyError::ResourceLimit {
                field: "measured.document_bytes",
            }
            .into());
        }
        let mut plan: Self = serde_json::from_slice(bytes)?;
        plan.original = Some(Arc::from(bytes));
        plan.prepare()?;
        Ok(plan)
    }
    /// Borrows exact originally admitted plan JSON, distinct from resolved values.
    ///
    /// Returns None only for directly constructed typed candidates.
    ///
    /// # Returns
    /// Borrowed exact plan bytes, or None for directly constructed typed plans.
    pub fn original_bytes(&self) -> Option<&[u8]> {
        self.original.as_deref()
    }
    /// Exports the complete original-valued schema after full preflight.
    ///
    /// Returns bounded JSON; source files retain all original octets.
    ///
    /// # Returns
    /// Owned bounded JSON retaining original-valued fields and source octets.
    ///
    /// # Errors
    /// Rejects unsupported schema, invalid associations, future schedules, incompatible
    /// composition, resource limits or serialization failure before source consumption.
    pub fn to_json(&self) -> ApiResult<String> {
        self.prepare()?;
        bounded_json(
            self,
            crate::common::constants::MAX_MEASURED_EXPERIMENT_BYTES,
        )
    }
    /// Projects measured observations into the existing bounded Rust model owners.
    ///
    /// Returns a frozen experiment/options pair. Raw time and grids stay retained;
    /// normalization/power synthesis are explicit models, never measured RF truth.
    /// Full preflight covers later events and every selected source before admission.
    ///
    /// # Returns
    /// Owned resolved engine input and options; no simulation has started.
    ///
    /// # Errors
    /// Rejects unsupported schema, invalid associations, future schedules, incompatible
    /// composition, resource limits or serialization failure before source consumption.
    pub fn prepare(&self) -> ApiResult<(ExperimentConfig, ExperimentOptions)> {
        let a = &self.association;
        if self.format != "merlin.upstream.measured-experiment" || self.schema_version != 1 {
            return Err(PhyError::Unsupported {
                field: "measured.format_version",
            }
            .into());
        }
        if a.adapter_evidence.trim().is_empty()
            || a.adapter_evidence.len() > 1024
            || a.earliest_epoch_seconds > a.latest_epoch_seconds
            || a.cmts_mac == [0; 6]
            || a.cmts_mac[0] & 1 != 0
            || a.if_index.is_some_and(|index| index == 0)
            || a.ccap_id_raw.as_ref().is_some_and(|id| id.len() != 256)
            || self.ucd_fragments.len() > 16
            || self.rxmer.len() > 256
            || self.pre_eq_events.len() > 255
            || self.probes.len() > 256
            || self.pre_eq_initial_id == 0
        {
            return Err(invalid("measured.association"));
        }
        self.configuration.validate()?;
        let mut document = self.configuration.clone();
        if document.simulation.synthetic_pre_eq.is_some()
            || document.simulation.synthetic_rxmer.is_some()
        {
            return Err(invalid("measured.duplicate_input_owner"));
        }
        let selected = document
            .presets
            .iter()
            .position(|p| p.id == document.selected_preset)
            .ok_or_else(|| invalid("measured.preset"))?;
        if !self.ucd_fragments.is_empty() {
            source_ids(&self.ucd_fragments)?;
            let fragments = self
                .ucd_fragments
                .iter()
                .map(|s| s.bytes.clone())
                .collect::<Vec<_>>();
            let captured = reassemble_ucd(&fragments)?;
            if captured.header().source != a.cmts_mac {
                return Err(invalid("measured.ucd_sender"));
            }
            document.presets[selected].channel = captured
                .project_channel(document.presets[selected].channel.encompassed_bandwidth_hz)?;
        }
        document.validate()?;
        let preset = &document.presets[selected];
        let (mut config, options) = document.prepare()?;
        let us = &config.us_configuration;
        source_ids(&self.rxmer)?;
        let n = us.waveform().numerology().fft_len();
        let mut targets = Vec::new();
        let mut pre_eq_flag = None;
        let mut previous_rxmer_epoch = None;
        for source in &self.rxmer {
            let captured = RxmerCapture::from_pnm(&source.bytes)?;
            let h = captured.header();
            self.check_time(h.capture_time_epoch_seconds)?;
            if matches!(self.rxmer_policy.as_str(), "hold" | "interpolate")
                && previous_rxmer_epoch.is_some_and(|epoch| epoch > h.capture_time_epoch_seconds)
            {
                return Err(invalid("measured.rxmer_time_order"));
            }
            previous_rxmer_epoch = Some(h.capture_time_epoch_seconds);
            if h.cm_mac != preset.assignment.cm_mac
                || Some(h.if_index) != a.if_index
                || Some(h.ccap_id_raw.as_slice()) != a.ccap_id_raw.as_deref()
                || Some(h.rpd_id) != a.rpd_id
                || Some(h.rpd_port) != a.rpd_port
                || h.number_of_averages == 0
                || h.number_of_averages > 255
                || h.pre_eq_on_off > 1
                || pre_eq_flag.is_some_and(|v| v != h.pre_eq_on_off)
                || h.subcarrier_zero_hz != preset.channel.subcarrier_zero_hz
                || h.subcarrier_spacing_khz != preset.channel.subcarrier_spacing_khz
            {
                return Err(invalid("measured.rxmer_identity_grid_or_profile"));
            }
            pre_eq_flag = Some(h.pre_eq_on_off);
            let mut grid = vec![None; n];
            let first = usize::from(h.first_active_subcarrier);
            grid[first..first + captured.readings_db().len()]
                .copy_from_slice(captured.readings_db());
            targets.push(SyntheticRxmer::new(us, source.id, grid)?);
        }
        config.rxmer = if targets.is_empty() {
            if self.rxmer_policy != "none" || self.rxmer_offsets.is_some() {
                return Err(invalid("measured.rxmer_policy"));
            }
            None
        } else {
            let policy = match (self.rxmer_policy.as_str(), &self.rxmer_offsets) {
                ("single", None) => RxmerPolicy::Single,
                ("average", None) => RxmerPolicy::Average,
                ("hold", Some(offsets)) => RxmerPolicy::Hold {
                    offsets: offsets.clone(),
                },
                ("interpolate", Some(offsets)) => RxmerPolicy::Interpolate {
                    offsets: offsets.clone(),
                },
                _ => return Err(invalid("measured.rxmer_policy")),
            };
            Some(RxmerPlayback::new(us, targets, policy)?)
        };
        if let Some(source) = &self.pre_eq_initial {
            if source.id != self.pre_eq_initial_id {
                return Err(invalid("measured.initial_identity"));
            }
            config.pre_eq = self.bank(source, preset, us)?;
        }
        let mut event_sources = Vec::new();
        let mut effective_ids = BTreeSet::from([self.pre_eq_initial_id]);
        let mut active = vec![(0, self.pre_eq_initial_id)];
        let mut bank_epochs = std::collections::BTreeMap::from([(
            self.pre_eq_initial_id,
            self.pre_eq_initial
                .as_ref()
                .map(|source| {
                    PreEqCapture::from_pnm(&source.bytes)
                        .map(|capture| capture.header().capture_time_epoch_seconds)
                })
                .transpose()?
                .unwrap_or(a.earliest_epoch_seconds),
        )]);
        let mut previous_event_epoch = bank_epochs[&self.pre_eq_initial_id];
        for event in &self.pre_eq_events {
            if event.application_status != 2 || !effective_ids.insert(event.source.id) {
                return Err(invalid("measured.event_status_or_identity"));
            }
            let captured = PreEqCapture::from_pnm(&event.source.bytes)?;
            self.check_pre_eq(&captured, preset)?;
            let event_epoch = captured.header().capture_time_epoch_seconds;
            if event_epoch < previous_event_epoch {
                return Err(invalid("measured.event_time_order"));
            }
            previous_event_epoch = event_epoch;
            let operation = match (
                event.operation.as_str(),
                captured.kind(),
                event.base_bank,
                event.probe,
            ) {
                ("snapshot", PreEqCaptureKind::Applied, None, None) => {
                    PreEqOperation::Snapshot(self.bank(&event.source, preset, us)?)
                }
                (
                    "initialize" | "adjust",
                    PreEqCaptureKind::LastUpdate,
                    Some(base_bank),
                    Some(probe),
                ) => {
                    let reference = self
                        .probes
                        .iter()
                        .find(|p| p.id == probe)
                        .ok_or_else(|| invalid("measured.missing_probe"))?;
                    let effective = active
                        .iter()
                        .rev()
                        .find(|(symbol, _)| *symbol <= reference.symbol)
                        .map(|(_, id)| *id);
                    if reference.bank_id != base_bank
                        || effective != Some(base_bank)
                        || reference.symbol >= event.symbol
                        || reference.epoch_seconds > captured.header().capture_time_epoch_seconds
                        || bank_epochs
                            .get(&base_bank)
                            .is_none_or(|epoch| *epoch > reference.epoch_seconds)
                    {
                        return Err(invalid("measured.stale_probe"));
                    }
                    self.check_time(reference.epoch_seconds)?;
                    let first = usize::from(captured.header().first_active_subcarrier);
                    let mut values = Vec::new();
                    for (offset, value) in captured.coefficients().iter().enumerate() {
                        let bin = first + offset;
                        if us.excluded().contains(&bin) {
                            if *value != Iq::default() {
                                return Err(invalid("measured.excluded_update"));
                            }
                        } else {
                            values.push((bin, *value));
                        }
                    }
                    if event.operation == "initialize" {
                        PreEqOperation::Initialize {
                            base_bank,
                            probe,
                            values,
                        }
                    } else {
                        PreEqOperation::Adjust {
                            base_bank,
                            probe,
                            values,
                        }
                    }
                }
                _ => return Err(invalid("measured.operation_context")),
            };
            active.push((event.symbol, event.source.id));
            bank_epochs.insert(event.source.id, event_epoch);
            event_sources.push(PreEqEvent {
                id: event.source.id,
                symbol: event.symbol,
                operation,
            });
        }
        for probe in &self.probes {
            self.check_time(probe.epoch_seconds)?;
            let effective = active
                .iter()
                .rev()
                .find(|(symbol, _)| *symbol <= probe.symbol)
                .map(|(_, id)| *id);
            if effective != Some(probe.bank_id)
                || probe.symbol >= us.grant().frames * us.waveform().frame_symbols()
                || bank_epochs
                    .get(&probe.bank_id)
                    .is_none_or(|epoch| *epoch > probe.epoch_seconds)
            {
                return Err(invalid("measured.probe_bank_or_time"));
            }
        }
        let probes = self
            .probes
            .iter()
            .map(|p| ProbeReference {
                id: p.id,
                bank_id: p.bank_id,
            })
            .collect();
        config.pre_eq_updates = Some(PreEqPlayback::new(
            us,
            self.pre_eq_initial_id,
            config.pre_eq.clone(),
            event_sources,
            probes,
        )?);
        docsis_ofdma::simulation::preview(&config, &options)?;
        Ok((config, options))
    }
    fn check_time(&self, epoch: u32) -> ApiResult<()> {
        if !(self.association.earliest_epoch_seconds..=self.association.latest_epoch_seconds)
            .contains(&epoch)
        {
            return Err(invalid("measured.capture_time"));
        }
        Ok(())
    }
    fn check_pre_eq(&self, capture: &PreEqCapture, preset: &ConfigurationPreset) -> ApiResult<()> {
        let h = capture.header();
        self.check_time(h.capture_time_epoch_seconds)?;
        if h.cm_mac != preset.assignment.cm_mac
            || h.cmts_mac != self.association.cmts_mac
            || h.upstream_channel_id != preset.channel.upstream_channel_id
            || h.subcarrier_zero_hz != preset.channel.subcarrier_zero_hz
            || h.subcarrier_spacing_khz != preset.channel.subcarrier_spacing_khz
        {
            return Err(invalid("measured.pre_eq_identity_or_grid"));
        }
        Ok(())
    }
    fn bank(
        &self,
        source: &CaptureSource,
        preset: &ConfigurationPreset,
        us: &UsConfiguration,
    ) -> ApiResult<PreEqBank> {
        let capture = PreEqCapture::from_pnm(&source.bytes)?;
        self.check_pre_eq(&capture, preset)?;
        if capture.kind() != PreEqCaptureKind::Applied {
            return Err(invalid("measured.expected_applied_bank"));
        }
        let mut raw = vec![Iq::default(); us.waveform().numerology().fft_len()];
        let first = usize::from(capture.header().first_active_subcarrier);
        let end = first + capture.coefficients().len();
        if (0..raw.len()).any(|bin| !us.excluded().contains(&bin) && !(first..end).contains(&bin)) {
            return Err(invalid("measured.incomplete_bank"));
        }
        raw[first..end].copy_from_slice(capture.coefficients());
        Ok(PreEqBank::normalized(us, raw, source.id)?)
    }
    /// Admits this complete capture plan as a typed core configuration.
    ///
    /// # Returns
    /// Owned resolved input, original generic context and immutable provenance.
    /// Loading remains idle and does not advance a source or noise generator.
    ///
    /// # Errors
    /// Rejects any invalid association, future update, composition, provenance
    /// serialization or resource budget before a session can be changed.
    pub fn admit(&self) -> ApiResult<MeasuredConfiguration> {
        let (configuration, options) = self.prepare()?;
        MeasuredConfiguration::new(
            configuration,
            options,
            self.configuration.clone(),
            self.provenance_json()?,
        )
    }
    /// Returns a source-preserving provenance document for completed/partial reports.
    ///
    /// Contains exact plan JSON and file bytes plus their independent SHA-256 hashes.
    ///
    /// # Returns
    /// Owned versioned JSON with original plan, source hashes and model assumptions.
    ///
    /// # Errors
    /// Rejects unsupported schema, invalid associations, future schedules, incompatible
    /// composition, resource limits or serialization failure before source consumption.
    pub fn provenance_json(&self) -> ApiResult<String> {
        self.prepare()?;
        self.configuration.to_json()?;
        #[derive(Serialize)]
        struct SourceDigest {
            family: &'static str,
            id: u64,
            sha256: String,
        }
        #[derive(Serialize)]
        struct Provenance<'plan> {
            format: &'static str,
            schema_version: u32,
            plan: &'plan MeasuredExperiment,
            original_plan_bytes: Option<&'plan [u8]>,
            source_hashes: Vec<SourceDigest>,
            model: &'static str,
        }
        let sources = self
            .ucd_fragments
            .iter()
            .map(|source| ("ucd", source))
            .chain(self.rxmer.iter().map(|source| ("rxmer", source)))
            .chain(
                self.pre_eq_initial
                    .iter()
                    .map(|source| ("pre_eq_initial", source)),
            )
            .chain(
                self.pre_eq_events
                    .iter()
                    .map(|e| ("pre_eq_event", &e.source)),
            );
        let hashes = sources
            .map(|(family, source)| SourceDigest {
                family,
                id: source.id,
                sha256: format!("{:x}", Sha256::digest(&source.bytes)),
            })
            .collect();
        bounded_json(
            &Provenance {
                format: "merlin.upstream.measured-provenance",
                schema_version: 1,
                plan: self,
                original_plan_bytes: self.original_bytes(),
                source_hashes: hashes,
                model: "normalized-corrective-bank-and-equalized-probe-target-unity-residual",
            },
            crate::common::constants::MAX_MEASURED_PROVENANCE_BYTES,
        )
    }
}
fn source_ids(sources: &[CaptureSource]) -> ApiResult<()> {
    let mut ids = BTreeSet::new();
    if sources
        .iter()
        .any(|s| s.id == 0 || s.bytes.len() > MAX_CAPTURE_SOURCE_BYTES || !ids.insert(s.id))
    {
        return Err(invalid("measured.source_identity_or_size"));
    }
    Ok(())
}
