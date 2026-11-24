/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Application identities; these do not replace backend configuration/run IDs.
use std::sync::atomic::{AtomicU64, Ordering};

// Zero is reserved for an unissued application identity. IDs never wrap or reuse.
static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

/// Direction whose independent application state owns an operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Downstream,
    Upstream,
}

/// Process-unique, opaque identity of one accepted application session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionId(u64);

/// Checked revision of accepted scientific configuration within one session.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConfigurationGeneration(pub(super) u64);

/// Checked run identity within one application session.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RunId(pub(super) u64);

/// Checked project-edit revision, separate from configuration and run identity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EditRevision(pub(super) u64);

/// Identifies the exact accepted application configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionKey {
    pub direction: Direction,
    pub session: SessionId,
    pub generation: ConfigurationGeneration,
}

/// Identifies one callback without relying on whichever tab is currently selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunToken {
    pub key: SessionKey,
    pub run: RunId,
}

/// Identity space is exhausted; the existing accepted state must be preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdentityExhausted;

impl std::fmt::Display for SessionId {
    /// Writes the opaque identity as lossless decimal text for UI transports.
    /// # Returns
    /// Formatter status; the number is never converted through JavaScript floating point.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::fmt::Display for ConfigurationGeneration {
    /// Writes the checked configuration generation as lossless decimal text.
    /// # Returns
    /// Formatter status without changing the accepted owner.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::fmt::Display for RunId {
    /// Writes the checked run identity as lossless decimal text.
    /// # Returns
    /// Formatter status without narrowing or reissuing the identity.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl SessionId {
    /// Allocates a process-unique application identity without wrapping.
    ///
    /// # Returns
    /// A fresh opaque identity, also distinct from separately created coordinators.
    ///
    /// # Errors
    /// Returns exhaustion once the process-wide identity sequence is consumed.
    pub(super) fn allocate() -> Result<Self, IdentityExhausted> {
        allocate_from(&NEXT_SESSION_ID).map(Self)
    }
}

fn allocate_from(sequence: &AtomicU64) -> Result<u64, IdentityExhausted> {
    // The terminal value is an exhausted sentinel, never an issued identity.
    sequence
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
            next.checked_add(1)
        })
        .map_err(|_| IdentityExhausted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_identity_text_preserves_integers_beyond_javascript_precision() {
        // Independent first inexact IEEE-754 integer and full unsigned-width anchors.
        const FIRST_INEXACT_JS_INTEGER: u64 = 9_007_199_254_740_993;
        assert_eq!(
            SessionId(FIRST_INEXACT_JS_INTEGER).to_string(),
            "9007199254740993"
        );
        assert_eq!(
            ConfigurationGeneration(u64::MAX).to_string(),
            "18446744073709551615"
        );
        assert_eq!(RunId(u64::MAX).to_string(), "18446744073709551615");
    }

    #[test]
    fn process_identity_exhaustion_never_wraps() {
        let sequence = AtomicU64::new(u64::MAX - 1);
        assert_eq!(allocate_from(&sequence), Ok(u64::MAX - 1));
        assert_eq!(allocate_from(&sequence), Err(IdentityExhausted));
        assert_eq!(sequence.load(Ordering::Relaxed), u64::MAX);
    }
}
