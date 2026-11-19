# Makes YOUR own code-signing certificate (self-signed) and exports it as a PFX that sign-code.ps1 uses.
#
#   powershell -ExecutionPolicy Bypass -File installer\make-cert.ps1 -Password "pick-a-password"
#
# The certificate is trusted on YOUR machines (it is added to the current user's trusted roots). Strangers
# will still see SmartScreen once — removing that for the public needs a paid certificate from a CA; the
# steps are the same, just use the PFX you bought instead of the one this makes.
param([Parameter(Mandatory = $true)][string]$Password, [string]$Name = "MIMO")

$ErrorActionPreference = 'Stop'
$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=$Name" -KeyUsage DigitalSignature `
    -FriendlyName "$Name code signing" -CertStoreLocation "Cert:\CurrentUser\My" `
    -NotAfter (Get-Date).AddYears(5)
Export-PfxCertificate -Cert $cert -FilePath "installer\$Name.pfx" -Password (ConvertTo-SecureString -String $Password -Force -AsPlainText) | Out-Null
# trust it on this machine so signed builds show as verified here
Import-Certificate -FilePath ("Cert:\CurrentUser\My\" + $cert.Thumbprint) -CertStoreLocation Cert:\CurrentUser\Root | Out-Null
Write-Output ""
Write-Output "Certificate : installer\$Name.pfx  (thumbprint $($cert.Thumbprint))"
Write-Output "Now sign builds with:"
Write-Output "  `$env:MIMO_PFX='installer\$Name.pfx'; `$env:MIMO_PFX_PASS='$Password'; powershell -File installer\sign-code.ps1"
