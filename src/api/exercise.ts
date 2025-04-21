import { Exercise, QuizSession, ValidateExerciseResult } from "../models";
import { getAppData, saveAppData } from "./appData";
import { saveUpdatedSession, updateSessionWithExercise } from "./session";
import { computeDuration } from "./utils";
import { v4 as uuidv4 } from "uuid";
import vocabulary from "../data/vocabulary.json";
import { updateStats } from "./stats";

export const createExercise = async (session: QuizSession): Promise<[Exercise, QuizSession]> => {
    const exercise = generateExercise();
    const updatedSession = updateSessionWithExercise(session, exercise);
    saveUpdatedSession(updatedSession);
    return [exercise, updatedSession];
};

export const generateExercise = (): Exercise => {
    const shuffledOptions = vocabulary.sort(() => Math.random() - 0.5).slice(0, 3);
    const randomIndex = Math.floor(Math.random() * shuffledOptions.length);
    const correctOption = shuffledOptions[randomIndex];
    const question = `${correctOption.word.hangul} (${correctOption.word.romanized})`;
    const currentDate = new Date();
    const exerciseType = "word-word";


    return {
        id: uuidv4(),
        duration: {
            seconds: 0,
            hhmmss: "00:00:00",
        },
        createdOn: currentDate.toISOString(),
        type: exerciseType,
        question,
        isCompleted: false,
        isCorrect: null,
        correctWordId: correctOption.id,
        options: shuffledOptions,
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
    }

    const result = {
        isCorrect: true,
        session,
        exercise,
    }

    return Promise.resolve(result);
}

const validateWordImageExercise = (session: QuizSession, exercise: Exercise, selectedWordId: number): ValidateExerciseResult => {
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