mod bind;
mod gaps;
mod layout;
mod spaces;

use crate::config::{self, load_config, save_config, ensure_config, config_path};
use crate::daemon;
use crate::generators;
use std::io::{self, Result};

pub fn start() -> Result<()> {
    ensure_config()?;

    if daemon::is_running() {
        println!("vimwm is already running");
        return Ok(());
    }

    let cfg = load_config()?;
    generators::generate_yabairc(&cfg)?;
    generators::generate_skhdrc(&cfg)?;
    daemon::start_daemon()?;
    println!("vimwm started");
    Ok(())
}

pub fn stop() -> Result<()> {
    if !daemon::is_running() {
        println!("vimwm is not running");
        return Ok(());
    }
    daemon::stop_daemon()?;
    println!("vimwm stopped");
    Ok(())
}

pub fn restart() -> Result<()> {
    if daemon::is_running() {
        daemon::stop_daemon()?;
    }
    let cfg = load_config()?;
    generators::generate_yabairc(&cfg)?;
    generators::generate_skhdrc(&cfg)?;
    daemon::start_daemon()?;
    println!("vimwm restarted");
    Ok(())
}

pub fn status() -> Result<()> {
    if daemon::is_running() {
        let cfg = load_config()?;
        println!("vimwm: running");
        println!("layout: {}", cfg.layout);
        println!("gaps: {}", cfg.gap_size);
        println!("bindings: {}", cfg.bindings.len());
        if let Some((yabai_pid, skhd_pid)) = daemon::read_pid() {
            println!("yabai pid: {yabai_pid}");
            println!("skhd pid: {skhd_pid}");
        }
    } else {
        println!("vimwm: stopped");
    }
    Ok(())
}

pub fn reload() -> Result<()> {
    if !daemon::is_running() {
        return Err(io::Error::new(io::ErrorKind::Other, "vimwm is not running"));
    }
    let cfg = load_config()?;
    generators::generate_yabairc(&cfg)?;
    generators::generate_skhdrc(&cfg)?;
    daemon::reload_daemon()?;
    println!("vimwm reloaded");
    Ok(())
}

pub fn bind(key: &str, action: &str) -> Result<()> {
    bind::bind_key(key, action)
}

pub fn unbind(key: &str) -> Result<()> {
    bind::unbind_key(key)
}

pub fn layout(mode: &str) -> Result<()> {
    layout::set_layout(mode)
}

pub fn gaps(size: u32) -> Result<()> {
    gaps::set_gaps(size)
}

pub fn padding(size: u32) -> Result<()> {
    gaps::set_padding(size)
}

pub fn spaces() -> Result<()> {
    spaces::list_spaces()
}

pub fn preset(name: &str) -> Result<()> {
    let bindings = config::presets::get_preset(name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("unknown preset: {name}. available: i3, vim, minimal")))?;

    let mut cfg = load_config()?;
    cfg.bindings = bindings;
    save_config(&cfg)?;
    println!("preset '{name}' applied");

    if daemon::is_running() {
        reload()?;
    }
    Ok(())
}

pub fn config_cmd(action: &str) -> Result<()> {
    match action {
        "path" => {
            println!("{}", config_path().display());
            Ok(())
        }
        "edit" => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".into());
            let path = config_path();
            ensure_config()?;
            std::process::Command::new(&editor)
                .arg(&path)
                .status()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to open editor '{editor}': {e}")))?;
            Ok(())
        }
        "reset" => {
            let cfg = config::Config::default();
            save_config(&cfg)?;
            println!("config reset to defaults");
            if daemon::is_running() {
                reload()?;
            }
            Ok(())
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("unknown config action: {action}. use: edit, path, reset"))),
    }
}
