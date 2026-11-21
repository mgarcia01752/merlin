/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Ordered indexed transport; exact unity bypass and explicitly supported models.
mod residual;
mod stream;
pub use residual::RxmerChannel;
pub use stream::{Channel, ChannelConfig};
