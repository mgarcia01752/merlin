/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! File: examples/api/rust/run_multi_rxmer_mce.rs
//! Run every captured profile using received-pilot channel correction and received-only decoder noise estimation (MCE).
//! See support/multi_rxmer.rs for named settings and the complete walkthrough.
//! This fixture has no CM response capture: explicit unity supplies its flat reference.
#[path = "support/multi_rxmer.rs"]
mod multi_rxmer;
use docsis_ofdm_fec::api::ReceiverMode;

const RECEIVER_MODE: ReceiverMode = ReceiverMode::MerlinChannelEstimated;

/// Runs the shared collection walkthrough with this example's receiver mode.
///
/// # Returns
/// Unit after printing actual per-profile status, FEC, BER and wall time.
/// # Errors
/// Propagates file and API errors; partial results remain visible in the output.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    multi_rxmer::run(RECEIVER_MODE)
}
