/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reusable unit conversion factors, independent of file formats.

/// Number of hertz in one kilohertz.
/// Multiply an integer kHz value by this factor to express it in Hz.
pub const HZ_PER_KHZ: u32 = 1_000;

/// Number of quarter-decibel steps in one decibel.
/// Divide a valid quarter-dB sample by this factor to express it in dB.
/// Format-specific unavailable-value handling belongs to the decoder.
pub const QUARTER_DB_PER_DB: f64 = 4.0;

/// Number of hundredth-decibel steps in one decibel.
pub const HUNDREDTH_DB_PER_DB: f64 = 100.0;

/// Fixed-point counts per unit for signed s2.13 components.
pub const S2_13_UNITS_PER_UNIT: f64 = 8192.0;

/// Fixed-point counts per unit for signed s1.14 components.
pub const S1_14_UNITS_PER_UNIT: f64 = 16384.0;

/// Number of microseconds in one second.
pub const MICROSECONDS_PER_SECOND: f64 = 1_000_000.0;

/// Number of bits in one decimal megabit; divide bit/s by this to show Mbit/s.
pub const BITS_PER_MEGABIT: super::BitRateBps = 1_000_000;
