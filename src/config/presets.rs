use std::collections::BTreeMap;

pub fn get_preset(name: &str) -> Option<BTreeMap<String, String>> {
    match name {
        "i3" => Some(i3_preset()),
        "vim" => Some(vim_preset()),
        "minimal" => Some(minimal_preset()),
        _ => None,
    }
}

fn i3_preset() -> BTreeMap<String, String> {
    let pairs = [
        ("mod+h", "focus west"),
        ("mod+j", "focus south"),
        ("mod+k", "focus north"),
        ("mod+l", "focus east"),
        ("mod+shift+h", "move west"),
        ("mod+shift+j", "move south"),
        ("mod+shift+k", "move north"),
        ("mod+shift+l", "move east"),
        ("mod+1", "space 1"),
        ("mod+2", "space 2"),
        ("mod+3", "space 3"),
        ("mod+4", "space 4"),
        ("mod+5", "space 5"),
        ("mod+6", "space 6"),
        ("mod+7", "space 7"),
        ("mod+8", "space 8"),
        ("mod+9", "space 9"),
        ("mod+shift+1", "move-to-space 1"),
        ("mod+shift+2", "move-to-space 2"),
        ("mod+shift+3", "move-to-space 3"),
        ("mod+shift+4", "move-to-space 4"),
        ("mod+shift+5", "move-to-space 5"),
        ("mod+shift+6", "move-to-space 6"),
        ("mod+shift+7", "move-to-space 7"),
        ("mod+shift+8", "move-to-space 8"),
        ("mod+shift+9", "move-to-space 9"),
        ("mod+enter", "terminal"),
        ("mod+shift+q", "close"),
        ("mod+f", "fullscreen"),
        ("mod+shift+space", "toggle-float"),
        ("mod+r", "rotate"),
        ("mod+shift+r", "reload"),
        ("mod+shift+e", "stop"),
        ("mod+e", "layout-cycle"),
        ("mod+v", "split-vertical"),
        ("mod+s", "split-horizontal"),
        ("mod+equal", "gaps-inc"),
        ("mod+minus", "gaps-dec"),
        ("mod+tab", "focus-recent"),
        ("mod+shift+b", "balance"),
        // i3-specific: resize mode keys
        ("mod+ctrl+h", "resize shrink-width"),
        ("mod+ctrl+j", "resize grow-height"),
        ("mod+ctrl+k", "resize shrink-height"),
        ("mod+ctrl+l", "resize grow-width"),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

fn vim_preset() -> BTreeMap<String, String> {
    let pairs = [
        // Pure vim: hjkl everywhere, gg/G style
        ("mod+h", "focus west"),
        ("mod+j", "focus south"),
        ("mod+k", "focus north"),
        ("mod+l", "focus east"),
        ("mod+shift+h", "move west"),
        ("mod+shift+j", "move south"),
        ("mod+shift+k", "move north"),
        ("mod+shift+l", "move east"),
        ("mod+1", "space 1"),
        ("mod+2", "space 2"),
        ("mod+3", "space 3"),
        ("mod+4", "space 4"),
        ("mod+5", "space 5"),
        ("mod+6", "space 6"),
        ("mod+7", "space 7"),
        ("mod+8", "space 8"),
        ("mod+9", "space 9"),
        ("mod+shift+1", "move-to-space 1"),
        ("mod+shift+2", "move-to-space 2"),
        ("mod+shift+3", "move-to-space 3"),
        ("mod+shift+4", "move-to-space 4"),
        ("mod+shift+5", "move-to-space 5"),
        ("mod+shift+6", "move-to-space 6"),
        ("mod+shift+7", "move-to-space 7"),
        ("mod+shift+8", "move-to-space 8"),
        ("mod+shift+9", "move-to-space 9"),
        ("mod+enter", "terminal"),
        ("mod+q", "close"),
        ("mod+f", "fullscreen"),
        ("mod+shift+space", "toggle-float"),
        ("mod+r", "rotate"),
        ("mod+shift+r", "reload"),
        ("mod+shift+e", "stop"),
        ("mod+tab", "focus-recent"),
        ("mod+shift+b", "balance"),
        // vim-specific: window splits like vim
        ("mod+ctrl+v", "split-vertical"),
        ("mod+ctrl+s", "split-horizontal"),
        ("mod+ctrl+h", "resize shrink-width"),
        ("mod+ctrl+j", "resize grow-height"),
        ("mod+ctrl+k", "resize shrink-height"),
        ("mod+ctrl+l", "resize grow-width"),
        ("mod+w", "layout-cycle"),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

fn minimal_preset() -> BTreeMap<String, String> {
    let pairs = [
        ("mod+h", "focus west"),
        ("mod+j", "focus south"),
        ("mod+k", "focus north"),
        ("mod+l", "focus east"),
        ("mod+shift+h", "move west"),
        ("mod+shift+j", "move south"),
        ("mod+shift+k", "move north"),
        ("mod+shift+l", "move east"),
        ("mod+1", "space 1"),
        ("mod+2", "space 2"),
        ("mod+3", "space 3"),
        ("mod+enter", "terminal"),
        ("mod+shift+q", "close"),
        ("mod+f", "fullscreen"),
        ("mod+shift+r", "reload"),
        ("mod+shift+e", "stop"),
    ];
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}
