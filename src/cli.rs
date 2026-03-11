use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vimwm", about = "Vim + i3 style window manager for macOS")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the window manager daemon
    Start,
    /// Stop the window manager daemon
    Stop,
    /// Restart the window manager daemon
    Restart,
    /// Show daemon status
    Status,
    /// Reload configuration without restart
    Reload,
    /// Add or update a keybinding
    Bind {
        /// Key combo (e.g. "mod+h")
        key: String,
        /// Action (e.g. "focus west")
        action: String,
    },
    /// Remove a keybinding
    Unbind {
        /// Key combo to remove
        key: String,
    },
    /// Set tiling layout (bsp, stack, float)
    Layout {
        /// Layout mode
        mode: String,
    },
    /// Set window gaps in pixels
    Gaps {
        /// Gap size
        size: u32,
    },
    /// Set window padding in pixels
    Padding {
        /// Padding size
        size: u32,
    },
    /// List active spaces/workspaces
    Spaces,
    /// Load a preset configuration (i3, vim, minimal)
    Preset {
        /// Preset name
        name: String,
    },
    /// Manage configuration (edit, path, reset)
    Config {
        /// Action: edit, path, reset
        action: String,
    },
}
