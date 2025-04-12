import React from "react";

interface Props {
    value: number;
    onCancel: () => void;
}

const ProgressBar: React.FC<Props> = ({ value, onCancel }) => {
    return (
        <div className="flex items-center justify-center my-4 w-full">
            <button
                onClick={onCancel}
                className="text-gray-300 hover:text-white p-2"
            >
                ✕
            </button>
            <div
                className="w-3/4 bg-gray-700 h-4 rounded-full overflow-hidden ml-2"
                data-value={`${value}%`}
            >
                <div
                    className="bg-blue-500 h-4 transition-all duration-300"
                    style={{ width: `${value}%` }}
                ></div>
            </div>
        </div>
    );
};

export default ProgressBar;