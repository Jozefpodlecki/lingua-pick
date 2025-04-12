export interface Option {
    id: number;
    word: {
        hangul: string;
        romanized: string;
    };
    translations: string[];
    imageSrc: string;
}

export interface SessionData {
    sessions: Session[];
    activeSessionId: string | null;
}

export interface Duration {
    seconds: number;
    hhmmss: string;
}

export interface Session {
    id: string;
    isFinished: boolean;
    duration: Duration;
    createdOn: string;
    updatedOn: string;
    canceledOn?: string;
    completedOn?: string;
    exerciseCount: number;
    exercises: Exercise[];
}

export interface Exercise {
    id: string;
    duration: Duration;
    createdOn: string;
    completedOn?: string;
    isCompleted: boolean;
    isCorrect: boolean | null;
    type: "word-image" | "word-word";
    options: Option[];
    question: string;
    correctOptionId: number;
}

export interface HistoryItem {
    word: string;
    createdOn: string;
}

export type SpeechRecognitionState =
    | { type: "idle" }
    | { type: "recording" }
    | { type: "error"; message: string }
    | { type: "recognized"; text: string };