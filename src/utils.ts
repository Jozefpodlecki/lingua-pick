import { HistoryItem } from "./models";

export const generateMockHistory = (): HistoryItem[] => {
    const mockHistory = [];
    for (let i = 1; i <= 100; i++) {
        mockHistory.push({
            word: `Word ${i}`,
            createdOn: new Date().toLocaleTimeString(),
        });
    }
    return mockHistory;
};
