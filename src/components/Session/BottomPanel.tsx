import React from "react";
import { Exercise } from "@/models";
import { getFeedbackStyleAndText } from "./utils";

interface Props {
    exercise: Exercise;
    onClick(): void;
}

const BottomPanel: React.FC<Props> = ({ exercise, onClick }) => {
    const { bgColor, message } = getFeedbackStyleAndText(exercise);
    const showButton = exercise.requiresManualCheck || exercise.isCompleted;
    const isCorrect = ("isCorrect" in exercise && exercise.isCorrect) || true;

    return (
        <div
            className={`flex items-center p-6 text-right transition-colors duration-300 ${bgColor} rounded-lg shadow-md`}
        >
            <div className="text-white text-lg mb-2">
                <span className="text-2xl font-semibold">{message}</span>
            </div>

            {showButton && (
                <button
                    type="button"
                    onClick={onClick}
                    className={`cursor-pointer rounded-md p-4 ml-auto text-xl text-white hover:bg-opacity-80 active:bg-opacity-90 transition-all ease-in-out duration-150 transform hover:scale-105 
                    ${exercise.isCompleted
                        ? isCorrect
                            ? "bg-green-900 hover:bg-green-900 active:bg-green-800"
                            : "bg-red-600 hover:bg-red-700 active:bg-red-800"
                        : "bg-gray-600 hover:bg-gray-700 active:bg-gray-800"}`}
                >
                    {exercise.isCompleted ? "Continue" : "Check"}
                </button>
            )}
        </div>
    )
};

export default BottomPanel;