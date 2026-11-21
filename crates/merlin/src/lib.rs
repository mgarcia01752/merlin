/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Unified public namespaces re-exporting independent scientific owners.
#![forbid(unsafe_code)]

/// Existing downstream API, with unchanged nominal types and behavior.
#[cfg(feature = "downstream")]
pub mod downstream {
    pub use docsis_ofdm_fec::api::*;
}

/// Upstream OFDMA API over the independent received-only simulation engine.
#[cfg(feature = "upstream")]
pub mod upstream {
    pub use docsis_ofdma::api::*;
}

/// Genuinely shared contracts, without universal configurations or reports.
pub mod common {
    pub use merlin_api::{Cancellation, ErrorCode, RunControl};
}
