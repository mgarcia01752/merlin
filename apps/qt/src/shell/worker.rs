/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded identified callbacks; known termination alone releases a worker lease.
use super::{
    admit_task, bounded_channel, run_task, AdmittedTask, DeliveryPermit, DeliveryReceiver,
    Retained, RetainedResources, RetentionBudget, RetentionError, RetentionReservation, RunLease,
    RunToken,
};
use std::{
    io,
    mem::size_of,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{mpsc, Arc, Mutex},
    time::Duration,
};

/// Terminal application outcome; scientific errors keep their direction's type.
#[derive(Debug, Eq, PartialEq)]
pub enum WorkerStatus<E> {
    Completed,
    Failed(E),
    Panicked,
}

/// One identified callback or the sole independently guarded terminal envelope.
pub enum WorkerEvent<T, E> {
    Message { token: RunToken, value: Retained<T> },
    Terminal(WorkerTerminal<E>),
}

/// Known completion owns the lease and delivery until adoption or disposal.
pub struct WorkerTerminal<E> {
    completion: KnownTerminal<E>,
    _delivery: RetentionReservation,
}

struct KnownTerminal<E> {
    status: WorkerStatus<E>,
    lease: Option<RunLease>,
}

enum Completion<T, E> {
    Message { token: RunToken, value: T },
    Terminal(KnownTerminal<E>),
}

/// Producer with a concrete finite inventory of separately reserved deliveries.
pub struct WorkerSender<T, E> {
    token: RunToken,
    permits: std::vec::IntoIter<DeliveryPermit<Completion<T, E>>>,
    _buffer: RetentionReservation,
}

struct MessageInventory<T, E> {
    permits: Box<[DeliveryPermit<Completion<T, E>>]>,
    buffer: RetentionReservation,
}

impl<T, E> MessageInventory<T, E> {
    fn into_sender(self, token: RunToken) -> WorkerSender<T, E> {
        WorkerSender {
            token,
            permits: Vec::from(self.permits).into_iter(),
            _buffer: self.buffer,
        }
    }
}

/// Polling endpoint; extracted messages/terminals retain their own frame charge.
pub struct WorkerReceiver<T, E> {
    receiver: DeliveryReceiver<Completion<T, E>>,
}

/// Actual admitted operation, recoverable late input and bounded receiver.
pub struct PreparedWorker<Input, T, E> {
    name: &'static str,
    task: AdmittedTask,
    pending: PendingInput<Input>,
    receiver: WorkerReceiver<T, E>,
}

// The recoverable lease/input slot is one real Arc pointee, separate from channel.
const RECOVERY_SLOT_SHARED_ROOTS: usize = 1;

type PendingInput<Input> = Arc<Retained<Mutex<Option<(RunLease, Input)>>>>;

/// Failed thread admission preserves the original lease for host acknowledgement.
pub struct WorkerSpawnError {
    cause: io::Error,
    lease: RunLease,
}

impl<E> WorkerTerminal<E> {
    /// Reads completion without copying a scientific error or exposing panic data.
    ///
    /// # Returns
    /// Original typed outcome, borrowed while the complete frame stays charged.
    pub fn status(&self) -> &WorkerStatus<E> {
        &self.completion.status
    }

    /// Reads the exact identity required before result adoption.
    ///
    /// # Returns
    /// Original direction/session/generation/run, independent of tab selection.
    pub fn token(&self) -> RunToken {
        self.completion
            .lease
            .as_ref()
            .expect("Terminal owns its lease")
            .token()
    }

    /// Transfers known termination to its owning session's finish operation.
    ///
    /// # Returns
    /// Sole lease; the host acknowledges it even if result adoption fails. The
    /// terminal's remaining status data drops before its frame charge releases.
    pub fn into_lease(mut self) -> RunLease {
        self.completion
            .lease
            .take()
            .expect("Terminal owns its lease")
    }
}

impl<E> Drop for KnownTerminal<E> {
    /// Acknowledges actual completion without joining or waiting for science.
    ///
    /// Queued disposal and failed delivery retain known-termination semantics.
    fn drop(&mut self) {
        if let Some(lease) = self.lease.take() {
            let _ = lease.acknowledge_termination();
        }
    }
}

impl<T, E> WorkerSender<T, E> {
    /// Publishes one previously admitted data delivery with worker backpressure.
    ///
    /// # Arguments
    /// * `value` - Guarded direction payload; no scientific child is cloned here.
    ///
    /// # Returns
    /// Unit after enqueue. Each frame has its own reservation, independent of
    /// how many prior frames the receiver has removed and kept.
    ///
    /// # Errors
    /// Returns the original payload on disconnect or after the caller's finite
    /// declared event inventory is exhausted. Excess data is never unguarded.
    pub fn send(&mut self, value: T) -> Result<(), T> {
        let Some(permit) = self.permits.next() else {
            return Err(value);
        };
        permit
            .send(Completion::Message {
                token: self.token,
                value,
            })
            .map_err(|completion| match completion {
                Completion::Message { value, .. } => value,
                Completion::Terminal(_) => unreachable!("Only data was published"),
            })
    }
}

