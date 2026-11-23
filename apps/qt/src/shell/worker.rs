/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded application callbacks; only known termination releases a worker lease.
use super::{RunLease, RunToken};
use std::{
    io,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{mpsc, Arc, Mutex},
};

/// Terminal application outcome; scientific errors keep their direction's type.
#[derive(Debug, Eq, PartialEq)]
pub enum WorkerStatus<E> {
    Completed,
    Failed(E),
    Panicked,
}

/// One identified callback or the sole known-terminal ownership envelope.
pub enum WorkerEvent<T, E> {
    Message { token: RunToken, value: T },
    Terminal(WorkerTerminal<E>),
}

/// Known completion owns the lease until adoption or disposal by its receiver.
///
/// Unlike an active lease, this envelope proves the scientific closure has
/// returned or unwound. Dropping it therefore acknowledges actual termination.
pub struct WorkerTerminal<E> {
    status: WorkerStatus<E>,
    lease: Option<RunLease>,
}

/// Producer endpoint; every queued payload retains its original worker identity.
pub struct WorkerSender<T, E> {
    token: RunToken,
    sender: mpsc::SyncSender<WorkerEvent<T, E>>,
}

/// Failed thread admission preserves the original lease for host acknowledgement.
pub struct WorkerSpawnError {
    cause: io::Error,
    lease: RunLease,
}

impl<E> WorkerTerminal<E> {
    /// Reads completion without copying a scientific error or exposing panic data.
    ///
    /// # Returns
    /// The direction-specific outcome; no accepted result is adopted here.
    pub fn status(&self) -> &WorkerStatus<E> {
        &self.status
    }

    /// Reads the exact identity that must be validated before result adoption.
    ///
    /// # Returns
    /// Original direction/session/generation/run, independent of tab selection.
    pub fn token(&self) -> RunToken {
        self.lease
            .as_ref()
            .expect("Terminal owns its lease")
            .token()
    }

    /// Transfers known termination to the owning session's finish operation.
    ///
    /// # Returns
    /// The sole lease; the host must acknowledge it even if result adoption fails.
    pub fn into_lease(mut self) -> RunLease {
        self.lease.take().expect("Terminal owns its lease")
    }
}

impl<E> Drop for WorkerTerminal<E> {
    /// Acknowledges known completion when the GUI owner discards this envelope.
    ///
    /// This never cancels, joins or waits for a scientific operation. An active
    /// ordinary lease still keeps admission Busy when abandoned.
    fn drop(&mut self) {
        if let Some(lease) = self.lease.take() {
            let _ = lease.acknowledge_termination();
        }
    }
}

impl<T, E> WorkerSender<T, E> {
    /// Sends an owned bounded callback using worker-side backpressure only.
    ///
    /// # Arguments
    /// * `value` - Direction payload, carrying any required retention reservation.
    ///
    /// # Returns
    /// Unit after enqueue; the caller must stop when the owner is disconnected.
    ///
    /// # Errors
    /// Returns the original payload after GUI destruction, without losing charges.
    pub fn send(&self, value: T) -> Result<(), T> {
        self.sender
            .send(WorkerEvent::Message {
                token: self.token,
                value,
            })
            .map_err(|error| match error.0 {
                WorkerEvent::Message { value, .. } => value,
                WorkerEvent::Terminal(_) => unreachable!("Only a data event was sent"),
            })
    }
}

impl WorkerSpawnError {
    /// Recovers the failed OS admission and the original unstarted worker lease.
    ///
    /// # Returns
    /// Cause and sole lease; the host acknowledges failure without replacing its gate.
    pub fn into_parts(self) -> (io::Error, RunLease) {
        (self.cause, self.lease)
    }
}

/// Dispatches a direction's public operation with exactly one terminal envelope.
///
/// # Arguments
/// * `name` - Static application thread label; NUL is rejected before spawning.
/// * `capacity` - Existing validated direction-specific callback-count policy.
/// * `lease` - Genuine global admission acquired before dispatching any science.
/// * `operation` - Worker-only public API calls; returns after all scientific work.
///
/// # Returns
/// Bounded callback receiver; Qt polls it without blocking or joining. Payload
/// reservations remain the producer's responsibility. The operation cannot clone
/// the sender or send another terminal envelope.
///
/// # Errors
/// Returns OS spawn failure with its original lease. Scientific errors and panics
/// arrive as terminal outcomes; the lease remains outside caught unwinding.
pub fn spawn_worker<T, E>(
    name: &'static str,
    capacity: usize,
    lease: RunLease,
    operation: impl FnOnce(WorkerSender<T, E>) -> Result<(), E> + Send + 'static,
) -> Result<mpsc::Receiver<WorkerEvent<T, E>>, WorkerSpawnError>
where
    T: Send + 'static,
    E: Send + 'static,
{
    spawn_with(capacity, lease, operation, |task| {
        if name.contains('\0') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid thread name",
            ));
        }
        std::thread::Builder::new()
            .name(name.to_owned())
            .spawn(task)
            .map(drop)
    })
}

fn spawn_with<T, E>(
    capacity: usize,
    lease: RunLease,
    operation: impl FnOnce(WorkerSender<T, E>) -> Result<(), E> + Send + 'static,
    start: impl FnOnce(Box<dyn FnOnce() + Send>) -> io::Result<()>,
) -> Result<mpsc::Receiver<WorkerEvent<T, E>>, WorkerSpawnError>
where
    T: Send + 'static,
    E: Send + 'static,
{
    // Keep recoverable ownership outside the OS closure until spawn succeeds.
    let pending = Arc::new(Mutex::new(Some(lease)));
    let recover = pending.clone();
    let (sender, receiver) = mpsc::sync_channel(capacity);
    let task = Box::new(move || {
        let lease = pending
            .lock()
            .unwrap_or_else(|cause| cause.into_inner())
            .take()
            .expect("One worker owns the admitted lease");
        let publisher = WorkerSender {
            token: lease.token(),
            sender: sender.clone(),
        };
        let status = match catch_unwind(AssertUnwindSafe(|| operation(publisher))) {
            Ok(Ok(())) => WorkerStatus::Completed,
            Ok(Err(error)) => WorkerStatus::Failed(error),
            Err(_) => WorkerStatus::Panicked,
        };
        let terminal = WorkerTerminal {
            status,
            lease: Some(lease),
        };

        // Failed delivery drops known completion and acknowledges the original
        // gate only after the scientific closure has returned or unwound.
        let _ = sender.send(WorkerEvent::Terminal(terminal));
    });
    if let Err(cause) = start(task) {
        let lease = recover
            .lock()
            .unwrap_or_else(|cause| cause.into_inner())
            .take()
            .expect("Failed spawn never starts its worker");
        return Err(WorkerSpawnError { cause, lease });
    }
    Ok(receiver)
}

#[cfg(test)]
mod tests;
