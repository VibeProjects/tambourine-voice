use crate::settings::{AppSettings, CleanupPromptSections, HotkeyConfig, SettingsManager};
use tauri::State;

/// Validate that a new hotkey doesn't conflict with other configured hotkeys
#[cfg(desktop)]
pub(crate) fn validate_no_duplicate_shortcut(
    new_hotkey: &HotkeyConfig,
    current_settings: &AppSettings,
    exclude_type: &str,
) -> Result<(), String> {
    let hotkeys_to_check: Vec<(&str, &HotkeyConfig)> = vec![
        ("toggle", &current_settings.toggle_hotkey),
        ("hold", &current_settings.hold_hotkey),
        ("paste_last", &current_settings.paste_last_hotkey),
    ];

    for (hotkey_type, existing_hotkey) in hotkeys_to_check {
        if hotkey_type != exclude_type && new_hotkey.is_same_as(existing_hotkey) {
            return Err(format!(
                "This shortcut is already used for the {} hotkey",
                hotkey_type.replace('_', " ")
            ));
        }
    }

    Ok(())
}

/// Generic helper for updating hotkeys with validation
/// Validates no duplicate shortcuts and that the shortcut can be parsed, then calls the update function
#[cfg(desktop)]
fn update_hotkey_with_validation<F>(
    hotkey: HotkeyConfig,
    hotkey_type: &str,
    settings_manager: &SettingsManager,
    update_fn: F,
) -> Result<(), String>
where
    F: FnOnce(&SettingsManager, HotkeyConfig) -> Result<(), String>,
{
    // Validate no duplicate
    let current_settings = settings_manager.get()?;
    validate_no_duplicate_shortcut(&hotkey, &current_settings, hotkey_type)?;

    // Validate the shortcut can be parsed
    hotkey.to_shortcut()?;

    // Save settings
    update_fn(settings_manager, hotkey)?;

    let display_name = match hotkey_type {
        "toggle" => "Toggle",
        "hold" => "Hold",
        "paste_last" => "Paste last",
        _ => hotkey_type,
    };
    log::info!(
        "{} hotkey updated. Restart required for changes to take effect.",
        display_name
    );
    Ok(())
}

/// Get the current application settings
#[tauri::command]
pub async fn get_settings(
    settings_manager: State<'_, SettingsManager>,
) -> Result<AppSettings, String> {
    settings_manager.get()
}

/// Update all settings at once
#[tauri::command]
pub async fn save_settings(
    settings: AppSettings,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update(settings)
}

/// Update just the toggle hotkey (saves settings only, use update_toggle_hotkey_live for runtime update)
#[tauri::command]
pub async fn update_toggle_hotkey(
    hotkey: HotkeyConfig,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_toggle_hotkey(hotkey)
}

/// Update just the hold hotkey (saves settings only, use update_hold_hotkey_live for runtime update)
#[tauri::command]
pub async fn update_hold_hotkey(
    hotkey: HotkeyConfig,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_hold_hotkey(hotkey)
}

/// Update just the paste last hotkey
#[tauri::command]
pub async fn update_paste_last_hotkey(
    hotkey: HotkeyConfig,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_paste_last_hotkey(hotkey)
}

/// Update toggle hotkey (saves settings, restart required for hotkey to take effect)
#[cfg(desktop)]
#[tauri::command]
pub async fn update_toggle_hotkey_live(
    hotkey: HotkeyConfig,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    update_hotkey_with_validation(hotkey, "toggle", &settings_manager, |sm, h| {
        sm.update_toggle_hotkey(h)
    })
}

/// Update hold hotkey (saves settings, restart required for hotkey to take effect)
#[cfg(desktop)]
#[tauri::command]
pub async fn update_hold_hotkey_live(
    hotkey: HotkeyConfig,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    update_hotkey_with_validation(hotkey, "hold", &settings_manager, |sm, h| {
        sm.update_hold_hotkey(h)
    })
}

/// Update paste last hotkey (saves settings, restart required for hotkey to take effect)
#[cfg(desktop)]
#[tauri::command]
pub async fn update_paste_last_hotkey_live(
    hotkey: HotkeyConfig,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    update_hotkey_with_validation(hotkey, "paste_last", &settings_manager, |sm, h| {
        sm.update_paste_last_hotkey(h)
    })
}

/// Update the selected microphone device
#[tauri::command]
pub async fn update_selected_mic(
    mic_id: Option<String>,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_selected_mic(mic_id)
}

/// Update the sound enabled setting
#[tauri::command]
pub async fn update_sound_enabled(
    enabled: bool,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_sound_enabled(enabled)
}

/// Update the cleanup prompt sections setting
#[tauri::command]
pub async fn update_cleanup_prompt_sections(
    sections: Option<CleanupPromptSections>,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_cleanup_prompt_sections(sections)
}

/// Update the STT provider setting
#[tauri::command]
pub async fn update_stt_provider(
    provider: Option<String>,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_stt_provider(provider)
}

/// Update the LLM provider setting
#[tauri::command]
pub async fn update_llm_provider(
    provider: Option<String>,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_llm_provider(provider)
}

/// Update the auto mute audio setting
#[tauri::command]
pub async fn update_auto_mute_audio(
    enabled: bool,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_auto_mute_audio(enabled)
}

/// Update the STT timeout setting
#[tauri::command]
pub async fn update_stt_timeout(
    timeout_seconds: Option<f64>,
    settings_manager: State<'_, SettingsManager>,
) -> Result<(), String> {
    settings_manager.update_stt_timeout(timeout_seconds)
}

/// Reset all hotkeys to their default values
/// Note: This only updates settings. App restart is required for hotkeys to take effect.
#[cfg(desktop)]
#[tauri::command]
pub async fn reset_hotkeys_to_defaults(
    settings_manager: State<'_, SettingsManager>,
) -> Result<bool, String> {
    log::info!("Resetting hotkeys to defaults...");

    // Create default hotkey configs
    let default_toggle = HotkeyConfig::default_toggle();
    let default_hold = HotkeyConfig::default_hold();
    let default_paste_last = HotkeyConfig::default_paste_last();

    // Save default settings
    settings_manager.update_toggle_hotkey(default_toggle)?;
    settings_manager.update_hold_hotkey(default_hold)?;
    settings_manager.update_paste_last_hotkey(default_paste_last)?;

    log::info!("Hotkey settings reset to defaults. Restart required for changes to take effect.");

    // Return true to indicate restart is needed
    Ok(true)
}
