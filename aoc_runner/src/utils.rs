use std::time::Duration;

use colored::{Colorize, ColoredString};

pub fn format_duration(duration: &Duration) -> ColoredString {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let mins = (duration.as_secs_f32() / 60f32) as u32;
    let secs = duration.as_secs() % 60;
    let millis = duration.as_millis() % 1000;
    
    format!("[{mins:02}:{secs:02}.{millis:03}]")
        .bright_blue().bold()
}