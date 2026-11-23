/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Common physical and PRBS15 definitions; no application resource/default policies.

/// Exact vacuum light speed in metres per second, used with an explicit velocity factor.
pub const SPEED_OF_LIGHT_METRES_PER_SECOND: f64 = 299_792_458.0;
/// Exact international-foot conversion to metres, independent of propagation velocity.
pub const METRES_PER_FOOT: f64 = 0.3048;
/// Exact microsecond-to-second scale used by time-domain diagnostic projections.
pub const SECONDS_PER_MICROSECOND: f64 = 1.0 / MICROSECONDS_PER_SECOND;
/// Exact count of microseconds in one second; reciprocal time scales share this unit definition.
pub const MICROSECONDS_PER_SECOND: f64 = 1_000_000.0;
/// Exact integer milliseconds in one second for checked sample-clock conversions.
/// This unit scale does not choose a source-observation or host-runtime budget.
pub const MILLISECONDS_PER_SECOND: u128 = 1_000;
/// Exact integer nanoseconds in one second for Duration-to-sample conversion.
pub const NANOSECONDS_PER_SECOND: u128 = 1_000_000_000;

/// Register width for the fixed PRBS15 recurrence x^15 + x^14 + 1.
pub const PRBS15_REGISTER_BITS: u32 = 15;
/// Highest register-bit position used by the existing LSB-first recurrence.
pub const PRBS15_FEEDBACK_SHIFT: u32 = PRBS15_REGISTER_BITS - 1;
/// Low-fifteen-bit register mask; 0x7fff means all fifteen participating bits are set.
pub const PRBS15_REGISTER_MASK: u16 = (1 << PRBS15_REGISTER_BITS) - 1;
/// Largest valid nonzero PRBS15 seed, determined by the register's participating bits.
pub const PRBS15_MAX_SEED: u16 = PRBS15_REGISTER_MASK;
/// Maximal period in output bits for this fixed polynomial, independent of chosen seed/default.
pub const PRBS15_PERIOD_BITS: u16 = (1 << PRBS15_REGISTER_BITS) - 1;
