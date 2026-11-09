/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Physical pilot vectors from PHY I20 pages148,168–169 and NCP eligibility.
use docsis_ofdm_fec::transmission::{
    layout::{
        ncp::{select_ncp_cells, PreInterleavingRole},
        CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole,
    },
    SubcarrierSpacing, TransmissionSettings,
};

fn setup(spacing: SubcarrierSpacing) -> (TransmissionSettings, CarrierLayoutSettings) {
    let transmission = TransmissionSettings {
        subcarrier_spacing: spacing,
        ..Default::default()
    };
    let divisor = match spacing {
        SubcarrierSpacing::Khz25 => 1,
        SubcarrierSpacing::Khz50 => 2,
    };
    let neighborhood = 4000 / divisor..4240 / divisor;
    let additional = (0..60)
        .map(|offset| (480 + 120 * offset) / divisor)
        .filter(|bin| !neighborhood.contains(bin) && *bin != 6000 / divisor)
        .take(48)
        .collect();
    let layout = CarrierLayoutSettings {
        zero_frequency_hz: 100_000_000,
        first_active_bin: 296 / divisor,
        active_end_exclusive: 7896 / divisor,
        plc_first_bin: 4112 / divisor,
        excluded_bins: vec![6000 / divisor],
        additional_continuous_pilots: additional,
    };
    (transmission, layout)
}

#[test]
fn physical_plc_and_predefined_pilots_follow_independent_table50_vectors() {
    let (transmission, layout) = setup(SubcarrierSpacing::Khz25);
    let map = CarrierLayout::new(&transmission, &layout).unwrap();
    let roles = map.symbol_roles(0);
    assert_eq!(roles.len(), 8192);
    assert!(roles[4112..4128]
        .iter()
        .all(|role| *role == PhysicalCarrierRole::Plc));
    for bin in [4082, 4064, 4042, 4018, 4157, 4175, 4197, 4221] {
        assert_eq!(roles[bin], PhysicalCarrierRole::ContinuousPilot);
    }
    assert_eq!(roles[0], PhysicalCarrierRole::Excluded);
    assert_eq!(roles[8191], PhysicalCarrierRole::Excluded);
    assert_eq!(roles[6000], PhysicalCarrierRole::Excluded);
    assert_eq!(map.frequency_hz(4112), Some(202_800_000));
    assert_eq!(map.frequency_hz(8192), None);

    let (transmission, layout) = setup(SubcarrierSpacing::Khz50);
    let roles = CarrierLayout::new(&transmission, &layout)
        .unwrap()
        .symbol_roles(0);
    assert_eq!(roles.len(), 4096);
    assert!(roles[2056..2064]
        .iter()
        .all(|role| *role == PhysicalCarrierRole::Plc));
    for bin in [2041, 2032, 2021, 2009, 2078, 2087, 2098, 2110] {
        assert_eq!(roles[bin], PhysicalCarrierRole::ContinuousPilot);
    }
}

#[test]
fn scattered_pilots_cover_both_8k_halves_and_repeat_at_128_symbols() {
    let (transmission, layout) = setup(SubcarrierSpacing::Khz25);
    let map = CarrierLayout::new(&transmission, &layout).unwrap();
    for (phase, bin) in [
        (0, 4128),
        (1, 4130),
        (63, 4254),
        (64, 4129),
        (65, 4131),
        (127, 4255),
    ] {
        assert_eq!(
            map.symbol_roles(phase)[bin],
            PhysicalCarrierRole::ScatteredPilot
        );
        assert_eq!(map.symbol_roles(phase), map.symbol_roles(phase + 128));
    }
    // Fixed CP overrides the scattered pattern at this phase.
    assert_eq!(
        map.symbol_roles(78)[4157],
        PhysicalCarrierRole::ContinuousPilot
    );
    // PLC and exclusions remain intact even when their bin matches the pattern.
    for phase in 0..128 {
        assert_eq!(map.symbol_roles(phase)[4112], PhysicalCarrierRole::Plc);
        assert_eq!(map.symbol_roles(phase)[6000], PhysicalCarrierRole::Excluded);
    }
    assert_eq!(map.symbol_roles(u64::MAX), map.symbol_roles(127));

    let (transmission, layout) = setup(SubcarrierSpacing::Khz50);
    let map = CarrierLayout::new(&transmission, &layout).unwrap();
    assert_eq!(
        map.symbol_roles(0)[2064],
        PhysicalCarrierRole::ScatteredPilot
    );
    assert_eq!(
        map.symbol_roles(1)[2065],
        PhysicalCarrierRole::ScatteredPilot
    );
    assert_eq!(
        map.symbol_roles(127)[2191],
        PhysicalCarrierRole::ScatteredPilot
    );
}

