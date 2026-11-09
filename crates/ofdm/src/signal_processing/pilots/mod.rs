/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate pilot-sequence, scaling and physical signal generation stages.
pub mod continuous;
mod model;
pub mod power;
pub mod scattered;
pub mod sequence;
pub use continuous::generate_continuous;
pub use model::PilotValue;
pub use power::PilotPower;
pub use scattered::generate_scattered;
pub use sequence::{PilotBit, PilotSequence};
