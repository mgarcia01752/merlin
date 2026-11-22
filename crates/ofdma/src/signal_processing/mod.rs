/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed upstream modulation, correction and waveform boundaries.
mod cyclic_extension;
mod demapping;
mod estimation;
mod pre_eq;
mod qam;
mod qam_tables;
mod transform;
mod transmit_window;
pub use cyclic_extension::cyclic_extension;
pub use demapping::max_log;
pub(crate) use demapping::MAX_LOG_LLR_MAGNITUDE;
pub use estimation::{equalize, estimate};
pub use pre_eq::PreEqBank;
pub use qam::{map_symbol, reference_energy};
pub use transform::Transform;
pub use transmit_window::transmit_window;
