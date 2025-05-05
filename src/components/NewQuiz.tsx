import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { createSession } from "../api";
import { useLanguage } from "../context/LanguageContext";
import { getExerciseTypes } from "../api/exercise";

const exerciseCounts = [5, 10, 15, 20];

const NewQuiz: React.FC = () => {
    const [exerciseCount, setExerciseCount] = useState<number>(5);
    const { selectedLanguage } = useLanguage();
    const [selectedExerciseTypes, setSelectedExerciseTypes] = useState<string[]>([]);
    const navigate = useNavigate();
    const availableExerciseTypes = getExerciseTypes(selectedLanguage!.isoCode);

    const onExerciseTypeChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const type = event.target.dataset.type!;
        setSelectedExerciseTypes((prev) =>
            prev.includes(type)
                ? prev.filter((t) => t !== type)
                : [...prev, type]
        );
    };

    const onExerciseCountChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        const value = Number(event.target.value);
        setExerciseCount(value);
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
                        {exerciseCounts.map((count) => (
                            <option key={count} value={count} data-value={count}>
                                {count}
                            </option>
                        ))}
                    </select>
                </div>
                <div className="mb-6">
                    <label className="block text-lg font-medium mb-2">Select Exercise Types</label>
                    <ul className="list-none space-y-4">
                        {availableExerciseTypes.map((et) => (
                            <li key={et.type} className="flex items-center space-x-2">
                                <input
                                    data-type={et.type}
                                    type="checkbox"
                                    id={et.type}
                                    checked={selectedExerciseTypes.includes(et.type)}
                                    onChange={onExerciseTypeChange}
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
                    type="button"
                    disabled={selectedExerciseTypes.length === 0}
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