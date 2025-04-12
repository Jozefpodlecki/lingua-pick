import { useParams, useNavigate, Link } from "react-router-dom";
import WordImageSelector from "./WordImageSelector";
import BottomPanel from "../BottomPanel";
import ProgressBar from "./ProgressBar";
import { useState, useEffect } from "react";
import { cancelSession, completeSession, createExercise, getSessionById, saveSession, validateExercise } from "../../api/api";
import { Exercise, Session } from "../../models";
import { Home, AlertCircle } from "lucide-react";
import NotFound from "./NotFound";
import Completed from "./Completed";

interface State {
    exercise: Exercise | null;
    selectedOptionId: number | null;
    isCorrect: boolean | null;
    isCompleted: boolean;
}

const SessionComponent: React.FC = () => {
    const { uuid } = useParams();
    const navigate = useNavigate();
    const [session, setSession] = useState<Session | null>(null);
    const [state, setState] = useState<State>({
        exercise: null,
        selectedOptionId: null,
        isCorrect: null,
        isCompleted: false,
    });

    useEffect(() => {
        onLoad();
    }, [uuid]);

    async function onLoad() {
        const currentSession = getSessionById(uuid || "");
        if (!currentSession) {
            setSession(null);
            return;
        }

        setSession(currentSession);

        const previousExercise = currentSession.exercises[currentSession.exercises.length - 1];
        const isLastExercise = currentSession.exerciseCount === currentSession.exercises.length;

        if (isLastExercise) {
            return;
        }

        if (!previousExercise || previousExercise.isCompleted) {
            const [newExercise, session] = await createExercise(currentSession);
            currentSession.exercises.push(newExercise);

            setState((prevState) => ({
                ...prevState,
                exercise: newExercise,
            }));
        } else {
            setState((prevState) => ({
                ...prevState,
                exercise: previousExercise,
            }));
        }
    }

    const onCheck = () => {
        if (state.exercise && session) {
            const {
                isCorrect,
                session: updatedSession,
                exercise: updatedExercise,
            } = validateExercise(session, state.exercise, state.selectedOptionId!);

            setState((prevState) => ({
                ...prevState,
                exercise: updatedExercise,
                isCorrect,
                isCompleted: true,
            }));

            setSession(updatedSession);
        }
    };

    const onContinue = async () => {
        if (!session) {
            return;
        }

        const isLastExercise = session.exerciseCount === session.exercises.length;
        if (isLastExercise) {
            const newSession = await completeSession(session);
            setSession(newSession);
            return;
        }

        const [newExercise, newSession] = await createExercise(session);
        setSession(newSession);

        setState({
            exercise: newExercise,
            selectedOptionId: null,
            isCorrect: null,
            isCompleted: false,
        });
    };

    const onClick = () => {
        if (!state.isCompleted) {
            onCheck();
        } else {
            onContinue();
        }
    };

    const onSelect = (id: number) => {
        setState((prevState) => ({
            ...prevState,
            selectedOptionId: id,
        }));
    };

    const onCancel = () => {
        cancelSession(session!);
        navigate("/");
    };

    if (!session) {
        return <NotFound />;
    }

    const isLastExercise = session.exerciseCount === session.exercises.length;
    if (isLastExercise && session.isFinished) {
        return <Completed />;
    }

    const correctWord = state.exercise?.options.find(
        (option) => option.id === state.exercise?.correctOptionId
    )?.word.hangul || "unknown";

    const completedExercises = session.exercises.filter((ex) => ex.isCompleted).length;
    const maxExercises = session.exerciseCount || 5;
    const progressPercentage = (completedExercises / maxExercises) * 100;

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
            <ProgressBar value={progressPercentage} onCancel={onCancel} />
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
            <BottomPanel
                isCompleted={state.isCompleted}
                isCorrect={state.isCorrect}
                correctWord={correctWord}
                onClick={onClick}
                isDisabled={state.selectedOptionId === null && !state.isCompleted}
            />
        </div>
    );
};

export default SessionComponent;