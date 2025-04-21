import { AppData, UserStats } from "../models";
import { getAppData } from "./appData";

export const updateStats = (stats: UserStats, wordId: number, isCorrect: boolean): void => {
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