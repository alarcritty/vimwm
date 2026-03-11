use std::io::{self, Result};
use std::process::Command;
use std::thread;
use std::time::Duration;

use super::pid;

pub fn start_daemon() -> Result<()> {
    check_dependency("yabai")?;
    check_dependency("skhd")?;

    let home = crate::config::home_dir();
    let yabai_config = home.join(".config/yabai/yabairc");
    let skhd_config = home.join(".config/skhd/skhdrc");

    let yabai = Command::new("yabai")
        .args(["-c", &yabai_config.to_string_lossy()])
        .spawn()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to start yabai: {e}")))?;

    // Brief pause to let yabai initialize before skhd
    thread::sleep(Duration::from_millis(500));

    let skhd = Command::new("skhd")
        .args(["-c", &skhd_config.to_string_lossy()])
        .spawn()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to start skhd: {e}")))?;

    pid::write_pid(yabai.id(), skhd.id())?;
    Ok(())
}

pub fn stop_daemon() -> Result<()> {
    // Try graceful stop via yabai message first
    let _ = Command::new("yabai").args(["--stop-service"]).output();
    let _ = Command::new("skhd").args(["--stop-service"]).output();

    if let Some((yabai_pid, skhd_pid)) = pid::read_pid() {
        let _ = kill_process(skhd_pid);
        let _ = kill_process(yabai_pid);
    }

    // Fallback: kill by name
    let _ = Command::new("pkill").arg("-x").arg("skhd").output();
    let _ = Command::new("pkill").arg("-x").arg("yabai").output();

    pid::remove_pid();
    Ok(())
}

pub fn reload_daemon() -> Result<()> {
    Command::new("yabai")
        .args(["--restart-service"])
        .output()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to reload yabai: {e}")))?;

    Command::new("skhd")
        .args(["--reload"])
        .output()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to reload skhd: {e}")))?;

    Ok(())
}
fn check_dependency(name: &str) -> Result<()> {
    Command::new("which")
        .arg(name)
        .output()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{e}")))?
        .status
        .success()
        .then_some(())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("{name} not found. install with: brew install koekeishiya/formulae/{name}"),
            )
        })
}

fn kill_process(p: u32) -> Result<()> {
    Command::new("kill")
        .arg(p.to_string())
        .output()
        .map(|_| ())
}
