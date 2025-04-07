export interface Option {
    id: number;
    word: {
        hangul: string;
        romanized: string;
    };
    imageSrc: string;
}

export interface Exercise {
    createdAt: string;
    type: string;
    options: Option[];
    question: string;
    correctOptionId: number;
}