#[test]
fn full_pilot_cycle_visits_every_eligible_bin_once_and_preserves_domain_size() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (transmission, layout) = setup(spacing);
        let map = CarrierLayout::new(&transmission, &layout).unwrap();
        let first = map.symbol_roles(0);
        let eligible = |role: &&PhysicalCarrierRole| {
            matches!(
                role,
                PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot
            )
        };
        let domain_size = first.iter().filter(eligible).count();
        let mut visits = vec![0; first.len()];
        for phase in 0..128 {
            let roles = map.symbol_roles(phase);
            assert_eq!(roles.iter().filter(eligible).count(), domain_size);
            for (bin, role) in roles.iter().enumerate() {
                if *role == PhysicalCarrierRole::ScatteredPilot {
                    visits[bin] += 1;
                }
            }
        }
        for (bin, role) in first.iter().enumerate() {
            assert_eq!(visits[bin], usize::from(eligible(&role)), "bin{bin}");
        }
    }
}

#[test]
fn invalid_geometry_and_collisions_return_errors_without_mutating_inputs() {
    let (transmission, original) = setup(SubcarrierSpacing::Khz25);
    let mut variants = Vec::new();
    let mut candidate = original.clone();
    candidate.active_end_exclusive = 9000;
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.active_end_exclusive = candidate.first_active_bin;
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.zero_frequency_hz = u32::MAX;
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.plc_first_bin += 1;
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.plc_first_bin = 0;
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.excluded_bins.push(4112);
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.excluded_bins.push(4000);
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.excluded_bins.push(6000);
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.additional_continuous_pilots[0] = 4082;
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.additional_continuous_pilots[0] = 6000;
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.additional_continuous_pilots[1] = candidate.additional_continuous_pilots[0];
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.additional_continuous_pilots.clear();
    variants.push(candidate);
    let mut candidate = original.clone();
    candidate.additional_continuous_pilots = vec![480; 121];
    variants.push(candidate);
    for candidate in variants {
        assert!(CarrierLayout::new(&transmission, &candidate).is_err());
    }
    let narrow = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        ..transmission.clone()
    };
    assert!(CarrierLayout::new(&narrow, &original).is_err());
    assert_eq!(original.excluded_bins, [6000]);
    CarrierLayout::new(&transmission, &original).unwrap();
}

#[test]
fn ncp_uses_descending_preinterleaving_data_and_skips_placeholders_and_zero_loading() {
    use PreInterleavingRole::{Data, ScatteredPilotPlaceholder, ZeroBitLoaded};
    let roles = [Data, Data, ScatteredPilotPlaceholder, Data, ZeroBitLoaded];
    assert_eq!(select_ncp_cells(&roles, 2).unwrap(), [3, 1]);
    assert!(select_ncp_cells(&roles, 0).unwrap().is_empty());
    assert!(select_ncp_cells(&roles, 4).is_err());
    assert!(select_ncp_cells(&vec![Data; 8193], 1).is_err());
    assert_eq!(roles[3], Data);
}
