/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded saved-workbench import into existing Qt-independent API controllers.
mod designer;
mod full_loop;
use crate::common::constants::{
    CHANNEL_MODEL_SCHEMA_VERSION, CHANNEL_POWER_SCHEMA_VERSION, MAX_CAPTURES, MAX_EMBEDDED_BYTES,
    MAX_TOTAL_ORIGINAL_BYTES, WORKBENCH_SCHEMA_V1, WORKBENCH_SCHEMA_V2,
};
use crate::CaptureLoader;
use base64::{engine::general_purpose::STANDARD, Engine};
pub use designer::{RxMerDesign, RxMerDesignLimits, RxMerDesignPosition, MAX_RXMER_RECIPE_BYTES};
use docsis_ofdm_fec::{api::*, transmission::TransmissionSettings};
pub(crate) use full_loop::{original_byte_count, prepare};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, io::Read, path::Path, sync::Arc};

pub use crate::common::constants::MAX_WORKBENCH_BYTES;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    schema_version: u32,
    application: String,
    transmission: TransmissionSettings,
    sections: Sections,
    receiver: Option<Receiver>,
    comparison_enabled: bool,
    dark: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Sections {
    source: Source,
    profile: Profile,
    measurements: Measurements,
    channel_model: Option<Loss>,
    channel_power: Option<Power>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Stored {
    name: String,
    byte_length: usize,
    sha256: String,
    base64: String,
}
impl merlin_api::retention_support::RetainedPayload for Stored {
    /// Counts the original saved-input strings beyond their fixed inline fields.
    ///
    /// # Returns
    /// Requested name, digest and base64 capacities, or None on checked overflow.
    fn retained_children(&self) -> Option<merlin_api::RetainedFootprint> {
        use merlin_api::retention_support::sum;
        let Self {
            name,
            byte_length,
            sha256,
            base64,
        } = self;
        sum([
            name.retained_children(),
            byte_length.retained_children(),
            sha256.retained_children(),
            base64.retained_children(),
        ])
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    file: Stored,
    profile_id: u8,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    file: Option<Stored>,
    options: SourceOptions,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceOptions {
    mode: String,
    seed: u16,
    append: bool,
    target_bits: u64,
    fill_seed: u16,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Measurements {
    rxmer: Vec<Stored>,
    response: Option<Stored>,
    response_tolerance: u32,
    options: MeasurementOptions,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct MeasurementOptions {
    mode: String,
    interval_symbols: String,
    tolerance_seconds: u32,
    #[serde(default)]
    selected_captures: Option<Vec<String>>,
}
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Receiver {
    receiver_mode: ReceiverMode,
    channel_filter: docsis_ofdm_fec::signal_processing::channel::filter::ChannelFilterSettings,
    mce: docsis_ofdm_fec::signal_processing::receiver::channel_estimation::MceSettings,
    #[serde(default = "legacy_all")]
    all_profiles: bool,
    selected_profile_ids: Option<Vec<u8>>,
    duration_ms: u64,
    max_iterations: usize,
    max_runtime_ms: u64,
    noise_seed: u64,
}
fn legacy_all() -> bool {
    true
}
impl Default for Receiver {
    fn default() -> Self {
        Self {
            receiver_mode: ReceiverMode::CmChannelEstimated,
            channel_filter: Default::default(),
            mce: Default::default(),
            all_profiles: false,
            selected_profile_ids: None,
            duration_ms: 3,
            max_iterations: 50,
            max_runtime_ms: 300_000,
            noise_seed: 41,
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Loss {
    schema_version: u32,
    additional_loss: docsis_ofdm_fec::signal_processing::channel::additional_loss::AdditionalLoss,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Power {
    schema_version: u32,
    conditioning: docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning,
}
fn invalid(message: impl Into<String>) -> ApiError {
    ApiError::new(ErrorCode::InvalidInput, message)
}
fn unsupported(message: &str) -> ApiError {
    ApiError::new(ErrorCode::UnsupportedCapability, message)
}
impl Stored {
    fn decode(&self, total: &mut usize) -> ApiResult<NamedCapture> {
        NamedCapture::new(&self.name, self.decode_bytes(total)?)
    }

    fn decode_bytes(&self, total: &mut usize) -> ApiResult<Vec<u8>> {
        if self.name.len() > crate::common::constants::MAX_EMBEDDED_NAME_BYTES
            || self.byte_length > MAX_EMBEDDED_BYTES
            || self.base64.len() > crate::common::constants::MAX_EMBEDDED_BASE64_BYTES
        {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Embedded WB capture exceeds limits",
            ));
        }
        *total = total
            .checked_add(self.byte_length)
            .ok_or_else(|| invalid("Embedded byte count overflow"))?;
        if *total > MAX_TOTAL_ORIGINAL_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Combined WB originals exceed 96 MiB",
            ));
        }
        let bytes = STANDARD
            .decode(&self.base64)
            .map_err(|e| invalid(e.to_string()))?;
        if bytes.len() != self.byte_length || format!("{:x}", Sha256::digest(&bytes)) != self.sha256
        {
            return Err(invalid("Embedded WB capture length or SHA-256 mismatch"));
        }
        Ok(bytes)
    }
}
/// Validated saved WB, retaining exact JSON and one independent controller per selected profile.
/// Loading and preflight never execute simulation. Original inputs remain immutable.
pub struct Workbench {
    original: Arc<[u8]>,
    controllers: Vec<Merlin>,
}
impl Workbench {
    /// Loads a bounded self-contained WB file without running simulation.
    /// # Arguments
    /// * `path` - Existing WB JSON; embedded inputs require no external capture paths.
    /// # Returns
    /// Owned preflighted controllers and original WB bytes.
    /// # Errors
    /// Structured I/O, size, schema, integrity, unsupported-mode or engine validation error.
    pub fn from_file(path: impl AsRef<Path>) -> ApiResult<Self> {
        let metadata = std::fs::metadata(path.as_ref())
            .map_err(|e| ApiError::new(ErrorCode::Io, e.to_string()))?;
        if !metadata.is_file() {
            return Err(ApiError::new(
                ErrorCode::Io,
                "WB input must be a regular file",
            ));
        }
        if metadata.len() > MAX_WORKBENCH_BYTES as u64 {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "WB exceeds 144 MiB",
            ));
        }
        let file =
            std::fs::File::open(path).map_err(|e| ApiError::new(ErrorCode::Io, e.to_string()))?;
        let mut bytes = Vec::new();
        file.take((MAX_WORKBENCH_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|e| ApiError::new(ErrorCode::Io, e.to_string()))?;
        Self::from_bytes(&bytes)
    }
    /// Decodes saved WB schema 1/2 and preflights the full selected profile sweep.
    /// # Arguments
    /// * `bytes` - Complete bounded UTF-8 JSON, copied for independent ownership.
    /// # Returns
    /// Idle controllers using saved settings; static single/average PRBS15 only.
    /// # Errors
    /// Rejects incompatible schema, missing accepted sections, corrupt originals,
    /// unsupported source/playback, invalid profile selection or inconsistent settings.
    pub fn from_bytes(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > MAX_WORKBENCH_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "WB exceeds 144 MiB",
            ));
        }
        let doc: Document = serde_json::from_slice(bytes).map_err(|e| invalid(e.to_string()))?;
        if ![WORKBENCH_SCHEMA_V1, WORKBENCH_SCHEMA_V2].contains(&doc.schema_version)
            || doc.application != "MERLIN"
        {
            return Err(unsupported("Unsupported WB schema/application"));
        }
        doc.transmission.validate()?;
        // Presentation theme and optional UI comparison display do not change engine FEC.
        let _presentation = (doc.dark, doc.comparison_enabled);
        let receiver = doc.receiver.ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingSetting,
                "WB has no accepted receiver settings",
            )
        })?;
        if doc.schema_version == WORKBENCH_SCHEMA_V1 && receiver.selected_profile_ids.is_some() {
            return Err(invalid("Explicit profile selection requires WB schema 2"));
        }
        if !(1..=1000).contains(&receiver.duration_ms)
            || !(1..=100).contains(&receiver.max_iterations)
            || !(1..=300_000).contains(&receiver.max_runtime_ms)
        {
            return Err(invalid("Invalid saved WB run limits"));
        }
        let source = &doc.sections.source;
        if source.options.mode != "prbs15" {
            return Err(unsupported(
                "WB API currently supports PRBS15 sources; file source is unsupported",
            ));
        }
        let mut total = 0;
        if let Some(file) = &source.file {
            file.decode(&mut total)?;
        }
        // These are dormant file-source editor fields in PRBS15 mode, as in Qt.
        let _file_choices = (
            source.options.append,
            source.options.target_bits,
            source.options.fill_seed,
        );
        let seed = PrbsSeed::new(source.options.seed)?;
        let profile_input = doc.sections.profile.file.decode(&mut total)?;
        let catalog = CaptureLoader::profile_ids(&profile_input)?;
        let active = ProfileId::new(doc.sections.profile.profile_id)?;
        if !catalog.contains(&active) {
            return Err(invalid("Active WB profile is absent from capture"));
        }
        let selected = if let Some(ids) = &receiver.selected_profile_ids {
            let mut selected = Vec::new();
            for id in ids {
                let id = ProfileId::new(*id)?;
                if selected.contains(&id) || !catalog.contains(&id) {
                    return Err(invalid("Duplicate or unavailable WB profile selection"));
                }
                selected.push(id);
            }
            selected.sort_by_key(|id| id.value());
            selected
        } else if receiver.all_profiles {
            catalog
        } else {
            vec![active]
        };
        if selected.is_empty() {
            return Err(invalid("Select at least one WB profile"));
        }
        let measurements = &doc.sections.measurements;
        if measurements.rxmer.is_empty() || measurements.rxmer.len() > MAX_CAPTURES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "WB requires 1..256 RxMER originals",
            ));
        }
        measurements
            .options
            .interval_symbols
            .parse::<u64>()
            .map_err(|_| invalid("Invalid WB interval"))?;
        if !["single", "average"].contains(&measurements.options.mode.as_str()) {
            return Err(unsupported(
                "WB API supports static single/average RxMER; timeline playback is unsupported",
            ));
        }
        let selection = measurements.options.selected_captures.as_ref();
        if let Some(hashes) = selection {
            let unique: BTreeSet<_> = hashes.iter().collect();
            if hashes.is_empty()
                || unique.len() != hashes.len()
                || hashes.len() > MAX_CAPTURES
                || hashes.iter().any(|hash| {
                    !measurements
                        .rxmer
                        .iter()
                        .any(|capture| &capture.sha256 == hash)
                })
            {
                return Err(invalid("Empty, duplicate or unknown WB RxMER selection"));
            }
        }
        let mut originals = Vec::new();
        let mut selected_indices = Vec::new();
        let mut hashes = BTreeSet::new();
        for stored in &measurements.rxmer {
            if !hashes.insert(&stored.sha256) {
                return Err(invalid("Duplicate WB RxMER original"));
            }
            let capture = designer::load(&stored.decode(&mut total)?)?;
            if selection.is_none_or(|hashes| hashes.contains(&stored.sha256)) {
                selected_indices.push(originals.len());
            }
            originals.push(capture);
        }
        let response = measurements
            .response
            .as_ref()
            .map(|stored| CaptureLoader::channel_estimate(&stored.decode(&mut total)?))
            .transpose()?;
        let mut processing = ProcessingOptions {
            mce: receiver.mce,
            channel_filter: receiver.channel_filter,
            ..Default::default()
        };
        if let Some(loss) = doc.sections.channel_model {
            if loss.schema_version != CHANNEL_MODEL_SCHEMA_VERSION {
                return Err(unsupported("Unsupported WB channel model version"));
            }
            processing.additional_loss = loss.additional_loss;
        }
        if let Some(power) = doc.sections.channel_power {
            if power.schema_version != CHANNEL_POWER_SCHEMA_VERSION {
                return Err(unsupported("Unsupported WB channel power version"));
            }
            processing.power = power.conditioning;
        }
        let mut controllers = Vec::new();
        for id in selected {
            let profile = CaptureLoader::profile(&profile_input, id)?;
            let ocd = Ocd::from_profile(
                ProtocolScope::Docsis31,
                &profile,
                OcdOverrides {
                    bandwidth: Some(BandwidthHz::new(doc.transmission.occupied_bandwidth_hz)?),
                    cyclic_prefix: Some(CyclicPrefixSamples::new(
                        doc.transmission.cyclic_prefix_samples,
                    )?),
                    roll_off: Some(RollOffSamples::new(doc.transmission.roll_off_samples)?),
                    interleaver_depth: Some(InterleaverDepth::new(
                        doc.transmission.time_interleaver_depth,
                    )?),
                },
            )?;
            if profile.grid().spacing_hz != doc.transmission.subcarrier_spacing.frequency_hz() {
                return Err(invalid("Saved OCD spacing contradicts embedded DPD"));
            }
            let mut sim = Merlin::new(ocd);
            sim.set_profile(profile)?;
            // WB bytes retain unchecked originals; import selected captures in saved order.
            let ids = selected_indices
                .iter()
                .map(|index| &originals[*index])
                .map(|capture| sim.add_rxmer(capture.clone()))
                .collect::<ApiResult<Vec<_>>>()?;
            let chosen = if measurements.options.mode == "single" {
                RxMerSelection::Individual(ids[0])
            } else {
                RxMerSelection::AverageAll
            };
            let channel = if let Some(response) = &response {
                // Qt requires profile/H timestamp association even for a fixed response.
                if response
                    .provenance()
                    .original_epoch()
                    .value()
                    .abs_diff(sim.profile().unwrap().sources()[0].original_epoch().value())
                    > measurements.response_tolerance
                {
                    return Err(ApiError::new(
                        ErrorCode::AssociationFailed,
                        "WB channel/profile timestamps exceed response tolerance",
                    ));
                }
                ChannelSelection::Explicit(sim.add_channel_estimate(response.clone())?)
            } else {
                ChannelSelection::Unity
            };
            sim.set_options(EvaluationOptions {
                rxmer_selection: chosen,
                channel_selection: Some(channel),
                receiver_mode: receiver.receiver_mode,
                processing: processing.clone(),
                observation: ObservationDuration::new(receiver.duration_ms)?,
                noise_seed: NoiseSeed::new(receiver.noise_seed)?,
                prbs_seed: seed,
                profile_tolerance: AssociationTolerance::new(
                    measurements.options.tolerance_seconds,
                )?,
                limits: RunLimits {
                    runtime: RuntimeLimit::new(receiver.max_runtime_ms)?,
                    iterations: IterationLimit::new(receiver.max_iterations)?,
                    ..Default::default()
                },
            })?;
            sim.prepare()?;
            controllers.push(sim);
        }
        Ok(Self {
            original: Arc::from(bytes),
            controllers,
        })
    }
    /// Shares exact original WB bytes without copying or exposing mutable state.
    /// # Returns
    /// Independently owned immutable storage, valid after this WB is dropped.
    pub fn original_document(&self) -> Arc<[u8]> {
        self.original.clone()
    }
    /// Borrows preflighted independent profile controllers without running them.
    /// # Returns
    /// Controllers sorted by captured profile ID, borrowing this WB.
    pub fn controllers(&self) -> &[Merlin] {
        &self.controllers
    }
    /// Transfers idle controllers for adapters or callers needing manual overrides.
    /// # Returns
    /// Owned controllers; each keeps original captures and controller-scoped IDs.
    pub fn into_controllers(self) -> Vec<Merlin> {
        self.controllers
    }
    /// Explicitly runs saved settings through the existing engine, profile by profile.
    /// # Returns
    /// One owned batch per profile; identifiers remain scoped to their own controller.
    /// # Errors
    /// Structured preflight errors; engine partial results remain in batch entries.
    pub fn run(&mut self) -> ApiResult<Vec<BatchReport>> {
        self.run_with_control(&RunControl::new())
    }
    /// Runs saved settings with one cancellation token shared across all profiles.
    /// # Arguments
    /// * `control` - Cooperative cancellation; later profiles become not-started.
    /// # Returns
    /// Owned complete/partial/not-started batches for every selected profile.
    /// # Errors
    /// Existing API preflight errors, without invoking Qt or a CLI subprocess.
    pub fn run_with_control(&mut self, control: &RunControl) -> ApiResult<Vec<BatchReport>> {
        self.controllers
            .iter_mut()
            .map(|sim| sim.run_with_control(control))
            .collect()
    }
}
