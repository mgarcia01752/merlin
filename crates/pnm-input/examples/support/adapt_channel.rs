/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit synthetic grid adaptation for the CL-style example only.

/// Resample complex values across aligned endpoints, preserving missing knots.
///
/// # Arguments
/// * `source` - Original complex samples, with missing readings retained.
/// * `count` - Destination grid size, bounded to one 8K transform.
///
/// # Returns
/// Owned linearly interpolated I/Q values, without extrapolation.
///
/// # Errors
/// Rejects undersized/oversized grids and nonfinite supplied values.
pub fn interpolate(
    source: &[Option<[f64; 2]>],
    count: usize,
) -> Result<Vec<Option<[f64; 2]>>, &'static str> {
    if !(2..=8192).contains(&source.len()) || !(2..=8192).contains(&count) {
        return Err("Channel grids must contain 2..8192 samples");
    }
    if source.iter().flatten().flatten().any(|v| !v.is_finite()) {
        return Err("Channel samples must be finite or missing");
    }
    let denominator = count - 1;
    Ok((0..count)
        .map(|index| {
            let numerator = index * (source.len() - 1);
            let lower = numerator / denominator;
            let remainder = numerator % denominator;
            if remainder == 0 {
                return source[lower];
            }
            let left = source[lower]?;
            let right = source[lower + 1]?;
            let weight = remainder as f64 / denominator as f64;
            Some([
                left[0] + weight * (right[0] - left[0]),
                left[1] + weight * (right[1] - left[1]),
            ])
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::interpolate;

    #[test]
    fn complex_ramp_and_missing_knots() {
        assert_eq!(
            interpolate(&[Some([2.0, -2.0]), Some([6.0, 2.0])], 3).unwrap(),
            vec![Some([2.0, -2.0]), Some([4.0, 0.0]), Some([6.0, 2.0])]
        );
        assert_eq!(
            interpolate(&[Some([2.0, -2.0]), None, Some([6.0, 2.0])], 5).unwrap(),
            vec![Some([2.0, -2.0]), None, None, None, Some([6.0, 2.0])]
        );
        assert!(interpolate(&[None], 3).is_err());
        assert!(interpolate(&[None, None], 8193).is_err());
        assert!(interpolate(&[Some([f64::NAN, 0.0]), None], 3).is_err());
    }
}
