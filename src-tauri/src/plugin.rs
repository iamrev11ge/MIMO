// Minimal data-only plugins. A plugin is one small JSON file in %APPDATA%\MIMO\plugins — never code:
// it names a source (a local file, or an https URL the user chose) and the app turns what it returns
// into rows under the pill. Nothing is executed, nothing beyond the user's own URL is contacted, and
// a broken plugin shows one "invalid" row instead of ever touching the rest of the app.
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde_json::{json, Value};
use windows::core::HSTRING;
use windows::Win32::System::Com::Urlmon::URLDownloadToFileW;

use crate::{settings, sys};

const MAX_PLUGINS: usize = 5; // one JSON file each
const MAX_BYTES: usize = 64 * 1024; // both the spec and every answer
const MAX_ROWS: usize = 12;
const MIN_SECONDS: u64 = 60;
const ICONS: [&str; 8] = ["plug", "sun", "cloud", "bolt", "cal", "gauge", "search", "jot"];

static REFRESH: AtomicBool = AtomicBool::new(false); // set when the folder may have changed

pub fn dir() -> PathBuf {
    let d = sys::data_dir().join("plugins");
    let _ = std::fs::create_dir_all(&d);
    d
}

#[derive(Clone, Debug, PartialEq)]
struct Spec {
    id: String,     // the file's name without .json
    title: String,
    icon: String,
    url: Option<String>, // None = read `path` instead
    path: Option<String>,
    seconds: u64,
    limit: usize,
}

/// A plugin spec is accepted only when it is small, well-formed and points somewhere sane.
fn parse_spec(id: &str, text: &str) -> Result<Spec, &'static str> {
    if text.len() > MAX_BYTES {
        return Err("too large");
    }
    let v: Value = serde_json::from_str(text).map_err(|_| "bad json")?;
    let title = v["title"].as_str().map(str::trim).filter(|s| !s.is_empty()).ok_or("no title")?;
    if title.chars().count() > 40 {
        return Err("title too long");
    }
    let icon = v["icon"].as_str().filter(|s| ICONS.contains(s)).unwrap_or("plug").to_string();
    let seconds = v["source"]["seconds"].as_u64().unwrap_or(300).max(MIN_SECONDS).min(86_400);
    let limit = v["render"]["limit"].as_u64().unwrap_or(8).clamp(1, MAX_ROWS as u64) as usize;
    let (url, path) = match v["source"]["type"].as_str() {
        Some("url") => {
            let u = v["source"]["url"].as_str().filter(|u| u.len() <= 500 && (u.starts_with("https://") || u.starts_with("http://"))).ok_or("bad url")?;
            (Some(u.to_string()), None)
        }
        Some("file") => {
            let p = v["source"]["path"].as_str().map(str::trim).filter(|p| !p.is_empty() && p.len() <= 400).ok_or("bad path")?;
            (None, Some(p.to_string()))
        }
        _ => return Err("bad source"),
    };
    Ok(Spec { id: id.to_string(), title: title.to_string(), icon, url, path, seconds, limit })
}

fn scan() -> Vec<Spec> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir()) else { return out };
    let mut names: Vec<_> = rd.flatten().filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false)).collect();
    names.sort_by_key(|e| e.file_name());
    for e in names.into_iter().take(MAX_PLUGINS) {
        let path = e.path();
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()).map(String::from) else { continue };
        if stem.chars().count() > 40 || !stem.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            continue;
        }
        if settings::plugins_off().iter().any(|off| off == &stem) {
            continue;
        }
        match std::fs::read_to_string(&path) {
            Ok(text) => match parse_spec(&stem, &text) {
                Ok(spec) => out.push(spec),
                Err(_) => {
                    // an invalid file is announced once, as a row that explains itself, then left alone
                    sys::helper(json!({ "t": "plugin", "id": stem, "title": stem, "icon": "plug", "ok": false, "error": "invalid" }));
                }
            },
            Err(_) => {}
        }
    }
    out
}

/// Fetches a url into a temp file (the updater's and the weather's way — no extra dependencies).
fn get_url(url: &str, name: &str) -> Option<String> {
    let to = std::env::temp_dir().join("MIMOPlugins").join(name);
    let _ = std::fs::create_dir_all(to.parent()?);
    let _ = std::fs::remove_file(&to);
    let full = format!("{url}?mimot={}", sys::now_ms()); // dodge caches
    unsafe { URLDownloadToFileW(None, &HSTRING::from(full), &HSTRING::from(to.to_string_lossy().as_ref()), 0, None).ok()? };
    let text = std::fs::read(&to).ok()?;
    if text.len() > MAX_BYTES {
        return None;
    }
    String::from_utf8(text).ok()
}

