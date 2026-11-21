/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Passive bounded owned QAM/SA snapshots; queries never rerun science.
mod metrics;
mod observer;
pub use metrics::{MatchedMer, MerValue};
mod spectrum;
pub use observer::{ObservationStore, QamPoint, QamSnapshot, Tap};
pub use spectrum::SpectrumSnapshot;
