/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reference and pilot-estimated receiver stages; full blind acquisition remains later work.
pub mod equalization;
pub mod extraction;
pub mod timing;

#[cfg(feature = "simulation")]
pub mod payload;

pub mod channel_estimation;
