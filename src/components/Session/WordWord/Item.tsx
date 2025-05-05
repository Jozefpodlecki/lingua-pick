interface Props {
    id: number;
    isSelected: boolean;
    isCorrect: boolean;
    showCorrect: boolean;
    onClick: (event: React.MouseEvent<HTMLElement>) => void;
    label: React.ReactNode;
}

const WordOptionItem: React.FC<Props> = ({ id, isSelected, isCorrect, showCorrect, onClick, label }) => {
    const borderColor = showCorrect
        ? isCorrect
            ? "border-green-500"
            : isSelected
            ? "border-red-500"
            : "border-gray-500"
        : "border-gray-500";

    const bgColor = isSelected ? "bg-gray-800 text-white" : "bg-gray-700 text-gray-300";

    return (
        <li
            data-id={id}
            className={`p-4 border-2 rounded-lg cursor-pointer text-center ${borderColor} ${bgColor} hover:bg-gray-600`}
            onClick={onClick}
        >
            {label}
        </li>
    );
};

export default WordOptionItem;