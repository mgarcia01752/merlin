/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate seeded-random and explicitly defined continuous PRBS experiment sources.
pub mod prbs;
pub mod seeded;
pub use prbs::{Prbs15, PrbsComparison, PrbsSeed, PrbsVerifier};
pub use seeded::SeededSource;
/// Reproducible noncryptographic simulation seed.
pub type SimulationSeed = u64;
