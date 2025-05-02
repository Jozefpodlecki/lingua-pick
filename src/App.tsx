import { Routes, Route } from "react-router-dom";
import Landing from "./components/Landing";
import QuizSession from "./components/Session/Main";
import DocsMain from "./components/Docs/Main";
import SpeechRecognition from "./components/SpeechRecognition/SpeechRecognizer";
import { LanguageProvider } from "./context/LanguageContext";
import NewQuiz from "./components/NewQuiz";
import Learning from "./components/Learning/Main";
import Typer from "./components/Typing/Typer";

const App = () => {
    return (
        <LanguageProvider>
            <div className="flex flex-col min-h-screen">
                <Routes>
                    <Route path="/" element={<Landing />} />
                    <Route path="/typing" element={<Typer />} />
                    <Route path="/learning" element={<Learning />} />
                    <Route path="/session/new" element={<NewQuiz />} />
                    <Route path="/session/:uuid" element={<QuizSession />} />
                    <Route path="/speech-recognition" element={<SpeechRecognition />} />
                    <Route path="/docs" element={<DocsMain />} />
                </Routes>
            </div>
        </LanguageProvider>
    );
};

export default App;