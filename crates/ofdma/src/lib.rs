/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent safe Rust upstream OFDMA configuration and processing.
#![forbid(unsafe_code)]

pub mod api;
pub mod bit_processing;
pub mod channel;
pub mod common;
pub mod configuration;
mod error;
pub mod grant;
pub mod instruments;
pub mod measurements;
pub mod receiver;
pub mod resources;
pub mod signal_processing;
pub mod simulation;
pub mod source;
pub mod transmitter;
pub mod types;

pub use error::{PhyError, PhyResult};
