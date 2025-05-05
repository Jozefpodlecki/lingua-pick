import wordInner from "../data/kr/vocabulary.json";

export interface KrWord {
    id: number;
    word: {
        hangul: string;
        romanized: string;
    }
    translations: string[];
    imageSrc: string;
}

export const words: KrWord[] = wordInner;