impl<T, E> WorkerReceiver<T, E> {
    /// Polls one identified event without blocking the GUI thread.
    ///
    /// # Returns
    /// A separately guarded value/terminal, independent of endpoint disposal.
    ///
    /// # Errors
    /// Ordinary std Empty/Disconnected outcomes; no lease is released early.
    pub fn try_recv(&self) -> Result<WorkerEvent<T, E>, mpsc::TryRecvError> {
        self.receiver.try_recv().map(event)
    }

    /// Waits for one event on a worker or fixture thread.
    ///
    /// # Returns
    /// Original identified guarded event; not intended for GUI event loops.
    ///
    /// # Errors
    /// Ordinary disconnection after the final producer/permit disappears.
    pub fn recv(&self) -> Result<WorkerEvent<T, E>, mpsc::RecvError> {
        self.receiver.recv().map(event)
    }

    /// Waits for one event within a finite worker or fixture deadline.
    ///
    /// # Arguments
    /// * `timeout` - Maximum receive duration; does not cancel science.
    ///
    /// # Returns
    /// Original identified guarded event without copying its children.
    ///
    /// # Errors
    /// Ordinary std timeout/disconnection outcomes.
    pub fn recv_timeout(
        &self,
        timeout: Duration,
    ) -> Result<WorkerEvent<T, E>, mpsc::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout).map(event)
    }
}

impl<Input, T, E> PreparedWorker<Input, T, E> {
    /// Supplies genuine admission and late runtime input to an already measured task.
    ///
    /// # Arguments
    /// * `lease` - Original canonical lease, acquired after transport preflight.
    /// * `input` - Exact pre-sized late value, such as the original-epoch timing Arc.
    ///
    /// # Returns
    /// Nonblocking receiver after OS dispatch; no closure or frame admission remains.
    ///
    /// # Errors
    /// Invalid static thread name or OS spawn failure returns the original lease;
    /// input and unused reservations drop without starting scientific work.
    pub fn spawn(
        self,
        lease: RunLease,
        input: Input,
    ) -> Result<WorkerReceiver<T, E>, WorkerSpawnError> {
        self.spawn_with(lease, input, |name, task| {
            if name.contains('\0') {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid thread name",
                ));
            }
            std::thread::Builder::new()
                .name(name.to_owned())
                .spawn(move || run_task(task))
                .map(drop)
        })
    }

    fn spawn_with(
        self,
        lease: RunLease,
        input: Input,
        start: impl FnOnce(&'static str, AdmittedTask) -> io::Result<()>,
    ) -> Result<WorkerReceiver<T, E>, WorkerSpawnError> {
        *self
            .pending
            .value()
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Some((lease, input));
        if let Err(cause) = start(self.name, self.task) {
            let (lease, _input) = self
                .pending
                .value()
                .lock()
                .unwrap_or_else(|error| error.into_inner())
                .take()
                .expect("Failed spawn never starts its worker");
            return Err(WorkerSpawnError { cause, lease });
        }
        Ok(self.receiver)
    }
}

impl WorkerSpawnError {
    /// Preserves an unstarted lease when preparation cannot admit transport.
    ///
    /// # Arguments
    /// * `error` - Checked application resource rejection before any science.
    /// * `lease` - Original host lease, returned without acknowledgement.
    ///
    /// # Returns
    /// Existing spawn-error shape with explicit I/O admission context.
    pub(crate) fn admission(error: RetentionError, lease: RunLease) -> Self {
        Self {
            cause: io::Error::other(format!("Worker transport admission failed: {error:?}")),
            lease,
        }
    }

    /// Recovers failed OS admission and its original unstarted lease.
    ///
    /// # Returns
    /// Cause and sole lease; acknowledge without replacing its canonical gate.
    pub fn into_parts(self) -> (io::Error, RunLease) {
        (self.cause, self.lease)
    }
}

