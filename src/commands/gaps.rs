use crate::config::{load_config, save_config};
use crate::daemon;
use crate::generators;
use std::io::Result;
use std::process::Command;

pub fn set_gaps(size: u32) -> Result<()> {
    let mut cfg = load_config()?;
    cfg.gap_size = size;
    save_config(&cfg)?;
    println!("gaps: {size}px");

    if daemon::is_running() {
        generators::generate_yabairc(&cfg)?;
        Command::new("yabai")
            .args(["-m", "space", "--gap", &format!("abs:{size}")])
            .output()?;
    }
    Ok(())
}

pub fn set_padding(size: u32) -> Result<()> {
    let mut cfg = load_config()?;
    cfg.padding = size;
    save_config(&cfg)?;
    println!("padding: {size}px");

    if daemon::is_running() {
        generators::generate_yabairc(&cfg)?;
        let arg = format!("abs:{size}:{size}:{size}:{size}");
        Command::new("yabai")
            .args(["-m", "space", "--padding", &arg])
            .output()?;
    }
    Ok(())
}
