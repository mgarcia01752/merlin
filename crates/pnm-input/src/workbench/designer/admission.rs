/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable typed Designer admission over the existing saved-recipe evaluator.
#[path = "retention.rs"]
mod retention;
use super::{is_designer, load, Baseline, RecipeInput, Stored};
use base64::{engine::general_purpose::STANDARD, Engine};
use docsis_ofdm_fec::{
    api::{
        ApiError, ApiResult, CaptureTime, Dpd, ErrorCode, MerDb, NamedCapture, ProfileCapture,
        RxMer, ScenarioKey,
    },
    profile::candidate::CarrierAssignment,
    signal_processing::rxmer_designer::{CaptureScope, RxmerRecipe, MAX_VALUE_EDITS},
    types::modulation::CarrierType,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Application transport bound for recipe configuration, separate from capture bytes.
pub const MAX_RXMER_RECIPE_BYTES: usize = 64 * 1024;

/// One-based position in the original baseline collection, independent of playback time.
#[derive(Clone, Copy)]
pub struct RxMerDesignPosition {
    pub index: usize,
    pub count: usize,
}

impl RxMerDesignPosition {
    /// Checks a one-based position using the existing Designer collection bound.
    /// # Arguments
    /// * `index` - One-based position within the complete original collection.
    /// * `count` - Collection size, 1 through 256.
    /// # Returns
    /// Checked position; grid-times-count admission remains with the design evaluator.
    /// # Errors
    /// Preserves existing count errors and rejects an unavailable position.
    pub fn new(index: usize, count: usize) -> ApiResult<Self> {
        CaptureScope::All.validate(count)?;
        if index == 0 || index > count {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Designer position is outside the original collection",
            ));
        }
        Ok(Self { index, count })
    }
}

/// Owned original baseline/recipe and materialized observation; no simulation state.
#[derive(Clone)]
pub struct RxMerDesign {
    original: NamedCapture,
    record: Arc<RecipeInput>,
    capture: Arc<RxMer>,
}

impl RxMerDesign {
    /// Loads an existing Designer envelope, preserving its exact original bytes.
    /// # Arguments
    /// * `input` - Bounded named version-1 Designer bytes; ordinary captures reject.
    /// # Returns
    /// Immutable design and evaluated observation, without PHY simulation/source advance.
    /// # Errors
    /// Preserves strict version/schema/baseline-integrity/recipe/work-limit errors.
    pub fn from_bytes(input: &NamedCapture) -> ApiResult<Self> {
        if !is_designer(input.bytes()) {
            return Err(ApiError::new(
                ErrorCode::UnsupportedFormat,
                "Expected a versioned RxMER Designer envelope",
            ));
        }
        let capture = load(input)?;
        let record = serde_json::from_slice(input.bytes())
            .map_err(|cause| ApiError::new(ErrorCode::InvalidInput, cause.to_string()))?;
        Ok(Self {
            original: input.clone(),
            record: Arc::new(record),
            capture: Arc::new(capture),
        })
    }

    /// Creates the existing flat-baseline format from exact original profile roles/grid.
    /// # Arguments
    /// * `profile` - Accepted original identity and roles; unavailable roles stay missing.
    /// * `level` - Explicit flat baseline MER; existing evaluator enforces -30..120 dB.
    /// * `epoch` - Explicit original anchor seconds, never a generated playback timestamp.
    /// * `recipe` - Typed independent edit configuration.
    /// * `position` - One-based original collection position/count.
    /// # Returns
    /// Immutable synthetic design; no source or simulation noise stream is advanced.
    /// # Errors
    /// Preserves baseline/grid/recipe/position and final 16 MiB envelope admission.
    pub fn flat(
        profile: &Dpd,
        level: MerDb,
        epoch: CaptureTime,
        recipe: RxmerRecipe,
        position: RxMerDesignPosition,
    ) -> ApiResult<Self> {
        Self::flat_source(
            profile.key(),
            profile.carriers(),
            level,
            epoch,
            recipe,
            position,
        )
    }

    /// Creates a flat design from original source roles without admitting a simulation ID.
    /// # Arguments
    /// * `capture`, `profile_id` - Immutable source catalog and original byte identity.
    /// * `level`, `epoch` - Explicit baseline MER and original anchor seconds.
    /// * `recipe`, `position` - Existing typed edits and original collection position.
    /// # Returns
    /// Immutable synthetic design with exact source grid and role mask; no PHY executes.
    /// # Errors
    /// InvalidIdentifier for an absent source; existing flat-design errors otherwise.
    pub fn flat_capture(
        capture: &ProfileCapture,
        profile_id: u8,
        level: MerDb,
        epoch: CaptureTime,
        recipe: RxmerRecipe,
        position: RxMerDesignPosition,
    ) -> ApiResult<Self> {
        let profile = capture.frame(profile_id)?;
        Self::flat_source(
            &profile.key,
            &profile.assignments,
            level,
            epoch,
            recipe,
            position,
        )
    }

