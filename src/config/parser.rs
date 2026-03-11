use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Result};
use std::path::PathBuf;

use super::defaults;

fn default_true() -> bool { true }
fn default_vim_toggle() -> String { "mod+space".into() }
fn default_vim_speed() -> u32 { 50 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub mod_key: String,
    pub terminal: String,
    pub layout: String,
    pub gap_size: u32,
    pub padding: u32,
    pub focus_follows_mouse: bool,
    pub mouse_follows_focus: bool,
    pub window_opacity: bool,
    pub active_opacity: f64,
    pub inactive_opacity: f64,
    pub border: bool,
    pub border_width: u32,
    pub active_border_color: String,
    pub inactive_border_color: String,
    #[serde(default)]
    pub external_bar: String,
    pub split_ratio: f64,
    #[serde(default = "default_true")]
    pub vim_mode: bool,
    #[serde(default = "default_vim_toggle")]
    pub vim_toggle_key: String,
    #[serde(default = "default_vim_speed")]
    pub vim_cursor_speed: u32,
    pub bindings: BTreeMap<String, String>,
    #[serde(default)]
    pub vim_bindings: BTreeMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        defaults::default_config()
    }
}

pub fn home_dir() -> PathBuf {
    dirs::home_dir().expect("could not determine home directory")
}

pub fn config_dir() -> PathBuf {
    home_dir().join(".config").join("vimwm")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn ensure_config() -> Result<()> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    let path = config_path();
    if !path.exists() {
        let cfg = Config::default();
        save_config(&cfg)?;
    }
    Ok(())
}

pub fn load_config() -> Result<Config> {
    ensure_config()?;
    let path = config_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to read {}: {e}", path.display())))?;
    let cfg: Config = toml::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid config: {e}")))?;
    Ok(cfg)
}

pub fn save_config(cfg: &Config) -> Result<()> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    let content = toml::to_string_pretty(cfg)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to serialize config: {e}")))?;
    fs::write(config_path(), content)
}
