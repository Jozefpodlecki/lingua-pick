import React from "react";
import { Option } from "../../models";

interface Props {
    isCompleted: boolean;
    isCorrect: boolean | null;
    correctOption: Option;
    onClick: () => void;
    isDisabled: boolean;
}

const BottomPanel: React.FC<Props> = ({ isCompleted, isCorrect, correctOption, onClick, isDisabled }) => {
    return (
        <div
            className={`flex items-center p-4 text-right transition-colors duration-300 ${
                isCompleted
                    ? isCorrect
                        ? "bg-[#568203]"
                        : "bg-[#58111A]"
                    : "bg-gray-800"
            }`}
        >
            <div className="text-white text-lg mb-4">
                <span className="text-2xl">
                    {isCompleted &&
                        (isCorrect
                            ? "That's right!"
                            : `That's wrong, it's ${correctOption.translations[0]} ${correctOption.word.hangul} (${correctOption.word.romanized})`)}
                </span>
            </div>
            <button
                type="button"
                disabled={isDisabled}
                onClick={onClick}
                className="cursor-pointer rounded bg-gray-600 p-4 ml-auto text-2xl text-white hover:bg-sky-500 active:bg-sky-700"
            >
                {isCompleted ? "Continue" : "Check"}
            </button>
        </div>
    );
};

export default BottomPanel;