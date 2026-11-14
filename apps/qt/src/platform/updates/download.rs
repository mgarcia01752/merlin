/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded HTTPS package transport and digest verification shared by installers.
use sha2::{Digest, Sha256};
use std::{
    io::{Read, Write},
    time::{Duration, Instant},
};

/// Restricts each download hop to approved HTTPS hosts without credentials.
/// # Arguments
/// * `address` - Parsed release URL or redirect target.
/// # Returns
/// True only for the exact GitHub asset hosts and their default HTTPS port.
pub(super) fn trusted(address: &url::Url) -> bool {
    address.scheme() == "https"
        && address.username().is_empty()
        && address.password().is_none()
        && address.port().is_none()
        && matches!(
            address.host_str(),
            Some(
                "github.com"
                    | "release-assets.githubusercontent.com"
                    | "objects.githubusercontent.com"
            )
        )
}

/// Streams a declared package into staging while checking its exact digest.
/// # Arguments
/// * `input`, `output` - Streaming source and new destination, never a capture.
/// * `size`, `sha256` - Validated release byte count and expected SHA-256.
/// * `progress` - Callback receiving the bounded number of downloaded bytes.
/// # Errors
/// Reports I/O errors, timeout, excess/truncated bytes or a digest mismatch.
pub(super) fn copy_verified(
    mut input: impl Read,
    mut output: impl Write,
    size: u64,
    sha256: &str,
    progress: impl Fn(u64),
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(900);
    let mut digest = Sha256::new();
    let mut received = 0_u64;
    let mut buffer = [0_u8; 65_536];
    loop {
        if Instant::now() >= deadline {
            return Err("Update download timed out.".into());
        }
        let count = input.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 {
            break;
        }
        received += count as u64;
        if received > size {
            return Err("Update exceeds its declared size.".into());
        }
        digest.update(&buffer[..count]);
        output
            .write_all(&buffer[..count])
            .map_err(|error| error.to_string())?;
        progress(received);
    }
    if received != size || format!("{:x}", digest.finalize()) != sha256.to_ascii_lowercase() {
        return Err("Update size or SHA-256 verification failed.".into());
    }
    Ok(())
}

/// Downloads a bounded package from approved GitHub HTTPS endpoints.
/// # Arguments
/// * `address` - Exact release URL validated by the platform installer.
/// * `output` - New staging file; partial data is never installed.
/// * `size`, `sha256` - Trusted release size and digest.
/// * `progress` - Bounded worker status callback.
/// # Errors
/// Rejects foreign redirects, timeouts, size mismatch or digest mismatch.
pub(super) fn fetch(
    address: &str,
    output: &mut impl Write,
    size: u64,
    sha256: &str,
    progress: impl Fn(String),
) -> Result<(), String> {
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout_connect(Duration::from_secs(15))
        .timeout_read(Duration::from_secs(20))
        .build();
    let mut address = url::Url::parse(address).map_err(|error| error.to_string())?;
    let mut response = None;
    for _ in 0..=5 {
        if !trusted(&address) {
            return Err("Untrusted update download redirect.".into());
        }
        let received = agent
            .get(address.as_str())
            .set("User-Agent", "MERLIN")
            .call()
            .map_err(|_| "Unable to download the update.")?;
        match received.status() {
            200 => {
                response = Some(received);
                break;
            }
            301 | 302 | 303 | 307 | 308 => {
                address = address
                    .join(
                        received
                            .header("Location")
                            .ok_or("Missing update redirect.")?,
                    )
                    .map_err(|error| error.to_string())?;
            }
            _ => return Err("Unexpected update download response.".into()),
        }
    }
    let response = response.ok_or("Too many update redirects.")?;
    copy_verified(response.into_reader(), output, size, sha256, |received| {
        progress(format!("Downloading update… {}%", received * 100 / size));
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn platform_independent_download_verification_bounds_and_hash() {
        let digest = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        let mut output = Vec::new();
        copy_verified(&b"abc"[..], &mut output, 3, digest, |_| {}).unwrap();
        assert_eq!(output, b"abc");
        for bytes in [&b"ab"[..], &b"abcd"[..], &b"abd"[..]] {
            assert!(copy_verified(bytes, Vec::new(), 3, digest, |_| {}).is_err());
        }
        assert!(!trusted(
            &url::Url::parse("https://github.com:444/download").unwrap()
        ));
        assert!(!trusted(
            &url::Url::parse("https://user@github.com/download").unwrap()
        ));
        assert!(!trusted(
            &url::Url::parse("https://example.org/download").unwrap()
        ));
        assert!(trusted(
            &url::Url::parse("https://release-assets.githubusercontent.com/package?sig=abc")
                .unwrap()
        ));
    }
}
