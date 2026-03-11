use crate::config::{load_config, save_config};
use crate::daemon;
use crate::generators;
use std::io::{self, Result};
use std::process::Command;

pub fn set_layout(mode: &str) -> Result<()> {
    match mode {
        "bsp" | "stack" | "float" => {}
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid layout: {mode}. use: bsp, stack, float"),
            ));
        }
    }

    let mut cfg = load_config()?;
    cfg.layout = mode.to_string();
    save_config(&cfg)?;
    println!("layout: {mode}");

    if daemon::is_running() {
        generators::generate_yabairc(&cfg)?;
        // Apply immediately
        Command::new("yabai")
            .args(["-m", "space", "--layout", mode])
            .output()?;
    }
    Ok(())
}
