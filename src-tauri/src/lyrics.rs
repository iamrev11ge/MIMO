// Lyrics for the playing track, from LRCLIB — free, no key, no account. When the track changes, only
// its title and artist are sent to ask for the words; the answer is cached on disk so the same song
// never asks twice. Nothing else about the machine or the listening habits ever leaves it.
use std::sync::Mutex;

use serde_json::{json, Value};
use windows::core::HSTRING;
use windows::Win32::System::Com::Urlmon::URLDownloadToFileW;

use crate::sys;

static LAST: Mutex<(String, bool)> = Mutex::new((String::new(), false)); // the track asked for, and whether it failed

fn dir() -> std::path::PathBuf {
    let d = std::env::temp_dir().join("MIMOLyrics");
    let _ = std::fs::create_dir_all(&d);
    d
}
/// A stable file name for a track key (the key itself holds characters a file name must not).
fn fnv(text: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in text.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}
/// Downloads a small document to a temp file and reads it back — the same way the updater fetches.
fn get(url: &str, name: &str) -> Option<String> {
    let to = dir().join(name);
    let _ = std::fs::remove_file(&to);
    let full = format!("{url}&t={}", sys::now_ms()); // the extra query keeps Windows from serving a cached copy
    unsafe { URLDownloadToFileW(None, &HSTRING::from(full), &HSTRING::from(to.to_string_lossy().as_ref()), 0, None).ok()? };
    std::fs::read_to_string(&to).ok()
}
/// Minimal percent-encoding for one query value (track names carry spaces and Persian letters).
fn enc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// One "[mm:ss.xx]" timestamp, in milliseconds from the start of the song.
fn stamp_ms(tag: &str) -> Option<u64> {
    let digits: String = tag.chars().filter(|c| c.is_ascii_digit() || *c == ':' || *c == '.').collect();
    if digits.len() < tag.len() {
        return None; // a metadata tag like [ar:] or [offset:+500] — digits and colons only, please
    }
    let (mm, rest) = digits.split_once(':')?;
    let mm: u64 = mm.parse().ok()?;
    let mut parts = rest.split('.');
    let ss: u64 = parts.next().unwrap_or("0").parse().ok()?;
    let frac = parts.next().unwrap_or("0");
    let ms = match frac.len() {
        0 => 0,
        1 => frac.parse::<u64>().ok()? * 100,
        2 => frac.parse::<u64>().ok()? * 10,
        _ => frac.parse::<u64>().ok()?,
    };
    Some(mm * 60_000 + ss * 1000 + ms)
}
/// Turns LRC text into (time, line) pairs, sorted; metadata lines are dropped, an [offset:] is applied.
fn parse_lrc(text: &str) -> Vec<(u64, String)> {
    let mut off: i64 = 0;
    let mut out: Vec<(u64, String)> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if !line.starts_with('[') {
            continue;
        }
        let mut times: Vec<u64> = Vec::new();
        let mut rest = line;
        let mut bad = false;
        while rest.starts_with('[') {
            let Some(end) = rest.find(']') else {
                bad = true;
                break;
            };
            let tag = &rest[1..end];
            if let Some(ms) = stamp_ms(tag) {
                times.push(ms);
            } else if tag.to_ascii_lowercase().starts_with("offset:") {
                off = tag[7..].trim().parse::<i64>().unwrap_or(0); // the LRC offset tag is in milliseconds
            } else {
                bad = true; // [ar: Songwriter] and friends say nothing about time
            }
            rest = &rest[end + 1..];
        }
        if bad || times.is_empty() {
            continue;
        }
        let text = rest.trim();
        if text.is_empty() {
            continue;
        }
        for t in times {
            out.push(((t as i64 + off).max(0) as u64, text.to_string()));
        }
    }
    out.sort_by_key(|(t, _)| *t);
    out.dedup_by_key(|(t, _)| *t);
    out
}

/// The best of LRCLIB's answers: synced words when any exist, else the plain text, closest duration first.
fn best_pick(list: &[Value], dur_ms: f64) -> Option<(bool, Value)> {
    let mut scored: Vec<(i64, usize)> = (0..list.len())
        .map(|i| {
            let d = list[i]["duration"].as_f64().unwrap_or(0.0);
            let near = if d <= 0.0 || dur_ms <= 0.0 { 0 } else { ((d * 1000.0 - dur_ms).abs() / 1000.0).round() as i64 };
            (near, i)
        })
        .collect();
    scored.sort();
    for (_, i) in &scored {
        if !list[*i]["syncedLyrics"].as_str().unwrap_or("").is_empty() {
            return Some((true, list[*i]["syncedLyrics"].as_str().unwrap().into()));
        }
    }
    for (_, i) in &scored {
        if !list[*i]["plainLyrics"].as_str().unwrap_or("").is_empty() {
            return Some((false, list[*i]["plainLyrics"].as_str().unwrap().into()));
        }
    }
    None
}

/// Called whenever the track changes (and only then). Fetches on a side thread so the media loop never waits.
pub fn on_track(title: &str, artist: &str, dur_ms: f64, key: &str) {
    if title.trim().is_empty() {
        return;
    }
    {
        let mut last = LAST.lock().unwrap();
        if last.0 == key {
            return;
        }
        last.0 = key.to_string();
        last.1 = false;
    }
    let (title, artist, key) = (title.to_string(), artist.to_string(), key.to_string());
    std::thread::spawn(move || {
        let cache = dir().join(format!("{}.json", fnv(&key)));
        let payload = if let Ok(saved) = std::fs::read_to_string(&cache) {
            serde_json::from_str::<Value>(&saved).ok().filter(|v| v["asked"] == true)
        } else {
            None
        };
        let Some(payload) = payload else {
            let url = format!("https://lrclib.net/api/search?track_name={}&artist_name={}", enc(title.trim()), enc(artist.trim()));
            let list: Vec<Value> = get(&url, "search.json")
                .and_then(|t| serde_json::from_str::<Value>(&t).ok())
                .and_then(|v| v.as_array().cloned())
                .unwrap_or_default();
            let Some((synced, text)) = best_pick(&list, dur_ms) else {
                sys::helper(json!({ "t": "lyrics", "key": key, "synced": false, "lines": [], "plain": "" }));
                return;
            };
            let payload = if synced {
                json!({ "asked": true, "key": key, "synced": true, "lines": parse_lrc(text.as_str().unwrap_or("")), "plain": "" })
            } else {
                json!({ "asked": true, "key": key, "synced": false, "lines": [], "plain": text.as_str().unwrap_or("") })
            };
            let _ = std::fs::write(&cache, payload.to_string());
            sys::helper(payload);
            return;
        };
        sys::helper(payload);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamps_parse() {
        assert_eq!(stamp_ms("03:25.32"), Some(205_320));
        assert_eq!(stamp_ms("00:12.5"), Some(12_500));
        assert_eq!(stamp_ms("12:00"), Some(720_000));
        assert_eq!(stamp_ms("ar:Singer"), None);
    }

    #[test]
    fn lrc_becomes_sorted_lines() {
        let lrc = "[ti:Test]\n[03:25.32] last line\n[00:10.00] first\n[00:10.00] first again\n[00:12] second\n[offset:-100]\n[00:05.00] early";
        let lines = parse_lrc(lrc);
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], (4_900, "early".into())); // offset shifts it back a second
        assert_eq!(lines[1], (10_000, "first".into()));
        assert_eq!(lines[3], (205_320, "last line".into()));
    }
}
