/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Observation of completed source/noise outputs; no generator reference enters a sink.
use crate::{
    simulation::test_points::{
        ProbeContext, ProbeData, ProbeObserver, ProbeOutcome, SignalDomain, TestPointId,
        TestPointProbe,
    },
    types::IqPair,
    LabError,
};
/// Observes already-generated binary data without accessing generator state.
/// # Arguments
/// * `bits` - Completed source chunk, retained unchanged.
/// * `context` - Explicit source/word identity, with no I/Q reference.
/// * `probe` - Finite selector.
/// * `observer` - Read-only sink.
/// # Returns
/// Disabled, filtered, exhausted or dispatched outcome.
/// # Errors
/// Propagates metadata/callback errors without modifying source bits or generator state.
pub fn source_payload(
    bits: &[u8],
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<ProbeOutcome, LabError> {
    probe.emit(
        TestPointId::SourcePayload,
        context,
        SignalDomain::CodewordBit,
        ProbeData::Bits(bits),
        observer,
    )
}
/// Observes completed equalized noisy symbols from the preexisting coding experiment.
/// # Arguments
/// * `iq` - Completed normalized output symbols; no new noise draws occur here.
/// * `context` - Word identity and explicit RMS reference.
/// * `probe` - Finite selector.
/// * `observer` - Read-only sink.
/// # Returns
/// The selector outcome, with original array offsets preserved.
/// # Errors
/// Propagates validation/callback failures without modifying symbols or RNG state.
pub fn noise_output(
    iq: &[IqPair],
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<ProbeOutcome, LabError> {
    probe.emit(
        TestPointId::LegacyNoiseOutput,
        context,
        SignalDomain::CodewordCell,
        ProbeData::Iq(iq),
        observer,
    )
}
