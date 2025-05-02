import { Exercise, ExerciseTypes, Language, QuizSession } from "../models";
import { getAppData, saveAppData } from "./appData";
import { computeDuration } from "./utils";
import { v4 as uuidv4 } from "uuid";

export const getSessionById = (sessionId: string): Promise<QuizSession | null> => {
    const { sessions } = getAppData();
    const session = sessions.find((session) => session.id === sessionId) || null;
    return Promise.resolve(session);
};

export const getSessions = (): QuizSession[] => {
    return getAppData().sessions;
};

export const getActiveSession = (): QuizSession | null => {
    const { sessions, activeSessionId } = getAppData();
    return activeSessionId
        ? sessions.find((session) => session.id === activeSessionId) || null
        : null;
};

export const setActiveSession = (sessionId: string): void => {
    const data = getAppData();
    saveAppData({ ...data, activeSessionId: sessionId });
};

export const saveSessions = (sessions: QuizSession[]): void => {
    const data = getAppData();
    saveAppData({ ...data, sessions });
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

    const data = getAppData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === updatedSession.id ? updatedSession : s
    );

    const updatedData = {
        ...data,
        sessions: updatedSessions,
        activeSessionId: null
    };

    updatedData.stats.sessionCount += 1;

    saveAppData(updatedData);

    return Promise.resolve(updatedSession);
}

export const saveSession = (session: QuizSession, updatedExercise: Exercise): void => {
    const data = getAppData();

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

export interface CreateSessionOptions {
    language: Language;
    exerciseCount: number;
    exerciseTypes: ExerciseTypes[];
}

export const createSession = (options: CreateSessionOptions): QuizSession => {
    const {
        exerciseCount,
        exerciseTypes,
        language
    } = options;
    const sessionId = uuidv4();
    const createdOn = new Date().toISOString();

    const newSession: QuizSession = {
        id: sessionId,
        language,
        isFinished: false,
        duration: {
            seconds: 0,
            hhmmss: "00:00:00",
        },
        createdOn,
        updatedOn: createdOn,
        exercises: [],
        exerciseCount,
        exerciseTypes,
        usedWordIds: []
    };

    const data = getAppData();
    const updatedSessions = [...data.sessions, newSession];
    const appData = {
        ...data,
        sessions: updatedSessions,
        activeSessionId: sessionId,
    };
    saveAppData(appData);

    return newSession;
};

export const cancelSession = (session: QuizSession): void => {

    const currentDate = new Date();
    const currentDateStr = currentDate.toISOString();
    const duration = computeDuration(session.createdOn, currentDate);

    session.isFinished = true;
    session.duration = duration;
    session.canceledOn = currentDateStr;
    session.updatedOn = currentDateStr;

    const data = getAppData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === session.id ? session : s
    );
    
    const appData = { ...data, sessions: updatedSessions, activeSessionId: null }
    appData.stats.sessionCount += 1;

    saveAppData(appData);
};

export const updateSessionWithExercise = (session: QuizSession, exercise: Exercise): QuizSession => {
    const currentDate = new Date();
    return {
        ...session,
        exercises: [...session.exercises, exercise],
        updatedOn: currentDate.toISOString(),
    };
};

export const saveUpdatedSession = (session: QuizSession): void => {
    const data = getAppData();
    const updatedSessions = data.sessions.map((s) =>
        s.id === session.id ? session : s
    );

    const updatedData = { ...data, sessions: updatedSessions };
    saveAppData(updatedData);
};