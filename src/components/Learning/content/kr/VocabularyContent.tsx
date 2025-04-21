import React, { useState } from "react";
import { Play, Loader } from "lucide-react";
import useSpeechSynthesis from "../../../../hooks/useSpeechSynthesis";

const VocabularyContent: React.FC = () => {
    const { speak } = useSpeechSynthesis("ko-KR");
    const [speakingWord, setSpeakingWord] = useState<string | null>(null);

    const words = [
        { hangul: "안녕하세요", romanized: "Annyeong-haseyo", english: "Hello" },
        { hangul: "감사합니다", romanized: "Gamsa-hamnida", english: "Thank you" },
        { hangul: "사랑", romanized: "Sa-rang", english: "Love" },
        { hangul: "행복", romanized: "Haeng-bok", english: "Happiness" },
        { hangul: "친구", romanized: "Chin-gu", english: "Friend" },
        { hangul: "학교", romanized: "Hak-gyo", english: "School" },
        { hangul: "음식", romanized: "Eum-sik", english: "Food" },
        { hangul: "여행", romanized: "Yeo-haeng", english: "Travel" },
        { hangul: "책", romanized: "Chaek", english: "Book" },
        { hangul: "음악", romanized: "Eum-ak", english: "Music" },
    ];

    const onPlay = async (event: React.MouseEvent<HTMLButtonElement>) => {
        const hangul = event.currentTarget.getAttribute("data-hangul");
        if (hangul) {
            setSpeakingWord(hangul);
            try {
                await speak(hangul);
            } catch (error) {
                console.error(`Error speaking: ${error}`);
            } finally {
                setSpeakingWord(null);
            }
        }
    };

    return (
        <div>
            <h1 className="text-3xl font-bold mb-4">Vocabulary</h1>
            <table className="table-auto w-full border-collapse border border-gray-700">
                <thead>
                    <tr>
                        <th className="border border-gray-700 px-4 py-2">Korean</th>
                        <th className="border border-gray-700 px-4 py-2">Romanized</th>
                        <th className="border border-gray-700 px-4 py-2">English</th>
                        <th className="border border-gray-700 px-4 py-2">Play</th>
                    </tr>
                </thead>
                <tbody>
                    {words.map((word, index) => (
                        <tr key={index}>
                            <td className="border border-gray-700 px-4 py-2">{word.hangul}</td>
                            <td className="border border-gray-700 px-4 py-2">{word.romanized}</td>
                            <td className="border border-gray-700 px-4 py-2">{word.english}</td>
                            <td className="border border-gray-700 px-4 py-2 text-center">
                                <button
                                    data-hangul={word.hangul}
                                    onClick={onPlay}
                                    disabled={!!speakingWord}
                                    className={` ${
                                        speakingWord ? "text-gray-400" : "cursor-pointer text-blue-500 hover:text-blue-700"
                                    }`}
                                >
                                    {speakingWord === word.hangul ? (
                                        <Loader className="w-5 h-5 animate-spin inline" />
                                    ) : (
                                        <Play className="w-5 h-5 inline" />
                                    )}
                                </button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

export default VocabularyContent;