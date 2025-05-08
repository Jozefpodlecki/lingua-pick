import { useEffect, useState } from "react";
import { WordsWordsExercise } from "@/models";
import { EnhancedMatchPairOption } from "../utils";

interface Props {
  exercise: WordsWordsExercise;
  onChange: (updated: WordsWordsExercise) => void;
}

interface MatcherState {
	id: string;
	selectedId: number | null;
	selectedMatchId: number | null;
	left: EnhancedMatchPairOption[];
    right: EnhancedMatchPairOption[];
}

const Matcher = ({ exercise, onChange }: Props) => {
	const [{
		selectedId,
		selectedMatchId,
		left,
		right
	}, setState] = useState<MatcherState>(defaultState(exercise));

  	useEffect(() => {
		if (!left.find(pr => !pr.isExcluded)) {
			onChange({
				...exercise,
				
			})
		}
	}, [left]);

	function defaultState(exercise: WordsWordsExercise): MatcherState {
		return {
			id: exercise.id,
			selectedId: null,
			selectedMatchId: null,
			left: exercise.left.map((pr) => ({...pr, isExcluded: false})),
			right: exercise.right.map((pr) => ({...pr, isExcluded: false})),
		}
	}

	const onSelect = (event: React.MouseEvent<HTMLElement>) => {
		const currentTarget = event.currentTarget;
		const id = Number(currentTarget.dataset.id);
		const matchId = Number(currentTarget.dataset.matchid);

		if (selectedId === null) {
			setState(pr => ({
				...pr,
				selectedId: id,
				selectedMatchId: matchId
			}))
			return;	
		}

		if(id === selectedId) {
			setState(pr => ({
				...pr,
				selectedId: null,
				selectedMatchId: null
			}))
			return;
		}

		if(selectedMatchId === matchId) {


			return;
		}
	};

  return (
    <div className="w-full max-w-xl mx-auto p-4 text-white">
      <h2 className="text-xl mb-4">Match the words</h2>
      <div className="grid grid-cols-2 gap-6">
        <div>
          <ul className="space-y-2">
            {left.map((item) => (
              <li
                key={item.id}
                data-id={item.id}
				data-matchid={item.matchId}
                onClick={onSelect}
                className={`cursor-pointer px-2 py-1 rounded hover:bg-gray-600 ${selectedId === item.id ? "bg-blue-600" : "bg-gray-700"}`}
              >
                {item.value}
              </li>
            ))}
          </ul>
        </div>

        <div>
          <ul className="space-y-2">
            {right.map((item) => (
              <li
                key={item.id}
				data-id={item.id}
				data-matchid={item.matchId}
                onClick={onSelect}
                className={`cursor-pointer px-2 py-1 rounded hover:bg-gray-600 ${selectedId === item.id ? "bg-green-600" : "bg-gray-700"}`}
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
