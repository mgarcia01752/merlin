/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#pragma once
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>
#include <shobjidl.h>
#include <wrl/client.h>
#include <QString>
#include <QUrl>

/**
 * Show Windows Common Item Dialogs in the isolated picker process.
 * @param title Native title; Rust owns file interpretation.
 * @param multiple Permit at most 256 local files.
 * @param save Select a destination with native overwrite confirmation.
 * @param workbench Restrict Open to MERLIN workbench JSON.
 * @return Newline-separated local URIs, empty cancellation, or ERROR marker.
 */
inline QString chooseWindowsSelection(const QString &title, bool multiple, bool save, bool workbench) {
    using Microsoft::WRL::ComPtr;
    const HRESULT initialized = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
    if (FAILED(initialized)) {
        return QStringLiteral("ERROR: Cannot initialize Windows native file selection.");
    }
    struct Apartment {
        ~Apartment() { CoUninitialize(); }
    } apartment;
    ComPtr<IFileDialog> chooser;
    HRESULT status = CoCreateInstance(save ? CLSID_FileSaveDialog : CLSID_FileOpenDialog,
        nullptr, CLSCTX_INPROC_SERVER, IID_PPV_ARGS(chooser.GetAddressOf()));
    if (FAILED(status)) {
        return QStringLiteral("ERROR: Windows native file dialog is unavailable.");
    }
    FILEOPENDIALOGOPTIONS options;
    status = chooser->GetOptions(&options);
    if (SUCCEEDED(status)) {
        options |= FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_NOCHANGEDIR;
        options |= save ? FOS_OVERWRITEPROMPT : FOS_FILEMUSTEXIST;
        if (multiple && !save) {
            options |= FOS_ALLOWMULTISELECT;
        }
        status = chooser->SetOptions(options);
    }
    const auto caption = title.toStdWString();
    if (SUCCEEDED(status)) {
        status = chooser->SetTitle(caption.c_str());
    }
    const COMDLG_FILTERSPEC filter[] = {{L"MERLIN Workbenches (*.merlin.json)", L"*.merlin.json"}};
    if (SUCCEEDED(status) && (workbench || save)) {
        status = chooser->SetFileTypes(1, filter);
    }
    if (SUCCEEDED(status) && save) {
        status = chooser->SetFileName(L"workbench.merlin.json");
        if (SUCCEEDED(status)) {
            status = chooser->SetDefaultExtension(L"merlin.json");
        }
    }
    const auto initial = QString::fromUtf8(qgetenv("MERLIN_NATIVE_OPEN_DIRECTORY")).toStdWString();
    if (!save && !initial.empty()) {
        ComPtr<IShellItem> folder;
        if (SUCCEEDED(SHCreateItemFromParsingName(initial.c_str(), nullptr, IID_PPV_ARGS(folder.GetAddressOf())))) {
            chooser->SetFolder(folder.Get());
        }
    }
    if (FAILED(status)) {
        return QStringLiteral("ERROR: Cannot configure Windows native file dialog.");
    }
    status = chooser->Show(nullptr);
    if (status == HRESULT_FROM_WIN32(ERROR_CANCELLED)) {
        return {};
    }
    if (FAILED(status)) {
        return QStringLiteral("ERROR: Windows native file selection failed.");
    }
    QString result;
    const auto append = [&result](IShellItem *item) -> bool {
        PWSTR path = nullptr;
        if (FAILED(item->GetDisplayName(SIGDN_FILESYSPATH, &path))) {
            return false;
        }
        const auto uri = QUrl::fromLocalFile(QString::fromWCharArray(path)).toString(QUrl::FullyEncoded);
        CoTaskMemFree(path);
        if (result.size() + uri.size() + 1 > 262144) {
            return false;
        }
        if (!result.isEmpty()) {
            result += QLatin1Char('\n');
        }
        result += uri;
        return true;
    };
    if (multiple && !save) {
        ComPtr<IFileOpenDialog> open;
        ComPtr<IShellItemArray> items;
        DWORD count = 0;
        if (FAILED(chooser.As(&open)) || FAILED(open->GetResults(&items)) ||
            FAILED(items->GetCount(&count)) || count > 256) {
            return QStringLiteral("ERROR: Select at most 256 local files.");
        }
        for (DWORD index = 0; index < count; ++index) {
            ComPtr<IShellItem> item;
            if (FAILED(items->GetItemAt(index, &item)) || !append(item.Get())) {
                return QStringLiteral("ERROR: Native selection is invalid or too large.");
            }
        }
    } else {
        ComPtr<IShellItem> item;
        if (FAILED(chooser->GetResult(&item)) || !append(item.Get())) {
            return QStringLiteral("ERROR: Native selection is invalid or too large.");
        }
    }
    return result;
}
