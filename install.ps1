$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'

function Write-Info($msg) { Write-Host "=> $msg" }
function Die($msg) { Write-Error "error: $msg"; exit 1 }

# Detect CI/non-interactive
$IsCI = ($env:CI -ne $null -or $env:GITHUB_ACTIONS -eq 'true' -or $env:TF_BUILD -ne $null)

# Determine platform/arch (Windows x86_64 only for now)
$arch = if ([Environment]::Is64BitOperatingSystem) { 'x86_64' } else { 'x86' }
if ($arch -ne 'x86_64') { Die "unsupported architecture: $arch" }

# Resolve latest version tag
Write-Info "resolving latest version..."
$rel = Invoke-RestMethod -Uri 'https://api.github.com/repos/shivamhwp/git-acm/releases/latest' -UseBasicParsing
$version = $rel.tag_name
if (-not $version) { Die 'could not determine latest version tag' }

$asset = "git-acm-windows-$arch.zip"
$base = "https://github.com/shivamhwp/git-acm/releases/download/$version"
$assetUrl = "$base/$asset"
$checksumsUrl = "$base/checksums.txt"

Write-Info "version: $version"
Write-Info "asset:   $asset"

# Create temp dir
$tmp = New-Item -ItemType Directory -Path (Join-Path $env:TEMP ("git-acm-" + [System.Guid]::NewGuid().ToString()))
try {
  $zipPath = Join-Path $tmp.FullName $asset
  $checksumsPath = Join-Path $tmp.FullName 'checksums.txt'

  Write-Info "downloading checksums..."
  Invoke-WebRequest -Uri $checksumsUrl -OutFile $checksumsPath -UseBasicParsing

  Write-Info "downloading asset..."
  Invoke-WebRequest -Uri $assetUrl -OutFile $zipPath -UseBasicParsing

  # Parse expected checksum for asset
  $expectedLine = Get-Content $checksumsPath | Where-Object { $_ -match [Regex]::Escape($asset) } | Select-Object -First 1
  if (-not $expectedLine) { Die "could not find expected checksum for $asset in checksums.txt" }
  $expected = ($expectedLine -split '\s+')[0]

  Write-Info "verifying checksum..."
  $actual = (Get-FileHash -Algorithm SHA256 -Path $zipPath).Hash
  if ($actual.ToLower() -ne $expected.ToLower()) {
    Write-Error "checksum verification failed for $asset"
    Write-Host  ("expected: {0}`nactual:   {1}`nurl:      {2}" -f $expected, $actual, $assetUrl)
    exit 1
  }

  # Extract
  $extractDir = Join-Path $tmp.FullName 'extract'
  New-Item -ItemType Directory -Path $extractDir | Out-Null
  Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force

  $exe = Get-ChildItem -Path $extractDir -Filter 'git-acm.exe' -Recurse -File | Select-Object -First 1
  if (-not $exe) { Die 'could not locate git-acm.exe in archive' }

  # Determine install dir (prefer Program Files if writable, else LocalAppData)
  $installDir = Join-Path $env:ProgramFiles 'git-acm'
  try {
    if (-not (Test-Path $installDir)) { New-Item -ItemType Directory -Path $installDir -Force | Out-Null }
    $testFile = Join-Path $installDir '.perm'
    Set-Content -Path $testFile -Value 'ok' -Force
    Remove-Item $testFile -Force
  } catch {
    $installDir = Join-Path $env:LOCALAPPDATA 'Programs/git-acm'
    if (-not (Test-Path $installDir)) { New-Item -ItemType Directory -Path $installDir -Force | Out-Null }
  }

  Copy-Item -Path $exe.FullName -Destination (Join-Path $installDir 'git-acm.exe') -Force

  # PATH edits (idempotent, user scope; skip in CI)
  $exeDir = $installDir
  $userPath = [Environment]::GetEnvironmentVariable('Path','User')
  if (-not $IsCI) {
    if (-not $userPath) { $userPath = '' }
    $segments = $userPath -split ';' | Where-Object { $_ -ne '' }
    if (-not ($segments -contains $exeDir)) {
      $newPath = ($userPath.TrimEnd(';') + ';' + $exeDir)
      [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')
      Write-Info "added $exeDir to user PATH"
    } else {
      Write-Info "PATH already contains $exeDir"
    }
  } else {
    Write-Info 'CI detected; skipping PATH edits'
  }

  Write-Info ("installed git-acm {0}" -f $version)
  Write-Info ("asset: {0}" -f $asset)
  Write-Info ("path:  {0}" -f (Join-Path $installDir 'git-acm.exe'))
  Write-Info "try it: git-acm.exe --help"
}
finally {
  if ($tmp -and (Test-Path $tmp.FullName)) { Remove-Item $tmp.FullName -Recurse -Force }
}


