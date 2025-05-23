import { useEffect, useState } from "react";
import { WordsWordsExercise } from "@/models";
import { EnhancedMatchPairOption } from "../utils";
import useSpeechSynthesis from "@/hooks/useSpeechSynthesis";

interface Props {
    exercise: WordsWordsExercise;
    onChange: (updated: WordsWordsExercise) => void;
}

interface MatcherState {
    id: string;
    selected: EnhancedMatchPairOption | null;
    left: EnhancedMatchPairOption[];
    right: EnhancedMatchPairOption[];
    flashRed: number[];
    flashGreen: number[];
}

const Matcher = ({ exercise, onChange }: Props) => {
    const { speak } = useSpeechSynthesis("ko-KR");

    const [state, setState] = useState<MatcherState>(getDefaultState(exercise));

    useEffect(() => {
        if (state.id !== exercise.id) {
            setState(getDefaultState(exercise));
        }
    }, [exercise.id]);

    useEffect(() => {
        const allMatched =
            state.left.every((item) => item.isExcluded) &&
            state.right.every((item) => item.isExcluded);

        if (allMatched) {
            onChange({ ...exercise });
        }
    }, [state.left, state.right]);

    function getDefaultState(exercise: WordsWordsExercise): MatcherState {
        return {
            id: exercise.id,
            selected: null,
            left: exercise.left.map((pr) => ({ ...pr, isExcluded: false })),
            right: exercise.right.map((pr) => ({ ...pr, isExcluded: false })),
            flashRed: [],
            flashGreen: [],
        };
    }

    const onSelect = (event: React.MouseEvent<HTMLElement>) => {
        const id = Number(event.currentTarget.dataset.id);
        const side = event.currentTarget.dataset.side as "left" | "right";

        const item =
            side === "left"
                ? state.left.find((x) => x.id === id)
                : state.right.find((x) => x.id === id);

        if (!item || item.isExcluded) {
            return;
        }

        const selected = state.selected;

        if (item.canPlay && (!selected || selected && selected.id !== item.id)) {
            speak(item.value);
        }

        if (!selected) {
            setState((prev) => ({ ...prev, selected: item }));
            return;
        }

        if (selected.id === item.id) {
            setState((prev) => ({ ...prev, selected: null }));
            return;
        }

        const isMatch = selected.matchId === item.matchId;

        if (isMatch) {
            setState((prev) => ({
				...prev,
				selected: null,
				flashGreen: [selected.id, item.id],
				left: prev.left.map((x) =>
					[item.id, selected.id].includes(x.id) ? { ...x, isExcluded: true } : x
				),
				right: prev.right.map((x) =>
					[item.id, selected.id].includes(x.id) ? { ...x, isExcluded: true } : x
				),
			}));

			setTimeout(() => {
				setState((prev) => ({
					...prev,
					flashGreen: [],
				}));
			}, 500);
        } else {
            setState((prev) => ({
                ...prev,
                flashRed: [selected.id, item.id],
            }));

            setTimeout(() => {
                setState((prev) => ({
                    ...prev,
					selected: null,
                    flashRed: [],
                }));
            }, 500);
        }
    };

    const getItemClass = (item: EnhancedMatchPairOption) => {
		if (state.flashRed.includes(item.id)) return "bg-red-900 text-white pointer-events-none";
        if (state.flashGreen.includes(item.id)) return "bg-green-900 text-white pointer-events-none";
        if (item.isExcluded) return "bg-gray-600 text-white opacity-50 pointer-events-none";
        if (state.selected?.id === item.id) return "cursor-pointer bg-blue-600 text-white";
        return "bg-gray-700 hover:bg-gray-600 cursor-pointer";
    };

    return (
        <div className="w-full max-w-xl mx-auto p-4 text-white">
            <h2 className="text-xl mb-4">Match the words</h2>
            <div className="grid grid-cols-2 gap-6">
                <div>
                    <ul className="space-y-2">
                        {state.left.map((item) => (
                            <li
                                key={item.id}
                                data-id={item.id}
                                data-matchid={item.matchId}
                                data-side="left"
                                onClick={onSelect}
                                className={`px-2 py-1 text-lg rounded text-center transition-colors ${getItemClass(item)}`}
                            >
                                {item.value}
                            </li>
                        ))}
                    </ul>
                </div>
                <div>
                    <ul className="space-y-2">
                        {state.right.map((item) => (
                            <li
                                key={item.id}
                                data-id={item.id}
                                data-matchid={item.matchId}
                                data-side="right"
                                onClick={onSelect}
                                className={`px-2 py-1 text-lg rounded text-center transition-colors ${getItemClass(item)}`}
                            >
                                {item.value}
                            </li>
                        ))}
                    </ul>
                </div>
            </div>
        </div>
    );
};

export default Matcher;
