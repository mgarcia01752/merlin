/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Duration-planned single-channel orchestration; DSP remains in stage modules.
pub mod config;
pub(crate) mod link;
pub mod results;
pub mod runner;
pub use config::ChannelRunSettings;
pub use results::{ChannelPhase, ChannelStep, ChannelSummary, ChannelWordResult};
pub use runner::ChannelRun;
