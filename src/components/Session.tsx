import { useParams } from "react-router-dom";
import WordImageSelector from "./WordImageSelector";
import { useState, useEffect } from "react";
import { getExercise } from "../api/api";
import { Exercise } from "../models";

const Session = () => {
    const { uuid } = useParams();
    const [state, setState] = useState<{
        exercise: Exercise | null;
        selectedOptionId: number | null;
        isCorrect: boolean | null;
    }>({
        exercise: null,
        selectedOptionId: null,
        isCorrect: null,
    });

    useEffect(() => {
        const loadExercise = async () => {
            const exercise = await getExercise();
            setState((prevState) => ({
                ...prevState,
                exercise,
            }));
        };
        loadExercise();
    }, []);

    const onClick = () => {
        if (state.exercise) {
            const isCorrect = state.selectedOptionId === state.exercise.correctOptionId;
            setState((prevState) => ({
                ...prevState,
                isCorrect,
            }));
        }
    };

    const onSelect = (id: number) => {
        setState((prevState) => ({
            ...prevState,
            selectedOptionId: id,
        }));
    };

    return (
        <div className="flex flex-col min-h-screen bg-black">
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
            <div className="p-4 text-right">
                <button
                    type="button"
                    disabled={state.selectedOptionId === null}
                    onClick={onClick}
                    className="cursor-pointer rounded bg-gray-600 p-4 text-2xl text-white hover:bg-sky-500 active:bg-sky-700"
                >
                    Check
                </button>
            </div>
        </div>
    );
};

export default Session;