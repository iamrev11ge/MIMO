// A slow look at the machine itself: how busy the CPU is, how much memory is used, and how fast the
// network is moving right now. The sysinfo crate (only its system and network parts) does the work;
// numbers are sent to the page every couple of seconds while the user has the feature on.
use std::sync::Mutex;
use std::time::Instant;

use serde_json::json;
use sysinfo::{Networks, System};

use crate::{settings, sys};

struct Watch {
    sys: System,
    nets: Networks,
    up: f64,
    down: f64,
    at: Instant,
}
static WATCH: Mutex<Option<Watch>> = Mutex::new(None);

/// Reads the counters once and tells the page. Cheap enough to run every couple of seconds.
pub fn poll() {
    if !settings::flag("statsOn") {
        return;
    }
    let mut guard = WATCH.lock().unwrap();
    let w = guard.get_or_insert_with(|| {
        let mut sys = System::new();
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        Watch { sys, nets: Networks::new_with_refreshed_list(), up: 0.0, down: 0.0, at: Instant::now() }
    });
    let elapsed = w.at.elapsed().as_secs_f64().max(0.2);
    w.at = Instant::now();
    w.sys.refresh_cpu_usage();
    w.sys.refresh_memory();
    w.nets.refresh(true);
    let (mut rx, mut tx) = (0u64, 0u64);
    for (_name, data) in &w.nets {
        rx += data.received(); // bytes since the previous refresh
        tx += data.transmitted();
    }
    w.down = rx as f64 / elapsed;
    w.up = tx as f64 / elapsed;
    sys::helper(json!({
        "t": "stats",
        "cpu": w.sys.global_cpu_usage().round() as i64,
        "ramUsed": w.sys.used_memory(),
        "ramTotal": w.sys.total_memory(),
        "down": w.down.round() as i64,
        "up": w.up.round() as i64,
    }));
}
