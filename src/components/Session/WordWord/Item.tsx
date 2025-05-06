interface Props {
    id: number;
    isSelected: boolean;
    isCorrect: boolean;
    isCompleted: boolean;
    onClick: (event: React.MouseEvent<HTMLElement>) => void;
    label: React.ReactNode;
}

const WordOptionItem: React.FC<Props> = ({
    id,
    isSelected,
    isCorrect,
    isCompleted,
    onClick,
    label,
}) => {
    const borderColor = isCompleted
        ? isCorrect
            ? "border-green-500"
            : "border-red-500"
        : isSelected
        ? "border-blue-500"
        : "border-gray-500";

    const bgColor = isSelected
        ? "bg-gray-800 text-white"
        : "bg-gray-700 text-gray-300";

    return (
        <li
            data-id={id}
            className={`p-4 w-full text-3xl border-2 rounded-lg text-center ${borderColor} ${bgColor} ${
                isCompleted ? "pointer-events-none" : "cursor-pointer hover:bg-gray-600"
            }`}
            onClick={onClick}
        >
            {label}
        </li>
    );
};

export default WordOptionItem;