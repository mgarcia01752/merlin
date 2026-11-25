/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive accepted settings, distinct from the retained source document.
use docsis_ofdma::api::*;

const SOURCE_50: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");
const SOURCE_25: &[u8] = include_bytes!("../../../examples/cli/upstream/us-25khz.json");

// Independent finite override fixtures; these are not production defaults/limits.
const OVERRIDE_GRANTS: usize = 2;
const OVERRIDE_CHUNK_SAMPLES: usize = 313;
const OVERRIDE_TIMEOUT_MS: u64 = 90_000;
const OVERRIDE_RETAINED_POINTS: usize = 29;

#[test]
fn accepted_options_borrow_is_passive_and_preserves_overrides_and_failed_replacements() {
    let mut references = Vec::new();
    for source in [SOURCE_50, SOURCE_25] {
        let initial = ExperimentOptions::default();
        let mut simulation = Simulation::new(initial.clone()).unwrap();
        assert_eq!(simulation.options(), &initial);
        assert!(simulation.configuration().is_none());
        simulation.configure_json(source).unwrap();
        let original = simulation.document().unwrap().to_json().unwrap();
        let (_, document_options) = simulation.document().unwrap().prepare().unwrap();
        assert_eq!(simulation.options(), &document_options);
        let accepted = ExperimentOptions {
            grants: OVERRIDE_GRANTS,
            chunk_samples: OVERRIDE_CHUNK_SAMPLES,
            timeout_ms: OVERRIDE_TIMEOUT_MS,
            retained_points: OVERRIDE_RETAINED_POINTS,
            retained_taps: vec![Tap::TxOutput],
            ..document_options
        };
        simulation.set_options(accepted.clone()).unwrap();
        let footprint = simulation.retained_footprint().unwrap();
        let owned = simulation.options().clone();
        assert_eq!(&owned, &accepted);
        assert_eq!(simulation.retained_footprint().unwrap(), footprint);
        assert_eq!(simulation.document().unwrap().to_json().unwrap(), original);
        let invalid = ExperimentOptions {
            grants: 0,
            ..accepted.clone()
        };
        assert!(simulation.set_options(invalid).is_err());
        assert_eq!(simulation.options(), &accepted);
        let report = simulation.run().unwrap();
        assert_eq!(report.options, owned);
        assert!(report.reference_recovered);
        assert_eq!(report.bit_errors, 0);
        references
            .push(serde_json::from_str::<serde_json::Value>(&report.to_json().unwrap()).unwrap());
        drop(simulation);
        assert_eq!(owned, accepted);
    }

    // Optional explicit evidence destination for a separate installed-wheel run.
    if let Some(destination) = std::env::var_os("MERLIN_OPTIONS_RUST_REFERENCE_JSON") {
        std::fs::write(destination, serde_json::to_vec_pretty(&references).unwrap()).unwrap();
    }
}
