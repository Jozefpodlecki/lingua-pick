import { Exercise, QuizSession, ValidateExerciseResult, WordImageExercise, WordWordExercise } from "../models";
import { getAppData, saveAppData } from "./appData";
import { saveUpdatedSession, updateSessionWithExercise } from "./session";
import { computeDuration } from "./utils";
import { v4 as uuidv4 } from "uuid";
import { updateStats } from "./stats";
import { Word, words } from "./words";

export const createExercise = async (session: QuizSession): Promise<[Exercise, QuizSession]> => {
    const exercise = generateExercise(session);
    const updatedSession = updateSessionWithExercise(session, exercise);
    saveUpdatedSession(updatedSession);
    return [exercise, updatedSession];
};

export const generateExercise = (session: QuizSession): Exercise => {
  
    const currentDate = new Date();

    const randomExerciseTypeIndex = Math.floor(Math.random() * session.exerciseTypes.length);
    const exerciseType = session.exerciseTypes[randomExerciseTypeIndex];

    const baseExercise = {
        id: uuidv4(),
        duration: {
            seconds: 0,
            hhmmss: "00:00:00",
        },
        createdOn: currentDate.toISOString(),
    }

    if (exerciseType === "word-word") {
        const availableWords = words.filter(
            (word: Word) => !session.usedWordIds.includes(word.id)
        );
        const shuffledOptions = availableWords.sort(() => Math.random() - 0.5).slice(0, 3);
        const randomIndex = Math.floor(Math.random() * shuffledOptions.length);
        const correctOption = shuffledOptions[randomIndex];
        let question = `${correctOption.word.hangul} (${correctOption.word.romanized})`;
        question = `${correctOption.translations[0]}`;
        session.usedWordIds.push(correctOption.id);

        return {
            ...baseExercise,
            type: exerciseType,
            question,
            isCompleted: false,
            isCorrect: null,
            correctWordId: correctOption.id,
            options: shuffledOptions,
        };
    }

    if (exerciseType === "word-image") {
        const availableWords = words.filter(
            (word: Word) => !session.usedWordIds.includes(word.id)
        );
        const shuffledOptions = availableWords.sort(() => Math.random() - 0.5).slice(0, 3);
        const randomIndex = Math.floor(Math.random() * shuffledOptions.length);
        const correctOption = shuffledOptions[randomIndex];
        let question = `${correctOption.word.hangul} (${correctOption.word.romanized})`;
        question = `${correctOption.translations[0]}`;
        session.usedWordIds.push(correctOption.id);

        return {
            ...baseExercise,
            type: exerciseType,
            question,
            isCompleted: false,
            isCorrect: null,
            correctWordId: correctOption.id,
            options: shuffledOptions,
        };
    }

    if(exerciseType == "sentence-typing") {

        return {
            ...baseExercise,
            type: exerciseType,
            isCompleted: false,
            isCorrect: null
        };
    }

    return {
        ...baseExercise,
        type: "unknown",
        isCompleted: false,
        isCorrect: null
    };
};

export const validateExercise = (
    session: QuizSession,
    exercise: Exercise,
    selectedWordId: number): Promise<ValidateExerciseResult> => {

    if(exercise.type === "word-image") {
        const result = validateWordImageExercise(session, exercise, selectedWordId);
        return Promise.resolve(result);
    }

    if(exercise.type === "word-word") {
        const result = validateWordImageExercise(session, exercise, selectedWordId);
        return Promise.resolve(result);
    }

    const result = {
        isCorrect: true,
        session,
        exercise,
    }

    return Promise.resolve(result);
}

const validateWordImageExercise = (session: QuizSession, exercise: WordWordExercise | WordImageExercise, selectedWordId: number): ValidateExerciseResult => {
    const currentDate = new Date();
    const isCorrect = selectedWordId === exercise.correctWordId;
    const duration = computeDuration(session.createdOn, currentDate);

    const updatedExercise: Exercise = {
        ...exercise,
        completedOn: currentDate.toISOString(),
        isCompleted: true,
        isCorrect,
    };

    const updatedExercises = session.exercises.map((ex) =>
        ex.id === updatedExercise.id ? updatedExercise : ex
    );

    session.exercises = updatedExercises;
    session.updatedOn = currentDate.toISOString();
    session.duration = duration;

    const data = getAppData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === session.id ? session : s
    );

    const updatedData = { ...data, sessions: updatedSessions };
    updateStats(updatedData.stats, exercise.correctWordId, isCorrect);
    saveAppData(updatedData);

    return {
        isCorrect,
        session,
        exercise: updatedExercise,
    };
}