import { useState, useEffect } from "react";
import { startSpeechRecognition, saveToHistory, loadHistory, clearHistory } from "../../api/speechApi";
import Breadcrumbs from "./Breadcrumbs";
import RecognitionHistory from "./History";
import Prompt from "./Prompt";
import Background from "./Background";
import { HistoryItem } from "../../models";
import { speechState } from "./speechStates";

const SpeechRecognizer: React.FC = () => {
    const [history, setHistory] = useState<HistoryItem[]>(loadHistory());
    const [state, setState] = useState(speechState.idle);

    useEffect(() => {
        const savedHistory = loadHistory();
        setHistory(savedHistory);
    }, []);

    const onClear = () => {
        clearHistory();
        setHistory([]);
    };

    const onSpeechRecognition = () => {
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
                    createdOn: new Date().toISOString(),
                };
                setHistory((prev) => [...prev, newHistoryItem]);
                saveToHistory(newHistoryItem);
                setState(speechState.recognized(transcript));
            },
            onError: (errorMessage) => {
                setState(speechState.error(errorMessage));
            },
            onEnd: () => {
                // setState(speechState.idle);
            },
        });
    };

    return (
        <div className="relative min-h-screen">
            <Background/>
            <Breadcrumbs current="Speech Recognition" />
            <RecognitionHistory history={history} onClear={onClear} />
            <div className="flex flex-col items-center justify-center" style={{ minHeight: "calc(100vh - 4rem)" }}>
                <Prompt state={state} onSpeechRecognition={onSpeechRecognition} />
            </div>
        </div>
    );
};

export default SpeechRecognizer;