/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! High-level, Qt-free simulation API with owned inputs and results.
//! Loading/configuration never executes jobs; only explicit run methods start simulation.
mod configuration;
mod controller;
mod error;
mod inputs;
mod reports;
mod simulation;
mod units;
pub use crate::signal_processing::receiver::channel_estimation::ReceiverMode;
pub use crate::transmission::SubcarrierSpacing;
pub use crate::types::ProtocolScope;
pub use configuration::*;
pub use controller::*;
pub use error::{ApiError, ApiResult, ErrorCode};
pub use inputs::*;
pub use reports::*;
pub use simulation::Simulation;
pub use units::*;
