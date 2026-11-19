// Updates: now and then the app asks GitHub whether a newer release exists, downloads its installer, checks
// that it was signed with the project's key, and tells the page. With "Update automatically" on (the default)
// it then installs itself at a quiet moment: right after the app has started, or once nobody has touched the
// computer for a few minutes. With it off the app still asks which version is out and, when a newer one
// exists, says so once — the next click downloads and installs it. The next start says what happened.
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

use serde_json::{json, Value};
use windows::core::HSTRING;
use windows::Win32::System::Com::Urlmon::URLDownloadToFileW;
use windows::Win32::System::SystemInformation::GetTickCount;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

use crate::{settings, sys};

const BASE: &str = "https://github.com/iamrev11ge/MIMO/releases/latest/download";
/// The public half of this project's own release-signing key (Ed25519, fingerprint 20734c80e3917559).
/// The private half lives only on the owner's machine (mimo-sign-key.pem, git-ignored) and in the
    /// repository secret MIMO_SIGN_KEY. Updates signed by any other key are thrown away.
const KEY: [u8; 32] = [71, 135, 211, 88, 215, 42, 220, 172, 198, 230, 122, 54, 27, 12, 193, 246, 110, 36, 160, 82, 177, 60, 206, 76, 151, 39, 44, 251, 0, 52, 225, 155];

static READY: Mutex<Option<(String, PathBuf)>> = Mutex::new(None);

fn dir() -> PathBuf {
    let d = std::env::temp_dir().join("MIMOUpdate");
    let _ = std::fs::create_dir_all(&d);
    d
}
fn fetch(name: &str) -> Option<PathBuf> {
    let to = dir().join(name);
    let _ = std::fs::remove_file(&to);
    let url = format!("{BASE}/{name}?t={}", sys::now_ms()); // the query keeps Windows from serving an old copy
    unsafe { URLDownloadToFileW(None, &HSTRING::from(url), &HSTRING::from(to.to_string_lossy().as_ref()), 0, None).ok()? };
    to.exists().then_some(to)
}
fn newer(a: &str, than: &str) -> bool {
    let n = |s: &str| s.trim_start_matches('v').split('.').map(|p| p.parse::<u32>().unwrap_or(0)).collect::<Vec<_>>();
    n(a) > n(than)
}
fn signed(file: &PathBuf, sig_file: &PathBuf) -> bool {
    use base64::Engine;
    let (Ok(data), Ok(sig)) = (std::fs::read(file), std::fs::read_to_string(sig_file)) else { return false };
    let Ok(sig) = base64::engine::general_purpose::STANDARD.decode(sig.trim()) else { return false };
    let (Ok(pk), Ok(sig)) = (ed25519_compact::PublicKey::from_slice(&KEY), ed25519_compact::Signature::from_slice(&sig)) else { return false };
    pk.verify(&data, &sig).is_ok()
}
/// The version number GitHub's latest release carries, if it could be reached.
fn latest() -> Option<String> {
    let info = fetch("latest.json").and_then(|p| std::fs::read_to_string(p).ok()).and_then(|t| serde_json::from_str::<Value>(&t).ok())?;
    Some(info["version"].as_str().unwrap_or("").to_string())
}
/// "none" | "ready 2.2.0" | "failed: why"
pub fn check() -> String {
    if let Some((v, _)) = READY.lock().unwrap().as_ref() {
        return format!("ready {v}");
    }
    let Some(version) = latest() else {
        return "failed: could not reach GitHub".into();
    };
    if !newer(&version, env!("CARGO_PKG_VERSION")) {
        return "none".into();
    }
    let (Some(setup), Some(sig)) = (fetch("MIMO-Setup.exe"), fetch("MIMO-Setup.exe.sig")) else {
        return "failed: download did not finish".into();
    };
    if !signed(&setup, &sig) {
        let _ = std::fs::remove_file(&setup);
        sys::log(&format!("update {version}: the installer's signature is wrong, it was thrown away"));
        return "failed: the download is not signed by this project".into();
    }
    *READY.lock().unwrap() = Some((version.clone(), setup));
    sys::helper(json!({ "t": "update", "version": version }));
    format!("ready {version}")
}
/// "none" | "available 2.2.0" | "failed: why" — only asks GitHub which version is out, without downloading
/// anything. Used when "Update automatically" is off: the app still tells the user that a new version exists.
pub fn peek() -> String {
    let Some(version) = latest() else {
        return "failed: could not reach GitHub".into();
    };
    if !newer(&version, env!("CARGO_PKG_VERSION")) {
        return "none".into();
    }
    format!("available {version}")
}
/// Runs the downloaded installer quietly; it closes this copy, replaces it and starts the new one.
/// If an update was announced but not fetched yet ("Update automatically" off), it is downloaded first.
/// It is started through cmd's "start", so it is not this app's child and outlives the app it closes.
pub fn install() -> bool {
    if READY.lock().unwrap().is_none() {
        check();
    }
    let Some((v, setup)) = READY.lock().unwrap().clone() else { return false };
    if !signed(&setup, &dir().join("MIMO-Setup.exe.sig")) {
        return false; // the file changed since it was checked
    }
    sys::log(&format!("installing update {v}"));
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let started = std::process::Command::new("cmd")
        .raw_arg(format!("/c start \"\" \"{}\" /S", setup.display()))
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .is_ok();
    if !started {
        sys::shell_open(&setup.to_string_lossy(), Some("/S"));
    }
    true
}

