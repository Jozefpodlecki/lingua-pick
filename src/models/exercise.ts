import { Duration, Sentence } from "./misc";

export type KrExerciseTypes = "hangul-match";

export type ExerciseTypes = "word-word" | "word-image" | "sentence-typing" | "words-words" | KrExerciseTypes;

export type Exercise = WordWordExercise 
    | WordImageExercise
    | SentenceTypingExercise
    | UnknownExercise
    | WordsWordsExercise
    | HangulMatchExercise;

export interface Option {
    id: number;
    value: string;
}

export interface ExerciseType {
    type: ExerciseTypes;
    description: string;
}

export interface BaseExercise {
    id: string;
    duration: Duration;
    createdOn: string;
    completedOn?: string;
    isCompleted: boolean;
    isCorrect: boolean | null;
}

export interface WordWordExercise extends BaseExercise {
    type: "word-word";
    options: Option[];
    prompt: string;
    selectedOptionId: number;
    correctOptionId: number;
}

export interface WordImageExercise extends BaseExercise {
    type: "word-image";
    options: Option[];
    prompt: string;
    selectedOptionId: number;
    correctOptionId: number;
}

export interface SentenceTypingExercise extends BaseExercise {
    type: "sentence-typing";
    sentence: Sentence;
    sentenceText: string;
    userTranslation: string;
}

export interface WordsWordsExercise extends BaseExercise {
    type: "words-words";
    left: Option[];
    right: Option[];
}

export interface HangulMatchExercise extends BaseExercise {
    type: "hangul-match";
    ids: Set<number>;
    matchedIds: Set<number>;
    items: Option[];
    correctCount: number;
    incorrectCount: number;
}

export interface UnknownExercise extends BaseExercise {
    type: "unknown";
}