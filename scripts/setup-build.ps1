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
$targetRoot = if ($env:CARGO_TARGET_ROOT) { $env:CARGO_TARGET_ROOT } else { 'D:\target' }
$buildRoot = if ($env:CARGO_BUILD_ROOT) { $env:CARGO_BUILD_ROOT } else { 'C:\code_build' }
$outputs = [ordered]@{
    target = Join-Path (Join-Path $targetRoot $repoName) $slot
    build = Join-Path (Join-Path $buildRoot $repoName) $slot
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
