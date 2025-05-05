import { BaseExercise, Exercise, ExerciseType, QuizSession, UserStats } from "../../models";
import { getAppData, saveAppData } from "../appData";
import { getKrWords } from "../kr";
import { updateStats } from "../stats";
import { computeDuration } from "../utils";
import { v4 as uuidv4 } from "uuid";

export const getExerciseTypes = (languageName: string): ExerciseType[] => {
    const baseTypes: ExerciseType[] = [
        { type: "word-image", description: "Match the word to the correct image." },
        { type: "word-word", description: "Match the word to its translation." },
        { type: "sentence-typing", description: "Type the sentence in the target language." },
        { type: "words-words", description: "Match each word with its correct translation." },
        { type: "words-match", description: "Match each word in a grid with its correct translation." },
    ];

    if(languageName === "kr") {
        baseTypes.push({ type: "hangul-match", description: "Match Hangul syllables to their romanized form." });
    }

    return baseTypes;
};

export const createBaseExercise = (): BaseExercise => {
    const now = new Date();
    return {
        id: uuidv4(),
        createdOn: now.toISOString(),
        duration: { seconds: 0, hhmmss: "00:00:00" },
        isCompleted: false,
        isCorrect: null,
        requiresManualCheck: false
    };
};

export const getRandomItem = <T>(arr: T[]): T => arr[Math.floor(Math.random() * arr.length)];

export const shuffleArray = <T>(array: T[]): T[] => {
    return [...array].sort(() => Math.random() - 0.5);
};

export const getAvailableWords = (stats: UserStats, session: QuizSession) => {
    const hasStats = stats.vocabulary?.words?.length > 0;

    const weakWordIds = hasStats
        ? new Set(
            stats.vocabulary.words
                .filter((entry) => entry.ratio < 0.5)
                .map((entry) => entry.id)
        )
        : null;
    
    const availableWords = getKrWords().filter((word) =>
        !session.usedWordIds.includes(word.id) &&
        (weakWordIds ? weakWordIds.has(word.id) : true)
    );
    
    return availableWords;
}
    

export const updateSessionState = (
    session: QuizSession,
    updatedExercise: Exercise,
    now: Date
): QuizSession => {
    const updatedExercises = session.exercises.map((ex) =>
        ex.id === updatedExercise.id ? updatedExercise : ex
    );
    return {
        ...session,
        updatedOn: now.toISOString(),
        exercises: updatedExercises,
        duration: computeDuration(session.createdOn, now),
    };
};

export const updateAppDataStats = (
    session: QuizSession,
    id: number,
    isCorrect: boolean
) => {
    const data = getAppData();
    updateStats(data.stats, id, isCorrect);
    const updatedSessions = data.sessions.map((s) => (s.id === session.id ? session : s));
    saveAppData({ ...data, sessions: updatedSessions });
};