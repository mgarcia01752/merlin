/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Array representation for numerical I/Q without a PHY serialization contract.

/// Cartesian `[real, imaginary]` values in the caller's explicit reference plane.
///
/// This numerical alias does not replace either PHY's public or saved-input types.
/// Transform admission checks both components before allocating output.
pub type ComplexSample = [f64; 2];