/// How long nobody has pressed a key or moved the mouse, anywhere (ms).
fn idle_ms() -> u64 {
    unsafe {
        let mut li = LASTINPUTINFO { cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
        if !GetLastInputInfo(&mut li).as_bool() {
            return 0;
        }
        GetTickCount().wrapping_sub(li.dwTime) as u64
    }
}
/// The first start after an update says so, once.
fn announce() {
    let now = env!("CARGO_PKG_VERSION");
    let before = settings::get("lastVersion").as_str().unwrap_or("").to_string();
    if before != now {
        settings::put("lastVersion", now.into());
        if !before.is_empty() && newer(now, &before) {
            sys::helper(json!({ "t": "updated", "version": now }));
        }
    }
}

/// "Update automatically" is off and a newer release is out: one notification per version, remembered in the
/// settings so restarting does not tell the same thing again.
fn announce_available(version: &str) {
    if settings::get("updateTold").as_str().unwrap_or("") == version {
        return;
    }
    settings::put("updateTold", version.into());
    sys::helper(json!({ "t": "updateAvailable", "version": version }));
}

const FRESH: u64 = 3 * 60_000; // the app started this recently: nobody is using it yet
const AWAY: u64 = 3 * 60_000; // nobody has touched the computer for this long
const EVERY: u64 = 6 * 3600_000; // how often GitHub is asked
const RETRY: u64 = 30 * 60_000; // ... and how soon again after it could not be reached

pub fn start() {
    std::thread::spawn(|| {
        let began = sys::now_ms();
        std::thread::sleep(Duration::from_secs(6)); // the page is listening by now
        announce();
        // an update told about earlier is still out there and the page has forgotten it: show it again,
        // quietly (no notification, just the row in settings and the badge)
        let told = settings::get("updateTold").as_str().unwrap_or("").to_string();
        if newer(&told, env!("CARGO_PKG_VERSION")) {
            sys::helper(json!({ "t": "updateAvailable", "version": told, "quiet": true }));
        }
        std::thread::sleep(Duration::from_secs(39));
        let mut next_check = 0u64;
        loop {
            let auto = settings::packaged() && settings::get("autoUpdate") != Value::Bool(false);
            let now = sys::now_ms();
            if now >= next_check {
                let r = if auto { check() } else { peek() }; // off: still watch for a new version, just say it
                next_check = now + if r.starts_with("failed") { RETRY } else { EVERY };
                if r.starts_with("failed") {
                    sys::log(&format!("update check {r}"));
                }
                if let Some(v) = r.strip_prefix("available ") {
                    announce_available(v);
                }
            }
            if auto && READY.lock().unwrap().is_some() {
                let fresh = sys::now_ms().saturating_sub(began) < FRESH;
                if (fresh || idle_ms() >= AWAY) && install() {
                    return; // the installer closes this copy and starts the new one
                }
            }
            std::thread::sleep(Duration::from_secs(20));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The installer in dist/ and the signature installer/sign.mjs wrote for it must be accepted; a changed file must not.
    /// (Skipped when no signed installer has been built here.)
    #[test]
    fn signature_made_by_the_release_script_is_accepted() {
        let dist = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("dist");
        let (setup, sig) = (dist.join("MIMO-Setup.exe"), dist.join("MIMO-Setup.exe.sig"));
        if !setup.exists() || !sig.exists() {
            return;
        }
        assert!(signed(&setup, &sig));
        let bad = std::env::temp_dir().join("mimo-tampered.exe");
        let mut data = std::fs::read(&setup).unwrap();
        data[1000] ^= 1;
        std::fs::write(&bad, data).unwrap();
        assert!(!signed(&bad, &sig));
        let _ = std::fs::remove_file(bad);
    }

    #[test]
    fn versions_compare_by_number() {
        assert!(newer("2.10.0", "2.9.9"));
        assert!(newer("v2.1.1", "2.1.0"));
        assert!(!newer("2.1.0", "2.1.0"));
        assert!(!newer("", "2.1.0"));
    }
}