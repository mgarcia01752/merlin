/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded one-result native dispatch; interpreter interrupts become cooperative cancellation.
use super::*;
use std::{
    sync::{mpsc, Mutex},
    time::Duration,
};

pub(super) fn execute(
    py: Python<'_>,
    request: api::FullLoopRequest,
    control: api::FullLoopControl,
) -> PyResult<api::FullLoopResult> {
    let (sender, receiver) = mpsc::sync_channel(1);
    let receiver = Mutex::new(receiver);
    let token = control.clone();
    let handle = std::thread::Builder::new()
        .name("merlin-downstream-api".into())
        .spawn(move || {
            let _ = sender.send(request.run(&token));
        })
        .map_err(|cause| {
            error(api::ApiError::new(
                api::ErrorCode::Engine,
                cause.to_string(),
            ))
        })?;
    loop {
        let message = py.detach(|| {
            receiver
                .lock()
                .map_or(Err(mpsc::RecvTimeoutError::Disconnected), |channel| {
                    channel.recv_timeout(Duration::from_millis(20))
                })
        });
        if py.check_signals().is_err() {
            control.cancel();
        }
        match message {
            Ok(result) => {
                let _ = py.detach(|| handle.join());
                return result.map_err(error);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                let _ = py.detach(|| handle.join());
                return Err(error(api::ApiError::new(
                    api::ErrorCode::Engine,
                    "Full-loop native worker ended without a result",
                )));
            }
        }
    }
}
