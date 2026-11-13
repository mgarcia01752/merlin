/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#pragma once
#include <QString>
#ifdef _WIN32
#include "windows_file_picker.h"
#endif
#ifdef __linux__
#pragma push_macro("signals")
#undef signals
#include <gtk/gtk.h>
#pragma pop_macro("signals")
#endif

/**
 * Open the desktop toolkit's modal chooser in the isolated picker process.
 * Return a local file URI, empty on Cancel, or an error marker on unavailable
 * native integration. No Qt file dialog or custom browser fallback is used.
 * GTK owns the selection UI only; Rust reads and validates selected bytes.
 * @param title Native window title, independent of filter selection.
 * @param multiple Whether more than one file may be selected.
 * @param save Whether to choose a destination with overwrite confirmation.
 * @param workbench Whether to show only *.merlin.json files and directories.
 * @return Selected URI(s), empty cancellation, or an explicit error marker.
 */
inline QString chooseNativeSelection(const QString &title, bool multiple, bool save = false, bool workbench = false) {
#ifdef __linux__
    // Match XCB/XWayland so the native chooser shares the application's display.
    if (qgetenv("QT_QPA_PLATFORM") == "xcb") {
        gdk_set_allowed_backends("x11");
    }
    if (!gtk_init_check(nullptr, nullptr)) {
        return QStringLiteral("ERROR: OS-native file selection is unavailable (GTK display initialization failed).");
    }
    struct Selection {
        GMainLoop *loop;
        int response;
    } selection{g_main_loop_new(nullptr, false), GTK_RESPONSE_CANCEL};
    const auto titleBytes = title.toUtf8();
    auto *chooser = gtk_file_chooser_native_new(titleBytes.constData(), nullptr,
        save ? GTK_FILE_CHOOSER_ACTION_SAVE : GTK_FILE_CHOOSER_ACTION_OPEN, save ? "Save" : "Open", "Cancel");
    gtk_file_chooser_set_select_multiple(GTK_FILE_CHOOSER(chooser), multiple);
    if (workbench) {
        auto *filter = gtk_file_filter_new();
        gtk_file_filter_set_name(filter, "MERLIN Workbenches (*.merlin.json)");
        gtk_file_filter_add_pattern(filter, "*.merlin.json");
        gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(chooser), filter);
        gtk_file_chooser_set_filter(GTK_FILE_CHOOSER(chooser), filter);
    }
    const auto initialDirectory = qgetenv("MERLIN_NATIVE_OPEN_DIRECTORY");
    if (!save && !initialDirectory.isEmpty()) {
        gtk_file_chooser_set_current_folder(GTK_FILE_CHOOSER(chooser), initialDirectory.constData());
    }
    if (save) {
        gtk_file_chooser_set_current_name(GTK_FILE_CHOOSER(chooser), "workbench.merlin.json");
        gtk_file_chooser_set_do_overwrite_confirmation(GTK_FILE_CHOOSER(chooser), true);
    }
    gtk_native_dialog_set_modal(GTK_NATIVE_DIALOG(chooser), true);
    g_signal_connect(chooser, "response", G_CALLBACK(+[](GtkNativeDialog *, int response, gpointer data) {
        auto *selection = static_cast<Selection *>(data);
        selection->response = response;
        g_main_loop_quit(selection->loop);
    }), &selection);
    gtk_native_dialog_show(GTK_NATIVE_DIALOG(chooser));
    g_main_loop_run(selection.loop);
    QString result;
    if (selection.response == GTK_RESPONSE_ACCEPT) {
        GSList *uris = gtk_file_chooser_get_uris(GTK_FILE_CHOOSER(chooser));
        if (g_slist_length(uris) > 256) {
            result = QStringLiteral("ERROR: Select at most 256 files.");
        } else {
            for (GSList *item = uris; item != nullptr; item = item->next) {
                if (!result.isEmpty()) {
                    result += QLatin1Char('\n');
                }
                result += QString::fromUtf8(static_cast<const char *>(item->data));
            }
        }
        g_slist_free_full(uris, g_free);
    }
    gtk_native_dialog_destroy(GTK_NATIVE_DIALOG(chooser));
    g_object_unref(chooser);
    g_main_loop_unref(selection.loop);
    return result;
#elif defined(_WIN32)
    return chooseWindowsSelection(title, multiple, save, workbench);
#else
    return QStringLiteral("ERROR: OS-native file selection is implemented for Ubuntu only.");
#endif
}

/** Open a single-file native chooser; preserve the existing adapter contract. */
inline QString chooseNativeFile(const QString &title) {
    return chooseNativeSelection(title, false);
}
/**
 * Open the main workbench chooser with an active *.merlin.json filter.
 * Directories remain navigable; this does not interpret or validate file bytes.
 * @param title User-facing native Open title.
 * @return Local URI, empty cancellation, or an integration error marker.
 */
inline QString chooseNativeWorkbench(const QString &title) {
    return chooseNativeSelection(title, false, false, true);
}
/** Open the same OS-native chooser with bounded multi-selection enabled. */
inline QString chooseNativeFiles(const QString &title) {
    return chooseNativeSelection(title, true);
}

/** Select a project destination with native overwrite confirmation. */
inline QString chooseNativeSave(const QString &title) {
    return chooseNativeSelection(title, false, true);
}