    fn flat_source(
        key: &ScenarioKey,
        assignments: &[CarrierAssignment],
        level: MerDb,
        epoch: CaptureTime,
        recipe: RxmerRecipe,
        position: RxMerDesignPosition,
    ) -> ApiResult<Self> {
        RxMerDesignPosition::new(position.index, position.count)?;
        Self::check_recipe_size(&recipe)?;
        let valid = assignments
            .iter()
            .map(|assignment| {
                !matches!(
                    assignment.carrier_type,
                    CarrierType::Exclusion | CarrierType::Unknown | CarrierType::Reserved
                )
            })
            .collect();
        Self::from_record(RecipeInput {
            merlin_rxmer_designer: 1,
            baseline: Baseline::Flat {
                key: key.clone(),
                anchor_epoch: epoch.value(),
                valid,
                baseline_db: level.value(),
            },
            recipe,
            capture_index: position.index,
            capture_count: position.count,
        })
    }

    /// Creates an immutable recipe around an ordinary original capture, retaining exact bytes.
    /// # Arguments
    /// * `profile` - Accepted original identity; mismatched device/channel/grid rejects.
    /// * `input` - Exact ordinary binary/decoded capture; nested recipes reject.
    /// * `recipe` - Typed edits; random variation remains flat-only.
    /// * `position` - One-based original collection position/count.
    /// # Returns
    /// Independently owned baseline and materialized designed RxMER.
    /// # Errors
    /// Preserves nested/integrity/format/grid/recipe and final envelope-size errors.
    pub fn imported(
        profile: &Dpd,
        input: &NamedCapture,
        recipe: RxmerRecipe,
        position: RxMerDesignPosition,
    ) -> ApiResult<Self> {
        Self::imported_source(profile.key(), input, recipe, position)
    }

    /// Wraps an ordinary baseline using an original source catalog's spatial identity.
    /// # Arguments
    /// * `capture`, `profile_id` - Immutable source catalog and original byte identity.
    /// * `input` - Exact ordinary original bytes; nested Designer baselines reject.
    /// * `recipe`, `position` - Existing typed edits and original collection position.
    /// # Returns
    /// Immutable noncompounding design; source bytes/epochs and catalog remain unchanged.
    /// # Errors
    /// InvalidIdentifier for an absent source; existing imported-design errors otherwise.
    pub fn imported_capture(
        capture: &ProfileCapture,
        profile_id: u8,
        input: &NamedCapture,
        recipe: RxmerRecipe,
        position: RxMerDesignPosition,
    ) -> ApiResult<Self> {
        Self::imported_source(&capture.frame(profile_id)?.key, input, recipe, position)
    }

    fn imported_source(
        key: &ScenarioKey,
        input: &NamedCapture,
        recipe: RxmerRecipe,
        position: RxMerDesignPosition,
    ) -> ApiResult<Self> {
        RxMerDesignPosition::new(position.index, position.count)?;
        Self::check_recipe_size(&recipe)?;
        if is_designer(input.bytes()) {
            return Err(ApiError::new(
                ErrorCode::UnsupportedCapability,
                "Nested Designer baselines are not supported",
            ));
        }
        let design = Self::from_record(RecipeInput {
            merlin_rxmer_designer: 1,
            baseline: Baseline::Imported {
                original: Stored {
                    name: input.name().to_owned(),
                    byte_length: input.bytes().len(),
                    sha256: format!("{:x}", Sha256::digest(input.bytes())),
                    base64: STANDARD.encode(input.bytes()),
                },
            },
            recipe,
            capture_index: position.index,
            capture_count: position.count,
        })?;
        design.check_key(key)?;
        Ok(design)
    }

    /// Reports the complete payload currently retained by this immutable design.
    ///
    /// Includes the fixed root, exact original input, typed baseline/recipe and
    /// cached materialized RxMER with requested spare capacities. Shared Arc
    /// payloads and owner edges are counted conservatively on every occurrence;
    /// allocator, tree-node and reference-count headers are excluded.
    /// This passive read does not copy inputs, evaluate a recipe or run science.
    ///
    /// # Returns
    /// Current-root metadata, separate from future-output reservations, temporary
    /// construction peaks, process memory and independently retained caller copies.
    ///
    /// # Errors
    /// Returns ResourceLimit if checked retained-payload arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }

