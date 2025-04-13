import { useRef } from "react";

const useSpeechSynthesis = (lang: string = "ko-KR") => {
    const utteranceRef = useRef<SpeechSynthesisUtterance | null>(null);

    if (!utteranceRef.current) {
        utteranceRef.current = new SpeechSynthesisUtterance();
        utteranceRef.current.lang = lang;
    }

    const speak = (text: string) => {
        return new Promise<void>((resolve, reject) => {
            if (!utteranceRef.current) {
                return reject(new Error("SpeechSynthesisUtterance is not initialized"));
            }

            utteranceRef.current.text = text;
            utteranceRef.current.onend = () => resolve();
            utteranceRef.current.onerror = (event) => reject(event.error);

            speechSynthesis.speak(utteranceRef.current);
        });
    };

    return { speak };
};

export default useSpeechSynthesis;