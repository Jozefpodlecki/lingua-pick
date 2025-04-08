import { Exercise } from "../models";
import { allOptions } from "./data";

import { v4 as uuidv4 } from "uuid";
import { Session } from "../models";

const SESSIONS_KEY = "sessions";
const ACTIVE_SESSION_KEY = "activeSession";

export const getSessionById = (id: string): Session | null => {
    const sessions = getSessions();
    return sessions.find((session) => session.id === id) || null;
};

export const getSessions = (): Session[] => {
    const savedSessions = localStorage.getItem(SESSIONS_KEY);
    return savedSessions ? JSON.parse(savedSessions) : [];
};

export const saveSessions = (sessions: Session[]): void => {
    localStorage.setItem(SESSIONS_KEY, JSON.stringify(sessions));
};

export const saveSession = (session: Session, updatedExercise: Exercise): void => {
    const updatedSession = {
        ...session,
        exercises: session.exercises.map((ex) =>
            ex.createdOn === updatedExercise.createdOn ? updatedExercise : ex
        ),
    };

    const sessions = getSessions().map((s) => (s.id === session.id ? updatedSession : s));
    saveSessions(sessions);
};

export const createSession = (): Session => {
    const sessionId = uuidv4();
    const newSession: Session = {
        id: sessionId,
        createdOn: new Date().toISOString(),
        exercises: [],
    };

    const sessions = getSessions();
    sessions.push(newSession);
    saveSessions(sessions);

    localStorage.setItem(ACTIVE_SESSION_KEY, sessionId);

    return newSession;
};

export const getActiveSession = (): Session | null => {
    const activeSessionId = localStorage.getItem(ACTIVE_SESSION_KEY);
    if (!activeSessionId) return null;

    const sessions = getSessions();
    return sessions.find((session) => session.id === activeSessionId) || null;
};

export const setActiveSession = (sessionId: string): void => {
    localStorage.setItem(ACTIVE_SESSION_KEY, sessionId);
};

export const createExercise = async (session: Session): Promise<Exercise> => {

    const shuffledOptions = allOptions.sort(() => Math.random() - 0.5).slice(0, 3);
    const correctOption = shuffledOptions[Math.floor(Math.random() * shuffledOptions.length)];
    const question = `${correctOption.word.hangul} (${correctOption.word.romanized})`;

    const exercise: Exercise = {
        id: uuidv4(),
        createdOn: new Date().toISOString(),
        type: "word-image",
        question,
        isCompleted: false,
        isCorrect: null,
        correctOptionId: correctOption.id,
        options: shuffledOptions
    }

    return Promise.resolve(exercise);
};