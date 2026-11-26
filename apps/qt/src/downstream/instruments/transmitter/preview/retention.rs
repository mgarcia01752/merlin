/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete prepared idle-TX owners, cross-host aliases and pre-generation product admission.
use super::{PreparedPreview, PreviewInput};
use crate::shell::{Retained, RetainedResources, RetentionBudget, RetentionError};
use merlin::{
    common::RetainedFootprint,
    downstream::{
        ApiError, PhysicalLayoutCarrier, PhysicalRoleSpan, TransmissionSettings,
        TransmitterPreview, TransmitterPreviewControl, TransmitterPreviewRequest,
    },
};
use std::{mem::size_of, sync::Arc};

// One immutable prepared allocation plus its separate retained-wrapper allocation.
const PREPARED_SHARED_ROOTS: usize = 2;

pub(super) type SharedPrepared = Arc<Retained<Arc<PreparedPreview>>>;

#[derive(Debug)]
pub(crate) enum PreviewError {
    Metadata(ApiError),
    Retention(RetentionError),
    Unavailable,
    TransferNotIdle,
    ForeignBudget,
    PersistentMetadata(String),
}

impl PreviewError {
    /// Formats existing backend or finite application diagnostics for the current UI.
    ///
    /// # Returns
    /// Original backend text or fixed application wording. Diagnostic String/control/
    /// channel infrastructure is a separate retained-owner consumer, not covered here.
    pub(crate) fn into_message(self) -> String {
        match self {
            Self::Metadata(error) => error.message().to_owned(),
            Self::Retention(
                RetentionError::ArithmeticOverflow
                | RetentionError::LimitExceeded { .. }
                | RetentionError::OrderedEntriesExceeded { .. }
                | RetentionError::SharedRootsExceeded { .. }
                | RetentionError::ChannelsExceeded { .. }
                | RetentionError::SlotsExceeded { .. }
                | RetentionError::PresentationUtf16BytesExceeded { .. }
                | RetentionError::PresentationEntriesExceeded { .. }
                | RetentionError::RetainedViewsExceeded { .. }
                | RetentionError::OpenViewsExceeded { .. },
            ) => "The session retained-data limit would be exceeded.".into(),
            Self::PersistentMetadata(error) => error,
            Self::Unavailable => "No accepted TX configuration.".into(),
            Self::TransferNotIdle => "Only an idle prepared transmitter can change host.".into(),
            Self::ForeignBudget => {
                "Prepared transmitter belongs to another application host.".into()
            }
        }
    }
}

/// Admits actual prepared settings/request/layout before final shared-owner allocation.
///
/// # Arguments
/// * `prepared` - Finite temporary candidate; original source and scientific values survive.
/// * `budget` - Canonical accepting host counter, also covering old and candidate owners.
///
/// # Returns
/// Two-root immutable owner; no opaque scientific graph is reconstructed or cloned.
///
/// # Errors
/// Checked metadata/layout arithmetic or any aggregate dimension before adoption.
pub(super) fn admit(
    prepared: PreparedPreview,
    budget: &RetentionBudget,
) -> Result<SharedPrepared, PreviewError> {
    let reservation = budget
        .reserve_resources(prepared_resources(&prepared)?)
        .map_err(PreviewError::Retention)?;
    Ok(Arc::new(reservation.retain(Arc::new(prepared))))
}

/// Reserves complete target-host coverage before aliasing the same immutable inner root.
///
/// # Arguments
/// * `source` - Original charged owner; existing worker/old aliases remain independent.
/// * `budget` - Exact destination counter; no clone-capacity equality is assumed.
///
/// # Returns
/// Newly target-charged outer wrapper, with the identical inner prepared allocation.
///
/// # Errors
/// Rejects metadata/target pressure before making a new alias or replacing source state.
pub(super) fn rebind(
    source: &SharedPrepared,
    budget: &RetentionBudget,
) -> Result<SharedPrepared, PreviewError> {
    let reservation = budget
        .reserve_resources(prepared_resources(source.value())?)
        .map_err(PreviewError::Retention)?;
    Ok(Arc::new(reservation.retain(Arc::clone(source.value()))))
}

/// Reserves an actual public future target before backend control claim or source advance.
///
/// # Arguments
/// * `prepared` - Admitted immutable source owner shared with the accepting analyzer.
/// * `control` - Existing one-shot public preview control, including cancellation.
/// * `budget` - Canonical host counter covering source, old/queued and new products.
///
/// # Returns
/// Charged actual product ready to move through the bounded Box queue and current slot.
/// Heavy bound preparation and generation both run on the existing worker thread.
///
/// # Errors
/// Public bound/engine errors or any target dimension, preserving prior accepted products.
pub(super) fn generate(
    prepared: SharedPrepared,
    control: &TransmitterPreviewControl,
    budget: &RetentionBudget,
) -> Result<Retained<TransmitterPreview>, PreviewError> {
    let preview = prepared
        .value()
        .preview
        .as_ref()
        .ok_or(PreviewError::Unavailable)?;
    let limit = preview
        .request
        .preview_retained_bound()
        .map_err(PreviewError::Metadata)?;
    let reservation = budget
        .reserve_resources(product_resources(limit).map_err(PreviewError::Retention)?)
        .map_err(PreviewError::Retention)?;
    let result = preview
        .request
        .generate_bounded(control, limit)
        .map_err(PreviewError::Metadata)?;
    Ok(reservation.retain(result))
}

fn add(left: usize, right: usize) -> Result<usize, PreviewError> {
    left.checked_add(right)
        .ok_or(PreviewError::Retention(RetentionError::ArithmeticOverflow))
}
fn multiply(left: usize, right: usize) -> Result<usize, PreviewError> {
    left.checked_mul(right)
        .ok_or(PreviewError::Retention(RetentionError::ArithmeticOverflow))
}
fn fixed<Value: Copy>(_: &Value) {}

