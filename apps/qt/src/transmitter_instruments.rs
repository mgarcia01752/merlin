/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded transmitter views over immutable actual closed-loop observations.
pub(crate) mod capture;
mod messages;
mod views;

pub(crate) use messages::messages;
pub(crate) use views::{spectrum, spectrum_state, stage};
