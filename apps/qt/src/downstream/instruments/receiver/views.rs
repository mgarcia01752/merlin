/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded views of real retained receiver observations.
use crate::downstream::simulation::closed_loop::latest;
use crate::downstream::workbench::session::Context;
use merlin::downstream::FullLoopStage;
use serde_json::Value;

/// Formats actual retained RX time-stage PSD or independent raw FFT-bin power.
/// # Arguments
/// * `context` - Exact downstream owner; never inferred from tab selection.
/// * `tap` - Channel output, framed, useful time samples or raw FFT bins.
/// * `start`, `count` - Original viewport, zero count selects the existing full span.
/// * `capture_axis` - Physical/baseband labels for time-stage PSD only.
/// # Returns
/// Owned bounded presentation over authoritative public power observations.
/// # Errors
/// Rejects invalid/unavailable taps and preserves numerical/viewport errors.
pub(crate) fn spectrum(
    context: &Context,
    tap: i32,
    start: i32,
    count: i32,
    capture_axis: bool,
) -> Result<Value, String> {
    let result = latest(context).ok_or("Run the closed loop to retain RX samples.")?;
    // Stable widget protocol: channel output, framing, CP removal, then FFT.
    // These ordinals select existing public observations; they are not PHY limits.
    let stage = match tap {
        0 => FullLoopStage::ChannelOutput,
        1 => FullLoopStage::RxFramed,
        2 => FullLoopStage::RxUseful,
        3 => FullLoopStage::RxFft,
        _ => return Err("Invalid tap".into()),
    };
    if stage != FullLoopStage::RxFft {
        return crate::downstream::instruments::channel::observed_spectrum(
            result.backend(),
            stage,
            None,
            start,
            count,
            capture_axis,
            "Retained contiguous RX samples from the completed/partial run",
        );
    }
    let observation = result
        .backend()
        .physical_bins(stage, 0, 0, 0)
        .map_err(|error| error.to_string())?;
    crate::shared::presentation::frequency_power::viewport(
        &observation,
        usize::try_from(start).map_err(|_| "Invalid start")?,
        usize::try_from(count).map_err(|_| "Invalid count")?,
    )
}
