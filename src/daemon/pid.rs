use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

fn pid_path() -> PathBuf {
    let dir = dirs::runtime_dir()
        .or_else(|| dirs::cache_dir())
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    dir.join("vimwm.pid")
}

pub fn write_pid(yabai_pid: u32, skhd_pid: u32) -> Result<()> {
    fs::write(pid_path(), format!("{yabai_pid}\n{skhd_pid}"))
}

pub fn read_pid() -> Option<(u32, u32)> {
    let content = fs::read_to_string(pid_path()).ok()?;
    let mut lines = content.lines();
    let yabai = lines.next()?.parse().ok()?;
    let skhd = lines.next()?.parse().ok()?;
    Some((yabai, skhd))
}

pub fn remove_pid() {
    let _ = fs::remove_file(pid_path());
}

pub fn is_running() -> bool {
    // Check if yabai and skhd processes are actually alive
    if let Some((yabai_pid, skhd_pid)) = read_pid() {
        process_alive(yabai_pid) && process_alive(skhd_pid)
    } else {
        // Fallback: check if yabai is running at all
        Command::new("pgrep")
            .arg("-x")
            .arg("yabai")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

fn process_alive(pid: u32) -> bool {
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
