/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Gaussian perturbation at an equalized symbol reference point.
use crate::types::IqPair;
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};
/// Adds independent Gaussian I/Q components to one symbol in place.
/// # Arguments
/// * `symbol` - Unit-energy reference symbol, mutated with additive noise.
/// * `power` - Validated E[|noise|²]; each real variance is half this value.
/// * `rng` - Job stream shared with source generation, preserving draw order.
/// # Returns
/// Unit after drawing the two real Gaussian components.
pub(crate) fn add_awgn(symbol: &mut IqPair, power: f64, rng: &mut impl Rng) {
    let standard_deviation = (power / 2.0).sqrt();
    for component in symbol {
        let gaussian: f64 = StandardNormal.sample(rng);
        *component += standard_deviation * gaussian;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        simulation::{
            source::observation,
            test_points::{
                ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver, ProbeSelection, RunPhase,
                SignalReference, TestPointId, TestPointProbe,
            },
        },
        SimulationError,
    };
    use rand::SeedableRng;
    #[test]
    fn rejected_noise_observation_does_not_change_future_gaussian_draws() {
        struct Failure;
        impl ProbeObserver for Failure {
            fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
                Err(SimulationError::new(
                    "TestFailure",
                    "Injected callback failure.",
                ))
            }
        }
        let mut actual = rand_chacha::ChaCha8Rng::seed_from_u64(91);
        let mut baseline = actual.clone();
        let mut first = [1.0, 0.0];
        let mut expected = first;
        add_awgn(&mut first, 0.01, &mut actual);
        add_awgn(&mut expected, 0.01, &mut baseline);
        assert_eq!(first, expected);
        let mut context = ProbeContext::new("noise-test", "seed91", RunPhase::Observation).unwrap();
        context.reference = SignalReference::OtherSubcarrierRms { rms: 1.0 };
        let mut probe = TestPointProbe::new(ProbeConfig {
            selections: vec![ProbeSelection {
                stage: TestPointId::LegacyNoiseOutput,
                elements: 0..1,
                source_symbols: None,
                channel_symbols: None,
            }],
            max_events: 1,
            max_elements_per_event: 1,
            max_total_elements: 1,
        })
        .unwrap();
        assert!(observation::noise_output(&[first], &context, &mut probe, &mut Failure).is_err());
        let mut next = [0.0, 1.0];
        let mut expected = next;
        add_awgn(&mut next, 0.01, &mut actual);
        add_awgn(&mut expected, 0.01, &mut baseline);
        assert_eq!(next, expected);
    }
}
