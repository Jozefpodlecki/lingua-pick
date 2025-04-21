import { Language } from "../models";
import { getAppData, saveAppData } from "./appData";

export const setSelectedLanguage = (language: Language): void => {
    const data = getAppData();
    saveAppData({ ...data, selectedLanguage: language });
};
