import { Exercise } from "../models";
import { allOptions } from "./data";

export const getExercise = async (): Promise<Exercise> => {

    const shuffledOptions = allOptions.sort(() => Math.random() - 0.5).slice(0, 3);
    const correctOption = shuffledOptions[Math.floor(Math.random() * shuffledOptions.length)];
    const question = `${correctOption.word.hangul} (${correctOption.word.romanized})`;

    const exercise: Exercise = {
        createdAt: new Date().toISOString(),
        type: "word-image",
        question,
        correctOptionId: 2,
        options: shuffledOptions
    }

    return Promise.resolve(exercise);
};