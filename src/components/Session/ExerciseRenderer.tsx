import { Exercise } from "../../models";
import SentenceTyping from "./SentenceTyping";
import WordImageSelector from "./WordImageSelector";
import WordWordSelector from "./WordWord/Selector";
import WordsWordsSelector from "./WordsWords/Selector";

interface Props {
    exercise: Exercise;
    onChange: (exercise: Exercise) => void;
}

const ExerciseRenderer: React.FC<Props> = ({ exercise, onChange }) => {
    switch (exercise.type) {
        case "word-image":
            return (
                <WordImageSelector
                    exercise={exercise}
                    onChange={onChange}
                />
            );
        case "word-word":
            return (
                <WordWordSelector
                    exercise={exercise}
                    onChange={onChange}
                />
            );
        case "sentence-typing":
            return <SentenceTyping exercise={exercise} onChange={onChange}/>
        case "words-words":
            return <WordsWordsSelector exercise={exercise} onChange={onChange}/>
        default:
            return <div className="text-white">Unknown exercise type.</div>;
    }
};

export default ExerciseRenderer;