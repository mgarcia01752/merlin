/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Source observations, exact grids and explicit capture association.
pub mod association;
pub mod document;
pub mod grid;
pub mod observation;
#[cfg(any(feature = "simulation", feature = "pnm-input"))]
pub mod pnm_json;
pub use grid::{FrequencyGrid, ScenarioKey, MAX_LAB_CARRIERS};
pub use observation::{ChannelFrame, RxMerFrame, SourceProvenance};
