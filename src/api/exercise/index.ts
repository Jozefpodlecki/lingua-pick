import { Exercise, QuizSession, ValidateExerciseResult } from "../../models";
import { saveUpdatedSession, updateSessionWithExercise } from "./../session";
import { createBaseExercise, getRandomItem } from "./utils";
import { generateHangulMatchExercise, generateSentenceTypingExercise, generateWordBasedExercise, generateWordsMatchExercise, generateWordsWordsExercise } from "./generate";
import { validateSentenceTyping, validateSimpleOptionExercise, validateWordsWords } from "./validate";

export * from "./utils";

export const createExercise = async (session: QuizSession): Promise<[Exercise, QuizSession]> => {
    const exercise = generateExercise(session);
    const updatedSession = updateSessionWithExercise(session, exercise);
    saveUpdatedSession(updatedSession);
    return [exercise, updatedSession];
};

export const generateExercise = (session: QuizSession): Exercise => {
    const type = getRandomItem(session.exerciseTypes);
    const base = createBaseExercise();

    switch (type) {
        case "word-word":
        case "word-image":
            return generateWordBasedExercise(type, session, base);
        case "sentence-typing":
            return generateSentenceTypingExercise(base);
        case "words-words":
            return generateWordsWordsExercise(base);
        case "words-match":
            return generateWordsMatchExercise(session, base);
        case "hangul-match":
            return generateHangulMatchExercise(base);
        default:
            return { ...base, type: "unknown", isCompleted: false, isCorrect: null };
    }
};


export const validateExercise = (
    session: QuizSession,
    exercise: Exercise,
    selectedId: number
): Promise<ValidateExerciseResult> => {
    switch (exercise.type) {
        case "word-image":
        case "word-word":
            return Promise.resolve(validateSimpleOptionExercise(session, exercise, selectedId));
        case "sentence-typing":
            return Promise.resolve(validateSentenceTyping(session, exercise));
        case "words-words":
            return Promise.resolve(validateWordsWords(session, exercise));
        default:
            return Promise.resolve({ isCorrect: true, session, exercise });
    }
};



