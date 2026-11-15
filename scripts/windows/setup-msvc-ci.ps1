#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Load the installed VS 2022 x64 tools and persist changes for later CI steps.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if (!$IsWindows -or !$env:GITHUB_ENV) {
    throw 'Run this helper on a Windows GitHub Actions runner.'
}
$Before = @{}
Get-ChildItem Env: | ForEach-Object { $Before[$_.Name] = $_.Value }
$VsWhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
$Installation = & $VsWhere -latest -products '*' -version '[17.0,18.0)' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if ($LASTEXITCODE -ne 0 -or !$Installation) {
    throw 'Visual Studio 2022 with x64 C++ tools was not found.'
}
$Launcher = Join-Path $Installation 'Common7/Tools/Launch-VsDevShell.ps1'
& $Launcher -Arch amd64 -HostArch amd64 -SkipAutomaticLocation
$Linker = (Get-Command link.exe -CommandType Application -ErrorAction Stop).Source
if (!$Linker.StartsWith("$Installation\", [StringComparison]::OrdinalIgnoreCase)) {
    throw "Expected the Visual Studio linker, found: $Linker"
}
foreach ($Tool in @('cl.exe', 'rc.exe')) {
    Get-Command $Tool -CommandType Application -ErrorAction Stop | Out-Null
}
# Export only variables changed by Visual Studio; never dump the runner environment.
foreach ($Variable in Get-ChildItem Env:) {
    if (!$Before.ContainsKey($Variable.Name) -or $Before[$Variable.Name] -cne $Variable.Value) {
        $Delimiter = 'MERLIN_' + [Guid]::NewGuid().ToString('N')
        @("$($Variable.Name)<<$Delimiter", $Variable.Value, $Delimiter) |
            Out-File -FilePath $env:GITHUB_ENV -Encoding utf8 -Append
    }
}
Write-Host "MSVC x64 environment ready: $Linker"
