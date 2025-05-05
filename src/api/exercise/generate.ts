import { BaseExercise, Exercise, HangulMatchExercise, QuizSession, SentenceTypingExercise, WordImageExercise, WordsWordsExercise, WordWordExercise } from "../../models";
import { getRandomSentence, getRomanizedToHangul } from "../kr";
import { getStats } from "../stats";
import { getAvailableWords, getRandomItem } from "./utils";

export const generateWordBasedExercise = (
    type: "word-word" | "word-image",
    session: QuizSession,
    base: Partial<Exercise>
): WordImageExercise | WordWordExercise => {
    const userStats = getStats();
    const options = getAvailableWords(userStats, session).sort(() => Math.random() - 0.5).slice(0, 3);
    const correct = getRandomItem(options);

    session.usedWordIds.push(correct.id);

    return {
        ...base,
        type,
        isCompleted: false,
        isCorrect: null,
        correctOptionId: correct.id,
        selectedOptionId: 0,
        prompt: correct.translations[0],
        options: options.map((word) => ({ id: word.id, value: word.word.hangul })),
    } as WordImageExercise | WordWordExercise;
};

export const generateSentenceTypingExercise = (
    base: BaseExercise
): SentenceTypingExercise => {
    const sentence = getRandomSentence()!;
    return {
        ...base,
        type: "sentence-typing" as const,
        sentence: sentence,
        sentenceText: sentence.hangul,
        userTranslation: "",
    };
};

export const generateWordsWordsExercise = (base: BaseExercise): WordsWordsExercise => {


    return {
        ...base,
        type: "words-words" as const,
        left: [],
        right: [],

    };
};

export const generateHangulMatchExercise = (base: BaseExercise): HangulMatchExercise => {

    const allPairs = getRomanizedToHangul();

    const selected = Object.entries(allPairs)
        .sort(() => Math.random() - 0.5)
        .slice(0, 10)

    const items = [];
    const ids = new Set<number>();

    for(const [romanized, hangul] of selected) {
        const id = hangul.charCodeAt(0);

        items.push({
            id,
            value: romanized
        });

        items.push({
            id,
            value: hangul
        });

        ids.add(id);
    }

    items.sort(() => Math.random() - 0.5);

    return {
        ...base,
        type: "hangul-match" as const,
        ids,
        matchedIds: new Set(),
        items,
        correctCount: 0,
        incorrectCount: 0
    };
};
