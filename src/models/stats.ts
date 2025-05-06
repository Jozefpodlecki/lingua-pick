export interface WordStats {
    id: number;
    ratio: number;
    correctCount: number;
    incorrectCount: number;
    updatedOn: string;
}

export interface Stat {
    ratio: number;
    correctCount: number;
    incorrectCount: number;
    updatedOn: string;
}

export interface UserStats {
    sessionCount: number;
    updatedOn: string;
    hangul: Record<string, Stat>;
    vocabulary: {
        words: WordStats[]
    }
}