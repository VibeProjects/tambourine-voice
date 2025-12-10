use crate::settings::HotkeyConfig;

// Tests for HotkeyConfig::to_shortcut_string()
#[test]
fn test_to_shortcut_string_single_modifier() {
    let hotkey = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    assert_eq!(hotkey.to_shortcut_string(), "ctrl+Space");
}

#[test]
fn test_to_shortcut_string_multiple_modifiers() {
    let hotkey = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
    };
    assert_eq!(hotkey.to_shortcut_string(), "ctrl+alt+Space");
}

#[test]
fn test_to_shortcut_string_preserves_key_case() {
    let hotkey = HotkeyConfig {
        key: "Backquote".to_string(),
        modifiers: vec!["CTRL".to_string(), "ALT".to_string()],
    };
    // Modifiers should be lowercase, key should preserve case
    assert_eq!(hotkey.to_shortcut_string(), "ctrl+alt+Backquote");
}

// Tests for HotkeyConfig::is_same_as()
#[test]
fn test_is_same_as_identical_hotkeys() {
    let a = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
    };
    let b = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
    };
    assert!(a.is_same_as(&b));
}

#[test]
fn test_is_same_as_different_modifier_order() {
    let a = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
    };
    let b = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Alt".to_string(), "Ctrl".to_string()],
    };
    assert!(a.is_same_as(&b)); // Order shouldn't matter
}

#[test]
fn test_is_same_as_case_insensitive() {
    let a = HotkeyConfig {
        key: "space".to_string(),
        modifiers: vec!["ctrl".to_string()],
    };
    let b = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    assert!(a.is_same_as(&b));
}

#[test]
fn test_is_same_as_different_keys() {
    let a = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    let b = HotkeyConfig {
        key: "Enter".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    assert!(!a.is_same_as(&b));
}

#[test]
fn test_is_same_as_different_modifiers() {
    let a = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    let b = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Alt".to_string()],
    };
    assert!(!a.is_same_as(&b));
}

#[test]
fn test_is_same_as_extra_modifier() {
    let a = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    let b = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
    };
    assert!(!a.is_same_as(&b)); // Different number of modifiers
}

#[test]
fn test_is_same_as_missing_modifier() {
    let a = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
    };
    let b = HotkeyConfig {
        key: "Space".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    assert!(!a.is_same_as(&b)); // Different number of modifiers
}
