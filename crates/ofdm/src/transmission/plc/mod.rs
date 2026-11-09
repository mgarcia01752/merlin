/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PLC message boundaries; waveform and acquisition descriptors are separate stages.
pub mod block_interleaving;
pub mod checksums;
#[cfg(feature = "simulation")]
pub mod coding;
pub mod descriptors;
#[cfg(feature = "simulation")]
pub mod frame;
pub mod geometry;
pub mod information;
pub mod mac_packets;
pub mod mapping;
pub mod message_channel;
pub mod nibbles;
pub mod preamble;
pub mod randomization;
pub mod reassembly;
pub mod scheduling;
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
pub use randomization::{DescrambledPlcBlock, PlcRandomizer, RandomizedPlcBlock};
