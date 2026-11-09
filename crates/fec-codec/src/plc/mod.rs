/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PLC (384,288) punctured LDPC from PHY I20 Table21's (480,288) mother code.
mod codec;
mod decoding;
mod encoding;
mod matrix;
pub mod parameters;
pub mod puncturing;
pub use codec::{PlcBlockResult, PlcFecCodec};
pub use parameters::{PLC_CODEWORD_BITS, PLC_INFORMATION_BITS, PLC_MOTHER_BITS};
