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

    /// Optional native admission and saved-project operations over existing owners.
    #[cfg(feature = "native-input")]
    pub mod inputs {
        pub use pnm_input::{
            read_capture, rxmer_export, save_json, CaptureLoader, CaptureScope, Notch,
            RandomVariation, Ripple, RippleSpacing, RxMerDesign, RxMerDesignPosition, RxmerRecipe,
            Tilt, ValueEdit, Workbench, MAX_RXMER_RECIPE_BYTES,
        };
    }
}

/// Upstream OFDMA API over the independent received-only simulation engine.
#[cfg(feature = "upstream")]
pub mod upstream {
    pub use docsis_ofdma::api::*;

    /// Optional provenance-preserving UCD, RxMER, Pre-Eq and association admission.
    #[cfg(feature = "native-input")]
    pub mod inputs {
        pub use pnm_input::upstream::{
            reassemble_ucd, CaptureAssociation, CaptureEvent, CaptureProbe, CaptureSource,
            MeasuredExperiment, PreEqCapture, PreEqCaptureKind, PreEqHeader, RxmerCapture,
            RxmerHeader, UcdBurst, UcdCapture, UcdHeader, UcdTlv,
        };
    }
}

/// Genuinely shared contracts, without universal configurations or reports.
pub mod common {
    pub use merlin_api::{Cancellation, ErrorCode, RunControl};
}
