/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Read-only trusted stable-release metadata for the update worker.
use serde::Deserialize;
use serde_json::{json, Value};
use std::{io::Read, time::Duration};

pub(crate) const PROJECT: &str = "https://github.com/mgarcia01752/merlin";
const ENDPOINT: &str = "https://api.github.com/repos/mgarcia01752/merlin/releases/latest";
const MAX_RESPONSE: u64 = 1_048_576;

#[derive(Deserialize)]
struct Release {
    draft: bool,
    prerelease: bool,
    tag_name: String,
    html_url: String,
    assets: Vec<Asset>,
}
#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
    size: u64,
    digest: Option<String>,
}

/// Validates exactly three numeric release components.
/// # Arguments
/// * `value` - Numeric version with an optional leading v.
/// # Returns
/// Comparable major/minor/patch components.
/// # Errors
/// Rejects missing, overflowing, prefixed-zero or nonnumeric components.
pub(super) fn version(value: &str) -> Result<[u64; 3], String> {
    let fields = value
        .strip_prefix('v')
        .unwrap_or(value)
        .split('.')
        .collect::<Vec<_>>();
    if fields.len() != 3 {
        return Err("Invalid release version.".into());
    }
    let mut parts = [0; 3];
    for (index, field) in fields.iter().enumerate() {
        if field.is_empty()
            || !field.bytes().all(|value| value.is_ascii_digit())
            || (field.len() > 1 && field.starts_with('0'))
        {
            return Err("Invalid release version.".into());
        }
        parts[index] = field
            .parse()
            .map_err(|_| "Release version exceeds limits.")?;
    }
    Ok(parts)
}

/// Lists exact supported desktop filenames in preference order.
/// # Arguments
/// * `number` - Already validated numeric release version.
/// # Returns
/// Canonical desktop name, followed by the compatibility name for older clients.
pub(super) fn desktop_asset_names(number: &str) -> [String; 2] {
    [
        format!("merlin-desktop_{number}_ubuntu22.04_amd64.deb"),
        format!("merlin_{number}_ubuntu22.04_amd64.deb"),
    ]
}

/// Validates one bounded stable release and selects its exact platform desktop installer.
/// # Arguments
/// * `installed` - Compiled numeric application version.
/// * `bytes` - GitHub metadata, at most one MiB.
/// # Returns
/// Bounded plain-text status and a trusted release-page URL when newer.
/// # Errors
/// Rejects invalid versions, unpublished releases and foreign release URLs.
pub(crate) fn parse(installed: &str, bytes: &[u8]) -> Result<Value, String> {
    parse_platform(installed, bytes, cfg!(target_os = "windows"))
}

fn parse_platform(installed: &str, bytes: &[u8], windows: bool) -> Result<Value, String> {
    if bytes.len() as u64 > MAX_RESPONSE {
        return Err("Release response exceeds limits.".into());
    }
    let release: Release =
        serde_json::from_slice(bytes).map_err(|_| "Invalid release metadata.")?;
    let latest = version(&release.tag_name)?;
    if release.draft
        || release.prerelease
        || release.tag_name != format!("v{}.{}.{}", latest[0], latest[1], latest[2])
    {
        return Err("A stable published release is required.".into());
    }
    let page = format!("{PROJECT}/releases/tag/{}", release.tag_name);
    if release.html_url != page {
        return Err("Release belongs to another project.".into());
    }
    if latest <= version(installed)? {
        return Ok(json!({"busy":false,"available":false,"status":"You are up to date.","url":""}));
    }
    let number = release.tag_name.trim_start_matches('v');
    let names = if windows {
        vec![format!("merlin-desktop_{number}_windows_amd64.msi")]
    } else {
        desktop_asset_names(number).to_vec()
    };
    // Prefer canonical metadata. Invalid/duplicate canonical entries never fall back.
    let name = if release.assets.iter().any(|asset| asset.name == names[0]) {
        &names[0]
    } else {
        names.last().unwrap()
    };
    let url = format!("{PROJECT}/releases/download/{}/{name}", release.tag_name);
    let candidates = release
        .assets
        .iter()
        .filter(|asset| asset.name == *name)
        .collect::<Vec<_>>();
    let valid = candidates.len() == 1
        && candidates.iter().all(|asset| {
            asset.browser_download_url == url
                && (1..=300_000_000).contains(&asset.size)
                && asset.digest.as_ref().is_some_and(|digest| {
                    digest.strip_prefix("sha256:").is_some_and(|hex| {
                        hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
                    })
                })
        });
    if !valid {
        return Ok(
            json!({"busy":false,"available":false,"status":"No verified desktop update is available for this platform.","url":""}),
        );
    }
    Ok(
        json!({"busy":false,"available":true,"status":format!("MERLIN {number} is available. Installed: {installed}."),"url":page,"version":number,"package_url":url,"size":candidates[0].size,"sha256":candidates[0].digest.as_ref().unwrap().trim_start_matches("sha256:").to_ascii_lowercase()}),
    )
}

