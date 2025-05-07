import { Option, Exercise, WordsMatchExercise, HangulMatchExercise } from "../../models";

interface FeedbackResult {
    bgColor: string;
    message: string | null;
}

export type EnhancedOption = Option & { 
    order: number;
    isExcluded: boolean;
};

export interface MatchState {
    id: string;
    selected: EnhancedOption | null;
    flashRed: number[];
    flashGreen: number[];
    correctIds: Set<number>;
    incorrectIds: Set<number>;
    items: EnhancedOption[]
}

export function getDefaultMatchState(exercise: HangulMatchExercise | WordsMatchExercise): MatchState {
    return {
        id: exercise.id,
        selected: null,
        flashRed: [],
        flashGreen: [],
        correctIds: new Set(),
        incorrectIds: new Set(),
        items: exercise.items.map((pr, index) => ({...pr, order: index, isExcluded: false}))
    };
}

export function getFeedbackStyleAndText(exercise: Exercise): FeedbackResult {
    if (!exercise.isCompleted) {
        return { bgColor: "bg-gray-800", message: null };
    }

    const isCorrect = "isCorrect" in exercise && typeof exercise.isCorrect === "boolean"
        ? exercise.isCorrect
        : true;

    if (isCorrect === true) {
        switch(exercise.type) {
            case "words-match":
            case "hangul-match":
                return { bgColor: "bg-green-700", message: "Good job!" };
            default:
                return { bgColor: "bg-green-700", message: "That's right!" };
        }
        
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