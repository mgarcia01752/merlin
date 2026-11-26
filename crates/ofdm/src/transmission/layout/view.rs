/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive original-bin layout projection over the existing physical-role owner.
use super::{CarrierLayout, PhysicalCarrierRole};
use crate::{error::invalid, SimulationError};
use serde::Serialize;

/// Application overlay budget; all requested per-bin roles are still preserved.
const MAX_PHYSICAL_ROLE_SPANS: usize = 1024;

/// One original increasing-frequency FFT location; Data may transport NCP or filler.
#[derive(Serialize)]
#[non_exhaustive]
pub struct PhysicalLayoutCarrier {
    pub physical_bin: u16,
    pub frequency_hz: u32,
    pub role: PhysicalCarrierRole,
}
/// Contiguous equal-role interval on the original physical grid, end exclusive.
#[derive(Serialize)]
#[non_exhaustive]
pub struct PhysicalRoleSpan {
    pub first_bin: u16,
    pub end_exclusive: u16,
    pub role: PhysicalCarrierRole,
}
/// Bounded layout window; original FFT geometry is independent of capture/display extents.
#[derive(Serialize)]
#[non_exhaustive]
pub struct PhysicalLayoutView {
    pub channel_symbol_index: u64,
    pub zero_frequency_hz: u32,
    pub spacing_hz: u32,
    pub fft_bins: usize,
    pub first_bin: usize,
    pub carriers: Vec<PhysicalLayoutCarrier>,
    pub role_spans: Vec<PhysicalRoleSpan>,
    pub omitted_role_spans: usize,
    pub plc_first_bin: u16,
    pub plc_end_exclusive: u16,
}

/// Copies original FFT geometry through the existing phase-specific role owner.
/// # Arguments
/// * `layout`, `symbol` - Accepted layout and original backend channel-symbol index.
/// * `start`, `count` - Original FFT window; zero count means remaining bins.
/// # Returns
/// At most 8192 carriers and 1024 spans, with omitted-span accounting.
/// # Errors
/// Rejects invalid windows, missing PLC or unrepresentable frequencies.
pub(crate) fn layout_view(
    layout: &CarrierLayout,
    symbol: u64,
    start: usize,
    count: usize,
) -> Result<PhysicalLayoutView, SimulationError> {
    let roles = layout.symbol_roles(symbol);
    let remaining = roles
        .len()
        .checked_sub(start)
        .filter(|n| *n > 0)
        .ok_or_else(|| invalid("Physical layout start is outside the original FFT grid"))?;
    let count = if count == 0 { remaining } else { count };
    if count > remaining {
        return Err(invalid(
            "Physical layout window is outside the original FFT grid",
        ));
    }
    let carriers = (start..start + count)
        .map(|bin| {
            Ok(PhysicalLayoutCarrier {
                physical_bin: bin as u16,
                frequency_hz: layout
                    .frequency_hz(bin as u16)
                    .ok_or_else(|| invalid("Physical carrier frequency is unavailable"))?,
                role: roles[bin],
            })
        })
        .collect::<Result<Vec<_>, SimulationError>>()?;
    let mut role_spans = Vec::new();
    let mut spans = 0;
    let mut first = start;
    while first < start + count {
        let mut end = first + 1;
        while end < start + count && roles[end] == roles[first] {
            end += 1;
        }
        spans += 1;
        if role_spans.len() < MAX_PHYSICAL_ROLE_SPANS {
            role_spans.push(PhysicalRoleSpan {
                first_bin: first as u16,
                end_exclusive: end as u16,
                role: roles[first],
            });
        }
        first = end;
    }
    let plc_first = roles
        .iter()
        .position(|role| *role == PhysicalCarrierRole::Plc)
        .ok_or_else(|| invalid("Physical layout has no PLC"))?;
    let plc_end = roles
        .iter()
        .rposition(|role| *role == PhysicalCarrierRole::Plc)
        .unwrap()
        + 1;
    Ok(PhysicalLayoutView {
        channel_symbol_index: symbol,
        zero_frequency_hz: layout.frequency_hz(0).unwrap(),
        spacing_hz: layout.subcarrier_spacing().frequency_hz(),
        fft_bins: roles.len(),
        first_bin: start,
        carriers,
        omitted_role_spans: spans - role_spans.len(),
        role_spans,
        plc_first_bin: plc_first as u16,
        plc_end_exclusive: plc_end as u16,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transmission::{
        layout::CarrierLayoutSettings, SubcarrierSpacing, TransmissionSettings,
    };

    fn layout(spacing: SubcarrierSpacing) -> CarrierLayout {
        let divisor = if spacing == SubcarrierSpacing::Khz25 {
            1
        } else {
            2
        };
        CarrierLayout::new(
            &TransmissionSettings {
                subcarrier_spacing: spacing,
                ..Default::default()
            },
            &CarrierLayoutSettings {
                zero_frequency_hz: 100_000_000,
                first_active_bin: 200 / divisor,
                active_end_exclusive: 7800 / divisor,
                plc_first_bin: 4112 / divisor,
                excluded_bins: vec![6000 / divisor],
                additional_continuous_pilots: vec![400, 410, 420, 430, 440, 450, 460, 470],
            },
        )
        .unwrap()
    }

    #[test]
    fn original_4k_8k_layout_windows_keep_literal_plc_pilot_and_frequency_vectors() {
        for (spacing, divisor, fft_bins) in [
            (SubcarrierSpacing::Khz25, 1, 8192),
            (SubcarrierSpacing::Khz50, 2, 4096),
        ] {
            let layout = layout(spacing);
            let full = layout_view(&layout, 0, 0, 0).unwrap();
            assert_eq!(full.fft_bins, fft_bins);
            assert_eq!(full.carriers.len(), fft_bins);
            assert_eq!(full.carriers[0].role, PhysicalCarrierRole::Excluded);
            assert_eq!(
                full.carriers[6000 / divisor].role,
                PhysicalCarrierRole::Excluded
            );
            assert_eq!(full.plc_first_bin as usize, 4112 / divisor);
            assert_eq!(full.plc_end_exclusive as usize, 4128 / divisor);
            assert_eq!(full.carriers[4112 / divisor].frequency_hz, 202_800_000);
            let pilot_bin = if divisor == 1 { 4082 } else { 2041 };
            assert_eq!(
                full.carriers[pilot_bin].role,
                PhysicalCarrierRole::ContinuousPilot
            );
            assert_eq!(
                full.carriers[4128 / divisor].role,
                PhysicalCarrierRole::ScatteredPilot
            );
            let shifted = layout_view(&layout, 1, 4128 / divisor, 3).unwrap();
            assert_eq!(shifted.first_bin, 4128 / divisor);
            assert_eq!(
                shifted.carriers[2 / divisor].role,
                PhysicalCarrierRole::ScatteredPilot
            );
            assert!(shifted
                .role_spans
                .iter()
                .all(|span| usize::from(span.first_bin) >= shifted.first_bin));
            assert!(full.role_spans.len() <= 1024);
        }
    }

    #[test]
    fn physical_window_bounds_reject_before_unchecked_arithmetic() {
        let layout = layout(SubcarrierSpacing::Khz25);
        for (start, count) in [(8192, 0), (8191, 2), (usize::MAX, 1), (0, usize::MAX)] {
            assert!(layout_view(&layout, 0, start, count).is_err());
        }
        let last = layout_view(&layout, u64::MAX, 8191, 0).unwrap();
        assert_eq!(last.carriers.len(), 1);
        assert_eq!(last.carriers[0].physical_bin, 8191);
    }
}
