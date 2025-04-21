import { AppData } from "../models";

const STORAGE_KEY = "appData";

const getDefaultAppData = (): AppData => {
    const availableLanguages = [
        { name: "Korean", isoCode: "kr", flagUrl: "https://flagcdn.com/w40/kr.png" },
        { name: "Chinese", isoCode: "cn", flagUrl: "https://flagcdn.com/w40/cn.png" },
    ];

    return {
        sessions: [],
        activeSessionId: null,
        stats: {
            sessionCount: 0,
            updatedOn: new Date().toISOString(),
            vocabulary: {
                words: []
            }
        },
        availableLanguages,
        selectedLanguage: availableLanguages[0]
    };
}

export const getAppData = (): AppData => {
    const savedData = localStorage.getItem(STORAGE_KEY);

    const result = savedData
        ? JSON.parse(savedData)
        : getDefaultAppData();

    return result;
};

export const saveAppData = (data: AppData): void => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
};