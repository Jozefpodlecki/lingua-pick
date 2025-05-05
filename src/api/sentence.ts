import sentences from "../data/kr/sentences.json";
import { Sentence } from "../models";

export const getRandomSentence = (): Sentence | null => {
    const randomIndex = Math.floor(Math.random() * sentences.length);
    return sentences[randomIndex] || null;
}