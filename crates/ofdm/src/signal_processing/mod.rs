/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reusable Rust signal-processing operations, independent of simulation jobs.
pub mod bit_loading;
pub mod interleaving;
pub mod modulation;
pub mod noise;
pub mod ofdm;
pub mod pilots;
pub mod rxmer_designer;
pub mod shannon;

pub mod channel;
pub mod receiver;
pub mod waveform;
