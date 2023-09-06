use std::time::Duration;

use colored::{Colorize, ColoredString};

pub fn format_duration(duration: &Duration) -> ColoredString {
    let mins = (duration.as_secs_f32() / 60f32) as u32;
    let secs = duration.as_secs() % 60;
    let millis = duration.as_millis() % 1000;
    
    format!("[{:02}:{:02}.{:03}]", mins, secs, millis)
        .bright_blue().bold()
}