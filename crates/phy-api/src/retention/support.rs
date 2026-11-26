/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Cross-crate ownership walkers; implementation support, not facade operations.
use super::RetainedFootprint;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::mem::{size_of, size_of_val};
use std::sync::Arc;

/// Owner implementation reporting dynamic children beyond its fixed value.
///
/// Implementations must visit cached/private ownership and requested spare
/// capacity. Borrowed static data is not owned. Do not count inline fields twice.
pub trait RetainedPayload {
    /// Measures reachable dynamic children without cloning or running science.
    ///
    /// # Returns
    /// Additional quantities, or None if any checked arithmetic overflows.
    fn retained_children(&self) -> Option<RetainedFootprint>;
}

/// Measures one root's fixed value and all dynamic children.
///
/// # Arguments
/// * `value` - Borrowed root whose owner implements the complete child audit.
///
/// # Returns
/// Checked quantities, or None on overflow; no payload allocation is performed.
pub fn retained_footprint<Value: RetainedPayload + ?Sized>(
    value: &Value,
) -> Option<RetainedFootprint> {
    sum([
        Some(payload_bytes(size_of_val(value))),
        value.retained_children(),
    ])
}

/// Combines independently audited child quantities using checked arithmetic.
///
/// # Arguments
/// * `parts` - Finite owner-provided metadata; None propagates overflow.
///
/// # Returns
/// Aggregate quantities, or None if a part or any addition overflows.
pub fn sum(
    parts: impl IntoIterator<Item = Option<RetainedFootprint>>,
) -> Option<RetainedFootprint> {
    parts.into_iter().try_fold(
        RetainedFootprint::default(),
        |total, part| -> Option<RetainedFootprint> {
            let part = part?;
            Some(RetainedFootprint {
                payload_bytes: total.payload_bytes.checked_add(part.payload_bytes)?,
                ordered_entries: total.ordered_entries.checked_add(part.ordered_entries)?,
                shared_roots: total.shared_roots.checked_add(part.shared_roots)?,
            })
        },
    )
}

/// Describes payload storage without claiming entries or shared owners.
///
/// # Arguments
/// * `bytes` - Independently measured fixed value or requested storage bytes.
///
/// # Returns
/// Metadata containing only the supplied byte quantity.
pub fn payload_bytes(bytes: usize) -> RetainedFootprint {
    RetainedFootprint {
        payload_bytes: bytes,
        ..RetainedFootprint::default()
    }
}

/// Counts requested sequence capacity without constructing or allocating values.
///
/// # Arguments
/// * `Value` - Stored element type; initialized children are counted separately.
/// * `capacity` - Requested element slots, including unused capacity.
///
/// # Returns
/// Storage bytes, or None if multiplication overflows the target usize.
pub fn capacity_bytes<Value>(capacity: usize) -> Option<RetainedFootprint> {
    Some(payload_bytes(capacity.checked_mul(size_of::<Value>())?))
}

/// Describes an explicitly audited Copy leaf without owned dynamic children.
///
/// # Arguments
/// * `Value` - Owner-audited fixed leaf, such as Duration or fixed codec counters;
///   a later heap-owning representation must still satisfy the Copy bound.
/// * `_value` - Borrowed inline leaf. Any borrowed/static payload belongs to its
///   actual owner; Copy alone does not audit arbitrary pointed-to storage.
///
/// # Returns
/// Zero dynamic children; the enclosing root/container counts fixed bytes once.
///
/// A heap-owning leaf cannot silently pass this fixed-storage audit:
///
/// ```compile_fail
/// use merlin_api::retention_support::fixed_children;
/// let owned_payload = vec![0_u8];
/// let _ = fixed_children(&owned_payload);
/// ```
pub fn fixed_children<Value: Copy>(_value: &Value) -> Option<RetainedFootprint> {
    Some(RetainedFootprint::default())
}

impl<Value: RetainedPayload> RetainedPayload for Vec<Value> {
    /// Counts requested capacity and each initialized element's dynamic children.
    ///
    /// # Returns
    /// Additional vector storage quantities, or None on checked overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum(std::iter::once(capacity_bytes::<Value>(self.capacity()))
            .chain(self.iter().map(RetainedPayload::retained_children)))
    }
}

