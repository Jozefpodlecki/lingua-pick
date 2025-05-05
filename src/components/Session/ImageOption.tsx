import { Option } from "../../models";

interface Props {
    option: Option;
    correctOptionId: number | null;
    selectedOptionId: number | null;
    onClick(event: React.MouseEvent<HTMLElement>): void;
}

const ImageOption = ({ option, correctOptionId, selectedOptionId, onClick }: Props) => {
    const isSelected = selectedOptionId === option.id;
    const isCorrect = correctOptionId === option.id;

    const boxShadowClass =
        correctOptionId !== null
            ? isCorrect
                ? "shadow-green-500"
                : isSelected
                ? "shadow-red-500"
                : "shadow-whites"
            : "shadow-whites";

    return (
        <li
            data-option-id={option.id}
            onClick={onClick}
            className={`p-2 cursor-pointer shadow-lg ${boxShadowClass} ${
                isSelected ? "opacity-100" : "opacity-50"
            }`}
            key={option.id}
        >
            <img
                src={option.imageSrc}
                alt={`Option ${option.id}`}
                className="w-96 h-128 object-cover"
            />
        </li>
    );
};

export default ImageOption;