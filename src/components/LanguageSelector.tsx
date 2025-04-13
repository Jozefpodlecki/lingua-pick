import React from "react";
import { useLanguage } from "../context/LanguageContext";

const LanguageSelector: React.FC = () => {
    const { selectedLanguage, availableLanguages, setLanguage } = useLanguage();

    const onChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        setLanguage(event.target.value);
    };

    return (
        <div className="flex items-center space-x-2">
            <img
                src={selectedLanguage?.flagUrl}
                alt={`${selectedLanguage} flag`}
                className="w-6 h-6"
            />
            <select
                id="language"
                value={selectedLanguage?.isoCode}
                onChange={onChange}
                className="bg-gray-800 text-white py-2 px-4 rounded"
            >
                {availableLanguages.map((language) => (
                    <option key={language.isoCode} value={language.isoCode}>
                        {language.name}
                    </option>
                ))}
            </select>
        </div>
    );
};

export default LanguageSelector;