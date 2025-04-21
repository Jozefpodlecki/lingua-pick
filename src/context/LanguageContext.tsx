import React, { createContext, useContext, useState } from "react";
import { getAppData, setSelectedLanguage } from "../api";
import { Language } from "../models";

interface LanguageContextProps {
    selectedLanguage: Language | undefined;
    setLanguage: (language: string) => void;
    availableLanguages: Language[];
}

const LanguageContext = createContext<LanguageContextProps | undefined>(undefined);

export const LanguageProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {

    const appData = getAppData();

    const [selectedLanguage, setSelectedLanguageState] = useState<Language>(appData.selectedLanguage);
    const availableLanguages = appData.availableLanguages;

    const setLanguage = (isoCode: string) => {
      
        const language = availableLanguages
            .find((lang) => lang.isoCode === isoCode)!;
        
        setSelectedLanguage(language);
        setSelectedLanguageState(language);
    };

    return (
        <LanguageContext.Provider value={{ selectedLanguage, setLanguage, availableLanguages }}>
            {children}
        </LanguageContext.Provider>
    );
};

export const useLanguage = (): LanguageContextProps => {
    const context = useContext(LanguageContext);
    if (!context) {
        throw new Error("useLanguage must be used within a LanguageProvider");
    }
    return context;
};