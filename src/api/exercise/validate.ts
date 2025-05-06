import { Exercise, HangulMatchExercise, QuizSession, SentenceTypingExercise, ValidateExerciseResult, WordImageExercise, WordsMatchExercise, WordsWordsExercise, WordWordExercise } from "../../models";
import { updateStats } from "../stats";
import { updateSessionState } from "./utils";

export const validateSimpleOptionExercise = (
    session: QuizSession,
    exercise: WordImageExercise | WordWordExercise
): ValidateExerciseResult => {
    const now = new Date();
    const isCorrect = exercise.selectedOptionId === exercise.correctOptionId;

    const updated: Exercise = {
        ...exercise,
        isCorrect,
        completedOn: now.toISOString(),
        isCompleted: true,
    };

    const updatedSession = updateSessionState(session, updated, now);
    updateStats(exercise);

    return {
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

    const updated: typeof exercise = {
        ...exercise,
        completedOn: now.toISOString(),
        isCompleted: true,
        isCorrect,
    };

    const updatedSession = updateSessionState(session, updated, now);
    updateStats(exercise);

    return {
        session: updatedSession,
        exercise: updated,
    };
};

export const validateWordsWords = (
    session: QuizSession,
    exercise: WordsWordsExercise
): ValidateExerciseResult => {

    const now = new Date();

    const updated: typeof exercise = {
        ...exercise,
        completedOn: now.toISOString(),
        isCompleted: true,
    };

    const updatedSession = updateSessionState(session, updated, now);
    updateStats(exercise);

    return {
        session: updatedSession,
        exercise: updated,
    };
};

export const validateMatchExercise = (
    session: QuizSession,
    exercise: WordsMatchExercise | HangulMatchExercise
): ValidateExerciseResult => {

    const now = new Date();

    const updated: typeof exercise = {
        ...exercise,
        completedOn: now.toISOString(),
        isCompleted: true,
    };

    const updatedSession = updateSessionState(session, updated, now);
    updateStats(exercise);

    return {
        session: updatedSession,
        exercise: updated,
    };
};


