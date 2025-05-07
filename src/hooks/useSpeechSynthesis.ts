import { useRef } from "react";

const useSpeechSynthesis = (lang: string = "ko-KR") => {
    const utteranceRef = useRef<SpeechSynthesisUtterance | null>(null);

    if (!utteranceRef.current) {
        (window as any).utterances = [];
        utteranceRef.current = new SpeechSynthesisUtterance();
        utteranceRef.current.lang = lang;
        (window as any).utterances.push( utteranceRef.current );
    }

    const speak = (text: string) => {
        return new Promise<void>((resolve, reject) => {
            if (!utteranceRef.current) {
                return reject(new Error("SpeechSynthesisUtterance is not initialized"));
            }

            const speechSynthesisUtterance = utteranceRef.current;
            // (window as any).utterances = [];
            // const speechSynthesisUtterance = new SpeechSynthesisUtterance();
            // speechSynthesisUtterance.lang = "ko-KR";
            // (window as any).utterances.push( speechSynthesisUtterance );

            speechSynthesisUtterance.text = text;
            speechSynthesisUtterance.onend = () => resolve();
            speechSynthesisUtterance.onerror = (event) => reject(event.error);
            
            setTimeout(() => {
                resolve();
            }, 1000);
            
            speechSynthesis.speak(speechSynthesisUtterance);
        });
    };

    return { speak };
};

export default useSpeechSynthesis;