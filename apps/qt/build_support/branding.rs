/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Embed unchanged Qt branding without a package-wide empty resource watch.
use std::path::Path;

/// Existing resource collection and aliases; shared by generation and dependency discovery.
const BRANDING_RESOURCE_FILE: &str = "branding.qrc";
/// Qt 6 resources use the same C++17 language contract as the pinned CXX-Qt builder.
const QT_RESOURCE_CPP_STANDARD: &str = "c++17";

/// Compiles and directly links the branding resource initializer through pinned build APIs.
///
/// # Returns
/// Unit after emitting explicit QRC/asset watches and a retained object-file link.
/// Only empty RCC-list paths are excluded; real resource paths stay unchanged.
/// Generated files remain in Cargo's output directory, with no SDK/cache mutation.
/// Direct object linkage preserves static resource registration before main.
///
/// # Panics
/// Fails the build when configured Qt/Core/RCC or the C++ compiler/resources are
/// unavailable, retaining normal native prerequisite failures.
pub(super) fn embed() {
    let resource = Path::new(BRANDING_RESOURCE_FILE);
    println!("cargo::rerun-if-changed={}", resource.display());
    let mut qt = qt_build_utils::QtBuild::new(vec!["Core".into()])
        .expect("Configured native Qt Core prerequisites are required for branding");
    for input in qt.qrc_list(&resource) {
        // The pinned parser includes RCC's terminal newline as an empty path.
        // Cargo interprets that path as the package directory, including target.
        if !input.as_os_str().is_empty() {
            println!("cargo::rerun-if-changed={}", input.display());
        }
    }
    let generated = qt.qrc(&resource);
    let mut compiler = cc::Build::new();
    qt.cargo_link_libraries(&mut compiler);
    compiler
        .cpp(true)
        .std(QT_RESOURCE_CPP_STANDARD)
        .flag_if_supported("/Zc:__cplusplus")
        .flag_if_supported("/permissive-")
        .includes(qt.include_paths())
        .file(generated);
    for object in compiler.compile_intermediates() {
        // An ordinary static archive could discard this unreferenced initializer.
        println!("cargo::rustc-link-arg={}", object.display());
    }
}
