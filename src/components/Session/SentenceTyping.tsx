import { useState } from "react";
import { SentenceTypingExercise, Option } from "../../models";

interface Props {
    exercise: SentenceTypingExercise;
    onChange: (updated: SentenceTypingExercise) => void;
}

const SentenceTyping = ({ exercise, onChange }: Props) => {
    const [selected, setSelected] = useState<Option[]>([]);

    const isSelected = (id: number) => selected.some((c) => c.id === id);

    const onChunkClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        const id = Number(e.currentTarget.dataset.id);
        const value = e.currentTarget.dataset.value!;
        if (isSelected(id)) return;

        const updated = [...selected, { id, value }];
        setSelected(updated);

        onChange({
            ...exercise,
            userTranslation: updated.map(c => c.value).join(" ")
        });
    };

    const onSelectedClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        const id = Number(e.currentTarget.dataset.id);
        const updated = selected.filter(c => c.id !== id);
        setSelected(updated);

        onChange({
            ...exercise,
            userTranslation: updated.map(c => c.value).join(" ")
        });
    };

    return (
        <div className="w-full max-w-xl mx-auto p-4 text-white">
            <h2 className="text-xl font-bold mb-4">{exercise.sentenceText}</h2>

            <div className="flex flex-wrap gap-2 mb-6 px-3 py-2 min-h-[3rem] bg-gray-800 border border-gray-600 rounded text-white">
                {selected.map(chunk => (
                    <button
                        key={chunk.id}
                        data-id={chunk.id}
                        onClick={onSelectedClick}
                        className="bg-green-700 hover:bg-green-800 text-white px-3 py-1 rounded text-lg"
                    >
                        {chunk.value}
                    </button>
                ))}
            </div>

            <div className="flex flex-wrap gap-2">
                {exercise.chunks
                    .filter(chunk => !isSelected(chunk.id))
                    .map(chunk => (
                        <button
                            key={chunk.id}
                            data-id={chunk.id}
                            data-value={chunk.value}
                            onClick={onChunkClick}
                            className="bg-blue-600 hover:bg-blue-700 text-white px-3 py-1 rounded text-lg"
                        >
                            {chunk.value}
                        </button>
                    ))}
            </div>
        </div>
    );
};

export default SentenceTyping;