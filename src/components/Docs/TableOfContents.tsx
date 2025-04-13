import React from "react";

interface TableOfContentsProps {
    tableOfContents: { title: string; content: string }[];
    onSelect: (title: string, content: string) => void;
}

const TableOfContents: React.FC<TableOfContentsProps> = ({ tableOfContents, onSelect }) => {
    return (
        <div className="w-64 bg-gray-800 p-4">
            <h2 className="text-lg font-bold mb-4">Table of Contents</h2>
            <ul className="space-y-2">
                {tableOfContents.map((item, index) => (
                    <li key={index}>
                        <button
                            onClick={() => onSelect(item.title, item.content)}
                            className="w-full text-left text-blue-400 hover:underline"
                        >
                            {item.title}
                        </button>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default TableOfContents;