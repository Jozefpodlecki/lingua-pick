import React from "react";
import { Exercise } from "../../models";

interface Props {
    exercise: Exercise;
    onClick(): void;
}

const BottomPanel: React.FC<Props> = ({ exercise, onClick }) => {
    return (
        <div
            className={`flex items-center p-4 text-right transition-colors duration-300 ${
                exercise.isCompleted
                    ? exercise.isCorrect
                        ? "bg-[#568203]"
                        : "bg-[#58111A]"
                    : "bg-gray-800"
            }`}
        >
            <div className="text-white text-lg mb-4">
                <span className="text-2xl">
                    {exercise.isCompleted &&
                        (exercise.isCorrect
                            ? "That's right!"
                            : `That's wrong, it's ${""})`)}
                </span>
            </div>
            {exercise.requiresManualCheck ?  <button
                type="button"
                disabled={false}
                onClick={onClick}
                className="cursor-pointer rounded bg-gray-600 p-4 ml-auto text-2xl text-white hover:bg-sky-500 active:bg-sky-700"
            >
                {exercise.isCompleted ? "Continue" : "Check"}
            </button> : null}
        </div>
    );
};

export default BottomPanel;