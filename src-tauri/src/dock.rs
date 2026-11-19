// The dock: one folder of program shortcuts, shown as icon tiles under the pill. Data only — the folder
// is the user's own choice, nothing is ever written into it, and launching is restricted to files that
// really are inside that folder with an allowed extension.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde_json::{json, Value};

use crate::{settings, sys};

const MAX_APPS: usize = 24;
const EXTS: [&str; 3] = ["lnk", "exe", "url"];
const ICON_CACHE_MAX: usize = 64;

static REFRESH: AtomicBool = AtomicBool::new(false);

/// The folder the dock shows. A folder set in Settings wins (when it exists); otherwise the app's own
/// dock folder, which is created on first use.
pub fn dir() -> PathBuf {
    let custom = settings::get("dockFolder").as_str().unwrap_or("").trim().to_string();
    dir_for(&custom)
}

fn dir_for(custom: &str) -> PathBuf {
    let custom = custom.trim();
    let d = if !custom.is_empty() && Path::new(custom).is_dir() {
        PathBuf::from(custom)
    } else {
        sys::data_dir().join("dock")
    };
    let _ = std::fs::create_dir_all(&d);
    d
}

pub fn poke() {
    REFRESH.store(true, Ordering::SeqCst);
}

/// Every shortcut in the dock folder (name, path, icon). Icons come from the shell and are cached by
/// path + size + modification time, so a rescan costs nothing until something in the folder changes.
fn scan(cache: &mut HashMap<String, Option<String>>) -> Vec<Value> {
    let d = dir();
    let Ok(rd) = std::fs::read_dir(&d) else { return Vec::new() };
    let mut files: Vec<_> = rd
        .flatten()
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .map(|x| EXTS.iter().any(|e2| x.eq_ignore_ascii_case(e2)))
                .unwrap_or(false)
        })
        .collect();
    files.sort_by_key(|e| e.file_name());
    files.truncate(MAX_APPS);
    files
        .into_iter()
        .filter_map(|e| {
            let path = e.path();
            let name = path.file_stem()?.to_str()?.trim().to_string();
            if name.is_empty() || name.chars().count() > 60 {
                return None;
            }
            let meta = e.metadata().ok()?;
            let mtime = meta.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_millis();
            let key = format!("{}|{}|{}", path.to_string_lossy(), meta.len(), mtime);
            let icon = if let Some(cached) = cache.get(&key) {
                cached.clone()
            } else {
                let ic = sys::file_icon_b64(&path.to_string_lossy());
                if cache.len() >= ICON_CACHE_MAX {
                    cache.clear();
                }
                cache.insert(key, ic.clone());
                ic
            };
            Some(json!({ "name": name, "path": path.to_string_lossy(), "icon": icon }))
        })
        .collect()
}

/// One quiet thread: the folder is rescanned every few seconds and the page hears about it only when
/// the list really changed. Opening the folder from the page pokes a rescan through.
pub fn start() {
    std::thread::spawn(|| {
        let mut cache: HashMap<String, Option<String>> = HashMap::new();
        let mut sig = String::new();
        loop {
            std::thread::sleep(Duration::from_secs(5));
            let poked = REFRESH.swap(false, Ordering::SeqCst);
            let apps = scan(&mut cache);
            let new_sig: String = apps.iter().map(|a| format!("{}\n", a["path"].as_str().unwrap_or(""))).collect();
            if poked || new_sig != sig {
                sig = new_sig;
                sys::helper(json!({ "t": "dockApps", "apps": apps }));
            }
        }
    });
}

/// A click on a dock tile. Only files that really live in the dock folder and carry an allowed
/// extension are ever handed to the shell.
pub fn launch(path: &str) -> bool {
    let d = dir();
    launch_in(&d, path)
}

fn launch_in(d: &Path, path: &str) -> bool {
    let Ok(canonical) = Path::new(path).canonicalize() else { return false };
    let Ok(dir_canonical) = d.canonicalize() else { return false };
    if !canonical.starts_with(&dir_canonical) {
        return false;
    }
    let allowed = Path::new(path)
        .extension()
        .and_then(|x| x.to_str())
        .map(|x| EXTS.iter().any(|e2| x.eq_ignore_ascii_case(e2)))
        .unwrap_or(false);
    if !allowed {
        return false;
    }
    sys::allow_foreground();
    sys::shell_open(path, None);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dock() -> PathBuf {
        let d = std::env::temp_dir().join(format!("MIMODockTest{}", std::process::id()));
        let _ = std::fs::create_dir_all(&d);
        d
    }

    #[test]
    fn launch_only_takes_allowed_files_from_the_dock_folder() {
        let d = temp_dock();
        let inside = d.join("tool.lnk");
        std::fs::write(&inside, b"").unwrap();
        let outside = std::env::temp_dir().join("mimo-dock-outside.lnk");
        std::fs::write(&outside, b"").unwrap();
        std::fs::write(d.join("script.bat"), b"").unwrap();

        assert!(launch_in(&d, &inside.to_string_lossy()), "a .lnk inside the dock folder must launch");
        assert!(!launch_in(&d, &outside.to_string_lossy()), "a .lnk outside the dock folder must be refused");
        assert!(!launch_in(&d, "not-a-real-path"), "a path that does not exist must be refused");
        let _ = std::fs::remove_dir_all(&d);
        let _ = std::fs::remove_file(&outside);
    }

    #[test]
    fn dir_uses_a_real_custom_folder_and_falls_back() {
        let d = temp_dock();
        assert_eq!(dir_for(&d.to_string_lossy()), d, "an existing custom folder wins");
        assert!(dir_for("C:\\definitely\\not\\here\\mimo").ends_with("dock"), "a missing custom folder falls back");
        assert!(dir_for("").ends_with("dock"));
        let _ = std::fs::remove_dir_all(&d);
    }
}