impl<Value: RetainedPayload> RetainedPayload for VecDeque<Value> {
    /// Counts both ring segments using total capacity and initialized children.
    ///
    /// # Returns
    /// Additional deque storage quantities, or None on checked overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum(std::iter::once(capacity_bytes::<Value>(self.capacity()))
            .chain(self.iter().map(RetainedPayload::retained_children)))
    }
}

impl RetainedPayload for String {
    /// Counts requested UTF-8 storage capacity, including unused bytes.
    ///
    /// # Returns
    /// Capacity bytes with no additional container or shared-owner counts.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        Some(payload_bytes(self.capacity()))
    }
}

impl RetainedPayload for str {
    /// Counts no children beyond the string slice storage measured by its owner.
    ///
    /// # Returns
    /// Zero; Box/Arc owners measure the dynamically sized pointee separately.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        Some(RetainedFootprint::default())
    }
}

impl<Value: RetainedPayload> RetainedPayload for Option<Value> {
    /// Visits the present value's children without counting inline storage again.
    ///
    /// # Returns
    /// Zero for None, or the present children; overflow propagates.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Some(value) => value.retained_children(),
            None => Some(RetainedFootprint::default()),
        }
    }
}

impl<Value: RetainedPayload, const LENGTH: usize> RetainedPayload for [Value; LENGTH] {
    /// Visits initialized inline array children without counting array bytes again.
    ///
    /// # Returns
    /// Checked child aggregate, or None on overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum(self.iter().map(RetainedPayload::retained_children))
    }
}

impl<First: RetainedPayload, Second: RetainedPayload> RetainedPayload for (First, Second) {
    /// Visits both tuple fields without counting their inline storage again.
    ///
    /// # Returns
    /// Checked child aggregate, or None on overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum([self.0.retained_children(), self.1.retained_children()])
    }
}

impl<Value: RetainedPayload> RetainedPayload for [Value] {
    /// Visits initialized slice children without counting slice bytes again.
    ///
    /// # Returns
    /// Checked child aggregate; the Box/Arc root accounts for slice storage.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum(self.iter().map(RetainedPayload::retained_children))
    }
}

impl<Value: RetainedPayload + ?Sized> RetainedPayload for Box<Value> {
    /// Visits the owned allocation including its fixed or dynamically sized value.
    ///
    /// # Returns
    /// Pointee payload quantities, excluding allocator bookkeeping.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        retained_footprint(self.as_ref())
    }
}

impl<Value: RetainedPayload + ?Sized> RetainedPayload for Arc<Value> {
    /// Counts this shared edge and its whole reachable payload conservatively.
    ///
    /// # Returns
    /// Pointee quantities plus one root, excluding private reference-count headers.
    /// Repeated aliases deliberately repeat payloads; no deduplication is allocated.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum([
            retained_footprint(self.as_ref()),
            Some(RetainedFootprint {
                shared_roots: 1,
                ..RetainedFootprint::default()
            }),
        ])
    }
}

impl<Key: RetainedPayload, Value: RetainedPayload> RetainedPayload for BTreeMap<Key, Value> {
    /// Counts entries, their fixed payloads and children, excluding private nodes.
    ///
    /// # Returns
    /// Checked payload/entry/child quantities, or None on overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum(std::iter::once(Some(RetainedFootprint {
            ordered_entries: self.len(),
            ..RetainedFootprint::default()
        }))
        .chain(
            self.iter()
                .map(|(key, value)| sum([retained_footprint(key), retained_footprint(value)])),
        ))
    }
}

impl<Value: RetainedPayload> RetainedPayload for BTreeSet<Value> {
    /// Counts entries and owned key payloads independently of private tree nodes.
    ///
    /// # Returns
    /// Checked payload/entry/child quantities, or None on overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        sum(std::iter::once(Some(RetainedFootprint {
            ordered_entries: self.len(),
            ..RetainedFootprint::default()
        }))
        .chain(self.iter().map(retained_footprint)))
    }
}

macro_rules! fixed_payload {
    ($($value:ty),+ $(,)?) => {
        $(impl RetainedPayload for $value {
            /// Reports no dynamic children for an inline primitive.
            ///
            /// # Returns
            /// Zero; the enclosing root or container counts its fixed storage.
            fn retained_children(&self) -> Option<RetainedFootprint> {
                Some(RetainedFootprint::default())
            }
        })+
    };
}

fixed_payload!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char
);
