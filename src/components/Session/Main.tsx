import { useParams, useNavigate, Link } from "react-router-dom";
import BottomPanel from "./BottomPanel";
import ProgressBar from "./ProgressBar";
import { useState, useEffect } from "react";
import { cancelSession, getSessionById, progressSession, validateExercise } from "../../api";
import { Exercise, QuizSession } from "../../models";
import { Home } from "lucide-react";
import NotFound from "./NotFound";
import Completed from "./Completed";
import ExerciseRenderer from "./ExerciseRenderer";

interface State {
    session: QuizSession | null;
    exercise: Exercise | null;
    isCompleted: boolean;
}

const SessionComponent: React.FC = () => {
    const { uuid } = useParams();
    const navigate = useNavigate();
    const [{
        session,
        exercise,
        isCompleted
    }, setState] = useState<State>({
        session: null,
        exercise: null,
        isCompleted: false,
    });

    useEffect(() => {
        onLoad();
    }, [uuid]);

    async function onLoad() {
        const currentSession = await getSessionById(uuid || "");
        if (!currentSession) {
            return;
        }

        setState(state => ({...state,
            session: currentSession,
            exercise: currentSession.currentExercise
        }));

    }

    const onCheck = async () => {
        if (!exercise || !session) {
            return;
        }

        await onCheckInner(session, exercise);
    };

    const onCheckInner = async (session: QuizSession, exercise: Exercise) => {
        const {
            session: updatedSession,
            exercise: updatedExercise,
        } = await validateExercise(session, exercise);

        setState((prevState) => ({
            ...prevState,
            session: updatedSession,
            exercise: updatedExercise,
            isCompleted: true
        }));
    }

    const onContinue = async () => {
        if (!session) {
            return;
        }

        const newSession = await progressSession(session);

        setState({
            session: newSession,
            exercise: newSession.currentExercise,
            isCompleted: false,
        });
    };

    const onClick = () => {
        if (!isCompleted) {
            onCheck();
            return;
        }

        onContinue();
    };

    const onChange = (exercise: Exercise) => {

        switch(exercise.type) {
            case "hangul-match":
            case "words-match":
            case "words-words":
                onCheckInner(session!, exercise);
                return;
                break;
            default:
                setState((prevState) => ({
                    ...prevState,
                    exercise,
                }));
                break;
        }
    };

    const onCancel = () => {
        cancelSession(session!);
        navigate("/");
    };

    if (!session) {
        return <NotFound />;
    }

    if (session.isFinished) {
        return <Completed session={session} />;
    }

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
            {exercise && (
                <ExerciseRenderer
                    exercise={exercise}
                    onChange={onChange}
                />
            )}
            </div>
            {exercise &&<BottomPanel
                exercise={exercise}
                onClick={onClick}
            />}
        </div>
    );
};

export default SessionComponent;