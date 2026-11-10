# Install pinned Cargo tools, Zola, and optionally GitHub CLI on Windows.
# Read tool versions from .devcontainer/.env so local installs and CI use the same pins.
# -DryRun shows the planned commands without installing tools or changing PATH.
[CmdletBinding()]
param(
    [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if ([System.Environment]::OSVersion.Platform -ne [System.PlatformID]::Win32NT) {
    throw "This installer requires Windows and WinGet."
}

$repositoryRoot = Split-Path -Parent $PSScriptRoot
# Share the tool versions with the devcontainer and Linux installer.
$versionFile = Join-Path $repositoryRoot ".devcontainer/.env"
if (-not (Test-Path $versionFile)) {
    throw "Could not find tool versions at $versionFile."
}

$versions = @{}
foreach ($line in Get-Content $versionFile) {
    if ($line -match '^\s*([A-Z][A-Z0-9_]*)=(.*?)\s*$') {
        $versions[$Matches[1]] = $Matches[2]
    }
}

$requiredVersions = @(
    "CARGO_BINSTALL_VERSION",
    "CARGO_MAKE_VERSION",
    "CARGO_NEXTEST_VERSION",
    "CARGO_DENY_VERSION",
    "CARGO_MACHETE_VERSION",
    "CARGO_LLVM_COV_VERSION",
    "ZIZMOR_VERSION",
    "SCCACHE_VERSION",
    "DPRINT_VERSION",
    "LYCHEE_VERSION",
    "ZOLA_VERSION",
    "TYPOS_VERSION"
)
# Validate every required pin up front so a partial tool installation cannot start with missing versions.
foreach ($name in $requiredVersions) {
    if (-not $versions.ContainsKey($name)) {
        throw "Missing $name in $versionFile."
    }
}

$cargo = Get-Command cargo -CommandType Application -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "Rust/Cargo is required. Install Rust with rustup, then rerun this script."
}

$winget = Get-Command winget -CommandType Application -ErrorAction SilentlyContinue
if ($null -eq $winget) {
    throw "WinGet is required to install GitHub CLI."
}

function Get-CargoInstallRoot {
    # Honor Cargo's explicit install root and configured root before using CARGO_HOME.
    if (-not [string]::IsNullOrWhiteSpace($env:CARGO_INSTALL_ROOT)) {
        return [System.IO.Path]::GetFullPath($env:CARGO_INSTALL_ROOT)
    }

    $cargoHome = if ([string]::IsNullOrWhiteSpace($env:CARGO_HOME)) {
        Join-Path $HOME ".cargo"
    }
    else {
        $env:CARGO_HOME
    }
    $cargoHome = [System.IO.Path]::GetFullPath($cargoHome)

    foreach ($configName in @("config.toml", "config")) {
        $configPath = Join-Path $cargoHome $configName
        if (-not (Test-Path $configPath)) {
            continue
        }

        $section = ""
        foreach ($line in Get-Content $configPath) {
            if ($line -match '^\s*\[([^\]]+)\]') {
                $section = $Matches[1]
                continue
            }
            if (
                $section -eq "install" -and
                $line -match '^\s*root\s*=\s*["''](?<root>[^"'']+)["'']'
            ) {
                $configuredRoot = $Matches.root
                if (-not [System.IO.Path]::IsPathRooted($configuredRoot)) {
                    $configuredRoot = Join-Path $cargoHome $configuredRoot
                }
                return [System.IO.Path]::GetFullPath($configuredRoot)
            }
        }
        break
    }

    return $cargoHome
}

function Invoke-ExternalCommand {
    param(
        [Parameter(Mandatory)]
        [string]$FilePath,
        [Parameter(Mandatory)]
        [string[]]$ArgumentList
    )

    # Show the exact command in both normal and dry-run modes.
    Write-Host "> $FilePath $($ArgumentList -join ' ')"
    if ($DryRun) {
        return
    }

    & $FilePath @ArgumentList
    if ($LASTEXITCODE -ne 0) {
        throw "$FilePath failed with exit code $LASTEXITCODE."
    }
}

function Install-ZolaRelease {
    param(
        [Parameter(Mandatory)]
        [string]$Version,
        [Parameter(Mandatory)]
        [string]$Destination
    )

    # Refuse versions without a pinned checksum so an unverified archive is never installed.
    $checksum = switch ($Version) {
        "0.23.6" { "79179284f816febc281adbedb27189806cf22f83e8be4d0fca6835fbeeb09587" }
        default { throw "No pinned Zola release checksum for version $Version." }
    }
    $asset = "zola-v$Version-x86_64-pc-windows-msvc.zip"
    $uri = "https://github.com/getzola/zola/releases/download/v$Version/$asset"
    Write-Host "> Install verified Zola $Version to $Destination"
    if ($DryRun) {
        return
    }

    $temporaryDirectory = Join-Path ([System.IO.Path]::GetTempPath()) ([guid]::NewGuid().ToString())
    New-Item -ItemType Directory -Path $temporaryDirectory | Out-Null
    try {
        $archive = Join-Path $temporaryDirectory $asset
        Invoke-WebRequest -Uri $uri -OutFile $archive
        $actualChecksum = (Get-FileHash -Path $archive -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($actualChecksum -ne $checksum) {
            throw "Zola archive checksum mismatch for $asset."
        }
        Expand-Archive -Path $archive -DestinationPath $temporaryDirectory
        New-Item -ItemType Directory -Force -Path $Destination | Out-Null
        Copy-Item -Path (Join-Path $temporaryDirectory "zola.exe") -Destination (Join-Path $Destination "zola.exe") -Force
    }
    finally {
        # Always remove the downloaded archive and extracted files, including after failures.
        Remove-Item -Path $temporaryDirectory -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Test-PathContainsDirectory {
    # Compare normalized paths so duplicate PATH entries are not added with alternate separators.
    param(
        [AllowEmptyString()]
        [string]$PathValue,
        [Parameter(Mandatory)]
        [string]$Directory
    )

    $trimChars = [char[]]@(
        [System.IO.Path]::DirectorySeparatorChar,
        [System.IO.Path]::AltDirectorySeparatorChar
    )
    $target = [System.IO.Path]::GetFullPath($Directory).TrimEnd($trimChars)
    foreach ($entry in ($PathValue -split ";")) {
        if ([string]::IsNullOrWhiteSpace($entry)) {
            continue
        }
        try {
            $candidate = [System.IO.Path]::GetFullPath($entry.Trim()).TrimEnd($trimChars)
            if ($candidate -ieq $target) {
                return $true
            }
        }
        catch [System.ArgumentException] {
            continue
        }
    }
    return $false
}

function Add-CargoBinToPath {
    # Update this process immediately and the user's persistent PATH for future shells.
    param(
        [Parameter(Mandatory)]
        [string]$CargoBin
    )

    if (-not (Test-PathContainsDirectory -PathValue $env:Path -Directory $CargoBin)) {
        if ($DryRun) {
            Write-Host "Would add $CargoBin to this process PATH."
        }
        else {
            $env:Path = "$CargoBin;$env:Path"
        }
    }

    $userPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
    if (-not (Test-PathContainsDirectory -PathValue $userPath -Directory $CargoBin)) {
        if ($DryRun) {
            Write-Host "Would add $CargoBin to the current user's PATH."
        }
        else {
            $newUserPath = if ([string]::IsNullOrWhiteSpace($userPath)) {
                $CargoBin
            }
            else {
                "$userPath;$CargoBin"
            }
            [System.Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")
            if (Test-PathContainsDirectory -PathValue $env:Path -Directory $CargoBin) {
                return
            }
            $env:Path = "$CargoBin;$env:Path"
        }
    }
}

# Avoid invoking sccache while bootstrapping it, then restore the caller's wrapper.
$previousWrapper = [System.Environment]::GetEnvironmentVariable("RUSTC_WRAPPER", "Process")
Push-Location $repositoryRoot
try {
    $env:RUSTC_WRAPPER = ""
    Invoke-ExternalCommand -FilePath "rustup" -ArgumentList @("component", "add", "llvm-tools-preview")
    Invoke-ExternalCommand -FilePath $cargo.Source -ArgumentList @(
        "install", "--locked", "cargo-binstall", "--version", $versions.CARGO_BINSTALL_VERSION
    )

    $installRoot = Get-CargoInstallRoot
    $cargoBin = Join-Path $installRoot "bin"
    $cargoBinstall = Join-Path $cargoBin "cargo-binstall.exe"
    if (-not $DryRun -and -not (Test-Path $cargoBinstall)) {
        throw "cargo-binstall was not found at $cargoBinstall after installation."
    }

    Invoke-ExternalCommand -FilePath $cargoBinstall -ArgumentList @(
        "--locked",
        "--no-confirm",
        "cargo-make@$($versions.CARGO_MAKE_VERSION)",
        "cargo-nextest@$($versions.CARGO_NEXTEST_VERSION)",
        "cargo-deny@$($versions.CARGO_DENY_VERSION)",
        "cargo-machete@$($versions.CARGO_MACHETE_VERSION)",
        "cargo-llvm-cov@$($versions.CARGO_LLVM_COV_VERSION)",
        "zizmor@$($versions.ZIZMOR_VERSION)",
        "sccache@$($versions.SCCACHE_VERSION)",
        "dprint@$($versions.DPRINT_VERSION)",
        "lychee@$($versions.LYCHEE_VERSION)",
        "typos-cli@$($versions.TYPOS_VERSION)"
    )
    Install-ZolaRelease -Version $versions.ZOLA_VERSION -Destination $cargoBin

    if ($null -eq (Get-Command gh -ErrorAction SilentlyContinue)) {
        Invoke-ExternalCommand -FilePath $winget.Source -ArgumentList @(
            "install",
            "--id", "GitHub.cli",
            "--exact",
            "--accept-package-agreements",
            "--accept-source-agreements"
        )
    }
    else {
        Write-Host "GitHub CLI is already installed; skipping WinGet."
    }

    Add-CargoBinToPath -CargoBin $cargoBin
}
finally {
    if ($null -eq $previousWrapper) {
        Remove-Item Env:RUSTC_WRAPPER -ErrorAction SilentlyContinue
    }
    else {
        $env:RUSTC_WRAPPER = $previousWrapper
    }
    Pop-Location
}
