import { Exercise } from "../../models";
import ImageOption from "../ImageOption";
interface Props {
    exercise: Exercise;
    selectedOptionId: number | null;
    isCorrect: boolean | null;
    onSelect: (id: number) => void;
}

const WordImageSelector = ({ exercise, selectedOptionId, isCorrect, onSelect }: Props) => {

    const onClick = (event: React.MouseEvent<HTMLElement>) => {

        const optionId = Number(event.currentTarget.dataset.optionId);
        onSelect(optionId);

    };

    return (
        <div className="">
            <h1 className="text-4xl font-bold mb-4 text-center noto-sans-kr-300">{exercise.question}</h1>
            <ul className="flex items-center justify-center space-y-4">
                {exercise.options.map((option) => <ImageOption
                    key={option.id}
                    selectedOptionId={selectedOptionId}
                    onClick={onClick} option={option}/>)}
            </ul>
        </div>
    );
};

export default WordImageSelector;