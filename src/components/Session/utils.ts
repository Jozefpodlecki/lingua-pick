import { Exercise } from "../../models";

interface FeedbackResult {
    bgColor: string;
    message: string | null;
}

export function getFeedbackStyleAndText(exercise: Exercise): FeedbackResult {
    if (!exercise.isCompleted) {
        return { bgColor: "bg-gray-800", message: null };
    }

    const isCorrect = "isCorrect" in exercise && typeof exercise.isCorrect === "boolean"
        ? exercise.isCorrect
        : null;

    if (isCorrect === true) {
        return { bgColor: "bg-green-700", message: "That's right!" };
    }

    if (isCorrect === false) {
        switch (exercise.type) {
            case "word-word":
            case "word-image": {
                const correct = exercise.options.find(o => o.id === exercise.correctOptionId)?.value;
                return {
                    bgColor: "bg-[#58111A]",
                    message: correct ? `That's wrong, it's "${correct}".` : "That's wrong.",
                };
            }
            case "sentence-typing":
                return {
                    bgColor: "bg-[#58111A]",
                    message: `Expected: "${exercise.sentenceText}"`,
                };
            default:
                return { bgColor: "bg-[#58111A]", message: "That's incorrect." };
        }
    }

    return { bgColor: "bg-gray-800", message: null };
}