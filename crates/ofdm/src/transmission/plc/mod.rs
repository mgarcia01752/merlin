/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PLC message boundaries; waveform and acquisition descriptors are separate stages.
pub mod block_interleaving;
#[cfg(feature = "simulation")]
pub mod coding;
#[cfg(feature = "simulation")]
pub mod frame;
pub mod geometry;
pub mod information;
pub mod message_channel;
pub mod nibbles;
pub mod timestamp;
#[cfg(feature = "simulation")]
pub mod timestamp_block;
#[cfg(feature = "simulation")]
pub use frame::PlcMessageFrame;
pub use geometry::PlcFrameGeometry;
pub use information::PlcInformationBlock;
pub use message_channel::MessageChannelBlock;
pub use timestamp::ExtendedTimestamp;
#[cfg(feature = "simulation")]
pub use timestamp_block::TimestampMessageBlock;

#[cfg(feature = "simulation")]
pub use coding::{DecodedPlcBlock, EncodedPlcBlock, PlcBlockCodec};

pub use block_interleaving::InterleavedPlcBlock;
pub use nibbles::{PlcCodewordNibbles, PlcNibble};
