/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable descriptions of existing Designer domains; validation remains with its owner.
use super::RxMerDesign;
use docsis_ofdm_fec::{
    common::constants::{
        MAX_DESIGNED_RXMER_DB, MAX_DESIGN_ABSOLUTE_TILT_DB, MAX_DESIGN_ECHO_DISTANCE_METRES,
        MAX_DESIGN_NOTCH_DEPTH_DB, MAX_DESIGN_RANDOM_STANDARD_DEVIATION_DB,
        MAX_DESIGN_RIPPLE_PEAK_TO_PEAK_DB, MIN_DESIGNED_RXMER_DB, MIN_DESIGN_ECHO_DISTANCE_METRES,
    },
    signal_processing::rxmer_designer::{MAX_DESIGN_CAPTURES, MAX_VALUE_EDITS},
};
use serde::Serialize;

/// Existing input/resource limits, separate from measured observations or PHY requirements.
#[derive(Clone, Copy, Serialize)]
#[non_exhaustive]
pub struct RxMerDesignLimits {
    /// Inclusive baseline/edit lower bound in dB.
    pub minimum_mer_db: f64,
    /// Inclusive baseline/edit upper bound in dB.
    pub maximum_mer_db: f64,
    /// Complete original collection size, independent of playback selection.
    pub maximum_captures: usize,
    /// Original-grid values across the collection, independent of encoded byte budgets.
    pub maximum_values: usize,
    /// Explicit value-edit rules, independent of capture count.
    pub maximum_value_edits: usize,
    /// Inclusive random-variation standard deviation in dB.
    pub maximum_random_standard_deviation_db: f64,
    /// Inclusive magnitude of signed tilt in dB.
    pub maximum_absolute_tilt_db: f64,
    /// Inclusive ripple peak-to-peak excursion in dB.
    pub maximum_ripple_peak_to_peak_db: f64,
    /// Inclusive notch depth in dB.
    pub maximum_notch_depth_db: f64,
    /// Inclusive positive echo-distance lower bound in metres.
    pub minimum_echo_distance_metres: f64,
    /// Inclusive echo-distance upper bound in metres.
    pub maximum_echo_distance_metres: f64,
}

impl RxMerDesign {
    /// Reports the existing scientific/input domains without evaluating a recipe.
    /// # Returns
    /// Owned named bounds read from their authoritative constants; no input, source,
    /// simulation or saved schema changes. Playback and UI limits remain separate.
    pub fn limits() -> RxMerDesignLimits {
        RxMerDesignLimits {
            minimum_mer_db: MIN_DESIGNED_RXMER_DB,
            maximum_mer_db: MAX_DESIGNED_RXMER_DB,
            maximum_captures: MAX_DESIGN_CAPTURES,
            maximum_values: crate::common::constants::MAX_DESIGNER_COLLECTION_VALUES,
            maximum_value_edits: MAX_VALUE_EDITS,
            maximum_random_standard_deviation_db: MAX_DESIGN_RANDOM_STANDARD_DEVIATION_DB,
            maximum_absolute_tilt_db: MAX_DESIGN_ABSOLUTE_TILT_DB,
            maximum_ripple_peak_to_peak_db: MAX_DESIGN_RIPPLE_PEAK_TO_PEAK_DB,
            maximum_notch_depth_db: MAX_DESIGN_NOTCH_DEPTH_DB,
            minimum_echo_distance_metres: MIN_DESIGN_ECHO_DISTANCE_METRES,
            maximum_echo_distance_metres: MAX_DESIGN_ECHO_DISTANCE_METRES,
        }
    }
}
