# vimwm

## Problem

macOS has no built-in tiling window manager. Managing windows means dragging, resizing, and clicking through everything manually. If you come from i3wm on Linux, the experience is painful. And if you want vim-style keyboard navigation across your entire system, not just inside a terminal, there is nothing that does it out of the box.

Setting up yabai and skhd separately works, but you end up maintaining two config files, wiring up keybindings by hand, and writing shell scripts to glue everything together.

## Solution

vimwm is a single CLI that gives you i3-style tiling and a global vim mode on macOS. One config file, one command to start.

It wraps [yabai](https://github.com/koekeishiya/yabai) and [skhd](https://github.com/koekeishiya/skhd), reads a single TOML config, and generates native configs for both. A toggleable vim mode intercepts bare keys system-wide and translates them into keyboard events, so you can navigate, select, copy, paste, and search without touching the mouse. Mouse input is blocked while vim mode is active.

## Install

```sh
brew tap alarcritty/vimwm
brew install vimwm
```

This installs vimwm along with yabai and skhd as dependencies. Swift helpers are compiled automatically on first `vimwm start`.

### Setup

yabai requires SIP to be partially disabled. Boot into Recovery Mode (hold power button on Apple Silicon, or Cmd+R on Intel), open Terminal, and run:

```sh
csrutil enable --without fs --without debug --without nvram
```

After reboot:

```sh
sudo yabai --load-sa
```

To avoid entering your password on every restart, add a sudoers entry:

```sh
sudo visudo -f /private/etc/sudoers.d/yabai
```

Add this line (replace `<user>` with your username and update the hash):

```
<user> ALL=(root) NOPASSWD: sha256:<hash> /opt/homebrew/bin/yabai --load-sa
```

Get the hash with: `shasum -a 256 $(which yabai)`

Grant accessibility permissions: System Settings > Privacy & Security > Accessibility > enable your terminal app.

### Build from source

```sh
git clone https://github.com/alarcritty/vimwm.git
cd vimwm
cargo build --release
cp target/release/vimwm ~/.cargo/bin/
```

## Usage

```sh
vimwm start       # start the daemon
vimwm stop        # stop it
vimwm restart     # restart with fresh config
vimwm status      # check if running
vimwm reload      # hot-reload config
```

## Window management

All keybindings use `alt` as the modifier (configurable).

| Keys | Action |
|------|--------|
| alt + h/j/k/l | Focus window left/down/up/right |
| alt + shift + h/j/k/l | Move window |
| alt + 1-9 | Switch workspace |
| alt + shift + 1-9 | Move window to workspace |
| alt + enter | Open terminal |
| alt + shift + q | Close window |
| alt + f | Toggle fullscreen |
| alt + r | Rotate layout |
| alt + e | Cycle layout (bsp/stack/float) |
| alt + v / s | Split vertical / horizontal |
| alt + =/- | Increase / decrease gaps |
| alt + shift + b | Balance window sizes |
| alt + tab | Focus recent workspace |
| alt + shift + r | Reload config |
| alt + shift + e | Stop vimwm |

## Vim mode

Press `alt + space` to toggle vim mode. Mouse input is blocked while active. Press `i` or `Escape` to exit.

### Navigation

| Key | Action |
|-----|--------|
| h / j / k / l | Arrow keys |
| shift + h/j/k/l | Select text |
| w / b | Word forward / back |
| shift + w/b | Select word forward / back |
| 0 | Line start |
| 4 | Line end |
| g | Top of document |
| shift + g | Bottom of document |
| u / d | Page up / down |

### Editing

| Key | Action |
|-----|--------|
| y | Copy |
| p | Paste |
| x | Delete |
| shift + x | Backspace |
| o | New line below |
| shift + o | New line above |
| a | Append (move right) |
| shift + a | End of line |
| shift + i | Start of line |

### Search and undo

| Key | Action |
|-----|--------|
| / | Find |
| n / shift + n | Find next / prev |
| ctrl + z | Undo |
| ctrl + r | Redo |
| ctrl + w | Delete word back |
| tab / shift + tab | Next / prev tab |

## Configuration

All config lives in `~/.config/vimwm/config.toml`. Edit it directly or use the CLI:

```sh
vimwm config edit         # open in $EDITOR
vimwm config path         # print config path
vimwm config reset        # reset to defaults
vimwm bind "mod+d" "terminal"
vimwm unbind "mod+d"
vimwm layout bsp          # bsp, stack, or float
vimwm gaps 12
vimwm padding 8
vimwm preset i3           # i3, vim, or minimal
```

### Example config

```toml
mod_key = "alt"
terminal = "alacritty"
layout = "bsp"
gap_size = 8
padding = 8
vim_mode = true
vim_toggle_key = "mod+space"

[bindings]
"mod+h" = "focus west"
"mod+j" = "focus south"
"mod+enter" = "terminal"

[vim_bindings]
h = "key left"
j = "key down"
k = "key up"
l = "key right"
y = "key cmd-c"
p = "key cmd-v"
```

## How it works

vimwm is a Rust CLI that reads your TOML config and generates native configs for yabai (tiling) and skhd (hotkeys). It manages both as background processes.

Vim mode uses skhd's mode system. When activated, bare keys like `h/j/k/l` are intercepted and translated into system key events via small Swift binaries that use Apple's CGEvent API. A separate mouseblock binary creates a CGEvent tap that drops all mouse input.

```
config.toml --> vimwm --> yabairc + skhdrc --> yabai + skhd
                                                  |
                                          Swift helpers (CGEvent)
                                          key, cursor, click,
                                          scroll, mouseblock
```

## Project structure

```
src/
  main.rs              entry point
  cli.rs               CLI definitions
  config/
    parser.rs          read/write TOML config
    defaults.rs        default keybindings
    presets.rs          i3, vim, minimal presets
  daemon/
    lifecycle.rs       start/stop/reload processes
    pid.rs             PID tracking
  generators/
    yabai.rs           config -> yabairc
    skhd.rs            config -> skhdrc
  commands/
    bind.rs            bind/unbind keys
    layout.rs          set layout
    gaps.rs            set gaps/padding
    spaces.rs          list workspaces

~/.config/vimwm/
  config.toml          user config
  key                  send keyboard events
  cursor               move mouse cursor
  click                send mouse clicks
  scroll               send scroll events
  mouseblock           block mouse input
```

## Troubleshooting

**yabai says SIP must be disabled**: See the install section above for SIP and sudoers setup.

**Vim mode not working**: Make sure your terminal has Accessibility permissions in System Settings > Privacy & Security > Accessibility.

**Mouse still moves in vim mode**: Check that the mouseblock binary is compiled and located at `~/.config/vimwm/mouseblock`.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Make your changes
4. Run `cargo build` and test manually
5. Commit your changes (`git commit -am 'Add your feature'`)
6. Push to the branch (`git push origin feature/your-feature`)
7. Open a pull request

Keep changes focused and minimal. Follow the existing code style. No unnecessary abstractions.

## License

MIT
