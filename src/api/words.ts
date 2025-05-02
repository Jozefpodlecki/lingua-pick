import wordInner from "../data/vocabulary.json";

export interface Word {
    id: number;
    word: {
        hangul: string;
        romanized: string;
    }
    translations: string[];
    imageSrc: string;
}

export const words: Word[] = wordInner;