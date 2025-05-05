export interface WordStats {
    id: number;
    ratio: number;
    correctCount: number;
    incorrectCount: number;
    updatedOn: string;
}

export interface UserStats {
    sessionCount: number;
    updatedOn: string;
    vocabulary: {
        words: WordStats[]
    }
}