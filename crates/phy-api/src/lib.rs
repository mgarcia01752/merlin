/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Direction-neutral contracts, independent of scientific implementations and adapters.
#![forbid(unsafe_code)]
mod control;
mod error;
mod retention;
pub use control::{Cancellation, RunControl};
pub use error::ErrorCode;
/// Cross-crate ownership-walker support, excluded from the high-level facade.
#[doc(hidden)]
pub use retention::support as retention_support;
pub use retention::RetainedFootprint;
