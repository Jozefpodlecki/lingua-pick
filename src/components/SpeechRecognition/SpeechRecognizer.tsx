import { useState, useEffect } from "react";
import { startSpeechRecognition, saveToHistory, loadHistory } from "../../api/speechApi";
import Breadcrumbs from "./Breadcrumbs";
import RecognitionHistory from "./History";
import Prompt from "./Prompt";
import Background from "./Background";
import { HistoryItem } from "@models";
import { generateMockHistory } from "../../utils";
import { speechState } from "./speechStates";

const SpeechRecognizer: React.FC = () => {
    const [history, setHistory] = useState<HistoryItem[]>(generateMockHistory());
    const [state, setState] = useState(speechState.idle);

    useEffect(() => {
        const savedHistory = loadHistory();
        setHistory(savedHistory);
    }, []);

    const clearHistory = () => {
        setHistory([]);
    };

    const handleSpeechRecognition = () => {
        if (state.type === "recording") {
            setState(speechState.idle);
            return;
        }

        startSpeechRecognition({
            onStart: () => {
                setState(speechState.recording);
            },
            onResult: (transcript) => {
                const newHistoryItem: HistoryItem = {
                    word: transcript,
                    createdOn: new Date().toLocaleTimeString(),
                };
                setHistory((prev) => [...prev, newHistoryItem]);
                saveToHistory(newHistoryItem);
                setState(speechState.recognized(transcript));
            },
            onError: (errorMessage) => {
                setState(speechState.error(errorMessage));
            },
            onEnd: () => {
                setState(speechState.idle);
            },
        });
    };

    return (
        <div className="relative min-h-screen">
            <Background/>
            <Breadcrumbs current="Speech Recognition" />
            <RecognitionHistory history={history} clearHistory={clearHistory} />
            <div className="flex flex-col items-center justify-center" style={{ minHeight: "calc(100vh - 4rem)" }}>
                <Prompt state={state} handleSpeechRecognition={handleSpeechRecognition} />
            </div>
        </div>
    );
};

export default SpeechRecognizer;