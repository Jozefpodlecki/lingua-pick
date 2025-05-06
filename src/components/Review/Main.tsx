import React, { useEffect, useState } from "react";
import { getRandomWord, KrWord } from "../../api";
import { Dice3Icon } from "lucide-react";
import Breadcrumbs from "./Breadcrumbs";

const Main: React.FC = () => {
    const [word, setWord] = useState<KrWord | null>(null);

    const fetchWord = () => {
        const newWord = getRandomWord();
        setWord(newWord);
    };

    useEffect(() => {
        fetchWord();
    }, []);

    return (
        <main className="relative min-h-screen flex items-center justify-center bg-black text-white">
        <div className="absolute top-4 left-4">
            <Breadcrumbs selectedTitle="Review" />
        </div>
    
        <div className="flex flex-col items-center justify-center">
            {word && (
                <div className="text-center space-y-4">
                    <h1 className="text-6xl font-bold">{word.hangul}</h1>
                    <p className="text-2xl text-gray-300">{word.romanized}</p>
                    <div className="text-xl text-gray-400">
                        {word.translations.join(", ")}
                    </div>
                </div>
            )}
            <button
                onClick={fetchWord}
                className="cursor-pointer mt-10 px-6 py-3 w-32 flex justify-center items-center rounded bg-sky-600 hover:bg-sky-500 active:bg-sky-700 text-xl font-semibold"
            >
                <Dice3Icon />
            </button>
        </div>
    </main>
    );
};

export default Main;
