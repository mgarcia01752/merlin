#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: GPL-3.0-only
#
# Normal-user Windows Installer handoff. Embedded by Rust, not installed as a
# script entry point. No execution-policy or Windows security settings change.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Get-MerlinProperty {
    <#
    .SYNOPSIS
    Read an indexed Windows Installer Automation property explicitly.
    .PARAMETER Object
    COM receiver; property access must not be dispatched as a method.
    .PARAMETER Member
    Documented property name.
    .PARAMETER Values
    Optional indexed-property arguments.
    #>
    param($Object, [string]$Member, [object[]]$Values = @())
    return ,($Object.GetType().InvokeMember($Member, [Reflection.BindingFlags]::GetProperty, $null, $Object, $Values))
}

function Invoke-MerlinMethod {
    <#
    .SYNOPSIS
    Invoke a Windows Installer Automation method without property ambiguity.
    .PARAMETER Object
    COM receiver.
    .PARAMETER Member
    Documented method name.
    .PARAMETER Values
    Optional method arguments.
    #>
    param($Object, [string]$Member, [object[]]$Values = @())
    return ,($Object.GetType().InvokeMember($Member, [Reflection.BindingFlags]::InvokeMethod, $null, $Object, $Values))
}

function Assert-MerlinPackageIdentity {
    <#
    .SYNOPSIS
    Reject a foreign, wrong-version or non-x64 MSI before requesting UAC.
    .PARAMETER Properties
    MSI Property table values and SummaryInformation template.
    .PARAMETER Version
    Exact numeric version selected from verified stable release metadata.
    #>
    param($Properties, [string]$Version)
    if ($Version -notmatch '^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$' -or
        $Properties.ProductName -cne 'MERLIN' -or
        $Properties.ProductVersion -cne $Version -or
        $Properties.UpgradeCode -ine '{C1858994-9ED6-43A5-82F8-B998E317B37C}' -or
        $Properties.Template.Split(';')[0] -cne 'x64') {
        throw 'The MSI identity does not match the selected MERLIN update.'
    }
}

function Get-MerlinInstallerArguments {
    <#
    .SYNOPSIS
    Build one Windows command line from validated paths and shortcut choice.
    .PARAMETER Plan
    Data-only handoff plan from the Rust staging worker.
    #>
    param($Plan)
    foreach ($Value in @($Plan.package, $Plan.directory, $Plan.log)) {
        if (!$Value -or $Value -match '["\r\n]' -or ![IO.Path]::IsPathRooted($Value)) {
            throw 'Invalid installer path.'
        }
    }
    # MSI quotes a directory without a trailing slash (no backslash/quote edge).
    $Directory = [IO.Path]::GetFullPath($Plan.directory).TrimEnd([char[]]'\/')
    $Package = [IO.Path]::GetFullPath($Plan.package)
    $Log = [IO.Path]::GetFullPath($Plan.log)
    $Arguments = '/i "{0}" /passive /norestart REBOOT=ReallySuppress INSTALLDIR="{1}" /L*V "{2}"' -f $Package, $Directory, $Log
    if ($Plan.desktop_shortcut) { $Arguments += ' DESKTOP_SHORTCUT=1' }
    return $Arguments
}

function Get-MerlinInstallMessage {
    <#
    .SYNOPSIS
    Interpret MSI completion without treating cancellation or failure as success.
    .PARAMETER Code
    Completed msiexec exit code; Windows restart is never forced.
    .PARAMETER Version
    Selected release version for a bounded user-facing result.
    #>
    param([int]$Code, [string]$Version)
    switch ($Code) {
        0 { return "MERLIN $Version was installed. Your workbench was restored." }
        3010 { return "MERLIN $Version was installed. Windows requires a restart to finish the update; save your work before restarting Windows." }
        1602 { return 'Update canceled. Your previous workbench was restored.' }
        default { return "Update failed (Windows Installer code $Code). Your workbench was restored. See the update installer log before retrying." }
    }
}

