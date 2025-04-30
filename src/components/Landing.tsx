import { Link } from "react-router-dom";
import { Play, Mic, BookOpen, InfoIcon, BarChart2, Keyboard } from "lucide-react";
import LanguageSelector from "./LanguageSelector";
import { useEffect, useState } from "react";
import { getStats } from "../api";

const Landing: React.FC = () => {
    const [stats, setStats] = useState<{
        sessionCount: number;
        totalWords: number;
        correctAnswers: number;
        incorrectAnswers: number;
    }>({
        sessionCount: 0,
        totalWords: 0,
        correctAnswers: 0,
        incorrectAnswers: 0,
    });

    useEffect(() => {
        const fetchStats = async () => {
            const statsData = getStats();
            setStats({
                sessionCount: statsData.sessionCount,
                totalWords: statsData.vocabulary.words.length,
                correctAnswers: statsData.vocabulary.words.reduce(
                    (sum, word) => sum + word.correctCount,
                    0
                ),
                incorrectAnswers: statsData.vocabulary.words.reduce(
                    (sum, word) => sum + word.incorrectCount,
                    0
                ),
            });
        };

        fetchStats();
    }, []);

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <div className="absolute top-6 left-6">
                <label htmlFor="language" className="block text-lg font-medium mb-2">
                    Select Language
                </label>
                <LanguageSelector />
            </div>
            <div className="mb-8 text-center">
                <h2 className="text-2xl font-bold mb-2 flex items-center justify-center">
                    <BarChart2 className="w-6 h-6 mr-2" />
                    Your Stats
                </h2>
                <p className="text-lg">Total Sessions: {stats.sessionCount}</p>
                <p className="text-lg">Words Learned: {stats.totalWords}</p>
                <p className="text-lg">Correct Answers: {stats.correctAnswers}</p>
                <p className="text-lg">Incorrect Answers: {stats.incorrectAnswers}</p>
            </div>
            <div className="grid grid-cols-2 gap-6">
                <Link
                    to="/session/new"
                    className="flex flex-col items-center justify-center bg-blue-600 hover:bg-blue-700 text-white font-bold py-6 px-4 rounded shadow-lg"
                >
                    <Play className="w-8 h-8 mb-2" />
                    Start Quiz
                </Link>
                <Link
                    to="/speech-recognition"
                    className="flex flex-col items-center justify-center bg-gray-600 hover:bg-gray-700 text-white font-bold py-6 px-4 rounded shadow-lg"
                >
                    <Mic className="w-8 h-8 mb-2" />
                    Practice Speaking
                </Link>
                <Link
                    to="/learning"
                    className="flex flex-col items-center justify-center bg-purple-600 hover:bg-purple-700 text-white font-bold py-6 px-4 rounded shadow-lg"
                >
                    <BookOpen className="w-8 h-8 mb-2" />
                    Learning
                </Link>
                <Link
                    to="/docs"
                    className="flex flex-col items-center justify-center bg-green-600 hover:bg-green-700 text-white font-bold py-6 px-4 rounded shadow-lg"
                >
                    <InfoIcon className="w-8 h-8 mb-2" />
                    Documentation
                </Link>
                <Link
                    to="/typing"
                    className="flex flex-col items-center justify-center bg-yellow-600 hover:bg-yellow-700 text-white font-bold py-6 px-4 rounded shadow-lg"
                >
                    <Keyboard className="w-8 h-8 mb-2" />
                    Typing
                </Link>
            </div>
        </div>
    );
};

export default Landing;