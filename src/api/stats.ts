import { Option, AppData, Exercise, UserStats, Stat } from "../models";
import { getAppData, saveAppData } from "./appData";

const defaultStat = (): Stat => {
    return {
        ratio: 0,
        correctCount: 0,
        incorrectCount: 0,
        updatedOn: ""
    }
}

export const updateStats = (exercise: Exercise) => {
    const data = getAppData();
    
    // updateAppDataStats(updatedSession, exercise.correctOptionId, isCorrect);

    switch(exercise.type) {
        case "word-word":
            updateWordStats(data.stats, exercise.selectedOptionId, exercise.isCorrect);
            break;
        case "hangul-match":
            updateHangulStats(
                data.stats,
                exercise.correctIds,
                exercise.incorrectIds,
                exercise.items);
            break;
    }

    // const updatedSessions = data.sessions.map((s) => (s.id === session.id ? session : s));
    saveAppData(data);
}

export const updateHangulStats = (
    stats: UserStats,
    correctIds: Set<number>,
    incorrectIds: Set<number>,
    items: Option[]): void => {
    
    for(const item of items) {

        if(correctIds.has(item.id)) {
            const stat = stats.hangul[item.id] || defaultStat();
            stat.correctCount++;
            stat.ratio = stat.incorrectCount ? stat.correctCount / stat.incorrectCount : 0;
            stat.updatedOn = new Date().toISOString();
            stats.hangul[item.id] = stat;
        }

        if(incorrectIds.has(item.id)) {
            const stat = stats.hangul[item.id] || defaultStat();
            stat.incorrectCount++;
            stat.ratio = stat.incorrectCount ? stat.correctCount / stat.incorrectCount : 0;
            stat.updatedOn = new Date().toISOString();
            stats.hangul[item.id] = stat;
        }
        
    }
}

export const updateWordStats = (stats: UserStats, wordId: number, isCorrect: boolean): void => {
    const currentDate = new Date().toISOString();

    const wordStats = stats.vocabulary.words.find((word) => word.id === wordId);
    if (wordStats) {
        if (isCorrect) {
            wordStats.correctCount = (wordStats.correctCount || 0) + 1;
        } else {
            wordStats.incorrectCount = (wordStats.incorrectCount || 0) + 1;
        }

        const total = wordStats.correctCount + wordStats.incorrectCount;

        if (total > 0) {
            wordStats.ratio = wordStats.correctCount / total;
        } else {
            wordStats.ratio = 0;
        }

        wordStats.updatedOn = currentDate;
        
        return;
    }

    const newWordStats = {
        id: wordId,
        correctCount: isCorrect ? 1 : 0,
        incorrectCount: isCorrect ? 0 : 1,
        updatedOn: currentDate,
        ratio: 0
    };
    stats.vocabulary.words.push(newWordStats);
};

export const getStats = () => {
    const appData: AppData = getAppData();
    return appData.stats;
};