/// The answer becomes rows: {"lines":[{"label","value"}]}, {"lines":["…"]} or plain text, one row a line.
/// An empty body is a valid (if useless) answer — Some(empty) — while a wrong shape or an oversized
/// body is None ("unreachable").
fn rows_from(body: &str, limit: usize) -> Option<Vec<(String, String)>> {
    let body = body.trim();
    if body.len() > MAX_BYTES {
        return None;
    }
    if let Ok(v) = serde_json::from_str::<Value>(body) {
        if let Some(lines) = v["lines"].as_array() {
            let rows: Vec<(String, String)> = lines.iter().take(limit).filter_map(|l| {
                if let Some(o) = l.as_object() {
                    let label = o.get("label").and_then(Value::as_str).unwrap_or("").trim().to_string();
                    let value = o.get("value").and_then(Value::as_str).unwrap_or("").trim().to_string();
                    if label.is_empty() && value.is_empty() { None } else { Some((label, value)) }
                } else {
                    let text = l.as_str()?.trim().to_string();
                    (!text.is_empty()).then(|| (String::new(), text))
                }
            }).collect();
            return Some(rows);
        }
        if let Some(s) = v["text"].as_str() {
            return Some(s.lines().take(limit).map(|l| (String::new(), l.trim().to_string())).filter(|(_, l)| !l.is_empty()).collect());
        }
        return None;
    }
    Some(body.lines().take(limit).map(|l| (String::new(), l.trim().to_string())).filter(|(_, l)| !l.is_empty()).collect())
}

fn fetch(spec: &Spec, seq: usize) -> Value {
    let body = match (&spec.url, &spec.path) {
        (Some(url), _) => get_url(url, &format!("{}.txt", spec.id)),
        (None, Some(path)) => {
            // the user's own file on their own machine: read it, capped, and nothing more
            std::fs::read(path).ok().filter(|b| b.len() <= MAX_BYTES).and_then(|b| String::from_utf8(b).ok())
        }
        _ => None,
    };
    match body.and_then(|b| rows_from(&b, spec.limit)) {
        Some(rows) if !rows.is_empty() => json!({
            "t": "plugin", "id": spec.id, "title": spec.title, "icon": spec.icon, "ok": true, "seq": seq,
            "lines": rows.iter().map(|(l, v)| json!([l, v])).collect::<Vec<_>>(),
        }),
        Some(_) => json!({ "t": "plugin", "id": spec.id, "title": spec.title, "icon": spec.icon, "ok": false, "error": "empty", "seq": seq }),
        None => json!({ "t": "plugin", "id": spec.id, "title": spec.title, "icon": spec.icon, "ok": false, "error": "unreachable", "seq": seq }),
    }
}

/// One quiet thread: rescans the folder now and then, fetches each enabled plugin on its own interval.
pub fn start() {
    std::thread::spawn(|| {
        let mut specs: Vec<Spec> = Vec::new();
        let mut next: Vec<u64> = Vec::new();
        let mut scanned = 0u64;
        let mut seq = 0usize;
        loop {
            std::thread::sleep(Duration::from_secs(5));
            let now = sys::now_ms();
            if REFRESH.swap(false, Ordering::SeqCst) || now - scanned > 30_000 {
                scanned = now;
                specs = scan();
                next = vec![0; specs.len()];
                let list: Vec<Value> = specs.iter().map(|s| json!({ "id": s.id, "title": s.title, "icon": s.icon, "seconds": s.seconds })).collect();
                sys::helper(json!({ "t": "pluginSpecs", "specs": list }));
            }
            for (i, spec) in specs.iter().enumerate() {
                if now >= next[i] {
                    next[i] = now + spec.seconds * 1000;
                    seq += 1;
                    let msg = fetch(spec, seq);
                    sys::helper(msg);
                }
            }
        }
    });
}

/// The page touched the folder (opened it, or asked for a rescan): pick up new or removed plugins at once.
pub fn poke() {
    REFRESH.store(true, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specs_parse_and_gate() {
        let ok = parse_spec("a", r#"{"title":"Prayer times","icon":"clock","source":{"type":"url","url":"https://x.example/y","seconds":10},"render":{"limit":99}}"#).unwrap();
        assert_eq!(ok.title, "Prayer times");
        assert_eq!(ok.seconds, 60); // below the floor it is lifted to one minute
        assert_eq!(ok.icon, "plug"); // "clock" is not one of the allowed icons
        assert_eq!(ok.limit, 12); // the cap wins over the file
        assert!(parse_spec("a", r#"{"title":"x","source":{"type":"url","url":"ftp://x"}}"#).is_err());
        assert!(parse_spec("a", r#"{"source":{"type":"file","path":"C:\\x"}}"#).is_err()); // no title
        assert!(parse_spec("a", "not json at all").is_err());
        let file = parse_spec("a", r#"{"title":"F","source":{"type":"file","path":"C:\\data\\now.json"}}"#).unwrap();
        assert_eq!(file.url, None);
        assert_eq!(file.path.as_deref(), Some("C:\\data\\now.json"));
    }

    #[test]
    fn answers_become_rows() {
        let r = rows_from(r#"{"lines":[{"label":"Fajr","value":"05:12"},{"label":"Dhuhr","value":"12:03"}]}"#, 8).unwrap();
        assert_eq!(r[0], ("Fajr".into(), "05:12".into()));
        assert_eq!(r.len(), 2);
        let plain = rows_from("one\ntwo\nthree", 2).unwrap();
        assert_eq!(plain.len(), 2);
        assert_eq!(plain[1], ("".into(), "two".into()));
        let list = rows_from(r#"{"lines":["a"," b ",""]}"#, 8).unwrap();
        assert_eq!(list.len(), 2); // the empty one is dropped
        assert!(rows_from("", 8).unwrap().is_empty());
        assert!(rows_from("{\"lines\":5}", 8).is_none()); // wrong shape
    }
}
