/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Qt-only icon bridge; no scientific computation crosses this boundary.
#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("platform/branding.h");
        /// Selects an embedded icon after Qt application initialization.
        #[rust_name = "set_workbench_icon"]
        fn setWorkbenchIcon(dark: bool);
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Branding = super::BrandingRust;

        /// Updates desktop identity when the workbench theme changes.
        ///
        /// # Arguments
        /// * `dark` - True selects the dark icon; false selects the light icon.
        #[qinvokable]
        fn update_icon(self: Pin<&mut Branding>, dark: bool);
    }
}
/// Stateless Qt icon adapter; it owns no simulation data or lifecycle.
#[derive(Default)]
pub struct BrandingRust;
impl bridge::Branding {
    /// Selects the embedded theme icon on the Qt GUI thread.
    ///
    /// # Arguments
    /// * `dark` - Whether to select the dark-theme asset.
    pub fn update_icon(self: core::pin::Pin<&mut Self>, dark: bool) {
        bridge::set_workbench_icon(dark);
    }
}
