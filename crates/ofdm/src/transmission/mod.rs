/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Fixed downstream transmission configuration and sample-clock scheduling.
pub mod layout;
pub mod packing;
pub mod settings;
pub mod timing;
pub use settings::{SubcarrierSpacing, TransmissionSettings};
pub use timing::TransmissionTiming;