function Invoke-MerlinUpdate {
    <#
    .SYNOPSIS
    Validate installed ownership, wait for app exit, install and relaunch normally.
    .DESCRIPTION
    Only msiexec receives RunAs. This helper remains the original desktop user.
    All paths arrive in a bounded JSON plan, never through command interpolation.
    #>
    $Plan = $null
    $Parent = $null
    $Committed = $false
    try {
        $Identity = [Security.Principal.WindowsIdentity]::GetCurrent()
        $Principal = [Security.Principal.WindowsPrincipal]::new($Identity)
        if ($Principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
            throw 'Start MERLIN as your normal desktop user before updating.'
        }
        $PlanPath = $env:MERLIN_UPDATE_PLAN
        if (!(Test-Path -LiteralPath $PlanPath -PathType Leaf) -or (Get-Item -LiteralPath $PlanPath).Length -gt 16384) {
            throw 'Missing or oversized update handoff plan.'
        }
        $Plan = Get-Content -LiteralPath $PlanPath -Raw | ConvertFrom-Json
        $Installer = New-Object -ComObject WindowsInstaller.Installer
        $MatchingProducts = @()
        $Products = Get-MerlinProperty $Installer 'RelatedProducts' @('{C1858994-9ED6-43A5-82F8-B998E317B37C}')
        $ProductCount = Get-MerlinProperty $Products 'Count'
        for ($ProductIndex = 0; $ProductIndex -lt $ProductCount; $ProductIndex++) {
            $Product = Get-MerlinProperty $Products 'Item' @($ProductIndex)
            $Installed = Get-MerlinProperty $Installer 'ComponentPath' @($Product, '{589DDD00-7D9B-52B4-B58C-9080EDCADA10}')
            if ([string]::Equals($Installed, $Plan.executable, [StringComparison]::OrdinalIgnoreCase)) {
                $MatchingProducts += $Product
            }
        }
        if ($MatchingProducts.Count -ne 1) { throw 'Update requires the registered MSI installation; source and extracted copies cannot install updates.' }
        $Previous = Get-MerlinProperty $Installer 'ProductInfo' @($MatchingProducts[0], 'VersionString')
        if ([version]$Plan.version -le [version]$Previous) { throw 'The update must be newer than the installed MSI.' }
        $Package = Get-Item -LiteralPath $Plan.package
        if ($Package.Length -ne $Plan.size -or (Get-FileHash -LiteralPath $Plan.package -Algorithm SHA256).Hash -ine $Plan.sha256) {
            throw 'The staged update failed size or SHA-256 verification.'
        }
        $Database = Invoke-MerlinMethod $Installer 'OpenDatabase' @($Plan.package, 0)
        $Properties = @{}
        $View = Invoke-MerlinMethod $Database 'OpenView' @('SELECT `Property`, `Value` FROM `Property`')
        Invoke-MerlinMethod $View 'Execute' | Out-Null
        $Record = Invoke-MerlinMethod $View 'Fetch'
        while ($null -ne $Record) {
            $Name = Get-MerlinProperty $Record 'StringData' @(1)
            $Properties[$Name] = Get-MerlinProperty $Record 'StringData' @(2)
            $Record = Invoke-MerlinMethod $View 'Fetch'
        }
        $Summary = Get-MerlinProperty $Database 'SummaryInformation' @(0)
        $Properties.Template = Get-MerlinProperty $Summary 'Property' @(7)
        Assert-MerlinPackageIdentity $Properties $Plan.version
        $Shortcut = Get-ItemProperty -LiteralPath 'HKCU:\Software\MERLIN\Installer' -Name DesktopShortcut -ErrorAction SilentlyContinue
        $Plan.desktop_shortcut = $null -ne $Shortcut -and $Shortcut.DesktopShortcut -eq 1
        $Arguments = Get-MerlinInstallerArguments $Plan
        $Parent = Get-Process -Id $Plan.parent_pid -ErrorAction Stop
        if (![string]::Equals($Parent.Path, $Plan.executable, [StringComparison]::OrdinalIgnoreCase)) { throw 'Update parent identity changed.' }
        $null = $Parent.Handle
        # Release COM handles before Windows Installer opens/replaces the payload.
        Invoke-MerlinMethod $View 'Close' | Out-Null
        [void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($Summary)
        [void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($Products)
        [void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($View)
        [void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($Database)
        [void][Runtime.InteropServices.Marshal]::FinalReleaseComObject($Installer)
        [IO.File]::WriteAllText($Plan.ready, 'ready')
        $Deadline = [DateTime]::UtcNow.AddSeconds(60)
        while (!(Test-Path -LiteralPath $Plan.commit)) {
            if ([DateTime]::UtcNow -gt $Deadline) { throw 'The app did not commit the update handoff.' }
            Start-Sleep -Milliseconds 100
        }
        $Committed = $true
        # Exact original process identity, not an unrelated future reused PID.
        if ($Parent) { $Parent.WaitForExit(60000) | Out-Null; if (!$Parent.HasExited) { throw 'MERLIN is still running. Close it before retrying the update.' } }
        $MsiExec = Join-Path $env:SystemRoot 'System32/msiexec.exe'
        $Process = Start-Process -FilePath $MsiExec -ArgumentList $Arguments -Verb RunAs -Wait -PassThru
        $Message = Get-MerlinInstallMessage $Process.ExitCode $Plan.version
    } catch {
        $Message = "Update did not complete: $($_.Exception.Message)"
    }
    $Message = $Message.Substring(0, [Math]::Min(512, $Message.Length))
    if ($Plan) {
        if (!$Committed) {
            [IO.File]::WriteAllText($Plan.failed, $Message)
            if ($Parent -and $Parent.HasExited) { Remove-Item -LiteralPath $Plan.staging -Recurse -Force -ErrorAction SilentlyContinue }
            return
        }
        @{message=$Message} | ConvertTo-Json -Compress | Set-Content -LiteralPath $Plan.result -Encoding UTF8
        try {
            if (!(Test-Path -LiteralPath $Plan.executable -PathType Leaf)) { throw 'MERLIN is missing. Repair it with the full MSI from Releases.' }
            if (!$Parent.HasExited) { throw 'The original MERLIN window is still open; close it before retrying.' }
            Start-Process -FilePath $Plan.executable -ArgumentList '--restore-update' -WorkingDirectory $Plan.directory | Out-Null
        } catch {
            Add-Type -AssemblyName PresentationFramework
            [void][Windows.MessageBox]::Show("$Message`n$($_.Exception.Message)`nYour recovery file remains in LOCALAPPDATA/merlin/update-recovery.", 'MERLIN Update')
        } finally {
            # Keep the external installer log and bounded result, remove MSI/data.
            Remove-Item -LiteralPath $Plan.staging -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

# Dot-sourcing exposes pure contract helpers to native tests without installing.
if ($MyInvocation.InvocationName -ne '.') { Invoke-MerlinUpdate }
