/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::{
        modulation::map_symbol,
        receiver::channel_estimation::{
            acquisition::CarrierOffsetCorrection,
            search::{search_acquisition, AcquisitionHypothesis, AcquisitionSearchSettings},
        },
        waveform::{SampleBlock, WaveformGeometry},
    },
    simulation::waveform::{WaveformReceiver, WaveformTransmitter},
    transmission::{
        frame::assemble,
        layout::{
            continuous::{generate_continuous_pilots, ContinuousPilotSettings},
            CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole,
        },
        settings::SAMPLE_RATE_HZ,
        SubcarrierSpacing, TransmissionSettings,
    },
    types::IqPair,
};
const SAMPLE_ORIGIN: u64 = 700;
const TIMING_OFFSET: usize = 23;
const INITIAL_PHASE: u64 = 53;

fn hypothesis(width: u32, spacing: SubcarrierSpacing) -> AcquisitionHypothesis {
    let divisor = if spacing == SubcarrierSpacing::Khz25 {
        1
    } else {
        2
    };
    let transmission = TransmissionSettings {
        occupied_bandwidth_hz: width,
        subcarrier_spacing: spacing,
        ..Default::default()
    };
    let geometry = WaveformGeometry::new(&transmission).unwrap();
    let first = 296 / divisor;
    let raw = CarrierLayoutSettings {
        zero_frequency_hz: 100_000_000,
        first_active_bin: first,
        active_end_exclusive: first + ((width - 2_000_000) / spacing.frequency_hz()) as u16,
        plc_first_bin: 752 / divisor,
        excluded_bins: Vec::new(),
        additional_continuous_pilots: Vec::new(),
    };
    let layout =
        generate_continuous_pilots(&transmission, &raw, &ContinuousPilotSettings::default())
            .unwrap();
    AcquisitionHypothesis { geometry, layout }
}
fn physical(
    hypothesis: &AcquisitionHypothesis,
    phase: u64,
) -> docsis_ofdm_fec::transmission::frame::PhysicalFrequencySymbol {
    let layout = CarrierLayout::new(hypothesis.geometry.settings(), &hypothesis.layout).unwrap();
    let reduced: Vec<_> = layout
        .symbol_roles(phase)
        .iter()
        .enumerate()
        .filter_map(|(bin, role)| match role {
            PhysicalCarrierRole::Data => Some(Some(
                map_symbol(256, ((bin * 73 + phase as usize * 19) % 256) as u16).unwrap(),
            )),
            PhysicalCarrierRole::ScatteredPilot => Some(None),
            _ => None,
        })
        .collect();
    let plc_width = if hypothesis.geometry.fft_samples() == 8192 {
        16
    } else {
        8
    };
    assemble(&layout, phase, &reduced, &vec![[1.0, 0.0]; plc_width], 1.0).unwrap()
}
fn stream(hypothesis: &AcquisitionHypothesis, cfo: f64, echo: bool) -> SampleBlock {
    let mut transmitter = WaveformTransmitter::new(&hypothesis.geometry, INITIAL_PHASE, 0);
    let mut transmitted = Vec::new();
    for index in 0..5 {
        transmitted.extend_from_slice(
            transmitter
                .transmit(&physical(hypothesis, INITIAL_PHASE + index))
                .unwrap()
                .output
                .samples(),
        );
    }
    impaired_samples(&transmitted, cfo, echo)
}
fn impaired_samples(transmitted: &[IqPair], cfo: f64, echo: bool) -> SampleBlock {
    // Direct time-domain convolution with two reflected paths; no TX H enters RX.
    let mut samples = vec![[0.0, 0.0]; TIMING_OFFSET];
    for index in 0..transmitted.len() {
        let mut point = transmitted[index];
        if echo {
            for (delay, gain) in [(4, 0.15), (12, -0.07)] {
                if index >= delay {
                    for (value, delayed) in point.iter_mut().zip(transmitted[index - delay]) {
                        *value += gain * delayed;
                    }
                }
            }
        }
        samples.push([
            0.8 * (0.9 * point[0] - 0.4358898943540673 * point[1]),
            0.8 * (0.4358898943540673 * point[0] + 0.9 * point[1]),
        ]);
    }
    let mut noise = 0x419abcdu32;
    for (index, point) in samples.iter_mut().enumerate() {
        let (sine, cosine) =
            (std::f64::consts::TAU * cfo * index as f64 / f64::from(SAMPLE_RATE_HZ)).sin_cos();
        let rotated = [
            point[0] * cosine - point[1] * sine,
            point[0] * sine + point[1] * cosine,
        ];
        noise = noise.wrapping_mul(1664525).wrapping_add(1013904223);
        *point = [
            rotated[0] + (f64::from(noise & 65535) / 32768.0 - 1.0) * 1e-5,
            rotated[1] + (f64::from(noise >> 16) / 32768.0 - 1.0) * 1e-5,
        ];
    }
    SampleBlock::new(SAMPLE_ORIGIN, samples).unwrap()
}
#[test]
fn both_spacings_three_widths_integer_offsets_and_reflected_channels_acquire() {
    for width in [24_000_000, 90_000_000, 192_000_000] {
        for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
            let candidates = [
                hypothesis(width, SubcarrierSpacing::Khz50),
                hypothesis(width, SubcarrierSpacing::Khz25),
            ];
            let selected = usize::from(spacing == SubcarrierSpacing::Khz25);
            for echo in [false, true] {
                let integer = if echo { -3 } else { 2 };
                let cfo = f64::from(integer) * f64::from(spacing.frequency_hz()) + 3700.0;
                let source = stream(&candidates[selected], cfo, echo);
                let settings = AcquisitionSearchSettings {
                    max_offset_samples: 40,
                    minimum_correlation: 0.97,
                    max_integer_carrier_offset: 3,
                };
                let (_, estimate, received) =
                    WaveformReceiver::acquire_search(&candidates, &source, &settings)
                        .unwrap_or_else(|error| {
                            panic!("width={width},spacing={spacing:?},echo={echo}: {error}")
                        });
                assert_eq!(estimate.hypothesis_index, selected);
                assert_eq!(estimate.integer_carrier_offset, integer);
                assert!((estimate.acquisition.carrier_offset_hz - cfo).abs() < 5.0);
                assert_eq!(estimate.acquisition.pilot_phase, INITIAL_PHASE);
                assert!(
                    estimate
                        .acquisition
                        .timing
                        .first_symbol_sample
                        .abs_diff(SAMPLE_ORIGIN + TIMING_OFFSET as u64)
                        <= 14
                );
                assert!(estimate.acquisition.normalized_pilot_error < 0.03);
                assert!(received.len() >= 4);
                let layout = CarrierLayout::new(
                    candidates[selected].geometry.settings(),
                    &candidates[selected].layout,
                )
                .unwrap();
                for symbol in &received {
                    let expected = physical(&candidates[selected], symbol.symbol_index);
                    let mut error = 0.0;
                    let mut power = 0.0;
                    for (bin, role) in layout.symbol_roles(symbol.symbol_index).iter().enumerate() {
                        if *role == PhysicalCarrierRole::Data {
                            let original = expected.bins[bin].unwrap();
                            error += (symbol.equalized[bin][0] - original[0]).powi(2)
                                + (symbol.equalized[bin][1] - original[1]).powi(2);
                            power += original[0].powi(2) + original[1].powi(2);
                        }
                    }
                    assert!(
                        error / power < 0.03,
                        "width={width},spacing={spacing:?},echo={echo}: EVM power={}",
                        error / power
                    );
                }
            }
        }
    }
}
#[test]
fn search_rejection_is_bounded_and_total_correction_preserves_chunk_phase() {
    let candidate = hypothesis(24_000_000, SubcarrierSpacing::Khz25);
    let settings = AcquisitionSearchSettings {
        max_offset_samples: 40,
        minimum_correlation: 0.97,
        max_integer_carrier_offset: 3,
    };
    let silent = SampleBlock::new(0, vec![[0.0, 0.0]; 20000]).unwrap();
    assert!(search_acquisition(&[], &silent, &settings).is_err());
    assert!(
        search_acquisition(&[candidate.clone(), candidate.clone()], &silent, &settings).is_err()
    );
    assert!(search_acquisition(std::slice::from_ref(&candidate), &silent, &settings).is_err());
    assert!(search_acquisition(
        std::slice::from_ref(&candidate),
        &SampleBlock::new(0, vec![[1.0, 0.0]; 100]).unwrap(),
        &settings
    )
    .is_err());
    let source = stream(&candidate, 4.0 * 25000.0 + 3700.0, false);
    assert!(search_acquisition(std::slice::from_ref(&candidate), &source, &settings).is_err());
    let correction =
        CarrierOffsetCorrection::with_integer_offset(&candidate.geometry, -900.0, -3, 500).unwrap();
    let points: Vec<IqPair> = (0..100).map(|index| [index as f64 / 100.0, 0.2]).collect();
    let full = correction
        .apply(&SampleBlock::new(500, points.clone()).unwrap())
        .unwrap();
    let left = correction
        .apply(&SampleBlock::new(500, points[..31].to_vec()).unwrap())
        .unwrap();
    let right = correction
        .apply(&SampleBlock::new(531, points[31..].to_vec()).unwrap())
        .unwrap();
    assert_eq!(full.samples(), [left.samples(), right.samples()].concat());
    assert!(
        CarrierOffsetCorrection::with_integer_offset(&candidate.geometry, 0.0, i16::MIN, 0)
            .is_err()
    );
    for policy in [
        AcquisitionSearchSettings {
            max_offset_samples: 1025,
            ..settings
        },
        AcquisitionSearchSettings {
            max_integer_carrier_offset: 17,
            ..settings
        },
    ] {
        assert!(search_acquisition(std::slice::from_ref(&candidate), &source, &policy).is_err());
    }
}

