/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
/// Builds only the bounded BCH adapter, without Qt or the AFF3CT framework.
///
/// # Panics
/// Fails the build when the native compiler or adapter source is unavailable.
fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("native/bch.cpp")
        .include(".")
        .std("c++17")
        .compile("pnm-bch");
    println!("cargo:rerun-if-changed=native/bch.cpp");
    println!("cargo:rerun-if-changed=native/bch.hpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
