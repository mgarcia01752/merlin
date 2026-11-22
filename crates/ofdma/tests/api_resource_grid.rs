/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original physical pilot vectors, finite views, admitted timing and passive API invariance.
use docsis_ofdma::{api::*, resources::ResourcePlan};

const PROFILE_ID: u16 = 224;
const DATA_IUC: u8 = 9;
const PROFILE_RANDOMIZER_SEED: u32 = 1;
const SOURCE_PRBS15_SEED: u16 = 1;
const PREFIX_SAMPLES: usize = 96;
const ROLLOFF_SAMPLES: usize = 32;
const MINIMUM_FRAME_SYMBOLS: usize = 6;
const FIXTURE_BANDWIDTH_HZ: u32 = 20_000_000;
const DATA_LOADING: u8 = 4;
const PROFILE_MINISLOT_COUNT: usize = 4;
const ZERO_LOADED_MINISLOT_INDEX: usize = 1;
const GRANT_ID: u64 = 17;
const FIRST_FRAME_ID: u64 = 11;
const GRANT_EPOCH_SAMPLE: u64 = 4096;
const GRANT_FRAME_COUNT: usize = 2;
const ESTIMATOR_PHASE_LIMIT: f64 = 0.02;
const ESTIMATOR_POWER_FLOOR: f64 = 1e-12;
const LDPC_ITERATION_BUDGET: usize = 50;
const FIRST_50KHZ_CARRIER: usize = 100;
const FIRST_25KHZ_CARRIER: usize = 200;

fn configuration(mode: Numerology, pattern: u8, symbols: usize) -> ExperimentConfig {
    let first = match mode {
        Numerology::Khz50 => FIRST_50KHZ_CARRIER,
        Numerology::Khz25 => FIRST_25KHZ_CARRIER,
    };
    let width = mode.carriers_per_minislot();
    let profile = Profile::new(
        PROFILE_ID,
        DATA_IUC,
        PROFILE_RANDOMIZER_SEED,
        (0..PROFILE_MINISLOT_COUNT)
            .map(|index| Minislot {
                first_carrier: first + index * width,
                loading: if index == ZERO_LOADED_MINISLOT_INDEX {
                    0
                } else {
                    DATA_LOADING
                },
                pilot_pattern: if index == ZERO_LOADED_MINISLOT_INDEX {
                    0
                } else {
                    pattern
                },
            })
            .collect(),
    )
    .unwrap();
    let grant = SyntheticGrant {
        id: GRANT_ID,
        profile_id: PROFILE_ID,
        epoch_sample: GRANT_EPOCH_SAMPLE,
        first_frame: FIRST_FRAME_ID,
        frames: GRANT_FRAME_COUNT,
        minislots: vec![0, 1, 2],
    };
    let waveform = Waveform::new(
        mode,
        PREFIX_SAMPLES,
        ROLLOFF_SAMPLES,
        symbols,
        FIXTURE_BANDWIDTH_HZ,
    )
    .unwrap();
    let us = UsConfiguration::new(waveform, vec![profile], grant, vec![first - 1]).unwrap();
    ExperimentConfig::new(
        us,
        SOURCE_PRBS15_SEED,
        ReceiverConfig::new(
            Estimator::FrameFlatLs,
            ESTIMATOR_PHASE_LIMIT,
            ESTIMATOR_POWER_FLOOR,
            LDPC_ITERATION_BUDGET,
        )
        .unwrap(),
    )
    .unwrap()
}
fn session(config: ExperimentConfig) -> Simulation {
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session.configure(config).unwrap();
    session
}

