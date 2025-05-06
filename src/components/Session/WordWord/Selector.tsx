import { WordWordExercise } from "../../../models";
import WordOptionItem from "./Item";

interface Props {
    exercise: WordWordExercise;
    onChange: (exercise: WordWordExercise) => void;
}

const WordWordSelector: React.FC<Props> = ({ exercise, onChange }) => {

    const _onSelect = (event: React.MouseEvent<HTMLElement>) => {
        const selectedOptionId = Number(event.currentTarget.dataset.id);
        const updated = {
            ...exercise,
            selectedOptionId
        };
        onChange(updated);
    }

    return (
        <div className="flex flex-col items-center justify-center space-y-6">
            <h1 className="text-2xl font-bold text-center">{exercise.prompt}</h1>
            <ul className="flex flex-col items-center gap-4">
                {exercise.options.map((option) => {
                    const isSelected = exercise.selectedOptionId === option.id;
                    const isCorrect = exercise.correctOptionId === option.id;

                    return (
                        <WordOptionItem
                            key={option.id}
                            id={option.id}
                            isSelected={isSelected}
                            isCorrect={isCorrect}
                            isCompleted={exercise.isCompleted}
                            onClick={_onSelect}
                            label={option.value}
                        />
                    );
                })}
            </ul>
        </div>
    );
};

export default WordWordSelector;