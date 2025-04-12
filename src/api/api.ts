import { Exercise, SessionData } from "../models";
import { allOptions } from "./data";

import { v4 as uuidv4 } from "uuid";
import { Session } from "../models";

const STORAGE_KEY = "sessionData";

const getSessionData = (): SessionData => {
    const savedData = localStorage.getItem(STORAGE_KEY);
    return savedData
        ? JSON.parse(savedData)
        : { sessions: [], activeSessionId: null };
};

const saveSessionData = (data: SessionData): void => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
};

export const getSessionById = (sessionId: string): Session | null => {
    const { sessions } = getSessionData();
    return sessions.find((session) => session.id === sessionId) || null;
};

export const getSessions = (): Session[] => {
    return getSessionData().sessions;
};

export const getActiveSession = (): Session | null => {
    const { sessions, activeSessionId } = getSessionData();
    return activeSessionId
        ? sessions.find((session) => session.id === activeSessionId) || null
        : null;
};

export const setActiveSession = (sessionId: string): void => {
    const data = getSessionData();
    saveSessionData({ ...data, activeSessionId: sessionId });
};

export const saveSessions = (sessions: Session[]): void => {
    const data = getSessionData();
    saveSessionData({ ...data, sessions });
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

export const completeSession = (session: Session) => {
}

export const saveSession = (session: Session, updatedExercise: Exercise): void => {
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
    saveSessionData(updatedData);
};

export const createSession = (): Session => {
    const sessionId = uuidv4();
    const createdOn = new Date().toISOString();

    const newSession: Session = {
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
    saveSessionData({ sessions: updatedSessions, activeSessionId: sessionId });

    return newSession;
};

export const cancelSession = (session: Session): void => {

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
    saveSessionData({ ...data, sessions: updatedSessions, activeSessionId: null });
};

interface ValidateExerciseResult {
    isCorrect: boolean;
    session: Session;
    exercise: Exercise;
}

export const validateExercise = (session: Session, exercise: Exercise, selectedOptionId: number): ValidateExerciseResult => {

    if(exercise.type === "word-image") {
        const result = validateWordImageExercise(session, exercise, selectedOptionId);
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

const validateWordImageExercise = (session: Session, exercise: Exercise, selectedOptionId: number): ValidateExerciseResult => {
    const currentDate = new Date();
    const isCorrect = selectedOptionId === exercise.correctOptionId;
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
    saveSessionData(updatedData);

    return {
        isCorrect,
        session,
        exercise: updatedExercise,
    };
}

const generateExercise = (): Exercise => {
    const shuffledOptions = allOptions.sort(() => Math.random() - 0.5).slice(0, 3);
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
        correctOptionId: correctOption.id,
        options: shuffledOptions,
    };
};

const updateSessionWithExercise = (session: Session, exercise: Exercise): Session => {
    const currentDate = new Date();
    return {
        ...session,
        exercises: [...session.exercises, exercise],
        updatedOn: currentDate.toISOString(),
    };
};

const saveUpdatedSession = (session: Session): void => {
    const data = getSessionData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === session.id ? session : s
    );

    const updatedData = { ...data, sessions: updatedSessions };
    saveSessionData(updatedData);
};

export const createExercise = async (session: Session): Promise<[Exercise, Session]> => {
    const exercise = generateExercise();
    const updatedSession = updateSessionWithExercise(session, exercise);
    saveUpdatedSession(updatedSession);
    return [exercise, updatedSession];
};