#[test]
fn encoded_block_recovers_after_acquired_equalization_for_all_widths_and_spacings() {
    use docsis_ofdm_fec::signal_processing::modulation::{interleaving::BitInterleaver, qam_llrs};
    use fec_codec::concatenated::{compare_payload, ConcatenatedCodec};
    let payload: Vec<u8> = (0..14232)
        .map(|index| u8::from((index * 73 + index / 19) % 11 < 5))
        .collect();
    let bits = BitInterleaver::new(256).unwrap();
    let mut codec = ConcatenatedCodec::new().unwrap();
    let labels = bits
        .encode_labels(&codec.encode(&payload).unwrap().ldpc_codeword)
        .unwrap();
    for width in [24_000_000, 90_000_000, 192_000_000] {
        for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
            let candidates = [
                hypothesis(width, SubcarrierSpacing::Khz50),
                hypothesis(width, SubcarrierSpacing::Khz25),
            ];
            let selected = usize::from(spacing == SubcarrierSpacing::Khz25);
            let candidate = &candidates[selected];
            let layout =
                CarrierLayout::new(candidate.geometry.settings(), &candidate.layout).unwrap();
            let mut transmitter = WaveformTransmitter::new(&candidate.geometry, INITIAL_PHASE, 0);
            let mut source = labels.iter().copied();
            let mut samples = Vec::new();
            for phase in INITIAL_PHASE..INITIAL_PHASE + 6 {
                let reduced: Vec<_> = layout
                    .symbol_roles(phase)
                    .iter()
                    .filter_map(|role| match role {
                        PhysicalCarrierRole::Data => {
                            Some(Some(map_symbol(256, source.next().unwrap_or(0)).unwrap()))
                        }
                        PhysicalCarrierRole::ScatteredPilot => Some(None),
                        _ => None,
                    })
                    .collect();
                let plc_width = if spacing == SubcarrierSpacing::Khz25 {
                    16
                } else {
                    8
                };
                let physical =
                    assemble(&layout, phase, &reduced, &vec![[1.0, 0.0]; plc_width], 1.0).unwrap();
                samples
                    .extend_from_slice(transmitter.transmit(&physical).unwrap().output.samples());
            }
            assert!(source.next().is_none());
            let received = impaired_samples(
                &samples,
                -2.0 * f64::from(spacing.frequency_hz()) - 1900.0,
                true,
            );
            let policy = AcquisitionSearchSettings {
                max_offset_samples: 40,
                minimum_correlation: 0.97,
                max_integer_carrier_offset: 3,
            };
            let (_, estimate, symbols) =
                WaveformReceiver::acquire_search(&candidates, &received, &policy).unwrap();
            assert_eq!(estimate.hypothesis_index, selected);
            let mut soft = Vec::new();
            for symbol in symbols {
                for (bin, role) in layout.symbol_roles(symbol.symbol_index).iter().enumerate() {
                    if *role == PhysicalCarrierRole::Data && soft.len() < 16200 {
                        soft.extend(qam_llrs(256, symbol.equalized[bin], 0.01).unwrap());
                    }
                }
            }
            assert_eq!(soft.len(), 16200);
            let decoded = codec.decode(&bits.decode_llrs(&soft).unwrap(), 50).unwrap();
            assert!(
                decoded.ldpc.success && decoded.bch.success,
                "width={width},spacing={spacing:?}"
            );
            assert_eq!(compare_payload(&payload, &decoded).unwrap().bit_errors, 0);
        }
    }
}

