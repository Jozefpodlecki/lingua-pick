import { Option } from "../models";

interface Props {
    option: Option;
    selectedOptionId: number | null;
    onClick(event: React.MouseEvent<HTMLElement>): void;
}

const ImageOption = ({ option, selectedOptionId, onClick }: Props) => {

    return <li
        data-option-id={option.id}
        onClick={onClick}
        className={`shadow-whites p-2 cursor-pointer ${
            selectedOptionId === option.id ? "opacity-100" : "opacity-50"
        }`}
        key={option.id}
    >
        <img
            src={option.imageSrc}
            alt={`Option ${option.id}`}
            className="w-96 h-128 object-cover"
        />
    </li>
}

export default ImageOption;