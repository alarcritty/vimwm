use std::fs;
use std::io::{self, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::config;

const HELPER_NAMES: &[&str] = &["key", "cursor", "click", "scroll", "mouseblock"];

fn helpers_dir() -> PathBuf {
    config::home_dir().join(".config").join("vimwm")
}

fn sources_dir() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_default();
    let exe_dir = exe.parent().unwrap_or(std::path::Path::new("/usr/local/bin"));

    // brew install: helpers are in ../share/vimwm/helpers/
    let share_dir = exe_dir.join("../share/vimwm/helpers");
    if share_dir.exists() {
        return share_dir;
    }

    // dev: helpers are in the project repo
    let project_dir = exe_dir
        .join("../../helpers")
        .canonicalize()
        .unwrap_or_default();
    if project_dir.exists() {
        return project_dir;
    }

    // fallback: check next to config
    helpers_dir()
}

pub fn ensure_helpers() -> Result<()> {
    let dest = helpers_dir();
    fs::create_dir_all(&dest)?;

    let missing: Vec<&&str> = HELPER_NAMES
        .iter()
        .filter(|name| !dest.join(name).exists())
        .collect();

    if missing.is_empty() {
        return Ok(());
    }

    let src = sources_dir();

    for name in &missing {
        let swift_file = src.join(format!("{name}.swift"));
        let output = dest.join(name);

        if swift_file.exists() {
            compile_swift(&swift_file, &output)?;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "swift source '{name}.swift' not found in {}. reinstall vimwm or place helpers manually.",
                    src.display()
                ),
            ));
        }
    }

    Ok(())
}

fn compile_swift(source: &PathBuf, output: &PathBuf) -> Result<()> {
    let name = output
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    let result = Command::new("swiftc")
        .args(["-O", "-o"])
        .arg(output)
        .arg(source)
        .output()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("failed to run swiftc for {name}: {e}"),
            )
        })?;

    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("failed to compile {name}: {stderr}"),
        ));
    }

    println!("compiled helper: {name}");
    Ok(())
}
