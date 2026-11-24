/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive validated editor inputs; execution and scientific stages keep existing owners.
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
mod source_info;
use super::{ApiError, ApiResult, Dpd, ErrorCode, NamedCapture, ObservationDuration, PrbsSeed};
use crate::{
    simulation::source::{
        file::{FilePayload, PrbsFill},
        PayloadSourceConfig,
    },
    transmission::{DurationTiming, TransmissionSettings},
};
use serde::Serialize;
use sha2::{Digest, Sha256};
pub use source_info::{SourceFileInfo, SourceFileLimits};
use std::time::Duration;

/// Existing desktop timing-editor byte ceiling, independent of PHY sample limits.
pub const MAX_TRANSMISSION_DOCUMENT_BYTES: usize = 4096;

/// Validated immutable numerology; accepted source/profile and run state are separate.
#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct TransmissionConfiguration {
    settings: TransmissionSettings,
}

/// Explicit finite append policy with a separately checked PRBS15 fill seed.
#[derive(Clone, Copy, Serialize)]
pub struct FileFillPolicy {
    /// Total meaningful file-plus-fill bits; checked against original input on admission.
    pub target_bits: u64,
    /// Independent fill recurrence seed; never a replacement for PHY scrambling.
    pub seed: PrbsSeed,
}

/// Passive owned source selection; no source advances and no verifier is retained.
#[derive(Clone)]
pub struct SourceInput {
    pub(crate) configuration: PayloadSourceConfig,
    seed: Option<PrbsSeed>,
    name: Option<String>,
    sha256: Option<String>,
}

impl FileFillPolicy {
    /// Validates an independent finite fill policy before a file is selected.
    ///
    /// # Arguments
    /// * `target_bits` - Total meaningful bits, at most the existing finite-file bound.
    /// * `seed` - Checked nonzero PRBS15 fill seed, independent of continuous source state.
    ///
    /// # Returns
    /// An admitted policy; the selected file separately checks its minimum length.
    ///
    /// # Errors
    /// Preserves existing finite target/work admission without generating any bits.
    pub fn new(target_bits: u64, seed: PrbsSeed) -> ApiResult<Self> {
        FilePayload::new(
            &[],
            Some(PrbsFill {
                target_bits,
                seed: seed.value(),
            }),
        )?;
        Ok(Self { target_bits, seed })
    }
}

impl TransmissionConfiguration {
    /// Validates and owns a numerology snapshot without changing any live session.
    ///
    /// # Arguments
    /// * `settings` - Existing typed bandwidth, spacing, prefix, taper and depth.
    ///
    /// # Returns
    /// Immutable admitted settings, independent of subsequent caller edits.
    ///
    /// # Errors
    /// Preserves existing bandwidth/CP/RP/depth validation and unsupported combinations.
    pub fn new(settings: TransmissionSettings) -> ApiResult<Self> {
        settings.validate()?;
        Ok(Self { settings })
    }

