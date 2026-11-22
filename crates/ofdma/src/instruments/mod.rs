/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Passive bounded owned QAM/SA snapshots; queries never rerun science.
mod metrics;
mod recovery;
pub use recovery::{
    FecSummary, FecWordPage, FecWordRequest, FecWordSnapshot, VerificationSnapshot,
    MAX_FEC_PAGE_DECISION_BITS, MAX_FEC_PAGE_WORDS,
};
mod observer;
pub use metrics::{MatchedMer, MerValue};
mod spectrum;
pub use observer::{ObservationStore, QamPoint, QamSnapshot, Tap};
pub use spectrum::SpectrumSnapshot;
