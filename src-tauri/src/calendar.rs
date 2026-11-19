// The calendar: .ics files the user exported (one file, or a folder of them) say what is coming up
// today and tomorrow. Fixed-time and all-day events are read; DAILY / WEEKLY / MONTHLY / YEARLY
// repeats (with INTERVAL, COUNT and UNTIL) are expanded far enough to land in that window. Everything
// stays on the machine — nothing about the calendar ever leaves it.
use std::sync::Mutex;

use serde_json::json;
use windows::Win32::System::SystemInformation::{GetLocalTime, GetSystemTime};

use crate::{settings, sys};

static LAST: Mutex<Option<String>> = Mutex::new(None); // what the page was last told, as compact text
static TOLD: Mutex<Vec<String>> = Mutex::new(Vec::new()); // events the quarter-hour reminder already fired for

// ---------------------------------------------------------------- dates, without a calendar library
/// Days since 1970-01-01 (Howard Hinnant's days_from_civil).
fn days_from_civil(y: i64, m: u64, d: u64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}
fn civil_from_days(z: i64) -> (i64, u64, u64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u64;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u64;
    (if m <= 2 { y + 1 } else { y }, m, d)
}
fn month_len(y: i64, m: u64) -> u64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) => 29,
        _ => 28,
    }
}
/// How far local time is from UTC right now (ms) — enough to place Zulu times without a timezone database.
fn utc_offset_ms() -> i64 {
    unsafe {
        let ms = |y, m, d, h, mi, s, mil: u16| days_from_civil(y as i64, m as u64, d as u64) * 86_400_000 + h as i64 * 3_600_000 + mi as i64 * 60_000 + s as i64 * 1000 + mil as i64;
        let l = GetLocalTime();
        let u = GetSystemTime();
        ms(l.wYear, l.wMonth, l.wDay, l.wHour, l.wMinute, l.wSecond, l.wMilliseconds)
            - ms(u.wYear, u.wMonth, u.wDay, u.wHour, u.wMinute, u.wSecond, u.wMilliseconds)
    }
}

/// A moment in local time: a day number and seconds into it (sec < 0 marks an all-day date).
#[derive(Clone, Copy, PartialEq)]
struct When {
    day: i64,
    sec: i32,
}
fn today() -> When {
    unsafe {
        let t = GetLocalTime();
        When { day: days_from_civil(t.wYear as i64, t.wMonth as u64, t.wDay as u64), sec: t.wHour as i32 * 3600 + t.wMinute as i32 * 60 + t.wSecond as i32 }
    }
}
fn hhmm(sec: i32) -> String {
    format!("{:02}:{:02}", sec / 3600, sec % 3600 / 60)
}
/// Parses an iCalendar date or date-time: "20261119", "20261119T093000", and the Zulu form shifted to local.
fn parse_when(s: &str, offset_ms: i64) -> Option<When> {
    let s = s.trim();
    let (body, zulu) = match s.strip_suffix('Z') {
        Some(b) => (b, true),
        None => (s, false),
    };
    if body.len() == 8 && body.bytes().all(|b| b.is_ascii_digit()) {
        return Some(When { day: days_from_civil(body[0..4].parse().ok()?, body[4..6].parse().ok()?, body[6..8].parse().ok()?), sec: -1 });
    }
    if body.len() != 15 || body.as_bytes()[8] != b'T' || !body.bytes().enumerate().all(|(i, b)| i == 8 || b.is_ascii_digit()) {
        return None;
    }
    let day = days_from_civil(body[0..4].parse().ok()?, body[4..6].parse().ok()?, body[6..8].parse().ok()?);
    let sec = body[9..11].parse::<i64>().ok()? * 3600 + body[11..13].parse::<i64>().ok()? * 60 + body[13..15].parse::<i64>().ok()?;
    if zulu {
        let local = day * 86_400 + sec + offset_ms / 1000;
        return Some(When { day: local.div_euclid(86_400), sec: local.rem_euclid(86_400) as i32 });
    }
    Some(When { day, sec: sec as i32 })
}

