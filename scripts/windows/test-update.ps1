#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Run pure Windows update contracts without downloading, elevating or installing.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/../../apps/qt/src/platform/updates/windows_handoff.ps1"

function Assert-UpdateRejected {
    <#
    .SYNOPSIS
    Require a contract action to fail without launching an installer.
    .PARAMETER Action
    Pure validation or argument-construction action.
    #>
    param([scriptblock]$Action)
    $Rejected = $false
    try { & $Action | Out-Null } catch { $Rejected = $true }
    if (!$Rejected) { throw 'Unsafe update input was accepted.' }
}

if ((Get-MerlinProperty 'MERLIN' 'Length') -ne 6 -or
    (Invoke-MerlinMethod 'merlin' 'ToUpperInvariant') -cne 'MERLIN') {
    throw 'Automation member dispatch is incorrect.'
}
$Properties = @{ProductName='MERLIN'; ProductVersion='0.3.0';
    UpgradeCode='{C1858994-9ED6-43A5-82F8-B998E317B37C}'; Template='x64;1033'}
Assert-MerlinPackageIdentity $Properties '0.3.0'
foreach ($Pair in @(@('ProductName','Foreign'), @('ProductVersion','0.1.0'),
    @('UpgradeCode','{00000000-0000-0000-0000-000000000000}'), @('Template','Intel;1033'))) {
    $Bad = $Properties.Clone(); $Bad[$Pair[0]] = $Pair[1]
    Assert-UpdateRejected { Assert-MerlinPackageIdentity $Bad '0.3.0' }
}
foreach ($Version in @('v0.3.0','0.03.0','0.3.0-rc1','0.3')) {
    Assert-UpdateRejected { Assert-MerlinPackageIdentity $Properties $Version }
}
$Plan = [pscustomobject]@{package="C:\Users\Test User's Folder\update.msi";
    directory='C:\Program Files\MERLIN\';log='C:\Users\Test User\last-install.log';desktop_shortcut=$false}
$Arguments = Get-MerlinInstallerArguments $Plan
if ($Arguments -notlike '*/passive /norestart REBOOT=ReallySuppress*' -or
    $Arguments -notlike '*INSTALLDIR="C:\Program Files\MERLIN"*' -or
    $Arguments -like '*DESKTOP_SHORTCUT*' -or $Arguments -notlike '*Test User''s Folder*') {
    throw "Incorrect installer arguments: $Arguments"
}
$Plan.desktop_shortcut = $true
if ((Get-MerlinInstallerArguments $Plan) -notlike '* DESKTOP_SHORTCUT=1') { throw 'Shortcut choice was lost.' }
foreach ($Path in @('relative.msi', 'C:\bad"path.msi', "C:\bad`npath.msi")) {
    $Plan.package=$Path
    Assert-UpdateRejected { Get-MerlinInstallerArguments $Plan }
}
foreach ($Case in @(@(0,'installed'),@(3010,'requires a restart'),@(1602,'canceled'),@(1603,'failed'))) {
    if ((Get-MerlinInstallMessage $Case[0] '0.3.0') -notlike "*$($Case[1])*") { throw 'Incorrect MSI result handling.' }
}
Write-Host 'Windows update contracts passed; no installer was run.'
