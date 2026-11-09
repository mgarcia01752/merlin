/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! DOCSIS downstream rate8/9 shortening; independent of QAM and waveform placement.
mod codec;
pub mod decoding;
pub mod encoding;
mod parameters;
pub use codec::{compare_payload, DecodedShortenedBlock, EncodedShortenedBlock, ShortenedCodec};
pub use parameters::Shortening;
