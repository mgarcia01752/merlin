/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate pilot orchestration, result and observation modules.
mod generator;
mod observation;
mod results;
pub use generator::PilotGenerator;
pub use results::PilotStageResult;