// ---------------------------------------------------------------- the file
struct Event {
    title: String,
    start: When,
    end_sec: i64, // how long it runs (an unknown end is one hour)
    freq: u8,     // 0 once, 1 daily, 2 weekly, 3 monthly, 4 yearly
    interval: i64,
    count: i64, // including the first occurrence (0 = endless)
    until: Option<When>,
}
/// Joins the folded continuation lines (a line starting with a space belongs to the one above it).
fn unfold(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for raw in text.lines() {
        if (raw.starts_with(' ') || raw.starts_with('\t')) && !out.is_empty() {
            out.last_mut().unwrap().push_str(raw[1..].trim_end());
        } else {
            out.push(raw.trim_end().to_string());
        }
    }
    out
}
fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('n') | Some('N') => out.push(' '),
                Some(o) => out.push(o),
                None => {}
            },
            _ => out.push(c),
        }
    }
    out.trim().to_string()
}
fn parse_ics(text: &str, offset_ms: i64) -> Vec<Event> {
    let mut out = Vec::new();
    let mut cur: Option<Event> = None;
    for line in unfold(text) {
        let Some((head, value)) = line.split_once(':') else { continue };
        let (name, _params) = head.split_once(';').unwrap_or((head, ""));
        let name = name.trim();
        if name.eq_ignore_ascii_case("BEGIN") && value.trim().eq_ignore_ascii_case("VEVENT") {
            cur = Some(Event { title: String::new(), start: When { day: 0, sec: 0 }, end_sec: 3600, freq: 0, interval: 1, count: 0, until: None });
            continue;
        }
        if name.eq_ignore_ascii_case("END") && value.trim().eq_ignore_ascii_case("VEVENT") {
            if let Some(e) = cur.take() {
                if e.start.day != 0 && !e.title.is_empty() {
                    out.push(e);
                }
            }
            continue;
        }
        let Some(e) = cur.as_mut() else { continue };
        if name.eq_ignore_ascii_case("SUMMARY") {
            e.title = unescape(value);
        } else if name.eq_ignore_ascii_case("DTSTART") {
            if let Some(w) = parse_when(value, offset_ms) {
                e.start = w;
            }
        } else if name.eq_ignore_ascii_case("DTEND") {
            if let Some(w) = parse_when(value, offset_ms) {
                e.end_sec = ((w.day - e.start.day) * 86_400 + w.sec as i64 - e.start.sec.max(0) as i64).max(1);
            }
        } else if name.eq_ignore_ascii_case("RRULE") {
            for part in value.split(';') {
                let (k, v) = part.split_once('=').unwrap_or(("", ""));
                if k.eq_ignore_ascii_case("FREQ") {
                    e.freq = match v.trim().to_ascii_uppercase().as_str() {
                        "DAILY" => 1,
                        "WEEKLY" => 2,
                        "MONTHLY" => 3,
                        "YEARLY" => 4,
                        _ => 0,
                    };
                } else if k.eq_ignore_ascii_case("INTERVAL") {
                    e.interval = v.trim().parse().unwrap_or(1).max(1);
                } else if k.eq_ignore_ascii_case("COUNT") {
                    e.count = v.trim().parse().unwrap_or(0);
                } else if k.eq_ignore_ascii_case("UNTIL") {
                    e.until = parse_when(v.trim(), offset_ms);
                }
            }
        }
    }
    out
}

