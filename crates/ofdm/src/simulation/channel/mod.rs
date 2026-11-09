/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Duration-planned single-channel orchestration; DSP remains in stage modules.
pub mod config;
mod impairment;
pub(crate) mod link;
pub mod measurement;
pub mod results;
pub mod runner;
pub use config::ChannelRunSettings;
pub use results::{ChannelPhase, ChannelStep, ChannelSummary, ChannelWordResult};
pub use runner::ChannelRun;

pub mod accounting;

pub mod job;
pub use job::{run_channel, ChannelJobLimits, ChannelJobResult, ChannelJobStatus, ChannelProgress};
pub mod document;
