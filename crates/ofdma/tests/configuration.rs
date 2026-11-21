/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent table boundaries and transactional immutable admission.
use docsis_ofdma::{configuration::*, types::*, PhyError};

fn configuration(
    mode: Numerology,
    slots: Vec<Minislot>,
    excluded: Vec<usize>,
) -> Result<UsConfiguration, PhyError> {
    let profile = Profile::new(224, 5, 0x7f_ffff, slots)?;
    let grant = SyntheticGrant {
        id: 1,
        profile_id: 224,
        epoch_sample: 73,
        first_frame: 2,
        frames: 1,
        minislots: vec![0],
    };
    UsConfiguration::new(
        Waveform::new(mode, 192, 64, 6, 20_000_000)?,
        vec![profile],
        grant,
        excluded,
    )
}

#[test]
fn original_numerology_and_all_prefix_rolloff_values() {
    for (mode, n, spacing, q) in [
        (Numerology::Khz50, 2048, 50_000, 8),
        (Numerology::Khz25, 4096, 25_000, 16),
    ] {
        assert_eq!(
            (
                mode.fft_len(),
                mode.spacing_hz(),
                mode.carriers_per_minislot()
            ),
            (n, spacing, q)
        );
        assert_eq!(SAMPLE_RATE_HZ, 102_400_000);
        for prefix in [96, 128, 160, 192, 224, 256, 288, 320, 384, 512, 640] {
            for rolloff in [0, 32, 64, 96, 128, 160, 192, 224] {
                assert_eq!(
                    Waveform::new(mode, prefix, rolloff, 6, 20_000_000).is_ok(),
                    rolloff < prefix
                );
            }
        }
        for (cp, rp) in [(0, 0), (97, 0), (192, 1), (192, 256), (192, 192)] {
            assert!(Waveform::new(mode, cp, rp, 6, 20_000_000).is_err());
        }
    }
}

#[test]
fn frame_bandwidth_thresholds_are_encompassed_bandwidth() {
    for (bandwidth, maximum) in [
        (47_999_999, 36),
        (48_000_000, 24),
        (71_999_999, 24),
        (72_000_000, 18),
    ] {
        for (mode, k) in [
            (Numerology::Khz50, maximum),
            (Numerology::Khz25, maximum / 2),
        ] {
            assert!(Waveform::new(mode, 192, 0, k, bandwidth).is_ok());
            assert!(Waveform::new(mode, 192, 0, k + 1, bandwidth).is_err());
            assert!(Waveform::new(mode, 192, 0, 5, bandwidth).is_err());
        }
    }
}

#[test]
fn explicit_iuc_inventory_and_rejected_candidate_preserve_snapshot() {
    let slot = Minislot {
        first_carrier: 128,
        loading: 4,
        pilot_pattern: 1,
    };
    let accepted = configuration(Numerology::Khz50, vec![slot.clone()], vec![0, 1]).unwrap();
    let snapshot = accepted.clone();
    assert_eq!(accepted.selected_profile().id(), 224);
    assert_eq!(accepted.selected_profile().iuc(), 5);
    assert_eq!(accepted.selected_profile().minislots()[0].loading, 4);
    let mut incompatible = slot.clone();
    incompatible.pilot_pattern = 8;
    assert!(configuration(Numerology::Khz50, vec![incompatible], vec![]).is_err());
    assert!(configuration(Numerology::Khz50, vec![slot.clone()], vec![130]).is_err());
    assert!(configuration(Numerology::Khz50, vec![slot.clone(), slot], vec![]).is_err());
    assert_eq!(accepted, snapshot);
    assert!(Profile::new(
        224,
        7,
        1,
        vec![Minislot {
            first_carrier: 128,
            loading: 4,
            pilot_pattern: 1
        }]
    )
    .is_err());
}

#[test]
fn overflow_and_nonfinite_boundaries_reject_without_wrapping() {
    assert!(BitRange::new(u64::MAX, 1).is_err());
    let range = BitRange::new(5, 3).unwrap();
    assert_eq!((range.start(), range.length(), range.end()), (5, 3, 8));
    let id = Identity {
        run: 1,
        generation: 1,
        grant: 0,
    };
    let mut block = SampleBlock {
        identity: id,
        start: u64::MAX,
        samples: vec![Iq { re: 0.0, im: -0.0 }],
    };
    assert!(block.validate(1).is_err());
    block.start = 0;
    assert_eq!(block.validate(1).unwrap(), 1);
    block.samples[0].re = f64::NAN;
    assert!(block.validate(1).is_err());
    block.samples[0].re = f64::INFINITY;
    assert!(block.validate(1).is_err());
    assert!(configuration(
        Numerology::Khz50,
        vec![Minislot {
            first_carrier: usize::MAX,
            loading: 4,
            pilot_pattern: 1
        }],
        vec![]
    )
    .is_err());
    for mode in [
        BurstMode::InitialRanging,
        BurstMode::FineRanging,
        BurstMode::Probing,
    ] {
        assert!(mode.validate().is_err());
    }
}
