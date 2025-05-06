import { useEffect, useState } from "react";
import { HangulMatchExercise, Option } from "../../../models";

type EnhancedOption = Option & { 
    order: number;
    isExcluded: boolean;
};

interface Props {
    exercise: HangulMatchExercise;
    onChange: (updated: HangulMatchExercise) => void;
}

interface State {
    id: string;
    selected: EnhancedOption | null;
    flashRed: number[];
    flashGreen: number[];
    correctIds: Set<number>;
    incorrectIds: Set<number>;
    items: EnhancedOption[];
}

const Matcher = ({ exercise, onChange }: Props) => {
    const [state, setState] = useState<State>(getDefaultState(exercise));

    useEffect(() => {
        if(state.id !== exercise.id) {
            setState(getDefaultState(exercise));
        }
    }, [state.id, exercise.id]);

    function getDefaultState(exercise: HangulMatchExercise): State {
        return {
            id: exercise.id,
            selected: null,
            flashRed: [],
            flashGreen: [],
            correctIds: new Set(),
            incorrectIds: new Set(),
            items: exercise.items.map((pr, index) => ({...pr, order: index, isExcluded: false}))
        };
    }

    const _onClick = (event: React.MouseEvent<HTMLElement>) => {
        const currentTarget = event.currentTarget;
        const id = Number(currentTarget.dataset.id);
        const order = Number(event.currentTarget.dataset.order);
        const item = state.items.find(pr => pr.order === order)!;
        const selected = state.selected;

        if (!selected) {
            setState((s) => ({ ...s, selected: item }));
            return;
        }

        if(selected.order === order) {
            setState((s) => ({ ...s, selected: null }));
            return;
        }

        const matched = id === selected.id;

        if (matched) {
            const newFlash = [selected.order, order];
            setState((s) => ({
                ...s,
                selected: null,
                flashRed: [],
                correctIds: state.correctIds.add(id),
                flashGreen: newFlash,
            }));

            setTimeout(() => {
                setState((s) => ({
                    ...s,
                    flashGreen: [],
                    items: s.items.map(pr => ({
                        ...pr,
                        isExcluded: pr.isExcluded || newFlash.includes(pr.order)
                    }))
                }));
            }, 500);
        } else {
            const newFlash = [selected.order, order];
            setState((s) => ({
                ...s,
                selected: null,
                incorrectIds: state.incorrectIds.add(id),
                flashRed: newFlash,
            }));

            setTimeout(() => {
                setState((s) => ({ ...s, flashRed: [] }));
            }, 500);
        }
    };

    const getButtonClass = (item: EnhancedOption) => {
        if (item.isExcluded) {
            return "bg-gray-600 text-white opacity-50 pointer-events-none";
        }

        if (state.flashRed.includes(item.order)) {
            return "bg-red-900 text-white pointer-events-none";
        }

        if (state.flashGreen.includes(item.order)) {
            return "bg-green-900 text-white pointer-events-none";
        }

        if (state.selected?.order === item.order) {
            return "bg-blue-600 text-white";
        }

        return "cursor-pointer bg-gray-800 hover:bg-gray-700";
    };

    useEffect(() => {
        if (state.items.length && !state.items.find(pr => !pr.isExcluded)) {
            onChange({
                ...exercise,
                correctIds: state.correctIds
            });
        }
    }, [state.items]);

    return (
        <div className="w-full max-w-xl mx-auto p-4 text-white">
            <div className="grid grid-cols-3 gap-4">
                {state.items.map((item) => (
                    <button
                        disabled={item.isExcluded}
                        key={item.order}
                        data-id={item.id}
                        data-order={item.order}
                        onClick={_onClick}
                        className={`p-3 text-3xl rounded text-center transition-colors ${getButtonClass(item)}`}
                    >
                        {item.value}
                    </button>
                ))}
            </div>
        </div>
    );
};

export default Matcher;
