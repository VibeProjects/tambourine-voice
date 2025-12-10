use crate::commands::settings::validate_no_duplicate_shortcut;
use crate::settings::{AppSettings, HotkeyConfig};

// Tests for validate_no_duplicate_shortcut()
#[test]
fn test_validate_no_duplicate_allows_unique_shortcut() {
    let new_hotkey = HotkeyConfig {
        key: "F1".to_string(),
        modifiers: vec!["Ctrl".to_string()],
    };
    let settings = AppSettings::default();
    assert!(validate_no_duplicate_shortcut(&new_hotkey, &settings, "toggle").is_ok());
}

#[test]
fn test_validate_no_duplicate_rejects_duplicate_with_hold() {
    let settings = AppSettings::default();
    let duplicate = settings.hold_hotkey.clone();
    let result = validate_no_duplicate_shortcut(&duplicate, &settings, "toggle");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("hold"));
}

#[test]
fn test_validate_no_duplicate_rejects_duplicate_with_toggle() {
    let settings = AppSettings::default();
    let duplicate = settings.toggle_hotkey.clone();
    let result = validate_no_duplicate_shortcut(&duplicate, &settings, "hold");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("toggle"));
}

#[test]
fn test_validate_no_duplicate_rejects_duplicate_with_paste_last() {
    let settings = AppSettings::default();
    let duplicate = settings.paste_last_hotkey.clone();
    let result = validate_no_duplicate_shortcut(&duplicate, &settings, "toggle");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("paste last"));
}

#[test]
fn test_validate_no_duplicate_excludes_own_type() {
    let settings = AppSettings::default();
    let same_as_toggle = settings.toggle_hotkey.clone();
    // Should succeed because we're excluding "toggle" from comparison
    assert!(validate_no_duplicate_shortcut(&same_as_toggle, &settings, "toggle").is_ok());
}

#[test]
fn test_validate_no_duplicate_excludes_hold_type() {
    let settings = AppSettings::default();
    let same_as_hold = settings.hold_hotkey.clone();
    // Should succeed because we're excluding "hold" from comparison
    assert!(validate_no_duplicate_shortcut(&same_as_hold, &settings, "hold").is_ok());
}

#[test]
fn test_validate_no_duplicate_excludes_paste_last_type() {
    let settings = AppSettings::default();
    let same_as_paste = settings.paste_last_hotkey.clone();
    // Should succeed because we're excluding "paste_last" from comparison
    assert!(validate_no_duplicate_shortcut(&same_as_paste, &settings, "paste_last").is_ok());
}

#[test]
fn test_validate_no_duplicate_case_insensitive_comparison() {
    let new_hotkey = HotkeyConfig {
        key: "space".to_string(),                               // lowercase
        modifiers: vec!["CTRL".to_string(), "ALT".to_string()], // uppercase
    };
    let settings = AppSettings::default(); // toggle is Ctrl+Alt+Space
                                           // Should detect as duplicate even with different case
    let result = validate_no_duplicate_shortcut(&new_hotkey, &settings, "hold");
    assert!(result.is_err());
}
