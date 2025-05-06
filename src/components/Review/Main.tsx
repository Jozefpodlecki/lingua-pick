import React, { useEffect, useState } from "react";
import { getRandomWord, KrWord } from "../../api";
import { Dice3Icon } from "lucide-react";

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
        <main className="flex-1 flex flex-col items-center justify-center h-full bg-black text-white">
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
                className="cursor-pointer mt-10 px-6 py-3 w-32 flex justify-center rounded bg-sky-600 hover:bg-sky-500 active:bg-sky-700 text-xl font-semibold"
            >
                <Dice3Icon/>
            </button>
        </main>
    );
};

export default Main;