    /// Admits the existing bounded timing-editor JSON through the typed constructor.
    ///
    /// # Arguments
    /// * `bytes` - UTF-8 settings JSON, at most 4096 bytes; unknown fields reject.
    ///
    /// # Returns
    /// A fresh immutable snapshot; loading never runs or changes another input.
    ///
    /// # Errors
    /// Rejects byte limits, malformed/unknown fields and existing numerology failures.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > MAX_TRANSMISSION_DOCUMENT_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "OCD request exceeds 4096 bytes.",
            ));
        }
        let settings = serde_json::from_slice(bytes)
            .map_err(|cause| ApiError::new(ErrorCode::InvalidInput, cause.to_string()))?;
        Self::new(settings)
    }

    /// Borrows exactly the admitted scientific settings without mutation rights.
    ///
    /// # Returns
    /// Existing typed settings valid for this snapshot's lifetime.
    pub fn settings(&self) -> &TransmissionSettings {
        &self.settings
    }

    /// Plans complete source-symbol timing using the existing integer-clock owner.
    ///
    /// # Arguments
    /// * `duration` - Checked observation duration in milliseconds.
    ///
    /// # Returns
    /// Exact sample/symbol/tail accounting, excluding acquisition and interleaver drain.
    ///
    /// # Errors
    /// Preserves overflow or insufficient-complete-symbol errors; starts no processing.
    pub fn timing(&self, duration: ObservationDuration) -> ApiResult<DurationTiming> {
        Ok(self
            .settings
            .duration_timing(Duration::from_millis(duration.value()))?)
    }

    /// Queries the minimum whole-millisecond source window completing a selected symbol.
    ///
    /// # Arguments
    /// * `last_symbol_index` - Zero-based source-observation OFDM index, not wall time.
    ///
    /// # Returns
    /// The smallest whole milliseconds containing more complete periods than this index.
    /// Acquisition, interleaver drain and window tails are separate. The threshold may
    /// exceed checked observation/UI/runtime budgets; this neither admits nor runs a job.
    /// Inputs remain unchanged and no source, waveform, receiver or RNG is advanced.
    ///
    /// # Errors
    /// Preserves existing timing validation and checked-arithmetic/output errors.
    pub fn minimum_observation_milliseconds_after(&self, last_symbol_index: u64) -> ApiResult<u64> {
        Ok(
            crate::transmission::timing::minimum_observation_milliseconds_after(
                self.settings(),
                last_symbol_index,
            )?,
        )
    }

    /// Plans the bounded Welch-average capacity of an explicit sample budget.
    ///
    /// # Arguments
    /// * `max_samples` - Sample-count ceiling, including at least one accepted FFT.
    ///
    /// # Returns
    /// Maximum complete overlapping analysis windows fitting that budget, capped
    /// by the existing spectrum owner's work policy. Actual retained availability
    /// is separate. Inputs stay unchanged; no source, FFT, receiver or RNG advances.
    /// Oversized positive budgets cannot widen analysis work or allocate samples.
    ///
    /// # Errors
    /// Rejects fewer than one complete FFT through the shared numerical owner.
    pub fn spectrum_average_capacity(&self, max_samples: usize) -> ApiResult<usize> {
        Ok(
            crate::signal_processing::waveform::spectrum::average_capacity(
                self.settings.subcarrier_spacing,
                max_samples,
            )?,
        )
    }

    /// Checks the existing profile editor's spacing and occupied-envelope contract.
    ///
    /// # Arguments
    /// * `profile` - Previously admitted immutable profile on its original grid.
    ///
    /// # Returns
    /// Unit for editor compatibility; full TX/transport readiness is a separate operation.
    ///
    /// # Errors
    /// Rejects mismatched spacing, overflow or a source grid outside the accepted channel.
    pub fn check_profile(&self, profile: &Dpd) -> ApiResult<()> {
        self.check_profile_grid(profile.grid())
    }

    /// Checks editor geometry for an original source ID without admitting a run profile.
    ///
    /// # Arguments
    /// * `capture` - Immutable complete source catalog retaining its original grid/provenance.
    /// * `profile_id` - Original byte-sized source identity, independent of checked run IDs.
    ///
    /// # Returns
    /// Unit when the selected source fits the existing spacing/FFT/bandwidth contract.
    /// Inputs remain unchanged; no source, channel or receiver is created or executed.
    ///
    /// # Errors
    /// InvalidIdentifier for an absent source; existing IncompatibleGrid errors otherwise.
    /// Success does not establish supported coding, layout or simulation readiness.
    pub fn check_profile_capture(
        &self,
        capture: &super::ProfileCapture,
        profile_id: u8,
    ) -> ApiResult<()> {
        self.check_profile_grid(&capture.profile_ref(profile_id)?.key.grid)
    }

    pub(crate) fn check_profile_grid(&self, grid: &super::FrequencyGrid) -> ApiResult<()> {
        if grid.spacing_hz != self.settings.subcarrier_spacing.frequency_hz() {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Capture spacing differs from the accepted OCD settings.",
            ));
        }
        let end = usize::from(grid.first_subcarrier_index).checked_add(grid.sample_count);
        let span = (grid.sample_count as u64).checked_mul(u64::from(grid.spacing_hz));
        if end.is_none_or(|end| end > self.settings.subcarrier_spacing.fft_samples() as usize)
            || span.is_none_or(|span| span > u64::from(self.settings.occupied_bandwidth_hz))
        {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Profile carrier span does not fit the accepted OCD channel.",
            ));
        }
        Ok(())
    }
}

impl SourceInput {
    /// Retains a checked continuous PRBS15 seed without creating a generator.
    ///
    /// # Arguments
    /// * `seed` - Valid nonzero 15-bit recurrence state, distinct from PHY scrambling.
    ///
    /// # Returns
    /// Passive repeatable input; every future explicit run constructs its own stream.
    pub fn prbs15(seed: PrbsSeed) -> Self {
        Self {
            configuration: PayloadSourceConfig::Prbs15,
            seed: Some(seed),
            name: None,
            sha256: None,
        }
    }

