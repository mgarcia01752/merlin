/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared measurement models and independent project restoration.
pub(crate) mod designer;
pub(crate) mod document;
pub(crate) mod model;
pub(crate) mod plc;
mod preview;
/// Independent accepted measurements reconstructed before project adoption.
pub(crate) struct RestoredMeasurements {
    pub playback: Option<crate::downstream::workbench::session::PlaybackInputs>,
    pub response: Option<merlin::downstream::ChannelEstimate>,
}
