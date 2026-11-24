/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Direct Rust control metadata reference for installed native Python parity.
use merlin::{common, downstream, upstream};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let full_loop = downstream::FullLoopControl::new();
    let transmitter = downstream::TransmitterPreviewControl::new();
    let plain_upstream = upstream::RunControl::new();
    let common_stop = common::RunControl::new();
    let shared_upstream = upstream::RunControl::with_cancellation(common_stop.clone());
    let mut observations = Vec::new();
    observations.push(observe(
        "idle",
        &full_loop,
        &transmitter,
        &plain_upstream,
        &shared_upstream,
    )?);
    full_loop.pause()?;
    plain_upstream.pause();
    shared_upstream.pause();
    observations.push(observe(
        "pause_requested",
        &full_loop,
        &transmitter,
        &plain_upstream,
        &shared_upstream,
    )?);
    full_loop.resume();
    plain_upstream.resume();
    shared_upstream.resume();
    observations.push(observe(
        "resumed",
        &full_loop,
        &transmitter,
        &plain_upstream,
        &shared_upstream,
    )?);
    full_loop.cancel();
    transmitter.cancel();
    plain_upstream.cancel();
    common_stop.cancel();
    observations.push(observe(
        "cancelled",
        &full_loop,
        &transmitter,
        &plain_upstream,
        &shared_upstream,
    )?);
    println!("{}", serde_json::to_string(&observations)?);
    Ok(())
}

fn observe(
    phase: &str,
    full_loop: &downstream::FullLoopControl,
    transmitter: &downstream::TransmitterPreviewControl,
    plain_upstream: &upstream::RunControl,
    shared_upstream: &upstream::RunControl,
) -> Result<Value, Box<dyn std::error::Error>> {
    Ok(json!({
        "phase": phase,
        "full_loop": {
            "footprint": full_loop.retained_footprint()?,
            "snapshot": full_loop.snapshot(),
            "age_ms": full_loop.last_progress_age_ms(),
        },
        "transmitter": {
            "footprint": transmitter.retained_footprint()?,
            "snapshot": transmitter.snapshot(),
        },
        "upstream": {
            "footprint": plain_upstream.retained_footprint()?,
            "progress": plain_upstream.progress(),
            "is_cancelled": plain_upstream.is_cancelled(),
        },
        "upstream_with_common": {
            "footprint": shared_upstream.retained_footprint()?,
            "progress": shared_upstream.progress(),
            "is_cancelled": shared_upstream.is_cancelled(),
        },
    }))
}
