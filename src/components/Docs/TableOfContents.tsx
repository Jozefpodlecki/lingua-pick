import React, { useCallback } from "react";
import { ContentItem } from "../../models";

const TableOfContentsItem: React.FC<{
    title: string;
    onClick: (event: React.MouseEvent<HTMLElement>) => void;
}> = ({ title, onClick }) => {
   
    return (
        <li>
            <button
                data-title={title}
                type="button"
                onClick={onClick}
                className="w-full text-left text-blue-400 hover:underline"
            >
                {title}
            </button>
        </li>
    );
};

interface TableOfContentsProps {
    tableOfContents: ContentItem[];
    onSelect: (item: ContentItem) => void;
}

const TableOfContents: React.FC<TableOfContentsProps> = ({ tableOfContents, onSelect }) => {
    const onClick = useCallback((event: React.MouseEvent<HTMLElement>) => {
        const target = event.currentTarget as HTMLElement;
        const title = target.dataset.title!;
        const contentItem = tableOfContents.find(pr => pr.title === title)!;
        onSelect(contentItem);
    }, [tableOfContents]);

    return (
        <div className="w-64 bg-gray-800 p-4">
            <h2 className="text-lg font-bold mb-4">Table of Contents</h2>
            <ul className="space-y-2">
                {tableOfContents.map((item, index) => (
                    <TableOfContentsItem
                        key={index}
                        title={item.title}
                        onClick={onClick}
                    />
                ))}
            </ul>
        </div>
    );
};

export default TableOfContents;