    /// Owns an exact-byte file selection and optional finite independently seeded fill.
    ///
    /// # Arguments
    /// * `input` - Original named bytes; extensions never select content or bit ordering.
    /// * `fill` - Optional total meaningful-bit target and independent fill seed.
    ///
    /// # Returns
    /// Immutable validated file source, including empty input, without advancing bits.
    ///
    /// # Errors
    /// Rejects existing 16 MiB/work ceilings or a target shorter than the original file.
    pub fn file(input: NamedCapture, fill: Option<FileFillPolicy>) -> ApiResult<Self> {
        Self::file_with_label(input.name().to_owned(), input.bytes(), fill)
    }

    /// Owns an exact-byte file whose original display label is explicitly absent.
    ///
    /// # Arguments
    /// * `bytes` - Original file bytes, including zeros, BOM and line endings.
    /// * `fill` - Optional finite total-bit target and independent checked fill seed.
    ///
    /// # Returns
    /// Immutable finite file source with the original empty label, exact bytes/hash
    /// and no continuous seed. Empty files are accepted. No bits or RNG advance;
    /// caller bytes can be changed or dropped after admission. Named capture/file
    /// constructors retain their existing nonempty-identity contract.
    ///
    /// # Errors
    /// Preserves existing file byte/work limits and rejects fill below original size.
    pub fn unnamed_file(bytes: &[u8], fill: Option<FileFillPolicy>) -> ApiResult<Self> {
        Self::file_with_label(String::new(), bytes, fill)
    }

    /// Reports the scientific source family without inspecting a filename.
    ///
    /// # Returns
    /// `prbs15` for the continuous test recurrence, or `file` for finite exact bytes.
    pub fn kind(&self) -> &'static str {
        match self.configuration {
            PayloadSourceConfig::Prbs15 => "prbs15",
            PayloadSourceConfig::File { .. } => "file",
        }
    }

    /// Reads the continuous-source seed, separate from any finite append policy.
    ///
    /// # Returns
    /// Some for PRBS15; None for a file whose fill seed belongs to its own policy.
    pub fn prbs_seed(&self) -> Option<PrbsSeed> {
        self.seed
    }

    /// Reads the original file identity without using it as a format hint.
    ///
    /// # Returns
    /// Borrowed display name for files, None for continuous PRBS15.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Borrows exact original file bytes, excluding generated fill and coding padding.
    ///
    /// # Returns
    /// Original immutable bytes for finite files; None for continuous PRBS15.
    pub fn original_bytes(&self) -> Option<&[u8]> {
        match &self.configuration {
            PayloadSourceConfig::Prbs15 => None,
            PayloadSourceConfig::File { payload } => Some(payload.bytes()),
        }
    }

    /// Reads the original input's cryptographic identity without regenerating content.
    ///
    /// # Returns
    /// Lowercase SHA-256 for files; None for a recurrence with no original byte file.
    pub fn original_sha256(&self) -> Option<&str> {
        self.sha256.as_deref()
    }

    /// Reports the finite meaningful-bit target before FEC padding and observation windows.
    ///
    /// # Returns
    /// File-plus-fill target, including zero for an empty file; None for continuous PRBS.
    pub fn target_bits(&self) -> Option<u64> {
        match &self.configuration {
            PayloadSourceConfig::Prbs15 => None,
            PayloadSourceConfig::File { payload } => Some(payload.target_bits()),
        }
    }

    /// Measures this complete immutable root and all reachable retained payloads.
    ///
    /// Includes requested spare buffer capacity, private caches and each owned
    /// Arc pointee/edge, conservatively repeating aliases. Ordered entries and
    /// shared roots are separate from payload bytes; allocator, tree-node and
    /// refcount bookkeeping are excluded. No source or numerical state advances.
    ///
    /// # Returns
    /// Passive current-root metadata including its fixed value exactly once.
    /// This does not reserve future output, construction scratch or copied views.
    ///
    /// # Errors
    /// Returns ResourceLimit if retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }

    fn file_with_label(
        name: String,
        bytes: &[u8],
        fill: Option<FileFillPolicy>,
    ) -> ApiResult<Self> {
        let payload = FilePayload::new(
            bytes,
            fill.map(|policy| PrbsFill {
                target_bits: policy.target_bits,
                seed: policy.seed.value(),
            }),
        )?;
        Ok(Self {
            configuration: PayloadSourceConfig::File { payload },
            seed: None,
            name: Some(name),
            sha256: Some(format!("{:x}", Sha256::digest(bytes))),
        })
    }
}

impl RetainedPayload for TransmissionConfiguration {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { settings } = self;
        merlin_api::retention_support::sum([settings.retained_children()])
    }
}

impl RetainedPayload for SourceInput {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            configuration,
            seed,
            name,
            sha256,
        } = self;
        merlin_api::retention_support::sum([
            configuration.retained_children(),
            seed.retained_children(),
            name.retained_children(),
            sha256.retained_children(),
        ])
    }
}
