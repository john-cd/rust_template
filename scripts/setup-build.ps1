param()

$ErrorActionPreference = 'Stop'
if (-not $IsWindows) { throw 'This setup task is for Windows checkouts.' }

$checkout = [IO.Path]::GetFullPath((Split-Path -Parent $PSScriptRoot))
$checkoutName = Split-Path -Leaf $checkout
$parent = Split-Path -Parent $checkout
$slot = if ((Split-Path -Leaf $parent) -eq '.worktrees') { $checkoutName } else { 'main' }
$mainRepo = if ($slot -eq 'main') { $checkout } else { Split-Path -Parent $parent }
if ($env:CODE_WORKTREE_SLOT) { $slot = $env:CODE_WORKTREE_SLOT }
if ($slot -notmatch '^(main|[abc])$') { throw "Invalid checkout slot: $slot" }

$repoName = Split-Path -Leaf $mainRepo
$targetRoot = if ($env:CODE_TARGET_ROOT) { $env:CODE_TARGET_ROOT } else { 'D:\target' }
$buildRoot = if ($env:CODE_BUILD_ROOT) { $env:CODE_BUILD_ROOT } else { 'C:\code_build' }
$targetOutput = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } elseif ($env:CARGO_BUILD_TARGET_DIR) { $env:CARGO_BUILD_TARGET_DIR } else { Join-Path (Join-Path $targetRoot $repoName) $slot }
$buildOutput = if ($env:CARGO_BUILD_BUILD_DIR) { $env:CARGO_BUILD_BUILD_DIR } else { Join-Path (Join-Path $buildRoot $repoName) $slot }
$outputs = [ordered]@{
    target = $targetOutput
    build = $buildOutput
}

foreach ($name in $outputs.Keys) {
    $link = Join-Path $checkout $name
    $destination = [IO.Path]::GetFullPath($outputs[$name])
    $existing = Get-Item -LiteralPath $link -Force -ErrorAction SilentlyContinue
    if ($existing) {
        if (-not ($existing.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            throw "Refusing to replace existing directory or file: $link"
        }
        $resolved = $existing.ResolveLinkTarget($true)
        if (-not $resolved -or [IO.Path]::GetFullPath($resolved.FullName) -ne $destination) {
            throw "Preserving existing $link; expected a link to $destination."
        }
    } else {
        New-Item -ItemType Directory -Path $destination -Force | Out-Null
        New-Item -ItemType Junction -Path $link -Target $destination | Out-Null
    }
    Write-Host "$link -> $destination"
}
