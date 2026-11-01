$ErrorActionPreference = 'Stop'
if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_OS -ne 'Windows') { throw 'This test runs only on an isolated Windows Actions runner.' }
$config = Get-Content src-tauri/tauri.conf.json -Raw | ConvertFrom-Json
if ($config.bundle.publisher -cne 'WK Arts' -or $config.productName -cne 'Scale Simulator') { throw 'Legacy installer registry identity changed.' }
$old = Join-Path $env:RUNNER_TEMP 'scale-stable-installer'
New-Item -ItemType Directory -Force $old | Out-Null
$asset = 'Scale-Simulator-0.2.1-x86_64-pc-windows-msvc-Scale.Simulator_0.2.1_x64-setup.exe'
& gh release download v0.2.1 --repo wkarts/ARGWS-Scale-Simulator --pattern $asset --dir $old
if ($LASTEXITCODE -ne 0) { throw 'Could not retrieve the pinned stable installer.' }
$oldExe = Join-Path $old $asset
if ((Get-FileHash $oldExe -Algorithm SHA256).Hash.ToLowerInvariant() -ne '8a9214974a0f4b0c0268afa0403d6540881ebf03695d2670fb3b32f577aee452') { throw 'Stable installer checksum mismatch.' }
$install = Join-Path $env:LOCALAPPDATA 'Scale Simulator Upgrade Smoke'
$data = Join-Path $env:APPDATA $config.identifier
New-Item -ItemType Directory -Force $data | Out-Null
$marker = Join-Path $data 'upgrade-preservation-fixture.txt'
Set-Content $marker 'synthetic-preserved-data'
function Invoke-Installer([string]$Exe, [string]$Arguments) {
  $p = Start-Process -FilePath $Exe -ArgumentList $Arguments -PassThru
  if (!$p.WaitForExit(180000)) { $p.Kill(); throw "Installer timed out: $Exe" }
  if ($p.ExitCode -ne 0) { throw "Installer failed: $Exe exit=$($p.ExitCode)" }
}
Invoke-Installer $oldExe "/S /D=$install"
$manufacturerKey = "HKCU:\Software\$($config.bundle.publisher)\$($config.productName)"
$legacyPath = (Get-Item $manufacturerKey).GetValue('')
if ($legacyPath -ne $install) { throw "New installer identity cannot locate the old installation: $manufacturerKey" }
$uninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\$($config.productName)"
$legacy = Get-ItemProperty $uninstallKey
if ($legacy.DisplayVersion -ne '0.2.1') { throw 'Stable install did not register version 0.2.1.' }
# Exactly the legacy directory that PageLeaveReinstall obtains from MANUPRODUCTKEY.
Invoke-Installer (Join-Path $legacyPath 'uninstall.exe') "/S _?=$legacyPath"
if (Test-Path (Join-Path $install 'scale_simulator_tauri.exe')) { throw 'Previous executable survived uninstall.' }
if (!(Test-Path $marker)) { throw 'Previous uninstall removed application data.' }
$current = @(Get-ChildItem "$env:CARGO_TARGET_DIR/x86_64-pc-windows-msvc/release/bundle/nsis" -Filter '*-setup.exe')
if ($current.Count -ne 1) { throw 'Expected exactly one current NSIS bundle.' }
Invoke-Installer $current[0].FullName "/S /D=$install"
if (!(Test-Path (Join-Path $install 'scale_simulator_tauri.exe'))) { throw 'Current executable was not installed.' }
if ((Get-ItemProperty $uninstallKey).DisplayVersion -ne $config.version) { throw 'Current installer registered wrong version.' }
if ((Get-Item $manufacturerKey).GetValue('') -ne $install) { throw 'Current registry path mismatch.' }
Invoke-Installer (Join-Path $install 'uninstall.exe') "/S _?=$install"
if (Test-Path (Join-Path $install 'scale_simulator_tauri.exe')) { throw 'Current uninstall left executable behind.' }
if ((Get-Content $marker -Raw).Trim() -ne 'synthetic-preserved-data') { throw 'Application data changed.' }
Write-Output 'Stable 0.2.1 install, legacy uninstall handoff, current install and uninstall passed; synthetic application data preserved.'
