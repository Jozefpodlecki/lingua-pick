import { Routes, Route } from "react-router-dom";
import Landing from "./components/Landing";
import QuizSession from "./components/Session/Main";
import DocsMain from "./components/Docs/Main";
import SpeechRecognition from "./components/SpeechRecognition/SpeechRecognizer";

const App = () => {
    return (
        <div className="flex flex-col min-h-screen">
            <Routes>
                <Route path="/" element={<Landing />} />
                <Route path="/session/:uuid" element={<QuizSession />} />
                <Route path="/speech-recognition" element={<SpeechRecognition />} />
                <Route path="/docs" element={<DocsMain />} />
            </Routes>
        </div>
    );
};

export default App;