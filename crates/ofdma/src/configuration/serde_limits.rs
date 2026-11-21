/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Shared bounded typed JSON sequence admission, before element allocation growth.
use serde::{
    de::{Error, IgnoredAny, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

/// Deserializes at most `MAX` owned elements from a typed sequence.
///
/// Returns the collected vector; exceeding the limit rejects before storing the
/// extra element. Deserializers retain their own input-size/depth admission rules.
pub fn bounded_vec<'de, D, T, const MAX: usize>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Bounded<T, const MAX: usize>(PhantomData<T>);
    impl<'de, T: Deserialize<'de>, const MAX: usize> Visitor<'de> for Bounded<T, MAX> {
        type Value = Vec<T>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "a sequence of at most {MAX} elements")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
            if sequence.size_hint().is_some_and(|length| length > MAX) {
                return Err(A::Error::custom("sequence exceeds resource limit"));
            }
            let mut result = Vec::new();
            for _ in 0..MAX {
                match sequence.next_element()? {
                    Some(value) => result.push(value),
                    None => return Ok(result),
                }
            }
            if sequence.next_element::<IgnoredAny>()?.is_some() {
                return Err(A::Error::custom("sequence exceeds resource limit"));
            }
            Ok(result)
        }
    }
    deserializer.deserialize_seq(Bounded::<T, MAX>(PhantomData))
}
/// Deserializes an optional bounded sequence while retaining explicit null.
///
/// Returns None for null or at most `MAX` typed elements; unknown nested fields
/// remain the responsibility of each element's strict typed deserializer.
pub fn optional_vec<'de, D, T, const MAX: usize>(
    deserializer: D,
) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Optional<T, const MAX: usize>(PhantomData<T>);
    impl<'de, T: Deserialize<'de>, const MAX: usize> Visitor<'de> for Optional<T, MAX> {
        type Value = Option<Vec<T>>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(formatter, "null or a sequence of at most {MAX} elements")
        }
        fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
        fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
        fn visit_some<D: Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> Result<Self::Value, D::Error> {
            bounded_vec::<D, T, MAX>(deserializer).map(Some)
        }
    }
    deserializer.deserialize_option(Optional::<T, MAX>(PhantomData))
}
