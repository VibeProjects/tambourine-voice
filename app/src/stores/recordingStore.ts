import type { PipecatClient } from "@pipecat-ai/client-js";
import { create } from "zustand";

/**
 * Explicit state machine for connection and recording states.
 * Prevents invalid state combinations (e.g., recording while disconnected).
 */
type ConnectionState =
	| "disconnected" // Not connected to server
	| "connecting" // Connection in progress
	| "idle" // Connected, ready to record
	| "recording" // Mic enabled, streaming audio
	| "processing"; // Waiting for server response

interface RecordingState {
	state: ConnectionState;
	client: PipecatClient | null;
	serverUrl: string | null;

	// Actions
	setClient: (client: PipecatClient | null) => void;
	setServerUrl: (url: string | null) => void;
	setState: (state: ConnectionState) => void;

	// State transitions
	startConnecting: () => void;
	handleConnected: () => void;
	handleDisconnected: () => void;
	startRecording: () => boolean; // Returns false if not in valid state
	stopRecording: () => boolean; // Returns false if not in valid state
	handleResponse: () => void;
	reset: () => void;
}

export const useRecordingStore = create<RecordingState>((set, get) => ({
	state: "disconnected",
	client: null,
	serverUrl: null,

	setClient: (client) => set({ client }),
	setServerUrl: (serverUrl) => set({ serverUrl }),
	setState: (state) => set({ state }),

	startConnecting: () => {
		const currentState = get().state;
		if (currentState === "disconnected") {
			set({ state: "connecting" });
		}
	},

	handleConnected: () => {
		const currentState = get().state;
		if (currentState === "connecting" || currentState === "disconnected") {
			set({ state: "idle" });
		}
	},

	handleDisconnected: () => {
		set({ state: "disconnected" });
	},

	startRecording: () => {
		const { state, client } = get();
		if (state !== "idle" || !client) {
			return false;
		}

		// Signal server to reset buffer and enable mic
		try {
			client.sendClientMessage("start-recording", {});
			client.enableMic(true);
			set({ state: "recording" });
			return true;
		} catch {
			return false;
		}
	},

	stopRecording: () => {
		const { state, client } = get();
		if (state !== "recording" || !client) {
			return false;
		}

		// Disable mic and tell server to flush buffer
		try {
			client.enableMic(false);
			client.sendClientMessage("stop-recording", {});
			set({ state: "processing" });
			return true;
		} catch {
			return false;
		}
	},

	handleResponse: () => {
		const currentState = get().state;
		if (currentState === "processing") {
			set({ state: "idle" });
		}
	},

	reset: () => set({ state: "disconnected", client: null }),
}));
