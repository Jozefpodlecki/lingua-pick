import { BaseExercise, Exercise, HangulMatchExercise, MatchPairOption, SentenceTypingExercise, WordImageExercise, WordsMatchExercise, WordsWordsExercise, WordWordExercise } from "@/models";
import { getRandomSentence, getRomanizedToHangul } from "../kr";
import { getStats } from "../stats";
import { getAvailableWords, getRandomItem, shuffleArray } from "./utils";

export const generateWordBasedExercise = (
    type: "word-word" | "word-image",
    usedWordIds: number[],
    base: Partial<Exercise>
): WordImageExercise | WordWordExercise => {
    const userStats = getStats();
    const options = getAvailableWords(userStats, usedWordIds, 3);
    const correct = getRandomItem(options);

    usedWordIds.push(correct.id);

    return {
        ...base,
        type,
        isCompleted: false,
        isCorrect: false,
        correctOptionId: correct.id,
        selectedOptionId: 0,
        prompt: correct.translations[0],
        requiresManualCheck: true,
        options: options.map((word) => ({ id: word.id, value: `${word.hangul} - ${word.romanized}` })),
    } as WordImageExercise | WordWordExercise;
};

export const generateSentenceTypingExercise = (
    base: BaseExercise
): SentenceTypingExercise => {
    const sentence = getRandomSentence()!;
    const useKoreanPrompt = Math.random() < 0.5;

    const sentenceText = useKoreanPrompt ? sentence.hangul : sentence.translation;
    const chunks = (useKoreanPrompt ? sentence.translation : sentence.hangul)
        .split(" ")
        .map((value, id) => ({
            id,
            value,
            canPlay: false,
        }));

    return {
        ...base,
        requiresManualCheck: true,
        type: "sentence-typing" as const,
        sentence,
        sentenceText,
        chunks,
        isCorrect: false,
        userTranslation: "",
    };
};

export const generateWordsWordsExercise = (usedWordIds: number[], base: BaseExercise): WordsWordsExercise => {
    const stats = getStats();
    const selected = getAvailableWords(stats, usedWordIds, 5);
    const left = new Array<MatchPairOption>();
    const right = new Array<MatchPairOption>();
    const ids = new Set<number>();
    let id = 0;

    for(const word of selected) {

        const translation = shuffleArray(word.translations)[0];
        const pair = [
            { id, matchId: word.id, value: word.hangul, canPlay: true },
            { id, matchId: word.id, value: translation, canPlay: false },
        ];
        const [source, target] = Math.random() < 0.5 ? [left, right] : [right, left];
        source.push(pair[0]);
        target.push(pair[1]);

        ids.add(id);
        id++;
    }

    return {
        ...base,
        requiresManualCheck: false,
        type: "words-words" as const,
        left,
        right,

    };
};

export const generateWordsMatchExercise = (usedWordIds: number[], base: BaseExercise): WordsMatchExercise => {
    const stats = getStats();
    const allPairs = getAvailableWords(stats, usedWordIds, 10);

    const selected = allPairs.sort(() => Math.random() - 0.5)

    const items = [];
    const ids = new Set<number>();
    let id = 0;

    for(const word of selected) {

        const translation = shuffleArray(word.translations)[0];

        items.push({ id, value: word.hangul, canPlay: true });
        items.push({ id, value: translation, canPlay: false });

        ids.add(id);
        id++;
    }

    items.sort(() => Math.random() - 0.5);

    return {
        ...base,
        requiresManualCheck: false,
        type: "words-match" as const,
        ids,
        items,
        correctIds: new Set(),
        incorrectIds: new Set()
    };
};


export const generateHangulMatchExercise = (base: BaseExercise): HangulMatchExercise => {

    const allPairs = getRomanizedToHangul();

    const selected = Object.entries(allPairs)
        .sort(() => Math.random() - 0.5)
        .slice(0, 5)

    const items = [];
    const ids = new Set<number>();

    for(const [romanized, hangul] of selected) {
        const id = hangul.charCodeAt(0);

        items.push({
            id,
            value: romanized,
            canPlay: false
        });

        items.push({
            id,
            value: hangul,
            canPlay: true
        });

        ids.add(id);
    }

    items.sort(() => Math.random() - 0.5);

    return {
        ...base,
        requiresManualCheck: false,
        type: "hangul-match" as const,
        ids,
        items,
        correctIds: new Set(),
        incorrectIds: new Set()
    };
};
