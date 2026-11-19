# Wiring MIMO to your GitHub repository â€” done list & what's left

The app, installer and README now point at **`github.com/iamrev11ge/MIMO`**. What was wired:

- âœ… Updater download base (`src-tauri/src/update.rs` â†’ `releases/latest/download`)
- âœ… README badges, download buttons, install links (English + Persian)
- âœ… In-app "what's new" links after an update (`ui/app.js`)
- âœ… Installer publisher fields (`installer/island.nsi`: Publisher `iamrev11ge`, URLInfoAbout)
- âœ… Release titles: `v1.0.0` publishes as **MIMO 1.0**, `v1.1.0` as **MIMO 1.1** (a real patch keeps all three: `MIMO 1.0.1`)
- âœ… Your own update key: `mimo-sign-key.pem` on this machine (git-ignored), public half compiled into `update.rs` (fingerprint `20734c80e3917559`)
- âœ… Authenticode signing hooked into CI (`installer/make-cert.ps1`, `installer/sign-code.ps1`)

What only you can do, in order:

1. **Create the repository** `iamrev11ge/MIMO` on GitHub and push this folder to `main`.
2. **Add the repository secret** `MIMO_SIGN_KEY` = the full contents of `mimo-sign-key.pem`
   (Settings â†’ Secrets and variables â†’ Actions â†’ New repository secret). Without it the release
   workflow's signing step fails on purpose â€” an unsigned installer would be rejected by every copy
   of the app.
3. **Optional, Windows signature:** make a certificate with `installer/make-cert.ps1`, then set the
   secrets `MIMO_PFX_B64` (the .pfx file, base64) and `MIMO_PFX_PASS`. CI then signs every release;
   Explorer shows "verified publisher".
4. **Release:** bump `version` in `src-tauri/Cargo.toml`, commit, then
   `git tag v1.0.0 && git push --tags`. The workflow builds, signs and publishes **MIMO 1.0**
   (`MIMO-Setup.exe`, `MIMO-Portable.exe`, `MIMO-Windows.zip`, `latest.json`, the `.sig`).
   Installed copies pick it up within ~6 hours, or instantly via Settings â†’ *Check for updates*.

Keep `mimo-sign-key.pem` safe (a password manager, a private drive). Losing it means every installed
copy can only ever update to releases signed with it â€” a new key needs a new manual install everywhere.
