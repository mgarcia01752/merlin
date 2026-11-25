/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reusable DSP calculations, independent of parsers, JSON and desktop APIs.
pub mod channel_response;
pub mod decision_boundary;
pub mod moving_average;
pub mod ofdm;
pub mod qam;
pub mod shannon;
pub mod spectrum;
pub mod spectrum_power;

pub mod bit_loading;

pub mod pre_equalization;
