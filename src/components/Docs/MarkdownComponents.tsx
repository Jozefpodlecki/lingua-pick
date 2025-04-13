import { Components } from "react-markdown";

const MarkdownComponents: Components = {
    h1: ({ children }) => (
        <h1 className="text-4xl font-bold text-blue-400 my-4">{children}</h1>
    ),
    h2: ({ children }) => (
        <h2 className="text-3xl font-semibold text-blue-300 my-3">{children}</h2>
    ),
    p: ({ children }) => (
        <p className="text-lg text-gray-300 my-2">{children}</p>
    ),
    a: ({ href, children }) => (
        <a
            href={href}
            className="text-blue-500 hover:underline"
            target="_blank"
            rel="noopener noreferrer"
        >
            {children}
        </a>
    ),
    ul: ({ children }) => (
        <ul className="list-disc list-inside text-gray-300">{children}</ul>
    ),
    ol: ({ children }) => (
        <ol className="list-decimal list-inside text-gray-300">{children}</ol>
    ),
    blockquote: ({ children }) => (
        <blockquote className="border-l-4 border-blue-500 pl-4 italic text-gray-400">
            {children}
        </blockquote>
    ),
    code: ({ children }) => (
        <pre className="bg-gray-800 text-gray-300 p-4 rounded">
            <code>{children}</code>
        </pre>
    ),
};

export default MarkdownComponents;