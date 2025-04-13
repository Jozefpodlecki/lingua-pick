export interface Option {
    id: number;
    word: {
        hangul: string;
        romanized: string;
    };
    translations: string[];
    imageSrc: string;
}

export interface WordStats {
    id: string;
    ratio: number;
    correctCount: number;
    incorrectCount: number;
    updatedOn: string;
}

export interface UserStats {
    vocabulary: {
        words: WordStats[]
    }
}

export interface AppData {
    sessions: QuizSession[];
    stats: UserStats;
    activeSessionId: string | null;
}

export interface Duration {
    seconds: number;
    hhmmss: string;
}

export interface QuizSession {
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
    correctWordId: number;
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