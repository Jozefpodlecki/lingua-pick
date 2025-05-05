import romanizedToHangulJson from "../../data/kr/romanized_to_hangul.json";

const romanizedToHangul: Record<string, string> = romanizedToHangulJson;
const wordRegex = /[a-zA-Z0-9]+|[^a-zA-Z0-9\s]/g;

export function convertRomanizedToHangul(romanizedString: string): string {

    const tokens = (romanizedString.match(wordRegex) || []).filter(t => t !== '-');
    let hangulResult = '';

    for (let token of tokens) {
        if (!/^[a-zA-Z0-9]+$/.test(token)) {
            hangulResult += token;
            // hangulResult += ' ';
            continue;
        }

        let syllable = '';
        let romanizedWord = token.toLowerCase();

        while (romanizedWord.length > 0) {
            let matched = false;

            for (let len = romanizedWord.length; len > 0; len--) {
                const sub = romanizedWord.slice(0, len);
                const hangul = romanizedToHangul[sub];
                console.log(tokens, sub);
                if (hangul) {
                    syllable += hangul;
                    romanizedWord = romanizedWord.slice(len);
                    matched = true;
                    break;
                }
            }

            if (!matched) {
                romanizedWord = romanizedWord.slice(1);
            }

        }

        hangulResult += syllable;
        hangulResult += ' ';
    }

    return hangulResult.trim();
}