    /// Checks exact spatial identity before an independently loaded design is adopted.
    /// # Arguments
    /// * `profile` - Accepted original device/channel/grid; time admission stays separate.
    /// # Returns
    /// Unit on equality without mutating either input or inferring tolerance.
    /// # Errors
    /// IncompatibleGrid for a different original device/channel/frequency grid.
    pub fn check_profile(&self, profile: &Dpd) -> ApiResult<()> {
        self.check_key(profile.key())
    }

    /// Checks original spatial association without narrowing the encoded source ID.
    /// # Arguments
    /// * `capture`, `profile_id` - Immutable catalog and original byte-sized identity.
    /// # Returns
    /// Unit on exact device/channel/grid equality, without changing epochs or recipes.
    /// # Errors
    /// InvalidIdentifier for an absent source; IncompatibleGrid for spatial mismatch.
    pub fn check_capture(&self, capture: &ProfileCapture, profile_id: u8) -> ApiResult<()> {
        self.check_key(&capture.frame(profile_id)?.key)
    }

    fn check_key(&self, key: &ScenarioKey) -> ApiResult<()> {
        if &self.capture.frame().key != key {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Designer baseline differs from the accepted profile identity/grid",
            ));
        }
        Ok(())
    }

    /// Replaces the recipe and evaluates the retained original baseline, without compounding.
    /// # Arguments
    /// * `recipe` - New typed configuration, independent of the preceding materialized result.
    /// # Returns
    /// Fresh immutable design; the original design/baseline remain unchanged.
    /// # Errors
    /// Preserves recipe/work/size errors atomically.
    pub fn with_recipe(&self, recipe: RxmerRecipe) -> ApiResult<Self> {
        let mut record = self.record.as_ref().clone();
        record.recipe = recipe;
        Self::from_record(record)
    }

    /// Admits bounded recipe JSON as a transport to the existing typed configuration.
    /// # Arguments
    /// * `bytes` - At most 64 KiB UTF-8 recipe JSON; unknown fields reject.
    /// # Returns
    /// Typed recipe; grid/count-specific validation occurs with the design baseline.
    /// # Errors
    /// Rejects transport size, malformed/unknown fields and excessive edit count.
    pub fn recipe_from_json(bytes: &[u8]) -> ApiResult<RxmerRecipe> {
        if bytes.len() > MAX_RXMER_RECIPE_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Designer recipe JSON exceeds 64 KiB",
            ));
        }
        let recipe: RxmerRecipe = serde_json::from_slice(bytes)
            .map_err(|cause| ApiError::new(ErrorCode::InvalidInput, cause.to_string()))?;
        Self::check_recipe_size(&recipe)?;
        Ok(recipe)
    }

    /// Copies the already evaluated observation into independent native ownership.
    /// # Returns
    /// Owned immutable RxMer with exact original-envelope and decoded provenance.
    pub fn materialize(&self) -> RxMer {
        self.capture.as_ref().clone()
    }

    /// Borrows exact loaded/generated envelope bytes, including original whitespace.
    /// # Returns
    /// Immutable bytes tied to this design's lifetime.
    pub fn original_bytes(&self) -> &[u8] {
        self.original.bytes()
    }

    /// Borrows original typed recipe configuration without mutation rights.
    /// # Returns
    /// Accepted recipe; adapters use validated design operations for evaluation.
    pub fn recipe(&self) -> &RxmerRecipe {
        &self.record.recipe
    }

    /// Reads the original collection position/count without inventing capture epochs.
    /// # Returns
    /// One-based original position, independent of a synthetic playback clock.
    pub fn position(&self) -> RxMerDesignPosition {
        RxMerDesignPosition {
            index: self.record.capture_index,
            count: self.record.capture_count,
        }
    }

    /// Distinguishes a flat synthetic baseline from a designed ordinary capture.
    /// # Returns
    /// True for flat synthetic, false for imported original baseline.
    pub fn is_synthetic(&self) -> bool {
        matches!(&self.record.baseline, Baseline::Flat { .. })
    }

    fn check_recipe_size(recipe: &RxmerRecipe) -> ApiResult<()> {
        if recipe.edits.len() > MAX_VALUE_EDITS {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Designer recipe exceeds the edit count bound",
            ));
        }
        Ok(())
    }

    fn from_record(record: RecipeInput) -> ApiResult<Self> {
        RxMerDesignPosition::new(record.capture_index, record.capture_count)?;
        Self::check_recipe_size(&record.recipe)?;
        let prefix = if matches!(&record.baseline, Baseline::Flat { .. }) {
            "Synthetic"
        } else {
            "Designed"
        };
        let name = format!("{prefix} RxMER {:03}", record.capture_index);
        let bytes = serde_json::to_vec(&record)
            .map_err(|cause| ApiError::new(ErrorCode::InvalidInput, cause.to_string()))?;
        Self::from_bytes(&NamedCapture::new(name, bytes)?)
    }
}
