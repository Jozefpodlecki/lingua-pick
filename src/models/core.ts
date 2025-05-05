import { Exercise, ExerciseTypes } from "./exercise";
import { Duration } from "./misc";
import { UserStats } from "./stats";

export interface AppData {
    sessions: QuizSession[];
    stats: UserStats;
    activeSessionId: string | null;
    availableLanguages: Language[];
    selectedLanguage: Language;
}

export interface Language {
    name: string;
    isoCode: string;
    flagUrl: string;
}

export interface QuizSession {
    id: string;
    language: Language;
    isFinished: boolean;
    duration: Duration;
    createdOn: string;
    updatedOn: string;
    canceledOn?: string;
    completedOn?: string;
    exerciseCount: number;
    exercises: Exercise[];
    exerciseTypes: ExerciseTypes[];
    usedWordIds: number[];
}