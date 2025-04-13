import { Link } from "react-router-dom";
import { createSession, getActiveSession } from "../api/api";
import { useEffect } from "react";

const Landing = () => {
    useEffect(() => {
        const activeSession = getActiveSession();
        if (activeSession) {
            window.location.href = `/session/${activeSession.id}`;
        }
    }, []);

    const handleCreateSession = () => {
        const newSession = createSession();
        window.location.href = `/session/${newSession.id}`;
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
            <Link
                to="/speech-recognition"
                className="bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-4 text-center"
            >
                Go to Speech Recognition
            </Link>
            <Link
                to="/docs"
                className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-4 rounded text-center"
            >
                View Documentation
            </Link>
        </div>
    );
};

export default Landing;