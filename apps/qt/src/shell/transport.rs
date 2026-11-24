/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit std-channel populations and separately admitted application deliveries.
use super::{Retained, RetainedResources, RetentionBudget, RetentionError, RetentionReservation};
use std::{
    mem::{size_of, size_of_val},
    sync::{mpsc, Arc},
    time::Duration,
};

type ChannelGuard = Arc<Retained<()>>;

// The shared witness is one real Arc allocation, independent of its endpoints.
const CHANNEL_GUARD_SHARED_ROOTS: usize = 1;

// One successful constructor creates one live std bounded channel owner.
const BOUNDED_CHANNEL_OWNERS: usize = 1;

/// Producer with one shared channel/population owner, independent of deliveries.
pub struct DeliverySender<T> {
    sender: mpsc::SyncSender<Retained<T>>,
    budget: RetentionBudget,
    guard: ChannelGuard,
}

/// Receiver whose disposal releases backpressure without waiting for a worker.
pub struct DeliveryReceiver<T> {
    receiver: mpsc::Receiver<Retained<T>>,
    _guard: ChannelGuard,
}

/// One pre-admitted frame, including its sender until publication or disposal.
pub struct DeliveryPermit<T> {
    sender: mpsc::SyncSender<Retained<T>>,
    reservation: RetentionReservation,
    _guard: ChannelGuard,
}

/// Actual boxed closure data and its last, nonoptional fixed-layout reservation.
pub struct AdmittedTask {
    operation: Box<dyn FnOnce() + Send>,
    _reservation: RetentionReservation,
}

/// Creates an ordinary bounded std queue after admitting its application owner.
///
/// # Arguments
/// * `T` - Delivery value; independently guarded children must remain guarded.
/// * `budget` - Exact canonical host budget shared with queued and old products.
/// * `capacity` - Configured slot count; zero preserves std rendezvous behavior.
///
/// # Returns
/// Endpoints sharing one exact guard pointee/root and one channel owner. Empty
/// slots count as population, not guessed std storage bytes. Private std
/// synchronization/bookkeeping is excluded. A detached delivery keeps its own
/// frame reservation after the last endpoints/permits are gone.
///
/// # Errors
/// Any resource ceiling before queue or shared guard allocation.
pub fn bounded_channel<T>(
    budget: &RetentionBudget,
    capacity: usize,
) -> Result<(DeliverySender<T>, DeliveryReceiver<T>), RetentionError> {
    let reservation = budget.reserve_resources(RetainedResources {
        payload_bytes: size_of::<Retained<()>>(),
        shared_roots: CHANNEL_GUARD_SHARED_ROOTS,
        channel_count: BOUNDED_CHANNEL_OWNERS,
        slot_count: capacity,
        ..RetainedResources::default()
    })?;
    let guard = Arc::new(reservation.retain(()));
    let (sender, receiver) = mpsc::sync_channel(capacity);
    Ok((
        DeliverySender {
            sender,
            budget: budget.clone(),
            guard: Arc::clone(&guard),
        },
        DeliveryReceiver {
            receiver,
            _guard: guard,
        },
    ))
}

impl<T> DeliverySender<T> {
    /// Reserves an actual delivery frame before producing its message value.
    ///
    /// # Returns
    /// A single-use permit charging the complete Retained<T> layout. Its sender
    /// keeps channel populations alive through blocked publication. There is no
    /// capacity-plus-one estimate: every independently held delivery stays charged.
    ///
    /// # Errors
    /// Checked resource admission without producing, executing or publishing data.
    pub fn reserve(&self) -> Result<DeliveryPermit<T>, RetentionError> {
        self.reserve_bytes(size_of::<Retained<T>>())
    }

