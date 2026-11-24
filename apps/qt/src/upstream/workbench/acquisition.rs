/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native source acquisition; parsing and scientific admission stay off the GUI.
use super::{Model, MAX_CONFIGURATION_BYTES};
use crate::{platform::input_file, shell::Direction};

impl Model {
    /// Imports one actual local upstream source without changing accepted state.
    ///
    /// # Arguments
    /// * `uri` - Native chooser result, or None when the user cancels.
    ///
    /// # Returns
    /// True after background preparation is queued; false on cancellation.
    /// Exact original bytes are protected against later project writes. Neither
    /// loading nor cancellation starts or stops a simulation.
    ///
    /// # Errors
    /// Busy, invalid local input, file/byte/path admission or preparation failure.
    /// Accepted configuration and reports remain unchanged until final adoption.
    pub(crate) fn load_configuration_uri(&mut self, uri: Option<&str>) -> Result<bool, String> {
        let Some(uri) = uri else {
            return Ok(false);
        };
        self.ensure_configuration_idle()?;
        let (_, bytes) = input_file::read_local(
            &self.application,
            Direction::Upstream,
            uri,
            MAX_CONFIGURATION_BYTES,
        )?;
        self.apply(&bytes)?;
        Ok(true)
    }

    /// Checks existing lifecycle admission before opening an acquisition editor.
    ///
    /// # Returns
    /// Unit while neither preparation nor application simulation is active.
    ///
    /// # Errors
    /// A busy owner is preserved without invoking a picker or touching files.
    pub(crate) fn ensure_configuration_idle(&self) -> Result<(), String> {
        if self.pending.is_some() || self.is_running() || self.application.active().is_some() {
            return Err(
                "Finish active simulation or preparation before editing upstream configuration."
                    .into(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
