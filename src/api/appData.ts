import { AppData } from "../models";
import { v4 as uuidv4 } from "uuid";

const STORAGE_KEY = "appData";

const getDefaultAppData = (): AppData => {
    const id = uuidv4();
    const createdOn = new Date().toISOString();

    const availableLanguages = [
        { name: "Korean", isoCode: "kr", flagUrl: "https://flagcdn.com/w40/kr.png" },
        { name: "Chinese", isoCode: "cn", flagUrl: "https://flagcdn.com/w40/cn.png" },
    ];

    return {
        id,
        createdOn,
        sessions: [],
        activeSessionId: null,
        stats: {
            sessionCount: 0,
            updatedOn: new Date().toISOString(),
            hangul: {},
            vocabulary: {
                words: []
            }
        },
        availableLanguages,
        selectedLanguage: availableLanguages[0]
    };
}

let appData: AppData | null = null;

export const getAppData = (): AppData => {

    if(appData) {
        return appData;
    }

    const savedData = localStorage.getItem(STORAGE_KEY);

    const result = savedData
        ? JSON.parse(savedData)
        : getDefaultAppData();

    appData = result;

    return result;
};

export const saveAppData = (data: AppData): void => {
    appData = data;
    // localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
};