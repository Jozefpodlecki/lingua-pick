import { useNavigate } from "react-router-dom";
import { v4 as uuidv4 } from "uuid";

const Landing = () => {
    const navigate = useNavigate();

    const createSession = () => {
        const sessionId = uuidv4();
        localStorage.setItem("sessionId", sessionId);
        navigate(`/session/${sessionId}`);
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <h1 className="text-4xl font-bold mb-4">Welcome to Lingua Pick</h1>
            <p className="text-lg mb-8">Start your language learning journey!</p>
            <button
                onClick={createSession}
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