/// The next occurrence after w (None when the event does not repeat). A month that is too short
/// carries its day clamped to its end (the 31st lands on the 28th of February), which is close enough
/// for a list of the next two days.
fn step(e: &Event, w: When) -> Option<When> {
    match e.freq {
        0 => None,
        1 => Some(When { day: w.day + e.interval, ..w }),
        2 => Some(When { day: w.day + 7 * e.interval, ..w }),
        3 => {
            let (mut y, mut m, d) = civil_from_days(w.day);
            let t = m as i64 - 1 + e.interval;
            y += t.div_euclid(12);
            m = t.rem_euclid(12) as u64 + 1;
            Some(When { day: days_from_civil(y, m, d.min(month_len(y, m))), ..w })
        }
        4 => {
            let (mut y, m, d) = civil_from_days(w.day);
            y += e.interval;
            Some(When { day: days_from_civil(y, m, d.min(month_len(y, m))), ..w })
        }
        _ => None,
    }
}
/// Every occurrence of the event that touches [from_day, to_day], walked from its start; the step cap
/// keeps even a decade of daily repeats cheap.
fn occurrences(e: &Event, from_day: i64, to_day: i64) -> Vec<When> {
    let mut hits = Vec::new();
    let mut w = e.start;
    let mut made = 0i64;
    for _ in 0..100_000 {
        let span = if w.sec < 0 { 86_400 } else { w.sec as i64 + e.end_sec };
        let (occ_start, occ_end) = (w.day * 86_400 + w.sec.max(0) as i64, w.day * 86_400 + span);
        if occ_start <= to_day * 86_400 + 86_399 && occ_end > from_day * 86_400 {
            hits.push(w);
        }
        made += 1;
        if e.count > 0 && made >= e.count {
            break;
        }
        let Some(n) = step(e, w) else { break };
        if let Some(u) = e.until {
            let (nu, uu) = (n.day * 86_400 + n.sec.max(0) as i64, u.day * 86_400 + u.sec.max(0) as i64);
            if nu > uu {
                break;
            }
        }
        if n.day > to_day {
            break;
        }
        w = n;
    }
    hits
}

