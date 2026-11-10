/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
mod branding;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

/// Starts the native workbench; scientific jobs are not connected in this increment.
///
/// # Panics
/// Terminates if Qt cannot construct its application or engine.
fn main() {
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
