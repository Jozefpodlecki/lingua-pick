import { useNavigate } from "react-router-dom";
import { createSession, getActiveSession } from "../api/api";
import { useEffect } from "react";

const Landing = () => {
    const navigate = useNavigate();

    useEffect(() => {
        const activeSession = getActiveSession();
        if (activeSession) {
            navigate(`/session/${activeSession.id}`);
        }
    }, [navigate]);

    const handleCreateSession = () => {
        const newSession = createSession();
        navigate(`/session/${newSession.id}`);
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <h1 className="text-4xl font-bold mb-4">Welcome to Lingua Pick</h1>
            <p className="text-lg mb-8">Start your language learning journey!</p>
            <button
                onClick={handleCreateSession}
                className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-4"
            >
                Start New Session
            </button>
            <button
                onClick={() => navigate("/speech-recognition")}
                className="bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
            >
                Go to Speech Recognition
            </button>
        </div>
    );
};

export default Landing;