// ---------------------------------------------------------------- the poll
/// Reads the calendar (a .ics file, or every .ics in a folder), tells the page what today and tomorrow
/// hold, and says one line about anything starting within a quarter hour. Called about once a minute.
pub fn poll() {
    let path = settings::get("calendarFile").as_str().unwrap_or("").to_string();
    let offset = utc_offset_ms();
    let mut events: Vec<Event> = Vec::new();
    if !path.is_empty() {
        let files: Vec<std::path::PathBuf> = match std::fs::metadata(&path) {
            Ok(m) if m.is_dir() => std::fs::read_dir(&path)
                .map(|rd| rd.flatten().map(|e| e.path()).filter(|p| p.extension().map(|x| x.eq_ignore_ascii_case("ics")).unwrap_or(false)).collect())
                .unwrap_or_default(),
            Ok(_) if path.to_ascii_lowercase().ends_with(".ics") => vec![std::path::PathBuf::from(&path)],
            _ => Vec::new(),
        };
        for f in files {
            if let Ok(text) = std::fs::read_to_string(&f) {
                events.append(&mut parse_ics(&text, offset));
            }
        }
    }
    let t = today();
    let mut lists: [Vec<(When, String)>; 2] = [Vec::new(), Vec::new()];
    let mut flat = String::new();
    for e in &events {
        for w in occurrences(e, t.day, t.day + 1) {
            let off = (w.day - t.day) as usize;
            if off > 1 {
                continue;
            }
            flat.push_str(&format!("{off}|{}|{};", w.sec, e.title));
            lists[off].push((w, e.title.clone()));
        }
    }
    for l in &mut lists {
        l.sort_by_key(|(w, _)| w.sec);
    }
    // one quiet line per event that starts within the next quarter hour — once per run of the app
    let now = t.day as i64 * 86_400 + t.sec as i64;
    for l in &lists {
        for (w, title) in l {
            if w.sec < 0 {
                continue;
            }
            let start = w.day as i64 * 86_400 + w.sec as i64;
            if start >= now && start - now <= 15 * 60 {
                let (wd, ws) = (w.day, w.sec);
                let key = format!("{wd}|{ws}|{title}");
                let mut told = TOLD.lock().unwrap();
                if !told.contains(&key) {
                    told.push(key);
                    sys::helper(json!({ "t": "eventSoon", "title": title, "time": hhmm(w.sec) }));
                }
            }
        }
    }
    // the list itself only travels when it changed
    {
        let mut last = LAST.lock().unwrap();
        if last.as_deref() == Some(&flat) {
            return;
        }
        *last = Some(flat);
    }
    sys::helper(json!({
        "t": "events",
        "days": (0..2).map(|i| json!({
            "label": if i == 0 { "Today" } else { "Tomorrow" },
            "items": lists[i].iter().map(|(w, title)| json!({
                "time": if w.sec < 0 { String::new() } else { hhmm(w.sec) },
                "allDay": w.sec < 0,
                "title": title,
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    }));
}
/// The user pointed at a new file or folder: say the new list even when it happens to look like the old one.
pub fn poke() {
    *LAST.lock().unwrap() = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_dates_round_trip() {
        for &(y, m, d) in &[(1970, 1, 1), (2000, 2, 29), (2026, 11, 19), (2100, 3, 1)] {
            assert_eq!(civil_from_days(days_from_civil(y, m, d)), (y, m, d));
        }
        assert_eq!(days_from_civil(1970, 1, 2), 1);
    }

    #[test]
    fn dates_and_times_parse() {
        let w = parse_when("20261119", 0).unwrap();
        assert_eq!((w.day, w.sec), (days_from_civil(2026, 11, 19), -1));
        let w = parse_when("20261119T093000", 0).unwrap();
        assert_eq!(w.sec, 9 * 3600 + 30 * 60);
        let w = parse_when("20261119T093000Z", 3 * 3600 * 1000).unwrap();
        assert_eq!((w.day, w.sec as i64), (days_from_civil(2026, 11, 19), 12 * 3600 + 30 * 60));
        assert!(parse_when("garbage", 0).is_none());
    }

    #[test]
    fn events_parse_with_rules() {
        let ics = "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:Standup with Ali\r\nDTSTART:20261119T093000\r\nDTEND:20261119T094500\r\nEND:VEVENT\r\nBEGIN:VEVENT\r\nSUMMARY:Gym\r\nDTSTART;VALUE=DATE:20261120\r\nRRULE:FREQ=WEEKLY;INTERVAL=2\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n";
        let es = parse_ics(ics, 0);
        assert_eq!(es.len(), 2);
        assert_eq!(es[0].title, "Standup with Ali");
        assert_eq!(es[0].end_sec, 15 * 60); // 09:30 → 09:45
        assert_eq!(es[1].freq, 2);
        assert_eq!(es[1].interval, 2);
        assert_eq!(es[1].start.sec, -1);
        let today = days_from_civil(2026, 11, 19);
        assert_eq!(occurrences(&es[0], today, today + 1).len(), 1);
        assert_eq!(occurrences(&es[1], today, today + 1).len(), 1); // all-day tomorrow
    }

    #[test]
    fn repeats_land_only_on_their_weeks() {
        let today = days_from_civil(2026, 11, 19);
        let mk = |start| Event { title: "x".into(), start: When { day: start, sec: 3600 }, end_sec: 3600, freq: 2, interval: 2, count: 0, until: None };
        assert_eq!(occurrences(&mk(today - 21), today, today + 1).len(), 0); // three weeks ago: odd week
        assert_eq!(occurrences(&mk(today - 14), today, today + 1).len(), 1); // two weeks ago: this week
        let daily = Event { title: "x".into(), start: When { day: today - 40, sec: 3600 }, end_sec: 3600, freq: 1, interval: 1, count: 0, until: Some(When { day: today - 1, sec: -1 }) };
        assert_eq!(occurrences(&daily, today, today + 1).len(), 0); // UNTIL yesterday
        let counted = Event { title: "x".into(), start: When { day: today - 3, sec: 3600 }, end_sec: 3600, freq: 1, interval: 1, count: 2, until: None };
        assert_eq!(occurrences(&counted, today, today + 1).len(), 0); // its two days are gone
    }

    #[test]
    fn monthly_and_yearly_step_through_short_months() {
        let jan31 = days_from_civil(2026, 1, 31);
        let m = Event { title: "x".into(), start: When { day: jan31, sec: 3600 }, end_sec: 3600, freq: 3, interval: 1, count: 0, until: None };
        let hits = occurrences(&m, days_from_civil(2026, 2, 1), days_from_civil(2026, 2, 28));
        assert_eq!(hits.len(), 1); // lands on Feb 28
        let y = Event { title: "x".into(), start: When { day: days_from_civil(2024, 2, 29), sec: 3600 }, end_sec: 3600, freq: 4, interval: 1, count: 0, until: None };
        let hits = occurrences(&y, days_from_civil(2026, 2, 1), days_from_civil(2026, 2, 28));
        assert_eq!(hits.len(), 1); // leap day lands on Feb 28
    }
}