#[test]
fn acquired_receiver_continues_across_raw_sample_block_splits() {
    let candidates = [
        hypothesis(24_000_000, SubcarrierSpacing::Khz50),
        hypothesis(24_000_000, SubcarrierSpacing::Khz25),
    ];
    let input = stream(&candidates[0], -3.0 * 50000.0 + 3700.0, true);
    let policy = AcquisitionSearchSettings {
        max_offset_samples: 40,
        minimum_correlation: 0.97,
        max_integer_carrier_offset: 3,
    };
    let (_, full_estimate, full) =
        WaveformReceiver::acquire_search(&candidates, &input, &policy).unwrap();
    let first = SampleBlock::new(input.start_index(), input.samples()[..18000].to_vec()).unwrap();
    let (mut receiver, split_estimate, mut split) =
        WaveformReceiver::acquire_search(&candidates, &first, &policy).unwrap();
    assert_eq!(
        split_estimate.acquisition.carrier_offset_hz,
        full_estimate.acquisition.carrier_offset_hz
    );
    let mut position = 18000;
    for block in input.samples()[position..].chunks(137) {
        split.extend(
            receiver
                .receive(
                    &SampleBlock::new(input.start_index() + position as u64, block.to_vec())
                        .unwrap(),
                )
                .unwrap(),
        );
        position += block.len();
    }
    assert_eq!(split.len(), full.len());
    for (actual, expected) in split.iter().zip(&full) {
        assert_eq!(actual.symbol_index, expected.symbol_index);
        assert_eq!(actual.bins, expected.bins);
        assert_eq!(actual.equalized, expected.equalized);
    }
}

#[test]
fn physically_indistinguishable_hypotheses_reject_instead_of_selecting_first() {
    let candidate = hypothesis(24_000_000, SubcarrierSpacing::Khz25);
    let mut wider = candidate.clone();
    let mut settings = wider.geometry.settings().clone();
    settings.occupied_bandwidth_hz = 30_000_000;
    wider.geometry = WaveformGeometry::new(&settings).unwrap();
    let samples = stream(&candidate, 3700.0, false);
    let policy = AcquisitionSearchSettings {
        max_offset_samples: 40,
        minimum_correlation: 0.97,
        max_integer_carrier_offset: 3,
    };
    // Occupied-width metadata differs, but the same active bins/pilots/samples
    // cannot distinguish the two bootstrap interpretations.
    let error = search_acquisition(&[candidate, wider], &samples, &policy).unwrap_err();
    assert!(error.message.contains("Ambiguous"));
}
