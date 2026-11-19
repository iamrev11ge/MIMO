# Signing MIMO — what makes SmartScreen calm down

SmartScreen is not a switch you can turn off in the installer; it is a reputation system. An
installer whose bytes no one has ever seen gets the "unknown publisher" warning until enough
people install a signed copy. Only a signature from a **real certificate authority (CA)** builds
that reputation. MIMO supports three tiers — pick the one that fits:

## 1. Update signatures (Ed25519) — free, always on

Not a Windows signature: the in-app updater's own proof that a release really comes from this
project. `installer/sign.mjs` signs `MIMO-Setup.exe` with the private key kept in the
`MIMO_SIGN_KEY` repository secret; the public half is compiled into `src-tauri/src/update.rs`
(`KEY`, fingerprint `20734c80e3917559`). An installer whose signature does not verify is deleted
before it ever runs. This works on every release and costs nothing — but it does nothing for
SmartScreen.

## 2. Azure Trusted Signing — ~$9.99/month, the cheapest CA-backed option

Microsoft signs your binaries from their own cloud certificate; you never hold a key. SmartScreen
trust builds automatically, and the certificate identity is tied to your verified organization.

1. An Azure subscription (the free tier is enough) with Trusted Signing enabled
   (Azure portal → Trusted Signing accounts → create; identity validation for `iamrev11ge`).
2. An App registration with a client secret, added as **Owner** of the Trusted Signing resource
   (role `Trusted Signing Certificate Profile Signer` on the certificate profile).
3. Set the repository secrets:
   - `TRUSTED_SIGNING_ENDPOINT` — the regional URL, e.g. `https://wus2.code.signing.azure.com`
   - `TRUSTED_SIGNING_ACCOUNT` — the account name
   - `TRUSTED_SIGNING_PROFILE` — the certificate profile name
   - `TRUSTED_SIGNING_TENANT_ID`, `TRUSTED_SIGNING_CLIENT_ID`, `TRUSTED_SIGNING_CLIENT_SECRET`
4. Push a tag. The workflow signs `MIMO-Setup.exe` and `MIMO-Portable.exe` in the cloud and the
   Explorer dialog becomes "MIMO — verified publisher".

## 3. SignPath — free for open-source projects, but review-gated

[signpath.io/oss](https://signpath.io) gives open-source projects free Authenticode signing with
a EV-grade cloud certificate. Requires applying and an identity review, plus their GitHub App or
a user key. Once approved, add a step that submits the artifact through `signpath`'s action —
the build already produces everything needed.

## 4. Self-signed (already wired, private trust only)

`installer/make-cert.ps1` makes your own certificate and `installer/sign-code.ps1` signs with it
( secrets `MIMO_PFX_B64` / `MIMO_PFX_PASS` in CI). Useful for private testing; other machines see
the same SmartScreen warning until the certificate is imported into their trusted roots, so it
does **not** help public distribution.

## Order matters

The Ed25519 signature always runs **last** in the workflow: Authenticode / Trusted Signing edit
the file's bytes, and the updater's `.sig` must cover the final bytes — signing first would make
every update fail verification.
