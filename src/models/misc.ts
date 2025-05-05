import { QuizSession } from "./core";
import { Exercise } from "./exercise";

export interface Duration {
    seconds: number;
    hhmmss: string;
}

export interface Sentence {
    hangul: string;
    romanized: string;
    translation: string;
}

export interface SentenceBreakdown {
    sentence: string;
    translation: string;
    breakdown: WordBreakdown[];
}

export interface WordBreakdown {
    word: string;
    part_of_speech: string;
    meaning: string;
    components: WordBreakdown[];
}

export interface ValidateExerciseResult {
    isCorrect: boolean;
    session: QuizSession;
    exercise: Exercise;
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

export interface ContentItem { 
    title: string;
    content: string;
}
    