# Signs the built binaries with YOUR Authenticode certificate (Windows shows "MIMO — verified publisher").
#
#   $env:MIMO_PFX='installer\MIMO.pfx'; $env:MIMO_PFX_PASS='...'; powershell -File installer\sign-code.ps1
#
# Skips with a note when no certificate is configured, so a plain build never fails because of it.
# In GitHub Actions set the secrets MIMO_PFX_B64 (the .pfx, base64) and MIMO_PFX_PASS — the workflow
# decodes and signs the same way.
$ErrorActionPreference = 'Stop'
if (-not $env:MIMO_PFX -or -not $env:MIMO_PFX_PASS) {
    Write-Output "No certificate configured (MIMO_PFX / MIMO_PFX_PASS) — binaries stay unsigned."
    Write-Output "Make one with: powershell -File installer\make-cert.ps1 -Password '...'"
    exit 0
}
$signtool = Get-ChildItem "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\signtool.exe" -ErrorAction SilentlyContinue |
    Sort-Object FullName -Descending | Select-Object -First 1
if (-not $signtool) { $signtool = Get-Command signtool.exe -ErrorAction SilentlyContinue }
if (-not $signtool) { throw "signtool.exe not found — install the Windows SDK, or put signtool on PATH" }
$stamp = if ($env:MIMO_TIMESTAMP) { $env:MIMO_TIMESTAMP } else { "http://timestamp.digicert.com" }
foreach ($f in @('dist\MIMO.exe', 'dist\MIMO-Portable.exe', 'dist\MIMO-Setup.exe')) {
    if (-not (Test-Path $f)) { continue }
    & $signtool.FullName sign /fd SHA256 /td SHA256 /tr $stamp /f $env:MIMO_PFX /p $env:MIMO_PFX_PASS $f
    if ($LASTEXITCODE -ne 0) { throw "signing $f failed" }
    Write-Output "signed $f"
}
