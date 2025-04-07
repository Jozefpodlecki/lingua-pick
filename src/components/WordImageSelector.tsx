import { useState, useEffect } from "react";
import { getExercise } from "../api/api";
import { Button } from "@heroui/button";
import { Option } from "../models";

interface State {
    options: Option[];
    question: string;
    correctOptionId: number;
    selectedOptionId: number | null;
}

const WordImageSelector = () => {
    const [state, setState] = useState<State>({
        options: [],
        question: "",
        correctOptionId: 0,
        selectedOptionId: null,
    });

    useEffect(() => {
        const loadOptions = async () => {
            const exercise = await getExercise();
            setState((prevState) => ({
                ...prevState,
                question: exercise.question,
                correctOptionId: exercise.correctOptionId,
                options: exercise.options,
            }));
        };
        loadOptions();
    }, []);

    const onSelect = (id: number) => {
        setState((prevState) => ({
            ...prevState,
            selectedOptionId: id,
        }));
    };

    const onClick = () => {
        console.log("Check button clicked with selected option:", state.selectedOptionId);
    };

    return (
        <div className="word-image-selector">
            <h1 className="text-4xl font-bold mb-4 text-center noto-sans-kr-300">{state.question}</h1>
            <ul className="flex items-center justify-center space-y-4">
                {state.options.map((option) => (
                    <li
                        onClick={() => onSelect(option.id)}
                        className={`p-2 cursor-pointer ${
                            state.selectedOptionId === option.id ? "opacity-100" : "opacity-50"
                        }`}
                        key={option.id}
                    >
                        <img
                            src={option.imageSrc}
                            alt={`Option ${option.id}`}
                            className="w-96 h-128 object-cover"
                        />
                    </li>
                ))}
            </ul>
            <div className="mt-4">
                <Button
                    color="danger"
                    className=""
                    onClickCapture={onClick}
                    disabled={state.selectedOptionId === null}
                >
                    Check
                </Button>
            </div>
        </div>
    );
};

export default WordImageSelector;