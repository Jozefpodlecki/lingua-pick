import { Routes, Route } from "react-router-dom";
import Landing from "./components/Landing";
import Session from "./components/Session";
import SpeechRecognition from "./components/SpeechRecognition/SpeechRecognizer";

const App = () => {
    return (
        <div className="flex flex-col min-h-screen">
            <Routes>
                <Route path="/" element={<Landing />} />
                <Route path="/session/:uuid" element={<Session />} />
                <Route path="/speech-recognition" element={<SpeechRecognition />} />
            </Routes>
        </div>
    );
};

export default App;