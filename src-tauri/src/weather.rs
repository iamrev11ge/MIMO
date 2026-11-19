// Weather from Open-Meteo: no key, no account, no tracking. The city name is looked up once, when the
// user picks it, and only the coordinates leave the machine afterwards — every fifteen minutes, and
// only to ask what the sky looks like there. The last report lives in memory; nothing is written down.
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde_json::{json, Value};
use windows::core::HSTRING;
use windows::Win32::System::Com::Urlmon::URLDownloadToFileW;

use crate::{settings, sys};

static REFRESH: AtomicBool = AtomicBool::new(true); // set when the city changed: fetch right away

fn dir() -> std::path::PathBuf {
    let d = std::env::temp_dir().join("MIMOWeather");
    let _ = std::fs::create_dir_all(&d);
    d
}
/// Downloads a small JSON document to a temp file and reads it back — the same way the updater fetches.
fn get(url: &str, name: &str) -> Option<String> {
    let to = dir().join(name);
    let _ = std::fs::remove_file(&to);
    let full = format!("{url}&t={}", sys::now_ms()); // the extra query keeps Windows from serving a cached copy
    unsafe { URLDownloadToFileW(None, &HSTRING::from(full), &HSTRING::from(to.to_string_lossy().as_ref()), 0, None).ok()? };
    std::fs::read_to_string(&to).ok()
}
/// Minimal percent-encoding for one query value (city names are often Persian).
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

/// Turns a city name into coordinates and remembers them. Answered to the page: { ok, name } or { ok: false, why }.
pub fn set_city(raw: &str) -> Value {
    let name: String = raw.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(60).collect();
    if name.is_empty() {
        return json!({ "ok": false, "why": "empty" });
    }
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json", enc(&name));
    let Some(text) = get(&url, "city.json") else {
        return json!({ "ok": false, "why": "could not reach the geocoding service" });
    };
    let Ok(v) = serde_json::from_str::<Value>(&text) else {
        return json!({ "ok": false, "why": "bad answer" });
    };
    let Some(hit) = v["results"].as_array().and_then(|a| a.first()) else {
        return json!({ "ok": false, "why": "not found" });
    };
    let (Some(lat), Some(lon)) = (hit["latitude"].as_f64(), hit["longitude"].as_f64()) else {
        return json!({ "ok": false, "why": "not found" });
    };
    let full = [hit["name"].as_str(), hit["country"].as_str()].iter().flatten().cloned().collect::<Vec<_>>().join(", ");
    settings::put("weatherName", full.clone().into());
    settings::put("weatherLat", lat.into());
    settings::put("weatherLon", lon.into());
    settings::broadcast();
    REFRESH.store(true, Ordering::SeqCst);
    json!({ "ok": true, "name": full })
}

/// One report: the current temperature and WMO weather code, plus the next three days. Off the air:
/// nothing is sent, the page keeps whatever it had.
fn poll_once() {
    let (lat, lon) = match (settings::get("weatherLat").as_f64(), settings::get("weatherLon").as_f64()) {
        (Some(a), Some(b)) if a != 0.0 || b != 0.0 => (a, b),
        _ => return, // no city picked yet
    };
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,weather_code&daily=weather_code,temperature_2m_max,temperature_2m_min&timezone=auto&forecast_days=4");
    let Some(text) = get(&url, "report.json") else { return };
    let Ok(v) = serde_json::from_str::<Value>(&text) else { return };
    let d = v["daily"].as_object();
    let pick = |k: &str, i: usize| d.and_then(|d| d.get(k)).and_then(|a| a.as_array()).and_then(|a| a.get(i));
    let days: Vec<Value> = (0..4)
        .filter_map(|i| {
            let date = pick("time", i)?.as_str()?.to_string();
            Some(json!({
                "date": date,
                "code": pick("weather_code", i).and_then(Value::as_i64).unwrap_or(0),
                "hi": pick("temperature_2m_max", i).and_then(Value::as_f64).map(|x| x.round()).unwrap_or(0.0),
                "lo": pick("temperature_2m_min", i).and_then(Value::as_f64).map(|x| x.round()).unwrap_or(0.0),
            }))
        })
        .collect();
    sys::helper(json!({
        "t": "weather",
        "name": settings::get("weatherName").as_str().unwrap_or(""),
        "temp": v["current"]["temperature_2m"].as_f64().map(|x| x.round()).unwrap_or(0.0),
        "code": v["current"]["weather_code"].as_i64().unwrap_or(0),
        "days": days,
    }));
}

pub fn start() {
    std::thread::spawn(|| {
        let mut next = 0u64;
        loop {
            std::thread::sleep(Duration::from_secs(5));
            let now = sys::now_ms();
            let due = REFRESH.swap(false, Ordering::SeqCst) || now >= next;
            if due && settings::flag("weatherOn") {
                poll_once();
                next = now + 15 * 60_000;
            }
        }
    });
}
