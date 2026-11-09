/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reference RX framing and equalization tests independent of the TX composer.
use docsis_ofdm_fec::{
    signal_processing::{
        receiver::{equalization::KnownReferenceEqualizer, timing::SymbolFramer},
        waveform::{SampleBlock, WaveformGeometry},
    },
    transmission::TransmissionSettings,
};
#[test]
fn framing_preserves_offsets_across_arbitrary_blocks_and_reports_partial_tail() {
    let geometry = WaveformGeometry::new(&TransmissionSettings::default()).unwrap();
    let stride = geometry.stride_samples();
    let samples: Vec<_> = (0..2 * stride + 17)
        .map(|index| [index as f64, 0.0])
        .collect();
    let mut framer = SymbolFramer::new(&geometry, 0, 13, 120).unwrap();
    let mut frames = Vec::new();
    let mut start = 0;
    while start < samples.len() {
        let end = (start + 137).min(samples.len());
        frames.extend(
            framer
                .push(&SampleBlock::new(start as u64, samples[start..end].to_vec()).unwrap())
                .unwrap(),
        );
        start = end;
    }
    assert_eq!(frames.len(), 2);
    assert_eq!(frames[0].symbol_index, 120);
    assert_eq!(frames[1].symbol_index, 121);
    assert_eq!(frames[0].period.start_index(), 13);
    assert_eq!(frames[0].period.samples(), &samples[13..13 + stride]);
    assert_eq!(
        frames[1].period.samples(),
        &samples[13 + stride..13 + 2 * stride]
    );
    assert_eq!(framer.pending_samples(), 4);
    let before = framer.pending_samples();
    assert!(framer
        .push(&SampleBlock::new(0, vec![[0.0, 0.0]]).unwrap())
        .is_err());
    assert_eq!(framer.pending_samples(), before);
    assert!(SymbolFramer::new(&geometry, 10, 9, 0).is_err());
}
#[test]
fn known_complex_gain_and_rms_are_removed_without_estimating_the_channel() {
    let equalizer = KnownReferenceEqualizer::new(&vec![[2.0, -1.0]; 4096], 0.5).unwrap();
    let recovered = equalizer.apply(&vec![[3.5, 2.0]; 4096]).unwrap(); // (2-j)*(1+j1.5)=3.5+j2 then divide reference0.5
    for sample in recovered {
        assert!((sample[0] - 2.0).abs() < 1e-14);
        assert!((sample[1] - 3.0).abs() < 1e-14);
    }
    for response in [
        vec![[0.0, 0.0]; 4096],
        vec![[f64::NAN, 0.0]; 4096],
        vec![[1.0, 0.0]; 10],
    ] {
        assert!(KnownReferenceEqualizer::new(&response, 1.0).is_err());
    }
    assert!(KnownReferenceEqualizer::new(&vec![[1.0, 0.0]; 4096], 0.0).is_err());
    assert!(SampleBlock::new(0, vec![[0.0, 0.0]; 65_537]).is_err());
    assert!(SampleBlock::new(u64::MAX, vec![[0.0, 0.0]]).is_err());
}

#[test]
fn protected_extraction_retains_bin_identity_and_scattered_placeholders() {
    use docsis_ofdm_fec::{
        signal_processing::receiver::extraction::extract,
        transmission::{
            layout::{CarrierLayout, PhysicalCarrierRole},
            SubcarrierSpacing,
        },
    };
    let settings = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        ..Default::default()
    };
    let raw = docsis_ofdm_fec::transmission::layout::CarrierLayoutSettings {
        zero_frequency_hz: 100_000_000,
        first_active_bin: 320,
        active_end_exclusive: 1280,
        plc_first_bin: 752,
        excluded_bins: vec![1200],
        additional_continuous_pilots: vec![352, 448, 544, 920, 1008, 1080, 1176, 1256],
    };
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    let bins: Vec<_> = (0..SubcarrierSpacing::Khz25.fft_samples())
        .map(|bin| [f64::from(bin), -f64::from(bin)])
        .collect();
    for symbol in [0, 119, 120, 127, 128] {
        let extracted = extract(&layout, symbol, &bins).unwrap();
        let roles = layout.symbol_roles(symbol);
        let mut rank = 0;
        for (bin, role) in roles.iter().enumerate() {
            match role {
                PhysicalCarrierRole::Data => {
                    assert_eq!(extracted.reduced[rank], Some(bins[bin]));
                    rank += 1;
                }
                PhysicalCarrierRole::ScatteredPilot => {
                    assert_eq!(extracted.reduced[rank], None);
                    rank += 1;
                }
                _ => {}
            }
        }
        for pilot in extracted.pilots {
            let bin = usize::from(pilot.bin);
            assert_eq!(pilot.iq, bins[bin]);
            assert_eq!(pilot.role, roles[bin]);
            assert!(matches!(
                pilot.role,
                PhysicalCarrierRole::ContinuousPilot | PhysicalCarrierRole::ScatteredPilot
            ));
        }
        assert_eq!(extracted.plc, bins[752..768]);
    }
    assert!(extract(&layout, 0, &bins[1..]).is_err());
}
