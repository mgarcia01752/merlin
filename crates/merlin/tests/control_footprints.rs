/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Public control metadata preserves directional aliases and passive ownership.

#[cfg(feature = "downstream")]
#[test]
fn downstream_controls_preserve_legacy_types_and_passive_shared_metadata() {
    use merlin::{common, downstream};

    const FULL_LOOP_SHARED_OWNERS: usize = 2;
    const TRANSMITTER_SHARED_OWNERS: usize = 2;
    let _: fn(
        &docsis_ofdm_fec::api::FullLoopControl,
    ) -> downstream::ApiResult<common::RetainedFootprint> =
        downstream::FullLoopControl::retained_footprint;
    let _: fn(
        &docsis_ofdm_fec::api::TransmitterPreviewControl,
    ) -> downstream::ApiResult<common::RetainedFootprint> =
        downstream::TransmitterPreviewControl::retained_footprint;

    let full_loop = downstream::FullLoopControl::new();
    let full_loop_alias = full_loop.clone();
    let full_loop_owned = full_loop.retained_footprint().unwrap();
    assert_eq!(full_loop_owned.shared_roots, FULL_LOOP_SHARED_OWNERS);
    assert_eq!(full_loop_owned.ordered_entries, 0);
    assert!(full_loop_owned.payload_bytes > std::mem::size_of_val(&full_loop));
    assert_eq!(
        full_loop_alias.retained_footprint().unwrap(),
        full_loop_owned
    );
    assert_eq!(full_loop.snapshot().state, downstream::FullLoopState::Idle);
    assert_eq!(full_loop.last_progress_age_ms(), None);
    full_loop.pause().unwrap();
    assert_eq!(full_loop.retained_footprint().unwrap(), full_loop_owned);
    assert!(full_loop.snapshot().pause_requested);
    assert_eq!(full_loop.snapshot().state, downstream::FullLoopState::Idle);
    full_loop.resume();
    full_loop.cancel();
    drop(full_loop);
    assert_eq!(
        full_loop_alias.retained_footprint().unwrap(),
        full_loop_owned
    );
    assert!(full_loop_alias.snapshot().cancellation_requested);
    assert!(!full_loop_alias.snapshot().pause_requested);
    assert_eq!(full_loop_alias.last_progress_age_ms(), None);

    let transmitter = downstream::TransmitterPreviewControl::new();
    let transmitter_alias = transmitter.clone();
    let transmitter_owned = transmitter.retained_footprint().unwrap();
    assert_eq!(transmitter_owned.shared_roots, TRANSMITTER_SHARED_OWNERS);
    assert_eq!(transmitter_owned.ordered_entries, 0);
    assert!(transmitter_owned.payload_bytes > std::mem::size_of_val(&transmitter));
    assert_eq!(
        transmitter.snapshot().state,
        downstream::TransmitterPreviewState::Idle
    );
    transmitter.cancel();
    drop(transmitter);
    assert_eq!(
        transmitter_alias.retained_footprint().unwrap(),
        transmitter_owned
    );
    assert!(transmitter_alias.snapshot().cancellation_requested);
    assert_eq!(
        transmitter_alias.snapshot().state,
        downstream::TransmitterPreviewState::Idle
    );
}

#[cfg(feature = "upstream")]
#[test]
fn upstream_metadata_preserves_owner_alias_optional_common_token_and_idle_state() {
    use merlin::{common, upstream};

    const UPSTREAM_SHARED_OWNERS: usize = 1;
    const UPSTREAM_WITH_COMMON_SHARED_OWNERS: usize = 2;
    let _: fn(
        &docsis_ofdma::api::RunControl,
    ) -> docsis_ofdma::PhyResult<common::RetainedFootprint> =
        upstream::RunControl::retained_footprint;
    let plain = upstream::RunControl::new();
    let plain_owned = plain.retained_footprint().unwrap();
    assert_eq!(plain_owned.shared_roots, UPSTREAM_SHARED_OWNERS);
    assert_eq!(plain_owned.ordered_entries, 0);
    let common_stop = common::RunControl::new();
    let upstream = upstream::RunControl::with_cancellation(common_stop.clone());
    let alias = upstream.clone();
    let owned = upstream.retained_footprint().unwrap();
    assert_eq!(owned.shared_roots, UPSTREAM_WITH_COMMON_SHARED_OWNERS);
    assert_eq!(owned.ordered_entries, 0);
    assert_eq!(
        owned.payload_bytes - plain_owned.payload_bytes,
        std::mem::size_of::<std::sync::atomic::AtomicBool>()
    );
    let before = upstream.progress();
    upstream.pause();
    assert_eq!(upstream.retained_footprint().unwrap(), owned);
    assert_eq!(upstream.progress(), before);
    upstream.resume();
    assert!(!common_stop.is_cancelled());
    common_stop.cancel();
    drop(upstream);
    assert_eq!(alias.retained_footprint().unwrap(), owned);
    assert!(alias.is_cancelled());
    assert_eq!(alias.progress(), before);
    assert_eq!(plain.retained_footprint().unwrap(), plain_owned);
    assert!(!plain.is_cancelled());
}
