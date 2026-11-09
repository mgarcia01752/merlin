/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Physical carrier roles, distinct from cells before time/frequency interleaving.
pub mod ncp;
pub mod physical;
pub use physical::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole};