/// Preflights a concrete finite worker without a lease, clock or scientific step.
///
/// # Arguments
/// * `Input` - Late runtime value, included in the exact recovery-slot layout.
/// * `T`, `E` - Guarded callback/error values; no dynamic child walker is added.
/// * `name` - Static thread label; validated by spawn before execution.
/// * `capacity` - Configured bounded slots, distinct from total data events.
/// * `message_count` - Existing operation's finite DATA maximum; terminal is separate.
/// * `budget` - Canonical host counter for old, candidate, queued and held owners.
/// * `operation` - Actual final closure, measured before dispatch; independently
///   admitted inline roots must be Box-moved before capture to avoid overlap.
///
/// # Returns
/// Exact admitted task, recovery Arc and channel; each data/terminal frame has
/// its own permit. The requested finite builder is admitted before allocation;
/// its retained permit buffer is normalized to an exact boxed-slice layout.
/// Early stop/panic disposes unused permits. No capacity-plus-one estimate is used.
///
/// # Errors
/// Any layout overflow or five-dimensional admission failure before lease/start.
pub fn prepare_worker<Input, T, E>(
    name: &'static str,
    capacity: usize,
    message_count: usize,
    budget: &RetentionBudget,
    operation: impl FnOnce(Input, WorkerSender<T, E>) -> Result<(), E> + Send + 'static,
) -> Result<PreparedWorker<Input, T, E>, RetentionError>
where
    Input: Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    let pending = budget.reserve_resources(RetainedResources {
        payload_bytes: size_of::<Retained<Mutex<Option<(RunLease, Input)>>>>(),
        shared_roots: RECOVERY_SLOT_SHARED_ROOTS,
        ..RetainedResources::default()
    })?;
    let pending = Arc::new(pending.retain(Mutex::new(None::<(RunLease, Input)>)));
    let worker_pending = Arc::clone(&pending);
    let (sender, receiver) = bounded_channel::<Completion<T, E>>(budget, capacity)?;
    let frame_bytes = size_of::<WorkerEvent<T, E>>();
    let terminal = sender.reserve_bytes(frame_bytes)?;
    let buffer_bytes = message_count
        .checked_mul(size_of::<DeliveryPermit<Completion<T, E>>>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    let buffer = budget.reserve_resources(RetainedResources {
        payload_bytes: buffer_bytes,
        ..RetainedResources::default()
    })?;
    let mut permits = Vec::with_capacity(message_count);
    for _ in 0..message_count {
        permits.push(sender.reserve_bytes(frame_bytes)?);
    }
    let inventory = MessageInventory {
        permits: permits.into_boxed_slice(),
        buffer,
    };
    let task = admit_task(budget, move || {
        let (lease, input) = worker_pending
            .value()
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .take()
            .expect("One worker owns its admitted input");
        let publisher = inventory.into_sender(lease.token());
        let status = match catch_unwind(AssertUnwindSafe(|| operation(input, publisher))) {
            Ok(Ok(())) => WorkerStatus::Completed,
            Ok(Err(error)) => WorkerStatus::Failed(error),
            Err(_) => WorkerStatus::Panicked,
        };

        // The sole terminal is reserved before science and owns known completion.
        // Failed delivery acknowledges only after the closure returned or unwound.
        let _ = terminal.send(Completion::Terminal(KnownTerminal {
            status,
            lease: Some(lease),
        }));
    })?;
    Ok(PreparedWorker {
        name,
        task,
        pending,
        receiver: WorkerReceiver { receiver },
    })
}

/// Prepares and starts a worker when the caller already holds its original lease.
///
/// # Arguments
/// * `name`, `capacity`, `message_count`, `budget` - Exact prepare_worker policies.
/// * `lease` - Genuine canonical lease, returned unchanged on any rejection.
/// * `operation` - Worker-only public calls; complete child admission stays explicit.
///
/// # Returns
/// Bounded guarded receiver; callers needing pre-lease rejection use prepare_worker.
///
/// # Errors
/// Retention is reported as an unstarted I/O admission error, or OS spawn failure;
/// both preserve the original lease. Terminal delivery owns scientific failure.
pub fn spawn_worker<T, E>(
    name: &'static str,
    capacity: usize,
    message_count: usize,
    budget: &RetentionBudget,
    lease: RunLease,
    operation: impl FnOnce(WorkerSender<T, E>) -> Result<(), E> + Send + 'static,
) -> Result<WorkerReceiver<T, E>, WorkerSpawnError>
where
    T: Send + 'static,
    E: Send + 'static,
{
    let prepared = match prepare_worker(name, capacity, message_count, budget, move |(), sender| {
        operation(sender)
    }) {
        Ok(prepared) => prepared,
        Err(error) => return Err(WorkerSpawnError::admission(error, lease)),
    };
    prepared.spawn(lease, ())
}

fn event<T, E>(delivery: Retained<Completion<T, E>>) -> WorkerEvent<T, E> {
    let (completion, reservation) = delivery.into_parts();
    match completion {
        Completion::Message { token, value } => WorkerEvent::Message {
            token,
            value: reservation.retain(value),
        },
        Completion::Terminal(completion) => WorkerEvent::Terminal(WorkerTerminal {
            completion,
            _delivery: reservation,
        }),
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod transport_retention_tests;
