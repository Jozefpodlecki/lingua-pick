import { WordsWordsExercise } from "../../../models";

interface Props {
    exercise: WordsWordsExercise;
    onChange: (updated: WordsWordsExercise) => void;
}

const Selector = ({ exercise, onChange }: Props) => {

    const _onChange = (event: React.ChangeEvent<HTMLInputElement>) => {
      
        onChange({
            ...exercise
        });
    };

    return (
        <div className="w-full max-w-xl mx-auto p-4 text-white">
          
        </div>
    );
};

export default Selector;