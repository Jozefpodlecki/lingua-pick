import { Exercise, QuizSession, SentenceTypingExercise, ValidateExerciseResult, WordImageExercise, WordWordExercise } from "../../models";
import { updateAppDataStats, updateSessionState } from "./utils";

export const validateSimpleOptionExercise = (
    session: QuizSession,
    exercise: WordImageExercise | WordWordExercise,
    selectedId: number
): ValidateExerciseResult => {
    const now = new Date();
    const isCorrect = selectedId === exercise.correctOptionId;

    const updated: Exercise = {
        ...exercise,
        completedOn: now.toISOString(),
        isCompleted: true,
        isCorrect,
    };

    const updatedSession = updateSessionState(session, updated, now);
    updateAppDataStats(updatedSession, exercise.correctOptionId, isCorrect);

    return {
        isCorrect,
        session: updatedSession,
        exercise: updated,
    };
};

export const validateSentenceTyping = (
    session: QuizSession,
    exercise: SentenceTypingExercise
): ValidateExerciseResult => {
    const now = new Date();
    const isCorrect =
        exercise.userTranslation?.trim().toLowerCase() ===
        exercise.sentence.translation.trim().toLowerCase();

    const updated: Exercise = {
        ...exercise,
        completedOn: now.toISOString(),
        isCompleted: true,
        isCorrect,
    };

    const updatedSession = updateSessionState(session, updated, now);
    // updateAppDataStats(updatedSession, null, isCorrect);

    return {
        isCorrect,
        session: updatedSession,
        exercise: updated,
    };
};

export const validateWordsWords = (
    session: QuizSession,
    exercise: Exercise
): ValidateExerciseResult => {
    return {
        isCorrect: true,
        session,
        exercise,
    };
};