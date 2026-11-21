/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Direction-neutral contracts, independent of scientific implementations and adapters.
#![forbid(unsafe_code)]
mod control;
mod error;
pub use control::{Cancellation, RunControl};
pub use error::ErrorCode;
