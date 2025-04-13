import { Exercise, AppData, UserStats } from "../models";
import vocabulary from "../data/vocabulary.json";

import { v4 as uuidv4 } from "uuid";
import { QuizSession } from "@models";

const STORAGE_KEY = "sessionData";

const defaultAppData: AppData = {
    sessions: [],
    activeSessionId: null,
    stats: {
        vocabulary: {
            words: []
        }
    },
}

const getSessionData = (): AppData => {
    const savedData = localStorage.getItem(STORAGE_KEY);
    return savedData
        ? JSON.parse(savedData)
        : defaultAppData;
};

const saveAppData = (data: AppData): void => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
};

export const getSessionById = (sessionId: string): QuizSession | null => {
    const { sessions } = getSessionData();
    return sessions.find((session) => session.id === sessionId) || null;
};

export const getSessions = (): QuizSession[] => {
    return getSessionData().sessions;
};

export const getActiveSession = (): QuizSession | null => {
    const { sessions, activeSessionId } = getSessionData();
    return activeSessionId
        ? sessions.find((session) => session.id === activeSessionId) || null
        : null;
};

export const setActiveSession = (sessionId: string): void => {
    const data = getSessionData();
    saveAppData({ ...data, activeSessionId: sessionId });
};

export const saveSessions = (sessions: QuizSession[]): void => {
    const data = getSessionData();
    saveAppData({ ...data, sessions });
};

const computeDuration = (createdOn: string, currentDate: Date): { seconds: number; hhmmss: string } => {
    const createdOnDate = new Date(createdOn);
    const totalSeconds = Math.floor((currentDate.getTime() - createdOnDate.getTime()) / 1000);

    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    const hhmmss = [hours, minutes, seconds]
        .map((unit) => String(unit).padStart(2, "0"))
        .join(":");

    return { seconds: totalSeconds, hhmmss };
};

export const completeSession = (session: QuizSession): Promise<QuizSession> => {
    const currentDate = new Date();
    const duration = computeDuration(session.createdOn, currentDate);

    const updatedSession: QuizSession = {
        ...session,
        isFinished: true,
        duration,
        updatedOn: currentDate.toISOString(),
    };

    const data = getSessionData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === updatedSession.id ? updatedSession : s
    );

    const updatedData = { ...data, sessions: updatedSessions, activeSessionId: null };
    saveAppData(updatedData);

    return Promise.resolve(updatedSession);
}

export const saveSession = (session: QuizSession, updatedExercise: Exercise): void => {
    const data = getSessionData();

    const updatedExercises = session.exercises.map((ex) =>
        ex.createdOn === updatedExercise.createdOn ? updatedExercise : ex
    );

    const currentDate = new Date();
    const duration = computeDuration(session.createdOn, currentDate);
    const updatedOn = new Date().toISOString();

    const updatedSession = {
        ...session,
        exercises: updatedExercises,
        duration,
        updatedOn,
    };

    const updatedSessions = data.sessions.map((s) =>
        s.id === session.id ? updatedSession : s
    );

    const updatedData = { ...data, sessions: updatedSessions };
    saveAppData(updatedData);
};

export const createSession = (): QuizSession => {
    const sessionId = uuidv4();
    const createdOn = new Date().toISOString();

    const newSession: QuizSession = {
        id: sessionId,
        isFinished: false,
        duration: {
            seconds: 0,
            hhmmss: "00:00:00",
        },
        createdOn,
        updatedOn: createdOn,
        exercises: [],
        exerciseCount: 5,
    };

    const data = getSessionData();
    const updatedSessions = [...data.sessions, newSession];
    const appData = {
        sessions: updatedSessions,
        stats: data.stats,
        activeSessionId: sessionId
    };
    saveAppData(appData);

    return newSession;
};

export const cancelSession = (session: QuizSession): void => {

    const currentDate = new Date();
    const duration = computeDuration(session.createdOn, currentDate);

    session.isFinished = true;
    session.duration = duration;
    session.canceledOn = new Date().toISOString();
    session.updatedOn = new Date().toISOString();

    const data = getSessionData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === session.id ? session : s
    );
    saveAppData({ ...data, sessions: updatedSessions, activeSessionId: null });
};

interface ValidateExerciseResult {
    isCorrect: boolean;
    session: QuizSession;
    exercise: Exercise;
}

export const validateExercise = (session: QuizSession, exercise: Exercise, selectedWordId: number): ValidateExerciseResult => {

    if(exercise.type === "word-image") {
        const result = validateWordImageExercise(session, exercise, selectedWordId);
        return result;
    }

    if(exercise.type === "word-word") {
    }

    return {
        isCorrect: true,
        session,
        exercise,
    };
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

    const data = getSessionData();
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

const updateStats = (stats: UserStats, wordId: string, isCorrect: boolean): void => {
    const currentDate = new Date().toISOString();

    const wordStats = stats.vocabulary.words.find((word) => word.id === wordId);
    if (wordStats) {
        if (isCorrect) {
            wordStats.correctCount = (wordStats.correctCount || 0) + 1;
        } else {
            wordStats.incorrectCount = (wordStats.incorrectCount || 0) + 1;
        }
        wordStats.updatedOn = currentDate;
        return;
    }

    const newWordStats = {
        id: wordId,
        correctCount: isCorrect ? 1 : 0,
        incorrectCount: isCorrect ? 0 : 1,
        updatedOn: currentDate,
    };
    stats.vocabulary.words.push(newWordStats);
};

const generateExercise = (): Exercise => {
    const shuffledOptions = vocabulary.sort(() => Math.random() - 0.5).slice(0, 3);
    const randomIndex = Math.floor(Math.random() * shuffledOptions.length);
    const correctOption = shuffledOptions[randomIndex];
    const question = `${correctOption.word.hangul} (${correctOption.word.romanized})`;
    const currentDate = new Date();

    return {
        id: uuidv4(),
        duration: {
            seconds: 0,
            hhmmss: "00:00:00",
        },
        createdOn: currentDate.toISOString(),
        type: "word-image",
        question,
        isCompleted: false,
        isCorrect: null,
        correctWordId: correctOption.id,
        options: shuffledOptions,
    };
};

const updateSessionWithExercise = (session: QuizSession, exercise: Exercise): QuizSession => {
    const currentDate = new Date();
    return {
        ...session,
        exercises: [...session.exercises, exercise],
        updatedOn: currentDate.toISOString(),
    };
};

const saveUpdatedSession = (session: QuizSession): void => {
    const data = getSessionData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === session.id ? session : s
    );

    const updatedData = { ...data, sessions: updatedSessions };
    saveAppData(updatedData);
};

export const createExercise = async (session: QuizSession): Promise<[Exercise, QuizSession]> => {
    const exercise = generateExercise();
    const updatedSession = updateSessionWithExercise(session, exercise);
    saveUpdatedSession(updatedSession);
    return [exercise, updatedSession];
};