#[test]
fn literal_edge_pilots_zero_exclusion_unallocated_timing_and_full_capacity_keep_original_coordinates(
) {
    const EXPECTED_FRAME_BITS: usize = 328;
    const EXPECTED_GRANT_BITS: usize = 656;
    const EXPECTED_SECOND_FRAME_PREFIX: u64 = 16_960;
    const EXPECTED_SYMBOL_TWO_PREFIX: u64 = 21_248;
    const EXPECTED_SYMBOL_TWO_USEFUL: u64 = 21_344;
    let config = configuration(Numerology::Khz50, 1, MINIMUM_FRAME_SYMBOLS);
    let session = session(config.clone());
    let snapshot = session
        .resource_grid(ResourceGridRequest {
            frame_index: 1,
            first_carrier: 99,
            carrier_count: 34,
            symbol_count: 0,
            ..Default::default()
        })
        .unwrap();
    let grid = &snapshot.grid;
    assert_eq!(snapshot.format, "merlin.upstream.resource_grid");
    assert_eq!(snapshot.schema_version, 1);
    assert_eq!(grid.cells.len(), 204);
    assert_eq!(grid.absolute_frame, 12);
    assert_eq!(grid.frame_prefix_sample, EXPECTED_SECOND_FRAME_PREFIX);
    assert_eq!(grid.frame_bits, EXPECTED_FRAME_BITS);
    assert_eq!(grid.grant_bits, EXPECTED_GRANT_BITS);
    assert_eq!(grid.profile_id, PROFILE_ID);
    assert_eq!(grid.iuc, DATA_IUC);
    let pilots: Vec<_> = grid
        .cells
        .iter()
        .filter(|cell| cell.role == ResourceGridRole::KnownPilot)
        .map(|cell| (cell.frame_symbol, cell.physical_carrier))
        .collect();
    assert_eq!(
        pilots,
        vec![
            (0, 100),
            (0, 107),
            (0, 116),
            (0, 123),
            (2, 100),
            (2, 107),
            (2, 116),
            (2, 123)
        ]
    );
    let counts = &grid.counts;
    assert_eq!(
        (
            counts.data,
            counts.complementary,
            counts.known_pilot,
            counts.zero_loaded,
            counts.excluded,
            counts.unallocated
        ),
        (80, 8, 8, 48, 6, 54)
    );
    let point = grid
        .cells
        .iter()
        .find(|cell| cell.frame_symbol == 2 && cell.physical_carrier == 100)
        .unwrap();
    assert_eq!(point.grant_symbol, 8);
    assert_eq!(point.prefix_start_sample, EXPECTED_SYMBOL_TWO_PREFIX);
    assert_eq!(point.useful_start_sample, EXPECTED_SYMBOL_TWO_USEFUL);
    for cell in &grid.cells {
        if cell.role == ResourceGridRole::ZeroLoaded {
            assert_eq!(cell.configured_data_loading, Some(0));
            assert_eq!(cell.cell_loading, Some(0));
            assert!(
                cell.visit_index.is_none()
                    && cell.amplitude_gain.is_none()
                    && cell.frame_bit_offset.is_none()
            );
            assert!(cell.grant_selected);
        }
        if cell.role == ResourceGridRole::Unallocated && cell.minislot_index == Some(3) {
            assert_eq!(cell.configured_data_loading, Some(DATA_LOADING));
            assert!(!cell.grant_selected);
            assert!(cell.cell_loading.is_none());
        }
    }
    let plan = ResourcePlan::new(&config.us_configuration).unwrap();
    for cell in grid.cells.iter().filter(|cell| cell.visit_index.is_some()) {
        let original = &plan.visits[cell.visit_index.unwrap()];
        assert_eq!(original.symbol, cell.frame_symbol);
        assert_eq!(original.carrier, cell.physical_carrier);
        assert_eq!(Some(original.gain), cell.amplitude_gain);
        assert_eq!(Some(original.bit_offset), cell.frame_bit_offset);
        assert_eq!(
            cell.grant_bit_offset,
            Some(EXPECTED_FRAME_BITS + original.bit_offset)
        );
    }
    let zoom = session
        .resource_grid(ResourceGridRequest {
            first_carrier: 100,
            carrier_count: 1,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(zoom.grid.grant_bits, grid.grant_bits);
    assert_eq!(zoom.grid.frame_bits, grid.frame_bits);
    assert_eq!(
        zoom.configuration_generation,
        snapshot.configuration_generation
    );
}

#[test]
fn all_admitted_patterns_frame_sizes_and_numerologies_preserve_authoritative_visit_metadata() {
    for (mode, patterns, maximum_symbols, first) in [
        (Numerology::Khz50, 1..=7, 36, 100),
        (Numerology::Khz25, 8..=14, 18, 200),
    ] {
        let width = mode.carriers_per_minislot();
        for pattern in patterns {
            for symbols in MINIMUM_FRAME_SYMBOLS..=maximum_symbols {
                let mut config = configuration(mode, pattern, symbols);
                let us = &config.us_configuration;
                let mut grant = us.grant().clone();
                grant.minislots.push(3);
                config.us_configuration = UsConfiguration::new(
                    us.waveform().clone(),
                    us.profiles().to_vec(),
                    grant,
                    us.excluded().to_vec(),
                )
                .unwrap();
                let plan = ResourcePlan::new(&config.us_configuration).unwrap();
                let snapshot = session(config)
                    .resource_grid(ResourceGridRequest {
                        first_carrier: first,
                        carrier_count: width * PROFILE_MINISLOT_COUNT,
                        symbol_count: 0,
                        ..Default::default()
                    })
                    .unwrap();
                assert_eq!(
                    snapshot.grid.cells.len(),
                    width * PROFILE_MINISLOT_COUNT * symbols
                );
                assert_eq!(snapshot.grid.full_frame_visits, plan.visits.len());
                for cell in snapshot
                    .grid
                    .cells
                    .iter()
                    .filter(|cell| cell.visit_index.is_some())
                {
                    let original = &plan.visits[cell.visit_index.unwrap()];
                    assert_eq!(
                        (cell.frame_symbol, cell.physical_carrier),
                        (original.symbol, original.carrier)
                    );
                    assert_eq!(cell.cell_loading, Some(original.loading));
                    assert_eq!(cell.amplitude_gain, Some(original.gain));
                    assert_eq!(cell.pilot_pattern, Some(pattern));
                    if cell.role == ResourceGridRole::KnownPilot {
                        assert_eq!(cell.cell_loading, Some(0));
                    }
                    if cell.role == ResourceGridRole::Complementary {
                        assert_eq!(cell.cell_loading, Some(1));
                    }
                }
            }
        }
    }
}

#[test]
fn missing_invalid_and_work_bounded_queries_preserve_generation_original_state_and_owned_results() {
    const EXPECTED_CELL_BUDGET: usize = 16_384;
    const OVER_CELL_BUDGET: usize = 16_385;
    let empty = Simulation::new(ExperimentOptions::default()).unwrap();
    assert_eq!(
        empty
            .resource_grid(ResourceGridRequest::default())
            .unwrap_err()
            .code(),
        "MissingSetting"
    );
    assert_eq!(MAX_RESOURCE_GRID_CELLS, EXPECTED_CELL_BUDGET);
    let mut session = session(configuration(Numerology::Khz25, 8, MINIMUM_FRAME_SYMBOLS));
    let original = session
        .resource_grid(ResourceGridRequest::default())
        .unwrap();
    let serialized = original.to_json().unwrap();
    let generation = original.configuration_generation;
    let preview = session.preview().unwrap();
    for query in [
        ResourceGridRequest {
            frame_index: GRANT_FRAME_COUNT,
            ..Default::default()
        },
        ResourceGridRequest {
            first_symbol: MINIMUM_FRAME_SYMBOLS,
            ..Default::default()
        },
        ResourceGridRequest {
            symbol_count: usize::MAX,
            ..Default::default()
        },
        ResourceGridRequest {
            first_carrier: usize::MAX,
            ..Default::default()
        },
        ResourceGridRequest {
            carrier_count: usize::MAX,
            ..Default::default()
        },
        ResourceGridRequest {
            max_cells: 0,
            ..Default::default()
        },
        ResourceGridRequest {
            max_cells: 1,
            ..Default::default()
        },
        ResourceGridRequest {
            max_cells: OVER_CELL_BUDGET,
            ..Default::default()
        },
        ResourceGridRequest {
            symbol_count: 0,
            ..Default::default()
        },
    ] {
        assert!(session.resource_grid(query).is_err());
    }
    assert!(serde_json::from_str::<ResourceGridRequest>(r#"{"unknown":1}"#).is_err());
    assert_eq!(session.preview().unwrap(), preview);
    assert_eq!(
        session
            .resource_grid(ResourceGridRequest::default())
            .unwrap()
            .configuration_generation,
        generation
    );
    let maximum = session
        .resource_grid(ResourceGridRequest {
            symbol_count: 4,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(maximum.grid.cells.len(), EXPECTED_CELL_BUDGET);
    assert!(maximum.to_json().unwrap().len() < MAX_RESOURCE_GRID_JSON_BYTES);
    session
        .configure(configuration(Numerology::Khz50, 1, MINIMUM_FRAME_SYMBOLS))
        .unwrap();
    assert_ne!(
        session
            .resource_grid(ResourceGridRequest::default())
            .unwrap()
            .configuration_generation,
        generation
    );
    assert_eq!(original.to_json().unwrap(), serialized);
}

#[test]
fn configured_grid_observation_does_not_advance_source_noise_receiver_or_configuration_identity() {
    const ALLOCATED_MINISLOTS: usize = 32;
    const RUN_FIRST_CARRIER: usize = 128;
    const RUN_PILOT_PATTERN: u8 = 1;
    const CHANNEL_NOISE_VARIANCE: f64 = 1e-5;
    const CHANNEL_NOISE_SEED: [u8; 32] = [23; 32];
    let mode = Numerology::Khz50;
    let width = mode.carriers_per_minislot();
    let profile = Profile::new(
        PROFILE_ID,
        DATA_IUC,
        PROFILE_RANDOMIZER_SEED,
        (0..ALLOCATED_MINISLOTS)
            .map(|slot| Minislot {
                first_carrier: RUN_FIRST_CARRIER + slot * width,
                loading: DATA_LOADING,
                pilot_pattern: RUN_PILOT_PATTERN,
            })
            .collect(),
    )
    .unwrap();
    let grant = SyntheticGrant {
        id: GRANT_ID,
        profile_id: PROFILE_ID,
        epoch_sample: GRANT_EPOCH_SAMPLE,
        first_frame: FIRST_FRAME_ID,
        frames: GRANT_FRAME_COUNT,
        minislots: (0..ALLOCATED_MINISLOTS).collect(),
    };
    let us = UsConfiguration::new(
        Waveform::new(
            mode,
            PREFIX_SAMPLES,
            ROLLOFF_SAMPLES,
            MINIMUM_FRAME_SYMBOLS,
            FIXTURE_BANDWIDTH_HZ,
        )
        .unwrap(),
        vec![profile],
        grant,
        vec![],
    )
    .unwrap();
    let mut config = ExperimentConfig::new(
        us,
        SOURCE_PRBS15_SEED,
        ReceiverConfig::new(
            Estimator::FrameFlatLs,
            ESTIMATOR_PHASE_LIMIT,
            ESTIMATOR_POWER_FLOOR,
            LDPC_ITERATION_BUDGET,
        )
        .unwrap(),
    )
    .unwrap();
    config.channel.noise_variance = CHANNEL_NOISE_VARIANCE;
    config.channel.noise_seed = CHANNEL_NOISE_SEED;
    let mut observed = session(config.clone());
    let snapshot = observed
        .resource_grid(ResourceGridRequest::default())
        .unwrap();
    assert!(snapshot.transmission_planned);
    let actual = observed.run().unwrap();
    let expected = session(config).run().unwrap();
    assert!(actual.reference_recovered && expected.reference_recovered);
    assert_eq!(actual.generated_source_bits, expected.generated_source_bits);
    assert_eq!(actual.compared_source_bits, expected.compared_source_bits);
    assert_eq!(actual.bit_errors, expected.bit_errors);
    assert_eq!(actual.grants[0].words, expected.grants[0].words);
    assert_eq!(
        actual.grants[0].identity.generation,
        snapshot.configuration_generation
    );
}

#[test]
fn adjacent_loaded_body_minislot_preserves_independent_pilot_and_complementary_counts() {
    const BODY_MINISLOT_INDEX: usize = 3;
    const EXPECTED_BODY_KNOWN_PILOTS: usize = 2;
    const EXPECTED_BODY_COMPLEMENTARY_CELLS: usize = 2;
    for (mode, pattern, first, expected_data) in [
        (Numerology::Khz50, 1, 100, 44),
        (Numerology::Khz25, 8, 200, 92),
    ] {
        let mut config = configuration(mode, pattern, MINIMUM_FRAME_SYMBOLS);
        let us = &config.us_configuration;
        let mut grant = us.grant().clone();
        grant.minislots.push(BODY_MINISLOT_INDEX);
        config.us_configuration = UsConfiguration::new(
            us.waveform().clone(),
            us.profiles().to_vec(),
            grant,
            us.excluded().to_vec(),
        )
        .unwrap();
        let width = mode.carriers_per_minislot();
        let snapshot = session(config)
            .resource_grid(ResourceGridRequest {
                first_carrier: first + BODY_MINISLOT_INDEX * width,
                carrier_count: width,
                symbol_count: 0,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(snapshot.grid.counts.known_pilot, EXPECTED_BODY_KNOWN_PILOTS);
        assert_eq!(
            snapshot.grid.counts.complementary,
            EXPECTED_BODY_COMPLEMENTARY_CELLS
        );
        assert_eq!(snapshot.grid.counts.data, expected_data);
        assert_eq!(snapshot.grid.counts.unallocated, 0);
        assert!(snapshot
            .grid
            .cells
            .iter()
            .all(|cell| cell.minislot_index == Some(BODY_MINISLOT_INDEX)));
    }
}
