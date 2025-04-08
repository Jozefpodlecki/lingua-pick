import { HistoryItem } from "../models";

export const loadHistory = (): HistoryItem[] => {
    const savedHistory = localStorage.getItem("speechRecognitionHistory");
    return savedHistory ? JSON.parse(savedHistory) : [];
};

export const saveToHistory = (item: HistoryItem): void => {
    const history = loadHistory();
    const updatedHistory = [...history, item];
    localStorage.setItem("speechRecognitionHistory", JSON.stringify(updatedHistory));
};

export const startSpeechRecognition = ({
    onStart,
    onResult,
    onError,
    onEnd,
}: {
    onStart: () => void;
    onResult: (transcript: string) => void;
    onError: (errorMessage: string) => void;
    onEnd: () => void;
}) => {
    const SpeechRecognition = (window as any).webkitSpeechRecognition;
    if (!SpeechRecognition) {
        onError("Speech recognition is not supported in this browser.");
        return;
    }

    const recognition = new SpeechRecognition();
    recognition.lang = "ko-KR";
    recognition.interimResults = false;
    recognition.maxAlternatives = 1;

    recognition.onstart = onStart;

    recognition.onresult = (event: SpeechRecognitionEvent) => {
        const transcript = event.results[0][0].transcript;
        onResult(transcript);
    };

    recognition.onerror = (event: SpeechRecognitionErrorEvent) => {
        if (event.error === "no-speech") {
            onError("No speech detected. Please try again.");
        } else if (event.error === "audio-capture") {
            onError("Audio capture failed. Please check your microphone.");
        } else if (event.error === "not-allowed") {
            onError("Permission to use microphone denied.");
        } else {
            onError(`Speech recognition error: ${event.error}`);
        }
    };

    recognition.onend = onEnd;

    recognition.start();
};