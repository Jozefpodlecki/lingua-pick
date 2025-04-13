import React, { useState } from "react";
import { Play, Loader } from "lucide-react";
import useSpeechSynthesis from "../../../hooks/useSpeechSynthesis";

const GrammarContent: React.FC = () => {
    const { speak } = useSpeechSynthesis("ko-KR");
    const [speakingExample, setSpeakingExample] = useState<string | null>(null);

    const grammarRules = [
        {
            title: "Basic Sentence Structure",
            description:
                "Korean sentences follow a Subject-Object-Verb (SOV) structure.",
            example: {
                english: "I eat an apple.",
                korean: "나는 사과를 먹어요.",
            },
        },
        {
            title: "Particles",
            description:
                "Particles indicate the role of a word in a sentence. For example, '은/는' (topic marker), '이/가' (subject marker), '을/를' (object marker).",
            example: {
                english: "The apple is delicious.",
                korean: "사과는 맛있어요.",
            },
        },
        {
            title: "Verb Conjugation",
            description:
                "Korean verbs are conjugated based on tense, politeness level, and mood.",
            example: {
                english: "I will eat.",
                korean: "나는 먹을 거예요.",
            },
        },
        {
            title: "Honorifics",
            description:
                "Honorifics are used to show respect when speaking to someone of higher status.",
            example: {
                english: "Please eat.",
                korean: "드세요.",
            },
        },
        {
            title: "Adjective Placement",
            description:
                "Adjectives in Korean are placed before the noun they modify.",
            example: {
                english: "A pretty flower.",
                korean: "예쁜 꽃.",
            },
        },
    ];

    const onPlayExample = async (korean: string) => {
        setSpeakingExample(korean);
        try {
            await speak(korean);
        } catch (error) {
            console.error(`Error speaking: ${error}`);
        } finally {
            setSpeakingExample(null);
        }
    };

    return (
        <div>
            <h1 className="text-3xl font-bold mb-6">Korean Grammar</h1>
            <div className="space-y-6">
                {grammarRules.map((rule, index) => (
                    <div
                        key={index}
                        className="p-4 border border-gray-700 rounded-lg bg-gray-800"
                    >
                        <h2 className="text-2xl font-semibold mb-2">{rule.title}</h2>
                        <p className="text-lg mb-4">{rule.description}</p>
                        <div>
                            <p className="text-lg font-medium">Example:</p>
                            <p className="text-lg">
                                <span className="font-bold mr-4">English:</span> {rule.example.english}
                            </p>
                            <p className="text-lg flex items-center">
                                <span className="font-bold mr-4">Korean:</span> {rule.example.korean}
                                <button
                                    onClick={() => onPlayExample(rule.example.korean)}
                                    disabled={speakingExample !== null}
                                    className={`ml-2 flex items-center justify-center w-6 h-6 rounded-full ${
                                        speakingExample === rule.example.korean
                                            ? "bg-gray-500"
                                            : "bg-blue-500 hover:bg-blue-600"
                                    }`}
                                >
                                    {speakingExample === rule.example.korean ? (
                                        <Loader className="w-4 h-4 animate-spin text-white" />
                                    ) : (
                                        <Play className="w-4 h-4 text-white" />
                                    )}
                                </button>
                            </p>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default GrammarContent;