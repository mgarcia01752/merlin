/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! High-level, Qt-free simulation API with owned inputs and results.
//! Loading/configuration never executes jobs; only explicit run methods start simulation.
mod admission;
mod configuration;
mod controller;
mod error;
mod inputs;
mod measurement;
mod reports;
mod sequence;
mod simulation;
mod units;
pub use crate::scenario::{FrequencyGrid, ScenarioKey};
pub use crate::signal_processing::receiver::channel_estimation::ReceiverMode;
pub use crate::transmission::SubcarrierSpacing;
pub use crate::transmission::{DurationTiming, TransmissionSettings};
pub use crate::types::ProtocolScope;
pub use admission::{
    FileFillPolicy, SourceInput, TransmissionConfiguration, MAX_TRANSMISSION_DOCUMENT_BYTES,
};
pub use configuration::*;
pub use controller::*;
pub use error::{ApiError, ApiResult, ErrorCode};
pub use inputs::*;
pub use measurement::*;
pub use reports::*;
pub use sequence::*;
pub use simulation::Simulation;
pub use units::*;
