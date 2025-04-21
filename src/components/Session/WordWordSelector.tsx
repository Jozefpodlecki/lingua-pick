import { Exercise } from "../../models";

interface Props {
    exercise: Exercise;
    selectedOptionId: number | null;
    correctOptionId: number | null;
    onSelect: (id: number) => void;
}

const WordWordSelector: React.FC<Props> = ({  exercise, selectedOptionId, correctOptionId, onSelect }) => {

    return (
        <div className="flex flex-col items-center justify-center space-y-6">
            <h1 className="text-2xl font-bold text-center">{exercise.question}</h1>
            <ul className="grid grid-cols-2 gap-4">
                {exercise.options.map((option) => {
                    const isSelected = selectedOptionId === option.id;
                    const isCorrect = correctOptionId === option.id;

                    const borderColor = correctOptionId
                        ? isCorrect
                            ? "border-green-500"
                            : isSelected
                            ? "border-red-500"
                            : "border-gray-500"
                        : "border-gray-500";

                    return (
                        <li
                            key={option.id}
                            className={`p-4 border-2 rounded-lg cursor-pointer text-center ${borderColor} ${
                                isSelected ? "bg-gray-800 text-white" : "bg-gray-700 text-gray-300"
                            } hover:bg-gray-600`}
                            onClick={() => onSelect(option.id)}
                        >
                            {option.word.hangul}
                        </li>
                    );
                })}
            </ul>
        </div>
    );
};

export default WordWordSelector;