    /// Reserves the largest actual source/adopted frame layout for common workers.
    ///
    /// # Arguments
    /// * `bytes` - Audited fixed layouts; never less than this raw retained frame.
    ///
    /// # Returns
    /// One delivery permit also covering a known terminal's adopted layout.
    ///
    /// # Errors
    /// Resource rejection before creating the direction's scientific output.
    pub(super) fn reserve_bytes(&self, bytes: usize) -> Result<DeliveryPermit<T>, RetentionError> {
        let reservation = self.budget.reserve_resources(RetainedResources {
            payload_bytes: bytes.max(size_of::<Retained<T>>()),
            ..RetainedResources::default()
        })?;
        Ok(DeliveryPermit {
            sender: self.sender.clone(),
            reservation,
            _guard: Arc::clone(&self.guard),
        })
    }
}

impl<T> DeliveryPermit<T> {
    /// Publishes the reserved frame with worker-side std backpressure.
    ///
    /// # Arguments
    /// * `value` - Produced only after this permit's admission. Dynamic child
    ///   allocations require their existing independent reservations.
    ///
    /// # Returns
    /// Unit after enqueue. The receiver retains the frame through actual disposal.
    ///
    /// # Errors
    /// Returns the original value on disconnection; its child guards remain owned.
    /// The failed frame no longer exists, so its fixed reservation is released.
    /// The returned raw value is transient caller execution storage and must be
    /// disposed or moved into an independently admitted destination immediately.
    pub fn send(self, value: T) -> Result<(), T> {
        self.sender
            .send(self.reservation.retain(value))
            .map_err(|error| error.0.into_parts().0)
    }
}

impl<T> DeliveryReceiver<T> {
    /// Polls one admitted delivery without waiting for its producer.
    ///
    /// # Returns
    /// A retained frame; keep its guard while consuming the value. Moving already
    /// guarded children into an admitted destination does not clone their data.
    ///
    /// # Errors
    /// Ordinary std Empty/Disconnected outcomes, without releasing other owners.
    pub fn try_recv(&self) -> Result<Retained<T>, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    /// Waits for one admitted delivery on a worker or fixture thread.
    ///
    /// # Returns
    /// Original retained frame; this method is not intended for a GUI event loop.
    ///
    /// # Errors
    /// Disconnected after the final producer/permit has gone.
    pub fn recv(&self) -> Result<Retained<T>, mpsc::RecvError> {
        self.receiver.recv()
    }

    /// Waits for one delivery within a caller's finite worker/fixture deadline.
    ///
    /// # Arguments
    /// * `timeout` - Maximum std receive duration; does not cancel the operation.
    ///
    /// # Returns
    /// Original retained frame, with no data copy or reservation release.
    ///
    /// # Errors
    /// Ordinary std timeout/disconnection outcomes.
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Retained<T>, mpsc::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }
}

/// Admits the actual closure body and its concrete task wrapper before boxing.
///
/// # Arguments
/// * `F` - Owned application operation with any independently admitted roots
///   moved into Boxes before capture; no speculative capture-tuple formula.
/// * `budget` - Canonical host counter covering source and target overlap.
/// * `operation` - Actual final closure value, measured with size_of_val.
///
/// # Returns
/// One task whose dynamic closure body and concrete fixed wrapper are charged.
/// Captured dynamic children not already guarded need their own prior admission.
///
/// # Errors
/// Checked layout overflow or resource rejection before final closure allocation.
pub fn admit_task<F>(budget: &RetentionBudget, operation: F) -> Result<AdmittedTask, RetentionError>
where
    F: FnOnce() + Send + 'static,
{
    let bytes = size_of_val(&operation)
        .checked_add(size_of::<AdmittedTask>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    let reservation = budget.reserve_resources(RetainedResources {
        payload_bytes: bytes,
        ..RetainedResources::default()
    })?;
    Ok(AdmittedTask {
        operation: Box::new(operation),
        _reservation: reservation,
    })
}

/// Executes an admitted task while retaining its guard through return or unwind.
///
/// # Arguments
/// * `task` - Previously admitted actual closure, consumed exactly once.
///
/// # Returns
/// Unit after the closure returns. A panic unwinds normally after captured data
/// disposal; this helper neither catches panics nor releases scientific leases.
pub fn run_task(task: AdmittedTask) {
    let AdmittedTask {
        operation,
        _reservation,
    } = task;
    operation();
    drop(_reservation);
}

#[cfg(test)]
#[path = "transport_tests.rs"]
mod tests;
