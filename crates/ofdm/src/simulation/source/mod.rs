/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Source generation; defined PRBS patterns will be implemented here separately.
pub mod seeded;
pub use seeded::SeededSource;
/// Reproducible noncryptographic simulation seed.
pub type SimulationSeed = u64;
