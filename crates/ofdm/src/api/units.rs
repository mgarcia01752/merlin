/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Checked conversion boundaries and read-only scientific result quantities.
use super::{error::invalid, ApiResult};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::Serialize;
macro_rules! quantity {
    ($name:ident, $primitive:ty, $check:expr, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(pub(crate) $primitive);
        impl $name {
            /// Converts a primitive at the explicit unit boundary.
            /// # Arguments
            /// * `value` - Value in the units named by this type.
            /// # Returns
            /// Immutable validated quantity.
            /// # Errors
            /// Rejects values outside this quantity's supported range.
            pub fn new(value: $primitive) -> ApiResult<Self> {
                if !($check)(value) {
                    return Err(invalid(concat!("Invalid ", stringify!($name))));
                }
                Ok(Self(value))
            }
            /// Reads the primitive in this quantity's declared units.
            /// # Returns
            /// Copy of the validated value; no implicit unit conversion.
            pub fn value(self) -> $primitive {
                self.0
            }
        }
    };
}
quantity!(
    ProfileId,
    u8,
    |v| v <= 15,
    "Captured profile identity, 0..15; wire mapping remains separate."
);
quantity!(
    CaptureTime,
    u32,
    |_| true,
    "Original capture epoch in whole seconds."
);
quantity!(
    AssociationTolerance,
    u32,
    |_| true,
    "Explicit maximum epoch distance in seconds."
);
quantity!(
    ObservationDuration,
    u64,
    |v| v > 0 && v <= 3_600_000,
    "Requested source observation in milliseconds."
);
quantity!(
    RuntimeLimit,
    u64,
    |v| v > 0 && v <= 300_000,
    "Per-experiment host budget in milliseconds."
);
quantity!(NoiseSeed, u64, |_| true, "Reproducible noise RNG seed.");
quantity!(
    PrbsSeed,
    u16,
    |v| v > 0 && v <= crate::common::constants::PRBS15_MAX_SEED,
    "Nonzero fifteen-bit PRBS15 source seed."
);
quantity!(
    IterationLimit,
    usize,
    |v| v > 0 && v <= 100,
    "Maximum LDPC iterations per decode."
);
quantity!(
    RetainedWordLimit,
    usize,
    |v| v <= 10_000,
    "Maximum retained detailed words."
);
quantity!(
    FrequencyHz,
    u32,
    |_| true,
    "Absolute frequency in Hz; zero is valid."
);
quantity!(
    BandwidthHz,
    u32,
    |v| (24_000_000..=192_000_000).contains(&v) && v % 6_000_000 == 0,
    "Occupied bandwidth in Hz, not data span."
);
quantity!(
    CyclicPrefixSamples,
    u16,
    |v| [192, 256, 512, 768, 1024].contains(&v),
    "Prefix length at 204.8 MS/s."
);
quantity!(
    RollOffSamples,
    u16,
    |v| [0, 64, 128, 192, 256].contains(&v),
    "Roll-off samples; zero is evaluation-only."
);
quantity!(
    InterleaverDepth,
    u8,
    |v| (1..=32).contains(&v),
    "Interleaver branches; OCD checks spacing-specific maximum."
);
quantity!(
    SubcarrierIndex,
    u16,
    |v| v < 8192,
    "Physical FFT bin coordinate."
);
quantity!(
    CarrierCount,
    usize,
    |v| v > 0 && v <= 65536,
    "Bounded original carrier count."
);
quantity!(
    MerDb,
    f64,
    |value: f64| value.is_finite(),
    "Finite MER in dB; run preflight additionally validates engine residual-power bounds."
);
quantity!(
    Weight,
    f64,
    |v: f64| v.is_finite() && (0.0..=1.0).contains(&v),
    "Normalized contribution weight."
);
macro_rules! count {
    ($name:ident,$ty:ty,$description:literal) => {
        #[doc=$description]
        #[derive(Debug, Clone, Copy, PartialEq, Serialize)]
        #[serde(transparent)]
        pub struct $name(pub(crate) $ty);
        impl $name {
            /// Reads the immutable result quantity in its declared units.
            /// # Returns
            /// Copy of the engine-derived quantity.
            pub fn value(self) -> $ty {
                self.0
            }
        }
    };
}
count!(CodewordCount, u64, "Completed simulated codeword count.");
count!(BitCount, u64, "Independently compared source bit count.");
count!(SampleCount, u64, "Valid same-bin diagnostic sample count.");
count!(
    ElapsedTime,
    u64,
    "Host elapsed milliseconds; not a reproducible scientific metric."
);
count!(
    BitErrorRatio,
    f64,
    "Post-FEC dimensionless bit-error ratio."
);
count!(
    PayloadRate,
    f64,
    "Payload information bits per second, not application goodput."
);
count!(
    LinearPower,
    f64,
    "Accumulated normalized power; not calibrated RF power."
);
impl ProfileId {
    pub const PROFILE_A: Self = Self(0);
}
impl CaptureTime {
    /// Converts original epoch seconds without inventing subsecond precision.
    /// # Arguments
    /// * `value` - Encoded whole epoch seconds.
    /// # Returns
    /// Valid capture-time value.
    /// # Errors
    /// Same checked conversion errors as `new`.
    pub fn from_epoch_seconds(value: u32) -> ApiResult<Self> {
        Self::new(value)
    }
}
impl AssociationTolerance {
    pub const ZERO: Self = Self(0);
    /// Converts an explicit timestamp tolerance in seconds.
    /// # Arguments
    /// * `value` - Maximum absolute epoch difference.
    /// # Returns
    /// Checked tolerance.
    /// # Errors
    /// Same conversion validation as `new`.
    pub fn from_seconds(value: u32) -> ApiResult<Self> {
        Self::new(value)
    }
}
impl ObservationDuration {
    /// Converts requested source milliseconds, independent of host runtime.
    /// # Arguments
    /// * `value` - Positive milliseconds, at most one hour.
    /// # Returns
    /// Valid observation request.
    /// # Errors
    /// Rejects zero or excessive duration.
    pub fn from_milliseconds(value: u64) -> ApiResult<Self> {
        Self::new(value)
    }
}

impl RetainedPayload for ProfileId {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self(value) = self;
        value.retained_children()
    }
}

impl RetainedPayload for CaptureTime {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self(value) = self;
        value.retained_children()
    }
}

impl RetainedPayload for AssociationTolerance {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self(value) = self;
        value.retained_children()
    }
}

impl RetainedPayload for BandwidthHz {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self(value) = self;
        value.retained_children()
    }
}

impl RetainedPayload for PrbsSeed {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self(value) = self;
        value.retained_children()
    }
}
