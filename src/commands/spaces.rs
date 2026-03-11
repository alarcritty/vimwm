use std::io::{self, Result};
use std::process::Command;

pub fn list_spaces() -> Result<()> {
    let output = Command::new("yabai")
        .args(["-m", "query", "--spaces"])
        .output()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("yabai not available: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("connection refused") || stderr.contains("socket") {
            println!("vimwm is not running. start it with: vimwm start");
            return Ok(());
        }
        return Err(io::Error::new(io::ErrorKind::Other, format!("yabai error: {stderr}")));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut current_space = 0u32;
    let mut space_type = String::new();
    let mut has_focus = false;
    let mut window_count = 0u32;
    let mut in_space = false;

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with('{') {
            in_space = true;
            current_space = 0;
            space_type.clear();
            has_focus = false;
            window_count = 0;
        }
        if in_space {
            if let Some(idx) = extract_json_u32(line, "\"index\"") {
                current_space = idx;
            }
            if let Some(t) = extract_json_str(line, "\"type\"") {
                space_type = t;
            }
            if let Some(f) = extract_json_bool(line, "\"has-focus\"") {
                has_focus = f;
            }
            if line.contains("\"windows\"") && line.contains('[') {
                let count = line.matches(',').count();
                if line.contains("[]") || line.contains("[ ]") {
                    window_count = 0;
                } else {
                    window_count = count as u32 + 1;
                }
            }
        }
        if line.starts_with('}') && in_space {
            let marker = if has_focus { " *" } else { "" };
            println!(
                "  [{current_space}] {space_type:<6} {window_count} window(s){marker}"
            );
            in_space = false;
        }
    }

    Ok(())
}

fn extract_json_u32(line: &str, key: &str) -> Option<u32> {
    let idx = line.find(key)?;
    let rest = &line[idx + key.len()..];
    let rest = rest.trim_start_matches([' ', ':', '\t']);
    let num: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    num.parse().ok()
}

fn extract_json_str(line: &str, key: &str) -> Option<String> {
    let idx = line.find(key)?;
    let rest = &line[idx + key.len()..];
    let rest = rest.trim_start_matches([' ', ':', '\t']);
    let rest = rest.trim_start_matches('"');
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn extract_json_bool(line: &str, key: &str) -> Option<bool> {
    let idx = line.find(key)?;
    let rest = &line[idx + key.len()..];
    let rest = rest.trim_start_matches([' ', ':', '\t']);
    if rest.starts_with("true") {
        Some(true)
    } else if rest.starts_with("false") {
        Some(false)
    } else {
        None
    }
}
