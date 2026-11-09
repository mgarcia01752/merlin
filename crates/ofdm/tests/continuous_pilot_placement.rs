/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use docsis_ofdm_fec::transmission::{
    layout::{
        continuous::{generate_continuous_pilots, ContinuousPilotSettings},
        CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole,
    },
    SubcarrierSpacing, TransmissionSettings,
};
fn setup(spacing: SubcarrierSpacing) -> (TransmissionSettings, CarrierLayoutSettings) {
    let divisor = if spacing == SubcarrierSpacing::Khz25 {
        1
    } else {
        2
    };
    (
        TransmissionSettings {
            subcarrier_spacing: spacing,
            ..Default::default()
        },
        CarrierLayoutSettings {
            zero_frequency_hz: 100_000_000,
            first_active_bin: 296 / divisor,
            active_end_exclusive: 7896 / divisor,
            plc_first_bin: 4112 / divisor,
            excluded_bins: Vec::new(),
            additional_continuous_pilots: Vec::new(),
        },
    )
}
#[test]
fn count_edges_plc_and_deterministic_disturbance_both_spacings() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (transmission, layout) = setup(spacing);
        let settings = ContinuousPilotSettings::default();
        let generated = generate_continuous_pilots(&transmission, &layout, &settings).unwrap();
        // Independent formula: ceil(48 * 189.975/190) =48 (25k); 189.950 (50k).
        assert_eq!(generated.additional_continuous_pilots.len(), 48);
        assert!(layout.additional_continuous_pilots.is_empty());
        assert_eq!(
            generated,
            generate_continuous_pilots(&transmission, &layout, &settings).unwrap()
        );
        let map = CarrierLayout::new(&transmission, &generated).unwrap();
        let plc = map.plc_neighborhood_hz().unwrap();
        for &bin in &generated.additional_continuous_pilots {
            let frequency = map.frequency_hz(bin).unwrap();
            assert!(!plc.contains(&frequency));
            assert!((bin - layout.first_active_bin) as u32 * spacing.frequency_hz() >= 1_000_000);
            assert!(
                (layout.active_end_exclusive - 1 - bin) as u32 * spacing.frequency_hz()
                    >= 1_000_000
            );
        }
        assert_eq!(
            map.static_roles()
                .iter()
                .filter(|role| **role == PhysicalCarrierRole::ContinuousPilot)
                .count(),
            56
        );
        let deltas: std::collections::BTreeSet<_> = generated
            .additional_continuous_pilots
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        assert!(deltas.len() > 3);
    }
}
#[test]
fn isolated_narrow_region_center_poor_bins_and_bounds() {
    let (transmission, mut layout) = setup(SubcarrierSpacing::Khz25);
    layout.excluded_bins = (350..450).chain(470..600).collect();
    let settings = ContinuousPilotSettings {
        avoid_bins: (2000..2200).collect(),
        ..Default::default()
    };
    let generated = generate_continuous_pilots(&transmission, &layout, &settings).unwrap();
    assert!(generated.additional_continuous_pilots.contains(&459));
    assert!(generated
        .additional_continuous_pilots
        .iter()
        .all(|bin| !(2000..2200).contains(bin)));
    for density in [0, 47, 121, 255] {
        assert!(generate_continuous_pilots(
            &transmission,
            &layout,
            &ContinuousPilotSettings {
                density,
                ..Default::default()
            }
        )
        .is_err());
    }
    assert!(generate_continuous_pilots(
        &transmission,
        &layout,
        &ContinuousPilotSettings {
            avoid_bins: vec![459],
            ..Default::default()
        }
    )
    .is_err());
    assert!(generate_continuous_pilots(
        &transmission,
        &layout,
        &ContinuousPilotSettings {
            avoid_bins: vec![2000, 2000],
            ..Default::default()
        }
    )
    .is_err());
}

#[test]
fn actual_active_span_and_predefined_pilot_coverage_are_respected() {
    let (transmission, mut layout) = setup(SubcarrierSpacing::Khz25);
    // The isolated PLC neighborhood already has its eight predefined pilots.
    // No additional pilot is allowed inside it, so it must not demand another.
    layout.excluded_bins = vec![3999, 4240];
    let generated =
        generate_continuous_pilots(&transmission, &layout, &ContinuousPilotSettings::default())
            .unwrap();
    assert_eq!(generated.additional_continuous_pilots.len(), 48);
    assert!(!generated
        .additional_continuous_pilots
        .iter()
        .any(|bin| (4000..4240).contains(bin)));
    layout.excluded_bins = (296..1296).collect();
    let generated = generate_continuous_pilots(
        &transmission,
        &layout,
        &ContinuousPilotSettings {
            density: 120,
            ..Default::default()
        },
    )
    .unwrap();
    // ceil(120 * (7895-1296) *25000 /190000000) =105.
    assert_eq!(generated.additional_continuous_pilots.len(), 105);
}
