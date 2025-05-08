import { Exercise, ExerciseTypes, QuizSession, ValidateExerciseResult } from "@/models";
import { createBaseExercise, getRandomItem } from "./utils";
import { generateHangulMatchExercise, generateSentenceTypingExercise, generateWordBasedExercise, generateWordsMatchExercise, generateWordsWordsExercise } from "./generate";
import { validateMatchExercise, validateSentenceTyping, validateSimpleOptionExercise, validateWordsWords } from "./validate";

export * from "./utils";

export interface CreateExerciseOptions {
    exerciseTypes: ExerciseTypes[];
    usedWordIds: number[];
}

export const generateExercise = (opions: CreateExerciseOptions): Exercise => {
    const type = getRandomItem(opions.exerciseTypes);
    const base = createBaseExercise();

    switch (type) {
        case "word-word":
        case "word-image":
            return generateWordBasedExercise(type, opions.usedWordIds, base);
        case "sentence-typing":
            return generateSentenceTypingExercise(base);
        case "words-words":
            return generateWordsWordsExercise(opions.usedWordIds, base);
        case "words-match":
            return generateWordsMatchExercise(opions.usedWordIds, base);
        case "hangul-match":
            return generateHangulMatchExercise(base);
        default:
            return { ...base, type: "unknown", isCompleted: false };
    }
};


export const validateExercise = (
    session: QuizSession,
    exercise: Exercise
): Promise<ValidateExerciseResult> => {
    switch (exercise.type) {
        case "word-image":
        case "word-word":
            return Promise.resolve(validateSimpleOptionExercise(session, exercise));
        case "sentence-typing":
            return Promise.resolve(validateSentenceTyping(session, exercise));
        case "words-words":
            return Promise.resolve(validateWordsWords(session, exercise));
        case "hangul-match":
        case "words-match":
            return Promise.resolve(validateMatchExercise(session, exercise));
        default:
            return Promise.resolve({ isCorrect: true, session, exercise });
    }
};



