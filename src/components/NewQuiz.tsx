import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { createSession } from "../api";
import { useLanguage } from "../context/LanguageContext";

const NewQuiz: React.FC = () => {
    const [exerciseCount, setExerciseCount] = useState<number>(5);
    const { selectedLanguage } = useLanguage();
    const navigate = useNavigate();

    const handleStartQuiz = () => {
        const newSession = createSession(selectedLanguage!, exerciseCount);
        navigate(`/session/${newSession.id}`);
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <h1 className="text-3xl font-bold mb-6">Start a New Quiz</h1>
            <div className="mb-6">
                <label htmlFor="exerciseCount" className="block text-lg font-medium mb-2">
                    Number of Exercises
                </label>
                <select
                    id="exerciseCount"
                    value={exerciseCount}
                    onChange={(e) => setExerciseCount(Number(e.target.value))}
                    className="bg-gray-800 text-white py-2 px-4 rounded"
                >
                    {[5, 10, 15, 20].map((count) => (
                        <option key={count} value={count}>
                            {count}
                        </option>
                    ))}
                </select>
            </div>
            <button
                onClick={handleStartQuiz}
                className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            >
                Start Quiz
            </button>
        </div>
    );
};

export default NewQuiz;