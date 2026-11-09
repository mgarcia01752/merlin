/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Optional read-only observation after scientific pilot results exist.
use super::{PilotGenerator, PilotStageResult};
use crate::{
    error::invalid,
    simulation::test_points::{
        ProbeContext, ProbeData, ProbeObserver, ProbeOutcome, SignalDomain, SignalReference,
        TestPointId, TestPointProbe,
    },
    types::PLC_FRAME_SYMBOLS,
    SimulationError,
};
impl PilotGenerator {
    /// Observes already generated results without modifying them or the generator.
    /// # Arguments
    /// * `result` - Completed result from matching numerology.
    /// * `context` - Caller run/config/time context; channel index and PLC phase
    ///   must match this physical symbol. Reference is supplied from the stage power.
    /// * `probe` - Optional bounded selector; exhausted budgets do not affect results.
    /// * `observer` - Borrowed read-only event sink; copies are explicitly caller-owned.
    /// # Returns
    /// Sequence, power, continuous and scattered dispatch outcomes, in that order.
    /// # Errors
    /// Rejects inconsistent context/numerology or propagates observer errors.
    /// Previously computed results remain available even if observation fails.
    pub fn observe(
        &self,
        result: &PilotStageResult,
        context: &ProbeContext,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<[ProbeOutcome; 4], SimulationError> {
        if result.spacing != self.sequence().spacing()
            || context.channel_symbol_index != Some(result.symbol_index)
            || context.plc_phase != Some((result.symbol_index % PLC_FRAME_SYMBOLS) as u8)
        {
            return Err(invalid(
                "Pilot observation requires matching physical-symbol context.",
            ));
        }
        context.validate()?;
        let mut context = context.clone();
        context.reference = SignalReference::NotApplicable;
        let sequence = probe.emit(
            TestPointId::PilotSequence,
            &context,
            SignalDomain::PhysicalFftBin,
            ProbeData::Bits(self.sequence().bits()),
            observer,
        )?;
        context.reference = SignalReference::OtherSubcarrierRms {
            rms: result.power.reference_rms(),
        };
        let power = probe.emit(
            TestPointId::PilotPower,
            &context,
            SignalDomain::Parameter,
            ProbeData::Power(&result.power),
            observer,
        )?;
        let continuous = probe.emit(
            TestPointId::ContinuousPilots,
            &context,
            SignalDomain::PhysicalFftBin,
            ProbeData::Pilots(&result.continuous),
            observer,
        )?;
        let scattered = probe.emit(
            TestPointId::ScatteredPilots,
            &context,
            SignalDomain::PhysicalFftBin,
            ProbeData::Pilots(&result.scattered),
            observer,
        )?;
        Ok([sequence, power, continuous, scattered])
    }
}
