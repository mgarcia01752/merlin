/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent safe Rust upstream OFDMA configuration and processing.
#![forbid(unsafe_code)]

pub mod bit_processing;
pub mod configuration;
mod error;
pub mod grant;
pub mod resources;
pub mod source;
pub mod types;

pub use error::{PhyError, PhyResult};
