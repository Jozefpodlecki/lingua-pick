import { Mic, StopCircle } from "lucide-react";
import React from "react";
import { SpeechRecognitionState } from "../../models";

interface Props {
    state: SpeechRecognitionState;
    onSpeechRecognition: () => void;
}

const Prompt: React.FC<Props> = ({ state, onSpeechRecognition }) => {
    const renderContent = () => {
        switch (state.type) {
            case "error":
                return (
                    <div className="mt-4 flex items-center text-red-500">
                        <Mic className="w-6 h-6 mr-2" />
                        <p>{state.message}</p>
                    </div>
                );
            case "recognized":
                return (
                    <p className="mt-4 text-lg">
                        Recognized Text: <span className="font-bold">{state.text}</span>
                    </p>
                );
            case "idle":
                return <p className="text-lg mb-8">Speak something in Korean, and we'll recognize it!</p>;
            default:
                return null;
        }
    };

    return (
        <div className="flex flex-col items-center justify-center">
            <div className="flex items-center justify-center mb-4">
                <button
                    onClick={onSpeechRecognition}
                    className={`flex items-center ${
                        state.type === "recording" ? "bg-red-600 animate-pulse" : "bg-gray-600"
                    } hover:bg-gray-700 text-white font-bold py-2 px-4 rounded`}
                >
                    {state.type === "recording" ? (
                        <>
                            <StopCircle className="w-5 h-5 mr-2" />
                            Recording...
                        </>
                    ) : (
                        <>
                            <Mic className="w-5 h-5 mr-2" />
                            Start Recording
                        </>
                    )}
                </button>
            </div>
            {renderContent()}
        </div>
    );
};

export default Prompt;