/// Queries public stable metadata with a deadline and a strict response bound.
/// # Arguments
/// * `installed` - Compiled application version; never capture data.
/// # Returns
/// Validated display metadata; HTTP 404 means no release yet.
/// # Errors
/// Reports transport, HTTP and metadata failures without blocking the UI thread.
pub(crate) fn check(installed: &str) -> Result<Value, String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(15))
        .redirects(0)
        .build();
    let response = match agent
        .get(ENDPOINT)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "MERLIN")
        .call()
    {
        Ok(response) => response,
        Err(ureq::Error::Status(404, _)) => {
            return Ok(
                json!({"busy":false,"available":false,"status":"No published desktop release is available yet.","url":""}),
            )
        }
        Err(_) => return Err("Unable to check for updates. Try again later.".into()),
    };
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(MAX_RESPONSE + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Unable to read release metadata.")?;
    parse(installed, &bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(installed: &str, bytes: &[u8]) -> Result<Value, String> {
        super::parse_platform(installed, bytes, false)
    }
    #[test]
    fn windows_requires_exact_verified_desktop_msi() {
        let mut data = metadata();
        assert_eq!(
            super::parse_platform("0.1.0", &serde_json::to_vec(&data).unwrap(), true).unwrap()
                ["available"],
            false
        );
        let name = "merlin-desktop_0.2.0_windows_amd64.msi";
        data["assets"][0]["name"] = json!(name);
        data["assets"][0]["browser_download_url"] =
            json!(format!("{PROJECT}/releases/download/v0.2.0/{name}"));
        assert_eq!(
            super::parse_platform("0.1.0", &serde_json::to_vec(&data).unwrap(), true).unwrap()
                ["available"],
            true
        );
        data["assets"][0]["digest"] = json!("unverified");
        assert_eq!(
            super::parse_platform("0.1.0", &serde_json::to_vec(&data).unwrap(), true).unwrap()
                ["available"],
            false
        );
    }
    fn metadata() -> Value {
        json!({"draft":false,"prerelease":false,"tag_name":"v0.2.0","html_url":format!("{PROJECT}/releases/tag/v0.2.0"),"assets":[{"name":"merlin-desktop_0.2.0_ubuntu22.04_amd64.deb","browser_download_url":format!("{PROJECT}/releases/download/v0.2.0/merlin-desktop_0.2.0_ubuntu22.04_amd64.deb"),"size":1000,"digest":format!("sha256:{}","a".repeat(64))}]})
    }
    #[test]
    fn stable_baseline_asset_and_numeric_versions() {
        let data = serde_json::to_vec(&metadata()).unwrap();
        assert_eq!(parse("0.1.9", &data).unwrap()["available"], true);
        assert_eq!(parse("0.2.0", &data).unwrap()["available"], false);
        assert_eq!(parse("0.10.0", &data).unwrap()["available"], false);
        assert!(parse("0.1.0", &vec![b' '; MAX_RESPONSE as usize + 1]).is_err());
        for field in [
            "v1.2",
            "v01.2.3",
            "v1.2.3-rc1",
            "v1.2.3/evil",
            "v18446744073709551616.0.0",
        ] {
            let mut bad = metadata();
            bad["tag_name"] = json!(field);
            assert!(parse("0.1.0", &serde_json::to_vec(&bad).unwrap()).is_err());
        }
    }
    #[test]
    fn canonical_is_preferred_and_legacy_support_cannot_mask_invalid_metadata() {
        let mut data = metadata();
        let mut legacy = data["assets"][0].clone();
        legacy["name"] = json!("merlin_0.2.0_ubuntu22.04_amd64.deb");
        legacy["browser_download_url"] = json!(format!(
            "{PROJECT}/releases/download/v0.2.0/merlin_0.2.0_ubuntu22.04_amd64.deb"
        ));
        data["assets"].as_array_mut().unwrap().push(legacy.clone());
        let parsed = parse("0.1.4", &serde_json::to_vec(&data).unwrap()).unwrap();
        assert_eq!(parsed["available"], true);
        assert!(parsed["package_url"]
            .as_str()
            .unwrap()
            .contains("/merlin-desktop_"));
        data["assets"][0]["digest"] = json!("unverified");
        assert_eq!(
            parse("0.1.4", &serde_json::to_vec(&data).unwrap()).unwrap()["available"],
            false
        );
        data["assets"] = json!([legacy.clone()]);
        let parsed = parse("0.1.3", &serde_json::to_vec(&data).unwrap()).unwrap();
        assert_eq!(parsed["available"], true);
        assert!(parsed["package_url"].as_str().unwrap().contains("/merlin_"));
        data["assets"].as_array_mut().unwrap().push(legacy);
        assert_eq!(
            parse("0.1.3", &serde_json::to_vec(&data).unwrap()).unwrap()["available"],
            false
        );
    }

    #[test]
    fn foreign_missing_duplicate_and_unverified_assets_never_notify() {
        for pointer in [
            "/assets/0/browser_download_url",
            "/assets/0/name",
            "/assets/0/digest",
        ] {
            let mut bad = metadata();
            *bad.pointer_mut(pointer).unwrap() = json!("https://evil.invalid");
            assert_eq!(
                parse("0.1.0", &serde_json::to_vec(&bad).unwrap()).unwrap()["available"],
                false
            );
        }
        let mut bad = metadata();
        bad["assets"][0]["size"] = json!(300_000_001);
        assert_eq!(
            parse("0.1.0", &serde_json::to_vec(&bad).unwrap()).unwrap()["available"],
            false
        );
        let mut duplicate = metadata();
        let extra = duplicate["assets"][0].clone();
        duplicate["assets"].as_array_mut().unwrap().push(extra);
        assert_eq!(
            parse("0.1.0", &serde_json::to_vec(&duplicate).unwrap()).unwrap()["available"],
            false
        );
        for key in ["draft", "prerelease"] {
            let mut bad = metadata();
            bad[key] = json!(true);
            assert!(parse("0.1.0", &serde_json::to_vec(&bad).unwrap()).is_err());
        }
        let mut bad = metadata();
        bad["html_url"] = json!("https://evil.invalid");
        assert!(parse("0.1.0", &serde_json::to_vec(&bad).unwrap()).is_err());
    }
}
