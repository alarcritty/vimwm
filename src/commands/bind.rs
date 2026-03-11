use crate::config::{load_config, save_config};
use crate::daemon;
use crate::generators;
use std::io::Result;

pub fn bind_key(key: &str, action: &str) -> Result<()> {
    let mut cfg = load_config()?;
    cfg.bindings.insert(key.to_string(), action.to_string());
    save_config(&cfg)?;
    println!("bound: {key} → {action}");

    if daemon::is_running() {
        generators::generate_skhdrc(&cfg)?;
        daemon::reload_daemon()?;
        println!("config reloaded");
    }
    Ok(())
}

pub fn unbind_key(key: &str) -> Result<()> {
    let mut cfg = load_config()?;
    if cfg.bindings.remove(key).is_some() {
        save_config(&cfg)?;
        println!("unbound: {key}");

        if daemon::is_running() {
            generators::generate_skhdrc(&cfg)?;
            daemon::reload_daemon()?;
            println!("config reloaded");
        }
    } else {
        println!("no binding for: {key}");
    }
    Ok(())
}
