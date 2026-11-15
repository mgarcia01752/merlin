/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! High-level, Qt-free, owned-input simulation high-level API. Loading never executes jobs.
mod configuration;
mod controller;
mod error;
mod inputs;
mod reports;
mod units;
pub use crate::transmission::SubcarrierSpacing;
pub use crate::types::ProtocolScope;
pub use configuration::*;
pub use controller::*;
pub use error::{ApiError, ApiResult, ErrorCode};
pub use inputs::*;
pub use reports::*;
pub use units::*;
