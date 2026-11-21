/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Complex baseband values; admission owners validate finite samples.
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

/// Complex value in the explicit unit-reference plane of its enclosing buffer.
///
/// Raw arithmetic can overflow; stage admission/publication checks finiteness.
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Iq {
    /// In-phase (real) amplitude; finite at admitted boundaries.
    pub re: f64,
    /// Quadrature (imaginary) amplitude; finite at admitted boundaries.
    pub im: f64,
}

impl Iq {
    /// Computes squared complex amplitude without normalization.
    ///
    /// Returns real power; callers reject overflow when admitting a result.
    pub fn power(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Checks both Cartesian components, including NaN and infinities.
    ///
    /// Returns true for finite zero, including either sign of zero.
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    /// Conjugates the quadrature component without changing reference scale.
    ///
    /// Returns an owned value; no buffer or caller state changes.
    pub fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Add for Iq {
    type Output = Self;
    /// Adds `rhs` componentwise in the same reference plane.
    ///
    /// Returns an owned value; stage owners check finite results.
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Iq {
    type Output = Self;
    /// Subtracts `rhs` componentwise without normalization.
    ///
    /// Returns an owned value; stage owners check finite results.
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul for Iq {
    type Output = Self;
    /// Multiplies by complex `rhs` using Cartesian arithmetic.
    ///
    /// Returns an owned value; stage owners check finite results.
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<f64> for Iq {
    type Output = Self;
    /// Scales both components by real `rhs`, exactly once.
    ///
    /// Returns an owned value; no signal-dependent renormalization occurs.
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}
