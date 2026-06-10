use core_graphics::event::CGEvent;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
struct ShakePayload {
    x: f64,
    y: f64,
}

#[derive(Clone, serde::Serialize)]
struct CursorPayload {
    x: f64,
    y: f64,
}

pub fn get_mouse_position() -> (f64, f64) {
    if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
        if let Ok(event) = CGEvent::new(source) {
            let loc = event.location();
            return (loc.x, loc.y);
        }
    }
    (0.0, 0.0)
}

pub fn start_tracking(app: AppHandle) {
    std::thread::spawn(move || {
        let mut history: Vec<(f64, f64, Instant)> = Vec::new();
        let mut last_shake = Instant::now() - Duration::from_secs(10);
        let shake_cooldown = Duration::from_millis(1000);
        
        loop {
            let (x, y) = get_mouse_position();
            let now = Instant::now();
            
            // Broadcast cursor position for UI to follow
            let _ = app.emit("cursor-moved", CursorPayload { x, y });
            
            history.push((x, y, now));
            
            // Keep only last 500ms
            history.retain(|&(_, _, t)| now.duration_since(t) < Duration::from_millis(500));
            
            if now.duration_since(last_shake) > shake_cooldown && history.len() > 10 {
                // Detect shake
                let mut direction_changes = 0;
                let mut last_dx_sign = 0;
                let mut min_x = x;
                let mut max_x = x;
                
                for i in 1..history.len() {
                    let prev = history[i-1];
                    let curr = history[i];
                    
                    if curr.0 < min_x { min_x = curr.0; }
                    if curr.0 > max_x { max_x = curr.0; }
                    
                    let dx = curr.0 - prev.0;
                    if dx.abs() > 5.0 { // Small threshold to ignore noise
                        let sign = if dx > 0.0 { 1 } else { -1 };
                        if last_dx_sign != 0 && sign != last_dx_sign {
                            direction_changes += 1;
                        }
                        last_dx_sign = sign;
                    }
                }
                
                // Shake condition: at least 4 direction changes and significant travel
                if direction_changes >= 4 && (max_x - min_x) > 150.0 {
                    let _ = app.emit("shake-detected", ShakePayload { x, y });
                    last_shake = now;
                    history.clear();
                }
            }
            
            std::thread::sleep(Duration::from_millis(16));
        }
    });
}
