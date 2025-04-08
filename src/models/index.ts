export interface Option {
    id: number;
    word: {
        hangul: string;
        romanized: string;
    };
    translations: string[];
    imageSrc: string;
}

export interface Exercise {
    createdAt: string;
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