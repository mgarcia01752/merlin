/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Relocatable curated example discovery, independent of the launch directory.
use std::path::{Path, PathBuf};

/// Finds the curated example inventory beside an installed or development executable.
///
/// # Arguments
/// * `executable` - Absolute executable path from the running application.
///
/// # Returns
/// An existing examples root marked by `manifest.json`, preferring installed
/// resources. Returns None when resources are absent; native Open remains usable.
pub(crate) fn locate(executable: &Path) -> Option<PathBuf> {
    let directory = executable.parent()?;
    let installed = [
        directory.join("files/examples/workbench"),
        directory.join("../share/merlin/examples/workbench"),
        directory.join("../Resources/examples/workbench"),
    ];
    for candidate in installed {
        if candidate.join("manifest.json").is_file() {
            return candidate.canonicalize().ok();
        }
    }
    for ancestor in executable.ancestors().skip(1).take(7) {
        let candidate = ancestor.join("examples/workbench");
        if ancestor.join("Cargo.toml").is_file() && candidate.join("manifest.json").is_file() {
            return candidate.canonicalize().ok();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::locate;
    use std::{fs, path::PathBuf};

    #[test]
    fn installed_and_development_layouts_resolve_without_launch_directory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/development/local/workbench-examples/path-test")
            .join(std::process::id().to_string());
        fs::create_dir_all(&root).unwrap();
        let root = root.canonicalize().unwrap();
        for (layout, executable, resources) in [
            ("portable", "merlin.exe", "files/examples/workbench"),
            ("linux", "bin/merlin-qt", "share/merlin/examples/workbench"),
            (
                "macos",
                "Contents/MacOS/merlin-qt",
                "Contents/Resources/examples/workbench",
            ),
            (
                "source",
                "apps/qt/target/debug/merlin-qt",
                "examples/workbench",
            ),
        ] {
            let prefix = root.join(layout);
            let executable = prefix.join(executable);
            fs::create_dir_all(executable.parent().unwrap()).unwrap();
            assert!(locate(&executable).is_none());
            let resources = prefix.join(resources);
            fs::create_dir_all(&resources).unwrap();
            fs::write(resources.join("manifest.json"), "{}").unwrap();
            if layout == "source" {
                fs::write(prefix.join("Cargo.toml"), "").unwrap();
            }
            assert_eq!(locate(&executable), Some(resources.canonicalize().unwrap()));
        }
        fs::remove_dir_all(root).unwrap();
    }
}
