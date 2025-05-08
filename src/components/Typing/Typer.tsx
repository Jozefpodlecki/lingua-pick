import React, { useEffect, useState } from "react";
import Breadcrumbs from "./Breadcrumbs";
import { convertRomanizedToHangul } from "./romanToHangulMap";
import useSpeechSynthesis from "@/hooks/useSpeechSynthesis";
import { Play } from "lucide-react";
import { getRandomSentence } from "@/api";

interface State {
    hangul: string;
    romanized: string;
    translation: string;
}

const Typer: React.FC = () => {
    const [sentence, setSentence] = useState<State | null>(null);
    const [text, setText] = useState("");
    const [convertedText, setConvertedText] = useState("");
    const [loading, setLoading] = useState(true);
    const { speak } = useSpeechSynthesis("ko-KR");

    useEffect(() => {
        const fetchSentence = async () => {
            setLoading(true);
            const randomSentence = getRandomSentence();
            setSentence(randomSentence);
            setLoading(false);
        };

        fetchSentence();
    }, []);

    const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const input = e.target.value;
        const converted = convertRomanizedToHangul(input);
        setConvertedText(converted);
        setText(input);
    };

    const onPlay = async (event: React.MouseEvent<HTMLButtonElement>) => {
        const hangul = event.currentTarget.getAttribute("data-hangul")!;
        await speak(hangul);
    };

    const hangulMapping = sentence
        ? sentence.hangul.split("").map((char, index) => ({
              char,
              isMatched: convertedText[index] === char,
          }))
        : [];

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
            <div className="absolute top-4 left-4">
                <Breadcrumbs selectedTitle={"Typing"} />
            </div>
            {loading ? (
                <div className="flex items-center justify-center h-full">
                    <p className="text-2xl font-bold text-gray-400">Loading...</p>
                </div>
            ) : (
                <>
                    <div className="mb-4 text-center">
                        <div className="flex items-center justify-center space-x-4">
                            <p
                                data-hangul={sentence?.hangul}
                                className="text-6xl font-bold"
                            >
                                {hangulMapping.map(({ char, isMatched }, index) => (
                                    <span
                                        key={index}
                                        className={isMatched ? "text-green-500" : "text-white"}
                                    >
                                        {char}
                                    </span>
                                ))}
                            </p>
                            <button
                                onClick={onPlay}
                                data-hangul={sentence?.hangul}
                                className="cursor-pointer p-2 bg-blue-600 hover:bg-blue-700 text-white rounded-full focus:outline-none focus:ring-2 focus:ring-blue-500"
                            >
                                <Play className="w-6 h-6" />
                            </button>
                        </div>
                        <p className="text-gray-400 text-lg mt-2">{sentence?.translation}</p>
                        <p className="text-gray-400 text-lg mt-2">{sentence?.romanized}</p>
                    </div>
                    <div className="relative w-3/4">
                        <textarea
                            value={text}
                            onChange={handleInputChange}
                            placeholder="Type here..."
                            className="w-full text-2xl h-32 p-4 rounded bg-gray-800 text-white border border-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                        />
                        <div className="absolute top-full left-0 mt-2 p-4 rounded shadow-lg bg-gray-800">
                            <p className="text-xl font-semibold">{convertedText}</p>
                        </div>
                    </div>
                </>
            )}
        </div>
    );
};

export default Typer;