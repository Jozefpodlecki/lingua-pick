import { useParams, useNavigate, Link } from "react-router-dom";
import WordImageSelector from "./WordImageSelector";
import { useState, useEffect } from "react";
import { createExercise, getSessionById, saveSession } from "../api/api";
import { Exercise, Session as SessionType } from "../models";
import { Home, AlertCircle } from "lucide-react";

const Session = () => {
    const { uuid } = useParams();
    const navigate = useNavigate();
    const [session, setSession] = useState<SessionType | null>(null);
    const [state, setState] = useState<{
        exercise: Exercise | null;
        selectedOptionId: number | null;
        isCorrect: boolean | null;
        isCompleted: boolean;
    }>({
        exercise: null,
        selectedOptionId: null,
        isCorrect: null,
        isCompleted: false,
    });

    useEffect(() => {
        const loadSession = async () => {
            const currentSession = getSessionById(uuid || "");
            if (!currentSession) {
                setSession(null);
                return;
            }

            setSession(currentSession);

            const lastExercise = currentSession.exercises[currentSession.exercises.length - 1];

            if (!lastExercise || lastExercise.isCompleted) {
                const newExercise = await createExercise(currentSession);
                currentSession.exercises.push(newExercise);

                setState((prevState) => ({
                    ...prevState,
                    exercise: newExercise,
                }));
            } else {
                setState((prevState) => ({
                    ...prevState,
                    exercise: lastExercise,
                }));
            }
        };

        loadSession();
    }, [uuid]);

    const handleCheck = () => {
        if (state.exercise && session) {
            const isCorrect = state.selectedOptionId === state.exercise.correctOptionId;
            setState((prevState) => ({
                ...prevState,
                isCorrect,
                isCompleted: true,
            }));

            const updatedExercise: Exercise = {
                ...state.exercise,
                completedOn: new Date().toISOString(),
                isCompleted: true,
                isCorrect,
            };

            saveSession(session, updatedExercise);
            setSession({
                ...session,
                exercises: session.exercises.map((ex) =>
                    ex.createdOn === updatedExercise.createdOn ? updatedExercise : ex
                ),
            });
        }
    };

    const handleContinue = async () => {
        if (session) {
            const newExercise = await createExercise(session);
            const updatedSession = {
                ...session,
                exercises: [...session.exercises, newExercise],
            };

            saveSession(updatedSession, newExercise);
            setSession(updatedSession);

            setState({
                exercise: newExercise,
                selectedOptionId: null,
                isCorrect: null,
                isCompleted: false,
            });
        }
    };

    const onClick = () => {
        if (!state.isCompleted) {
            handleCheck();
        } else {
            handleContinue();
        }
    };

    const onSelect = (id: number) => {
        setState((prevState) => ({
            ...prevState,
            selectedOptionId: id,
        }));
    };

    if (!session) {
        return (
            <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
                <AlertCircle className="w-16 h-16 text-red-500 mb-4" />
                <h1 className="text-4xl font-bold mb-4">Session Not Found</h1>
                <p className="text-lg mb-8">The session you are looking for does not exist or is no longer active.</p>
                <Link
                    to="/"
                    className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                >
                    Go to Home
                </Link>
            </div>
        );
    }

    return (
        <div className="flex flex-col min-h-screen bg-black">
            <nav className="bg-gray-800 w-full p-4">
                <div className="flex items-center space-x-2 text-gray-300">
                    <Link to="/" className="hover:text-white flex items-center">
                        <Home className="w-5 h-5 mr-1" />
                        Home
                    </Link>
                    <span className="text-gray-400">/ Session</span>
                </div>
            </nav>
            <div className="p-2 flex-1 flex items-center justify-center">
                {state.exercise && (
                    <WordImageSelector
                        exercise={state.exercise}
                        selectedOptionId={state.selectedOptionId}
                        isCorrect={state.isCorrect}
                        onSelect={onSelect}
                    />
                )}
            </div>
            <div
                className={`flex items-center p-4 text-right transition-colors duration-300 ${
                    state.isCompleted
                        ? state.isCorrect
                            ? "bg-[#568203]"
                            : "bg-[#58111A]"
                        : "bg-gray-800"
                }`}
            >
                <div className="text-white text-lg mb-4">
                    <span className="text-2xl">{state.isCompleted &&
                        (state.isCorrect
                            ? "That's right!"
                            : `That's wrong, it's ${
                                  state.exercise?.options.find(
                                      (option) => option.id === state.exercise?.correctOptionId
                                  )?.translations[0] || "unknown"
                              }`)}</span>
                </div>
                <button
                    type="button"
                    disabled={state.selectedOptionId === null && !state.isCompleted}
                    onClick={onClick}
                    className="cursor-pointer rounded bg-gray-600 p-4 ml-auto text-2xl text-white hover:bg-sky-500 active:bg-sky-700"
                >
                    {state.isCompleted ? "Continue" : "Check"}
                </button>
            </div>
        </div>
    );
};

export default Session;