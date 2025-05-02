import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { createSession } from "../api";
import { useLanguage } from "../context/LanguageContext";

const exerciseTypes = [
    { type: "word-image", description: "Match the word to the correct image." },
    { type: "word-word", description: "Match the word to its translation." },
    { type: "sentence-typing", description: "Type the sentence in the target language." },
];

const NewQuiz: React.FC = () => {
    const [exerciseCount, setExerciseCount] = useState<number>(5);
    const { selectedLanguage } = useLanguage();
    const [selectedExerciseTypes, setSelectedExerciseTypes] = useState<string[]>([]);
    const navigate = useNavigate();

    const onExerciseTypeChange = (type: string) => {
        setSelectedExerciseTypes((prev) =>
            prev.includes(type)
                ? prev.filter((t) => t !== type)
                : [...prev, type]
        );
    };

    const onExerciseCountChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const { value } = e.target.dataset;
        if (value) {
            setExerciseCount(Number(value));
        }
    };

    const onStartQuiz = () => {
        const newSession = createSession({
            language: selectedLanguage!,
            exerciseCount,
            exerciseTypes: selectedExerciseTypes as any,
        });
        navigate(`/session/${newSession.id}`);
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <div>
                <h1 className="text-3xl font-bold mb-6">Start a New Quiz</h1>
                <div className="mb-6">
                    <label htmlFor="exerciseCount" className="block text-lg font-medium mb-2">
                        Number of Exercises
                    </label>
                    <select
                        id="exerciseCount"
                        value={exerciseCount}
                        onChange={onExerciseCountChange}
                        className="bg-gray-800 text-white py-2 px-4 rounded"
                    >
                        {[5, 10, 15, 20].map((count) => (
                            <option key={count} value={count} data-value={count}>
                                {count}
                            </option>
                        ))}
                    </select>
                </div>
                <div className="mb-6">
                    <label className="block text-lg font-medium mb-2">Select Exercise Types</label>
                    <ul className="list-none space-y-4">
                        {exerciseTypes.map((et) => (
                            <li key={et.type} className="flex items-center space-x-2">
                                <input
                                    type="checkbox"
                                    id={et.type}
                                    checked={selectedExerciseTypes.includes(et.type)}
                                    onChange={() => onExerciseTypeChange(et.type)}
                                    className="form-checkbox text-blue-600"
                                />
                                <label htmlFor={et.type} className="text-white">
                                    {et.description}
                                </label>
                            </li>
                        ))}
                    </ul>
                </div>
                <button
                    onClick={onStartQuiz}
                    className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    Start Quiz
                </button>
            </div>
        </div>
    );
};

export default NewQuiz;