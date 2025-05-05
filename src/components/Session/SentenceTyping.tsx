import { SentenceTypingExercise } from "../../models";

interface Props {
    exercise: SentenceTypingExercise;
    onChange: (updated: SentenceTypingExercise) => void;
}

const SentenceTyping = ({ exercise, onChange }: Props) => {

    const _onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const userTranslation = event.target.value;
        const updated = {
            ...exercise,
            userTranslation
        };
        onChange(updated);
    };

    return (
        <div className="w-full max-w-xl mx-auto p-4 text-white">
            <h2 className="text-xl font-bold mb-4">{exercise.sentenceText}</h2>
            <input
                type="text"
                value={exercise.userTranslation || ""}
                onChange={_onChange}
                placeholder="..."
                className="w-full px-4 py-2 rounded bg-gray-700 text-white border border-gray-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
        </div>
    );
};

export default SentenceTyping;