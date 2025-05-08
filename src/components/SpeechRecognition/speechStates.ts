import { SpeechRecognitionState } from "@/models";

export const speechState = {
    idle: { type: "idle" } as SpeechRecognitionState,
    recording: { type: "recording" } as SpeechRecognitionState,
    error: (message: string): SpeechRecognitionState => ({ type: "error", message }),
    recognized: (text: string): SpeechRecognitionState => ({ type: "recognized", text }),
};