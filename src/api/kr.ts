import sentences from "../data/kr/sentences.json";
import { Sentence } from "../models";
import wordInner from "../data/kr/vocabulary.json";
import romanizedToHangul from "../data/kr/romanized_to_hangul.json";

export const getRandomSentence = (): Sentence | null => {
    const randomIndex = Math.floor(Math.random() * sentences.length);
    return sentences[randomIndex] || null;
}

export const getRomanizedToHangul = (): Record<string, string> => romanizedToHangul;

export interface KrWord {
    id: number;
    hangul: string;
    romanized: string;
    translations: string[];
    imageSrc: string;
}

export const getKrWords = (): KrWord[] => wordInner;