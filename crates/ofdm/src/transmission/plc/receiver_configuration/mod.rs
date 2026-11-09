/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Encapsulated receiver acquisition: received packets become a complete data-plane configuration.
pub(crate) mod channel;
mod collection;
mod configuration;
mod profile;
mod state;
pub use channel::ReceivedChannel;
pub use collection::MissingDescriptors;
pub use configuration::ReceiverConfiguration;
pub use profile::ReceivedProfile;
pub use state::{ReceiverConfigurationState, ReceiverConfigurationStatus};
