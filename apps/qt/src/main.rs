/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
mod about_info;
mod branding;
mod input_file;
mod measurement_input;
mod native_file_picker;
mod ofdm_settings;
mod profile_input;
mod receiver_instruments;
mod source_input;
mod transmitter_analyzer;
mod workbench_session;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

/// Starts the native workbench; scientific jobs are not connected in this increment.
///
/// # Panics
/// Terminates if Qt cannot construct its application or engine.
fn main() {
    // GTK's native chooser runs before Qt starts, in a separate presentation-only
    // process, so the two toolkits cannot consume each other's pending UI events.
    if matches!(
        std::env::args().nth(1).as_deref(),
        Some("--native-file-picker" | "--native-file-picker-multiple")
    ) {
        native_file_picker::run_helper(
            &std::env::args().nth(2).unwrap_or_default(),
            std::env::args().nth(1).as_deref() == Some("--native-file-picker-multiple"),
        );
        return;
    }
    std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Fusion");
    let mut application = QGuiApplication::new();
    application
        .as_mut()
        .expect("Qt application unavailable")
        .set_application_name(&QString::from("merlin"));
    QGuiApplication::set_desktop_file_name(&QString::from("merlin"));
    let mut engine = QQmlApplicationEngine::new();
    engine
        .as_mut()
        .expect("Qt QML engine unavailable")
        .load(&QUrl::from("qrc:/qt/qml/Merlin/Workbench/qml/Main.qml"));
    application
        .as_mut()
        .expect("Qt application unavailable")
        .exec();
}
