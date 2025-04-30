import React, { useState } from "react";
import Breadcrumbs from "./Breadcrumbs";
import * as hangul from 'hangul-js';
import kroman from "kroman";

const romanToHangulMap: { [key: string]: string } = {
    'a': '아', 'ae': '애', 'ya': '야', 'yae': '얘', 'eo': '어', 'e': '에',
    'yeo': '여', 'ye': '예', 'o': '오', 'wa': '와', 'wae': '왜', 'oe': '외',
    'yo': '요', 'u': '우', 'wo': '워', 'we': '웨', 'wi': '위', 'yu': '유',
    'eu': '으', 'yi': '이', 'i': '이', 'g': 'ㄱ', 'kk': 'ㄲ', 'n': 'ㄴ',
    'd': 'ㄷ', 'tt': 'ㄸ', 'r': 'ㄹ', 'm': 'ㅁ', 'b': 'ㅂ', 'pp': 'ㅃ',
    's': 'ㅅ', 'ss': 'ㅆ', 'ng': 'ㅇ', 'j': 'ㅈ', 'jj': 'ㅉ', 'ch': 'ㅊ',
    'k': 'ㅋ', 't': 'ㅌ', 'p': 'ㅍ', 'h': 'ㅎ'
};

const initialConsonants = ['g', 'kk', 'n', 'd', 'tt', 'r', 'm', 'b', 'pp', 's', 'ss', 'ng', 'j', 'jj', 'ch', 'k', 't', 'p', 'h'];
const vowels = ['a', 'ae', 'ya', 'yae', 'eo', 'e', 'yeo', 'ye', 'o', 'wa', 'wae', 'oe', 'yo', 'u', 'wo', 'we', 'wi', 'yu', 'eu', 'yi', 'i'];
const finalConsonants = ['g', 'n', 'd', 'r', 'm', 'b', 's', 'ng', 'j', 'ch', 'k', 't', 'p', 'h'];

function reverseRomanize(romanized: string): string {
    let hangul = '';
    let i = 0;

    let initial = '';
    let vowel = '';
    let final = '';

    // Check for initial consonant
    for (const initialConsonant of initialConsonants) {
        if (romanized.slice(i, i + initialConsonant.length) === initialConsonant) {
            initial = romanToHangulMap[initialConsonant];
            i += initialConsonant.length;
            break;
        }
    }

    // Check for vowel
    for (const vowelOption of vowels) {
        if (romanized.slice(i, i + vowelOption.length) === vowelOption) {
            vowel = romanToHangulMap[vowelOption];
            i += vowelOption.length;
            break;
        }
    }

    console.log("vowel " + vowel);
    // Check for final consonant (batchim)
    for (const finalConsonant of finalConsonants) {
        const test = romanized.slice(i, i + finalConsonant.length);
        console.log("final");
        if (test === finalConsonant) {
            final = romanToHangulMap[finalConsonant];
            i += finalConsonant.length;
            break;
        }
    }


    if (initial && vowel) {
        console.log("test");
        if (final) {
            hangul += initial + vowel + final;
        } else {
            hangul += initial + vowel;
        }
    } else if (vowel) {
        hangul += vowel;
    }

    return hangul;
}

const Typer: React.FC = () => {
    const [text, setText] = useState("");
    const [convertedText, setConvertedText] = useState("");

    const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const input = e.target.value;
        setText(input);

        // Convert romanized Korean to Hangul
        const converted = hangul.assemble(hangul.disassemble(input));
        
        console.log(reverseRomanize("eun"));
        
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white">
              <div className="p-4">
                <Breadcrumbs selectedTitle={"test"} />
            </div>
            <textarea
                value={text}
                onChange={handleInputChange}
                placeholder="..."
                className="w-3/4 h-32 p-4 rounded bg-gray-800 text-white border border-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
            />
             <div className="absolute top-full left-0 mt-2 w-full bg-gray-800 p-4 rounded shadow-lg">
                    <p className="text-lg font-medium">You typed:</p>
                    <p className="text-xl font-semibold">{convertedText}</p>
                </div>
        </div>
    );
};

export default Typer;