/// Measures this actual prepared graph, including requested public layout capacities.
///
/// # Arguments
/// * `prepared` - Borrowed unchanged candidate or inner shared root; no allocations here.
///
/// # Returns
/// Concrete prepared+Retained wrapper layout, request children and two shared edges.
/// TransmissionConfiguration is audited to own only fixed TransmissionSettings;
/// the current public carrier/span fields are fixed leaves. Non_exhaustive layout
/// additions and private configuration changes require reviewing this owner audit.
///
/// # Errors
/// Public metadata or checked aggregate/vector-layout overflow.
pub(super) fn prepared_resources(
    prepared: &PreparedPreview,
) -> Result<RetainedResources, PreviewError> {
    let PreparedPreview { timing, preview } = prepared;
    let TransmissionSettings {
        occupied_bandwidth_hz,
        subcarrier_spacing,
        cyclic_prefix_samples,
        roll_off_samples,
        time_interleaver_depth,
    } = timing.settings();
    fixed(occupied_bandwidth_hz);
    fixed(subcarrier_spacing);
    fixed(cyclic_prefix_samples);
    fixed(roll_off_samples);
    fixed(time_interleaver_depth);
    let mut payload_bytes = add(
        size_of::<PreparedPreview>(),
        size_of::<Retained<Arc<PreparedPreview>>>(),
    )?;
    let mut ordered_entries = 0;
    let mut shared_roots = PREPARED_SHARED_ROOTS;
    if let Some(PreviewInput { request, layout }) = preview {
        // The external layout structs are non_exhaustive. Audit all current fields
        // and require their leaves to remain Copy; newly added fields need review.
        for PhysicalLayoutCarrier {
            physical_bin,
            frequency_hz,
            role,
            ..
        } in &layout.carriers
        {
            fixed(physical_bin);
            fixed(frequency_hz);
            fixed(role);
        }
        for PhysicalRoleSpan {
            first_bin,
            end_exclusive,
            role,
            ..
        } in &layout.role_spans
        {
            fixed(first_bin);
            fixed(end_exclusive);
            fixed(role);
        }
        let metadata = request
            .retained_footprint()
            .map_err(PreviewError::Metadata)?;
        let children = metadata
            .payload_bytes
            .checked_sub(size_of::<TransmitterPreviewRequest>())
            .ok_or(PreviewError::Retention(RetentionError::ArithmeticOverflow))?;
        payload_bytes = add(payload_bytes, children)?;
        payload_bytes = add(
            payload_bytes,
            multiply(
                layout.carriers.capacity(),
                size_of::<PhysicalLayoutCarrier>(),
            )?,
        )?;
        payload_bytes = add(
            payload_bytes,
            multiply(layout.role_spans.capacity(), size_of::<PhysicalRoleSpan>())?,
        )?;
        ordered_entries = metadata.ordered_entries;
        shared_roots = add(shared_roots, metadata.shared_roots)?;
    }
    Ok(RetainedResources {
        payload_bytes,
        ordered_entries,
        shared_roots,
        ..Default::default()
    })
}

/// Adds only the actual product reservation wrapper to a complete public scientific bound.
///
/// # Arguments
/// * `scientific` - Exact public target bound, including the preview's fixed root once.
///
/// # Returns
/// Complete retained-product layout and original scientific entries/shared roots.
/// Box queue and current slot move this same owner, introducing no extra scientific Arc.
///
/// # Errors
/// Checked root-replacement or wrapper arithmetic overflow.
pub(super) fn product_resources(
    scientific: RetainedFootprint,
) -> Result<RetainedResources, RetentionError> {
    let children = scientific
        .payload_bytes
        .checked_sub(size_of::<TransmitterPreview>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: children
            .checked_add(size_of::<Retained<TransmitterPreview>>())
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: scientific.ordered_entries,
        shared_roots: scientific.shared_roots,
        ..Default::default()
    })
}

/// Admits one actual preview control before shared publication or worker dispatch.
///
/// # Arguments
/// * `budget` - Canonical host counter covering prepared, queued and old products.
///
/// # Returns
/// A single guarded owner shared by the Job and worker; no raw control clone escapes.
///
/// # Errors
/// Public metadata, checked wrapper arithmetic or target pressure before generation.
pub(super) fn admit_control(
    budget: &RetentionBudget,
) -> Result<Arc<Retained<TransmitterPreviewControl>>, PreviewError> {
    let control = TransmitterPreviewControl::new();
    let scientific = control
        .retained_footprint()
        .map_err(PreviewError::Metadata)?;
    let reservation = budget
        .reserve_resources(control_resources(scientific).map_err(PreviewError::Retention)?)
        .map_err(PreviewError::Retention)?;
    Ok(Arc::new(reservation.retain(control)))
}

/// Replaces the once-counted scientific handle with its actual retained wrapper.
///
/// # Arguments
/// * `scientific` - Complete public control metadata for this actual target layout.
///
/// # Returns
/// Shared retained-control quantities; allocator headers stay outside payload bytes.
///
/// # Errors
/// Checked root replacement or shared-root addition overflow.
pub(super) fn control_resources(
    scientific: RetainedFootprint,
) -> Result<RetainedResources, RetentionError> {
    let children = scientific
        .payload_bytes
        .checked_sub(size_of::<TransmitterPreviewControl>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: children
            .checked_add(size_of::<Retained<TransmitterPreviewControl>>())
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: scientific.ordered_entries,
        shared_roots: scientific
            .shared_roots
            .checked_add(1)
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ..Default::default()
    })
}
