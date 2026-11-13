#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Build/check/package the Windows UI from an existing native MSVC environment.
param([ValidateSet('setup','build','check','package')][string]$Action = 'build')
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $Root
if (!$IsWindows) {
    throw 'Use a native Windows x64 PowerShell 7 session.'
}
if (!$env:QT_SDK_DIR) {
    $env:QT_SDK_DIR = "$Root/local/qt/6.8.3/msvc2022_64"
}
# Setup is explicit; ordinary build/check never downloads SDKs.
if ($Action -eq 'setup') {
    # The setup helper only writes its checkout-local default SDK.
    $DefaultSdk = "$Root/local/qt/6.8.3/msvc2022_64"
    if ($env:QT_SDK_DIR -eq $DefaultSdk) {
        Remove-Item Env:QT_SDK_DIR
    }
    & python tools/development/qt_sdk.py install
    if ($LASTEXITCODE -ne 0) {
        throw 'Qt SDK setup failed'
    }
    exit 0
}
if (!(Get-Command cl.exe -ErrorAction SilentlyContinue) -or !(Get-Command rc.exe -ErrorAction SilentlyContinue)) {
    throw 'Open the Visual Studio 2022 x64 Native Tools environment first (MSVC and Windows SDK required).'
}
$env:PATH = "$env:QT_SDK_DIR/bin;$env:PATH"
$env:QMAKE = "$env:QT_SDK_DIR/bin/qmake.exe"
$env:QT_VERSION_MAJOR = '6'
$env:QT_QUICK_CONTROLS_STYLE = 'Fusion'
$env:CARGO_HOME = "$Root/local/merlin-cargo"
$env:CARGO_INCREMENTAL = '0'
$env:CARGO_PROFILE_DEV_DEBUG = '0'
$env:CARGO_PROFILE_TEST_DEBUG = '0'
& python tools/development/qt_sdk.py check
if ($LASTEXITCODE -ne 0) {
    throw 'Qt SDK check failed'
}
& python tools/release/versions.py
if ($LASTEXITCODE -ne 0) {
    throw 'Version metadata differs'
}
$Manifest = "$Root/apps/qt/Cargo.toml"
if ($Action -eq 'check') {
    & cargo +1.85.1 fmt --manifest-path $Manifest -- --check
    if ($LASTEXITCODE -ne 0) {
        throw 'Rust formatting failed'
    }
    & cargo +1.85.1 clippy --manifest-path $Manifest --all-targets --locked -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        throw 'Qt adapter Clippy failed'
    }
    & cargo +1.85.1 test --manifest-path $Manifest --locked
    if ($LASTEXITCODE -ne 0) {
        throw 'Qt adapter tests failed'
    }
    $env:QT_QPA_PLATFORM = 'offscreen'
    $env:QT_QUICK_BACKEND = 'software'
    $QmlLogs = "$Root/dist/windows-qml-logs"
    $Screenshots = "$Root/docs/development/local/visual-workspace"
    New-Item -ItemType Directory -Force -Path $Screenshots | Out-Null
    New-Item -ItemType Directory -Force -Path $QmlLogs | Out-Null
    # A GUI executable may not attach its test output to the Actions console.
    # Wait explicitly and retain both test loggers and startup stderr on failure.
    $QmlArguments = @('-input', "`"$Root/apps/qt/tests`"",
        '-o', "`"$QmlLogs/results.txt,txt`"",
        '-o', "`"$QmlLogs/results.xml,junitxml`"", '-v1')
    try {
        $QmlProcess = Start-Process -FilePath "$env:QT_SDK_DIR/bin/qmltestrunner.exe" `
            -ArgumentList $QmlArguments -WorkingDirectory $Screenshots -Wait -PassThru -NoNewWindow `
            -RedirectStandardOutput "$QmlLogs/stdout.txt" `
            -RedirectStandardError "$QmlLogs/stderr.txt"
        $QmlExit = $QmlProcess.ExitCode
        $QmlExitHex = '{0:X8}' -f [BitConverter]::ToUInt32([BitConverter]::GetBytes([int]$QmlExit), 0)
        "QML runner exit: $QmlExit (0x$QmlExitHex)" |
            Tee-Object -FilePath "$QmlLogs/exit.txt"
        if ($QmlExit -ne 0) {
            throw "QML tests failed: exit $QmlExit (0x$QmlExitHex); see dist/windows-qml-logs"
        }
    } finally {
        foreach ($LogName in @('stdout.txt', 'stderr.txt', 'results.txt')) {
            if (Test-Path "$QmlLogs/$LogName") {
                Write-Host "QML $LogName"
                Get-Content "$QmlLogs/$LogName"
            }
        }
        Remove-Item Env:QT_QPA_PLATFORM
    }
}
$BuildArgs = @('+1.85.1', 'build', '--manifest-path', $Manifest, '--locked')
if ($Action -eq 'package') {
    $BuildArgs += '--release'
}
& cargo @BuildArgs
if ($LASTEXITCODE -ne 0) {
    throw 'Windows desktop build failed'
}
if ($Action -eq 'package') {
    $Version = (Get-Content VERSION -Raw).Trim()
    if ($env:GITHUB_REF_TYPE -eq 'tag' -and $env:GITHUB_REF_NAME -ne "v$Version") {
        throw 'Tag/version mismatch'
    }
    $VsWhere = "${env:ProgramFiles(x86)}/Microsoft Visual Studio/Installer/vswhere.exe"
    $Vs = & $VsWhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if ($LASTEXITCODE -ne 0 -or !$Vs) {
        throw 'Visual Studio redistributable directory is unavailable'
    }
    $Crt = Get-ChildItem "$Vs/VC/Redist/MSVC/*/x64/Microsoft.VC*.CRT" -Directory |
        Sort-Object FullName -Descending | Select-Object -First 1
    if (!$Crt) {
        throw 'Missing x64 redistributable CRT files'
    }
    & python tools/packaging/windows_desktop.py --binary "$Root/apps/qt/target/release/merlin-qt.exe" `
        --qt-sdk $env:QT_SDK_DIR --crt-dir $Crt.FullName --output "$Root/dist/windows-desktop"
    if ($LASTEXITCODE -ne 0) {
        throw 'Windows desktop MSI generation failed'
    }
}
