/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded numerical leaves; PHY configuration, state, units and errors stay with callers.
#![forbid(unsafe_code)]

pub mod common;
pub mod complex;
mod error;
pub mod prbs15;
pub mod transform;

pub use complex::ComplexSample;
pub use error::DspError;
