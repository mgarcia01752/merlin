/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent codeword trials, not physical OFDM-symbol/channel simulation.
pub mod qam256;
pub use qam256::{NoiselessQam256Codec, Qam256BlockResult};
