#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Verify native MSI installation/repair/removal on a disposable CI runner only.
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$Root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $Root
if ($env:GITHUB_ACTIONS -ne 'true' -or !$IsWindows) {
    throw 'Machine-changing lifecycle tests require a disposable Windows CI runner.'
}
$Installers = @(Get-ChildItem "$Root/dist/windows-desktop/merlin-desktop_*_windows_amd64.msi")
if ($Installers.Count -ne 1) {
    throw 'Expected exactly one Windows desktop MSI'
}
$Msi = $Installers[0].FullName
$Install = Join-Path $env:ProgramFiles 'MERLIN'
$Logs = "$Root/dist/windows-installer-logs"
$UserFiles = Join-Path $env:RUNNER_TEMP ('MERLIN user files ' + [guid]::NewGuid())
$Menu = Join-Path ([Environment]::GetFolderPath('CommonPrograms')) 'MERLIN'
$Desktop = Join-Path ([Environment]::GetFolderPath('CommonDesktopDirectory')) 'MERLIN.lnk'
foreach ($Path in @($Install, $Menu, $Desktop)) {
    if (Test-Path $Path) {
        throw "Existing MERLIN content: $Path"
    }
}
New-Item -ItemType Directory $Logs, $UserFiles | Out-Null
Copy-Item "$Root/examples/workbench/single/workbench.merlin.json" "$UserFiles/user.merlin.json"
$Before = (Get-FileHash "$UserFiles/user.merlin.json").Hash

# Run MSI operations synchronously with retained Windows Installer diagnostics.
function Invoke-Msi([string[]]$Arguments, [string]$Name) {
    $Process = Start-Process msiexec.exe -ArgumentList ($Arguments + @('/qn', '/norestart', '/L*v', "`"$Logs/$Name.log`"")) -PassThru
    if (!$Process.WaitForExit(300000)) {
        $Process.Kill()
        throw "Windows Installer timed out: $Name"
    }
    $Process.Refresh()
    if ($Process.ExitCode -notin @(0, 3010)) {
        throw "MSI $Name failed: $($Process.ExitCode)"
    }
}

# Check the expected registered native x64 product, then run with only system PATH.
function Assert-Installed {
    if (!(Test-Path "$Menu/MERLIN.lnk")) {
        throw 'Missing Start menu shortcut'
    }
    $Registered = @(Get-ItemProperty 'HKLM:/Software/Microsoft/Windows/CurrentVersion/Uninstall/*' |
        Where-Object { $_.PSObject.Properties['DisplayName'] -and $_.DisplayName -eq 'MERLIN' })
    if ($Registered.Count -ne 1 -or !$Registered[0].UninstallString -or !$Registered[0].DisplayIcon) {
        throw 'Missing/ambiguous MSI registration or Installed apps icon'
    }
    & python tools/packaging/windows_desktop.py --smoke $Install
    if ($LASTEXITCODE -ne 0) {
        throw 'Installed Qt runtime/examples failed'
    }
}
try {
    Invoke-Msi @('/i', "`"$Msi`"") 'install-default'
    Assert-Installed
    if (Test-Path $Desktop) {
        throw 'Desktop shortcut must default to unchecked'
    }
    # Reinstall/repair the same MSI and exercise the opted-in shortcut separately.
    Invoke-Msi @('/fa', "`"$Msi`"") 'repair'
    Assert-Installed
    Invoke-Msi @('/x', "`"$Msi`"") 'remove-default'
    Invoke-Msi @('/i', "`"$Msi`"", 'DESKTOP_SHORTCUT=1') 'install-shortcut'
    Assert-Installed
    if (!(Test-Path $Desktop)) {
        throw 'Missing opted-in desktop shortcut'
    }
} finally {
    Invoke-Msi @('/x', "`"$Msi`"") 'uninstall'
}
foreach ($Path in @($Install, $Menu, $Desktop)) {
    if (Test-Path $Path) {
        throw "Uninstall left packaged content: $Path"
    }
}
$Remaining = @(Get-ItemProperty 'HKLM:/Software/Microsoft/Windows/CurrentVersion/Uninstall/*' |
    Where-Object { $_.PSObject.Properties['DisplayName'] -and $_.DisplayName -eq 'MERLIN' })
if ($Remaining.Count -ne 0) {
    throw 'Uninstall left product registration'
}
if ((Get-FileHash "$UserFiles/user.merlin.json").Hash -ne $Before) {
    throw 'User WB changed'
}
Write-Output 'MERLIN_MSI_INSTALL_REPAIR_SHORTCUT_UNINSTALL_OK'
