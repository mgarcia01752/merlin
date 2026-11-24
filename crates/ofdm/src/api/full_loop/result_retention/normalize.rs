/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact final capacities for moved values; bounded temporary builders remain separate.
use crate::{
    profile::candidate::CapturedProfile,
    scenario::{ChannelFrame, RxMerFrame, SourceProvenance},
    signal_processing::channel::{
        filter::ChannelFilterSettings,
        impairments::{ChannelImpairments, NoiseModel},
    },
    simulation::channel::{measurement_playback::PlaybackConfiguration, ChannelRunSettings},
};

/// Moves initialized elements through their exact boxed layout.
/// # Arguments
/// * `values` - Owned final vector; no element is copied or removed.
/// # Returns
/// Unit with capacity equal to length for nonzero-sized elements.
/// Temporary conversion storage is separate.
pub(in crate::api) fn vector<T>(values: &mut Vec<T>) {
    *values = std::mem::take(values).into_boxed_slice().into_vec();
}

/// Moves unchanged UTF-8 bytes through their exact boxed layout.
/// # Arguments
/// * `value` - Owned final text; content is preserved exactly.
/// # Returns
/// Unit with byte capacity equal to byte length.
pub(in crate::api) fn text(value: &mut String) {
    *value = std::mem::take(value).into_boxed_str().into_string();
}

/// Normalizes original source metadata without changing any identity or value.
/// # Arguments
/// * `value` - Owned source metadata prepared before execution.
/// # Returns
/// Unit with exact owned text capacities.
pub(in crate::api) fn source(value: &mut SourceProvenance) {
    let SourceProvenance {
        name,
        sha256,
        byte_length: _,
        capture_time: _,
        file_kind,
        format_major: _,
        format_minor: _,
    } = value;
    text(name);
    text(sha256);
    text(file_kind);
}

/// Normalizes an unchanged captured profile's owned storage.
/// # Arguments
/// * `value` - Owned profile; the scenario key is fixed-size.
/// # Returns
/// Unit with exact assignment/text capacities.
pub(in crate::api) fn profile(value: &mut CapturedProfile) {
    let CapturedProfile {
        key: _,
        source: provenance,
        capture_time: _,
        profile_id: _,
        assignments,
    } = value;
    source(provenance);
    vector(assignments);
}

/// Normalizes one original residual observation without recalculation.
/// # Arguments
/// * `value` - Owned measured frame, including original nulls.
/// # Returns
/// Unit with exact sample/text capacities.
pub(in crate::api) fn rxmer(value: &mut RxMerFrame) {
    let RxMerFrame {
        key: _,
        source: provenance,
        capture_time: _,
        mer_db,
    } = value;
    source(provenance);
    vector(mer_db);
}

/// Normalizes original H storage without changing its reference plane.
/// # Arguments
/// * `value` - Owned channel frame, including original nulls.
/// # Returns
/// Unit with exact sample/text capacities.
pub(in crate::api) fn response(value: &mut ChannelFrame) {
    let ChannelFrame {
        key: _,
        source: provenance,
        capture_time: _,
        iq,
    } = value;
    source(provenance);
    vector(iq);
}

/// Normalizes the configured filter's sole dynamic variant.
/// # Arguments
/// * `value` - Accepted filter policy; scalar variants own no payload.
/// # Returns
/// Unit without fitting or applying a filter.
pub(in crate::api) fn filter(value: &mut ChannelFilterSettings) {
    match value {
        ChannelFilterSettings::Diagonal => {}
        ChannelFilterSettings::Echoes { paths } => vector(paths),
        ChannelFilterSettings::CapturedFir {
            max_delay_samples: _,
            causal_shift_samples: _,
            maximum_rms_error: _,
            maximum_bin_error: _,
        } => {}
    }
}

fn impairments(value: &mut ChannelImpairments) {
    let ChannelImpairments {
        seed: _,
        noise,
        response: channel,
        additional_loss: _,
        power: _,
    } = value;
    match noise {
        NoiseModel::Noiseless | NoiseModel::Awgn { sample_power: _ } => {}
        NoiseModel::Rxmer { frame } => rxmer(frame),
    }
    if let Some(frame) = channel {
        response(frame);
    }
}

/// Freezes all nonshared settings storage before the execution token is admitted.
/// # Arguments
/// * `value` - Owned clone of validated settings; file Arcs keep immutable ownership.
/// # Returns
/// Unit with exact destination capacities and unchanged scientific values.
pub(in crate::api) fn settings(value: &mut ChannelRunSettings) {
    let ChannelRunSettings {
        source: _,
        data_bit_loading,
        impairment_timeline,
        playback,
        protected_constellation: _,
        receiver_mode: _,
        channel_filter,
        mce: _,
        constellation: _,
        impairments: configured,
        transmission: _,
        layout,
        duration: _,
        ncp_modulation: _,
        prbs_seed: _,
        llr_variance: _,
        max_iterations: _,
        receive_block_samples: _,
    } = value;
    if let Some(loading) = data_bit_loading {
        vector(loading);
    }
    for epoch in impairment_timeline.iter_mut() {
        let crate::simulation::channel::timeline::ImpairmentEpoch {
            first_observation_symbol: _,
            capture_time: _,
            impairments: configured,
        } = epoch;
        impairments(configured);
    }
    vector(impairment_timeline);
    if let Some(PlaybackConfiguration {
        profile: captured,
        frames,
        mode: _,
        interval_symbols: _,
        tolerance_seconds: _,
    }) = playback
    {
        profile(captured);
        for frame in frames.iter_mut() {
            rxmer(frame);
        }
        vector(frames);
    }
    filter(channel_filter);
    impairments(configured);
    self::layout(layout);
}

/// Normalizes both owned physical layout vectors without rebuilding geometry.
/// # Arguments
/// * `value` - Validated immutable destination snapshot.
/// # Returns
/// Unit with exact excluded/pilot capacities and unchanged physical indices.
pub(in crate::api) fn layout(value: &mut crate::transmission::layout::CarrierLayoutSettings) {
    let crate::transmission::layout::CarrierLayoutSettings {
        zero_frequency_hz: _,
        first_active_bin: _,
        active_end_exclusive: _,
        plc_first_bin: _,
        excluded_bins,
        additional_continuous_pilots,
    } = value;
    vector(excluded_bins);
    vector(additional_continuous_pilots);
}
