import { Alert, Button, Text } from "@mantine/core";
import { AlertCircle, Info, RotateCcw } from "lucide-react";
import {
	DEFAULT_HOLD_HOTKEY,
	DEFAULT_PASTE_LAST_HOTKEY,
	DEFAULT_TOGGLE_HOTKEY,
} from "../../lib/hotkeyDefaults";
import {
	useResetHotkeysToDefaults,
	useSettings,
	useUpdateHoldHotkey,
	useUpdatePasteLastHotkey,
	useUpdateToggleHotkey,
} from "../../lib/queries";
import type { HotkeyConfig } from "../../lib/tauri";
import { HotkeyInput } from "../HotkeyInput";

export function HotkeySettings() {
	const { data: settings, isLoading } = useSettings();
	const updateToggleHotkey = useUpdateToggleHotkey();
	const updateHoldHotkey = useUpdateHoldHotkey();
	const updatePasteLastHotkey = useUpdatePasteLastHotkey();
	const resetHotkeys = useResetHotkeysToDefaults();

	// Collect any errors from mutations
	const error =
		updateToggleHotkey.error ||
		updateHoldHotkey.error ||
		updatePasteLastHotkey.error ||
		resetHotkeys.error;

	// Show restart message when any hotkey is changed
	const needsRestart =
		updateToggleHotkey.isSuccess ||
		updateHoldHotkey.isSuccess ||
		updatePasteLastHotkey.isSuccess ||
		resetHotkeys.isSuccess;

	const handleToggleHotkeyChange = (config: HotkeyConfig) => {
		updateToggleHotkey.mutate(config);
	};

	const handleHoldHotkeyChange = (config: HotkeyConfig) => {
		updateHoldHotkey.mutate(config);
	};

	const handlePasteLastHotkeyChange = (config: HotkeyConfig) => {
		updatePasteLastHotkey.mutate(config);
	};

	return (
		<div className="settings-section animate-in animate-in-delay-3">
			<h3 className="settings-section-title">Hotkeys</h3>
			{error && (
				<Alert
					icon={<AlertCircle size={16} />}
					color="red"
					mb="md"
					title="Error"
				>
					{error instanceof Error ? error.message : String(error)}
				</Alert>
			)}
			{needsRestart && (
				<Alert
					icon={<Info size={16} />}
					color="blue"
					mb="md"
					title="Restart Required"
				>
					Hotkey changes require an app restart to take effect.
				</Alert>
			)}
			<div className="settings-card">
				<HotkeyInput
					label="Toggle Recording"
					description="Press once to start recording, press again to stop"
					value={settings?.toggle_hotkey ?? DEFAULT_TOGGLE_HOTKEY}
					onChange={handleToggleHotkeyChange}
					disabled={isLoading}
				/>

				<div style={{ marginTop: 20 }}>
					<HotkeyInput
						label="Hold to Record"
						description="Hold to record, release to stop"
						value={settings?.hold_hotkey ?? DEFAULT_HOLD_HOTKEY}
						onChange={handleHoldHotkeyChange}
						disabled={isLoading}
					/>
				</div>

				<div style={{ marginTop: 20 }}>
					<HotkeyInput
						label="Paste Last Transcription"
						description="Paste the most recent transcription"
						value={settings?.paste_last_hotkey ?? DEFAULT_PASTE_LAST_HOTKEY}
						onChange={handlePasteLastHotkeyChange}
						disabled={isLoading}
					/>
				</div>

				<div
					style={{
						marginTop: 24,
						paddingTop: 16,
						borderTop: "1px solid var(--mantine-color-dark-4)",
						display: "flex",
						alignItems: "center",
						justifyContent: "space-between",
					}}
				>
					<Text size="sm" c="dimmed">
						Reset all hotkeys to their default values
					</Text>
					<Button
						variant="light"
						color="gray"
						size="xs"
						leftSection={<RotateCcw size={14} />}
						onClick={() => resetHotkeys.mutate()}
						loading={resetHotkeys.isPending}
						disabled={isLoading}
					>
						Reset to Defaults
					</Button>
				</div>
			</div>
		</div>
	);
}
