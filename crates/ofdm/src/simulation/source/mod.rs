/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate seeded-random, continuous PRBS and finite exact-byte file experiment sources.
pub mod prbs;
#[cfg(feature = "simulation")]
pub mod seeded;
pub use prbs::{Prbs15, PrbsComparison, PrbsSeed, PrbsVerifier};
#[cfg(feature = "simulation")]
pub use seeded::SeededSource;
/// Reproducible noncryptographic simulation seed.
pub type SimulationSeed = u64;

#[cfg(feature = "simulation")]
pub mod observation;

pub mod file;
mod selection;
pub use selection::PayloadSourceConfig;
#[cfg(feature = "simulation")]
pub(crate) use selection::{PayloadSource, PayloadVerifier};
