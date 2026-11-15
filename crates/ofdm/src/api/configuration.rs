/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! OCD resolution with explicit setting origins and no invented bandwidth.
use super::*;
use crate::{
    scenario::ScenarioKey,
    transmission::{
        layout::{CarrierLayout, CarrierLayoutSettings},
        TransmissionSettings,
    },
};
use serde::Serialize;
/// Source of a resolved configuration value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SettingOrigin {
    Explicit,
    Derived,
    Defaulted,
}
/// Immutable resolved value and its origin.
#[derive(Debug, Clone, Serialize)]
pub struct Resolved<T> {
    pub(crate) value: T,
    origin: SettingOrigin,
}
impl<T> Resolved<T> {
    /// Borrows the resolved value without permitting mutation.
    /// # Returns
    /// Value with this configuration's lifetime.
    pub fn value(&self) -> &T {
        &self.value
    }
    /// Reads the setting source, distinguishing defaults from measurements.
    /// # Returns
    /// Explicit, derived or application-default classification.
    pub fn origin(&self) -> SettingOrigin {
        self.origin
    }
}
/// Validated settings snapshot; layout remains the existing engine's domain model.
#[derive(Debug, Clone, Serialize)]
struct ResolvedValues {
    pub(crate) bandwidth: Resolved<BandwidthHz>,
    cyclic_prefix: Resolved<CyclicPrefixSamples>,
    roll_off: Resolved<RollOffSamples>,
    interleaver_depth: Resolved<InterleaverDepth>,
    protocol: ProtocolScope,
}
/// Complete read-only channel geometry, protected layout and resolved value origins.
#[derive(Debug, Clone, Serialize)]
pub struct ResolvedConfiguration {
    #[serde(flatten)]
    values: ResolvedValues,
    grid: crate::scenario::FrequencyGrid,
    layout: CarrierLayoutSettings,
}
impl ResolvedConfiguration {
    /// Borrows the exact configured original frequency grid.
    /// # Returns
    /// Validated immutable grid; no resampling or bandwidth inference.
    pub fn grid(&self) -> &crate::scenario::FrequencyGrid {
        &self.grid
    }
    /// Borrows the validated protected PLC/pilot/exclusion geometry.
    /// # Returns
    /// Existing Rust engine domain model, frozen for this configuration.
    pub fn layout(&self) -> &CarrierLayoutSettings {
        &self.layout
    }
    /// Reads resolved prefix samples and their source.
    /// # Returns
    /// Validated sample-clock quantity and origin.
    pub fn cyclic_prefix(&self) -> &Resolved<CyclicPrefixSamples> {
        &self.values.cyclic_prefix
    }
    /// Reads resolved roll-off samples and their source.
    /// # Returns
    /// Validated taper quantity and origin.
    pub fn roll_off(&self) -> &Resolved<RollOffSamples> {
        &self.values.roll_off
    }
    /// Reads spacing-validated interleaver depth and its source.
    /// # Returns
    /// Number of branches and application/capture origin.
    pub fn interleaver_depth(&self) -> &Resolved<InterleaverDepth> {
        &self.values.interleaver_depth
    }
    /// Reads occupied bandwidth and its declared origin.
    /// # Returns
    /// Immutable Hz quantity, not data-carrier span.
    pub fn bandwidth(&self) -> &Resolved<BandwidthHz> {
        &self.values.bandwidth
    }
}
/// Optional overrides; None means derive before choosing a documented default.
#[derive(Debug, Clone, Default)]
pub struct OcdOverrides {
    pub bandwidth: Option<BandwidthHz>,
    pub cyclic_prefix: Option<CyclicPrefixSamples>,
    pub roll_off: Option<RollOffSamples>,
    pub interleaver_depth: Option<InterleaverDepth>,
}
/// Checked manual OCD request with exact device/grid association.
#[derive(Debug, Clone)]
pub struct OcdConfiguration {
    pub(crate) key: ScenarioKey,
    pub(crate) layout: CarrierLayoutSettings,
    pub(crate) overrides: OcdOverrides,
}
impl OcdConfiguration {
    /// Validates a manual geometry request before profile creation.
    /// # Arguments
    /// * `key` - Original-domain device/channel/grid identity.
    /// * `layout` - Explicit physical envelope, PLC and protected pilots/exclusions.
    /// * `overrides` - Occupied bandwidth is mandatory; timing may use defaults.
    /// # Returns
    /// Checked request, validated again for protocol when constructing OCD.
    /// # Errors
    /// Rejects invalid grids, missing bandwidth and inconsistent physical extents.
    pub fn new(
        key: ScenarioKey,
        layout: CarrierLayoutSettings,
        overrides: OcdOverrides,
    ) -> ApiResult<Self> {
        key.grid.validate()?;
        if overrides.bandwidth.is_none() {
            return Err(ApiError::new(
                ErrorCode::MissingSetting,
                "Explicit occupied bandwidth is required",
            ));
        }
        if key.grid.zero_frequency_hz != layout.zero_frequency_hz
            || key.grid.first_subcarrier_index != layout.first_active_bin
            || usize::from(layout.active_end_exclusive)
                .checked_sub(usize::from(layout.first_active_bin))
                != Some(key.grid.sample_count)
        {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Manual grid must exactly cover the configured active envelope",
            ));
        }
        Ok(Self {
            key,
            layout,
            overrides,
        })
    }
}
/// Immutable validated OCD with explicit/derived/defaulted origins.
#[derive(Debug, Clone, Serialize)]
pub struct Ocd {
    pub(crate) key: ScenarioKey,
    pub(crate) transmission: TransmissionSettings,
    pub(crate) layout: CarrierLayoutSettings,
    pub(crate) configuration: ResolvedConfiguration,
}
fn resolve<T: Copy>(explicit: Option<T>, derived: Option<T>, default: T) -> Resolved<T> {
    match (explicit, derived) {
        (Some(value), _) => Resolved {
            value,
            origin: SettingOrigin::Explicit,
        },
        (_, Some(value)) => Resolved {
            value,
            origin: SettingOrigin::Derived,
        },
        _ => Resolved {
            value: default,
            origin: SettingOrigin::Defaulted,
        },
    }
}
impl Ocd {
    fn settings(
        protocol: ProtocolScope,
        key: &ScenarioKey,
        overrides: OcdOverrides,
        derived_bandwidth: Option<BandwidthHz>,
    ) -> ApiResult<(TransmissionSettings, ResolvedValues)> {
        let bandwidth=match(overrides.bandwidth,derived_bandwidth){(Some(value),Some(derived)) if value!=derived=>return Err(super::error::invalid("Bandwidth override contradicts established occupied bandwidth")),(Some(value),_)=>Resolved{value,origin:SettingOrigin::Explicit},(_,Some(value))=>Resolved{value,origin:SettingOrigin::Derived},_=>return Err(ApiError::new(ErrorCode::MissingSetting,"Occupied bandwidth is not established by this capture; provide OcdOverrides.bandwidth"))};
        let cp = resolve(overrides.cyclic_prefix, None, CyclicPrefixSamples(512));
        let rp = resolve(overrides.roll_off, None, RollOffSamples(128));
        let depth = resolve(overrides.interleaver_depth, None, InterleaverDepth(16));
        let spacing = match key.grid.spacing_hz {
            25000 => SubcarrierSpacing::Khz25,
            50000 => SubcarrierSpacing::Khz50,
            _ => return Err(super::error::invalid("Unsupported subcarrier spacing")),
        };
        let transmission = TransmissionSettings {
            occupied_bandwidth_hz: bandwidth.value.0,
            subcarrier_spacing: spacing,
            cyclic_prefix_samples: cp.value.0,
            roll_off_samples: rp.value.0,
            time_interleaver_depth: depth.value.0,
        };
        transmission.validate()?;
        Ok((
            transmission,
            ResolvedValues {
                bandwidth,
                cyclic_prefix: cp,
                roll_off: rp,
                interleaver_depth: depth,
                protocol,
            },
        ))
    }
    /// Resolves capture-established geometry and optional timing overrides.
    /// # Arguments
    /// * `protocol` - Explicit interpretation; never inferred from file format.
    /// * `dpd` - Selected original or hypothesis profile.
    /// * `overrides` - Explicit values take priority, subject to consistency checks.
    /// # Returns
    /// Frozen OCD; CP/RP/depth default to 512/128/16 with origin recorded.
    /// # Errors
    /// Rejects unresolved bandwidth, invalid timing and unsupported protected roles.
    pub fn from_profile(
        protocol: ProtocolScope,
        dpd: &Dpd,
        overrides: OcdOverrides,
    ) -> ApiResult<Self> {
        let (transmission, values) =
            Self::settings(protocol, &dpd.profile.key, overrides, dpd.bandwidth)?;
        let layout =
            crate::simulation::channel::measurement::captured_layout(&dpd.profile, &transmission)?;
        Ok(Self {
            key: dpd.profile.key.clone(),
            transmission,
            configuration: ResolvedConfiguration {
                values,
                grid: dpd.profile.key.grid.clone(),
                layout: layout.clone(),
            },
            layout,
        })
    }
    /// Admits a manually configured channel before creating a DPD hypothesis.
    /// # Arguments
    /// * `protocol` - Explicit DOCSIS scope.
    /// * `configuration` - Checked full geometry and settings request.
    /// # Returns
    /// Validated immutable OCD with no measured-setting claim.
    /// # Errors
    /// Rejects unsupported layout, bandwidth, timing or grid.
    pub fn new(protocol: ProtocolScope, configuration: OcdConfiguration) -> ApiResult<Self> {
        let (transmission, resolved) =
            Self::settings(protocol, &configuration.key, configuration.overrides, None)?;
        CarrierLayout::new(&transmission, &configuration.layout)?;
        Ok(Self {
            configuration: ResolvedConfiguration {
                values: resolved,
                grid: configuration.key.grid.clone(),
                layout: configuration.layout.clone(),
            },
            key: configuration.key,
            transmission,
            layout: configuration.layout,
        })
    }
    /// Borrows resolved values with their origins.
    /// # Returns
    /// Immutable configuration snapshot.
    pub fn configuration(&self) -> &ResolvedConfiguration {
        &self.configuration
    }
}
