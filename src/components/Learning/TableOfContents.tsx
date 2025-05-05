import React from "react";

interface Props {
    tableOfContents: { title: string }[];
    onSelect: (title: string) => void;
}

const TableOfContents: React.FC<Props> = ({ tableOfContents, onSelect }) => {
    const _onSelect = (event: React.MouseEvent<HTMLButtonElement>) => {
        const title = event.currentTarget.dataset.title!;
        onSelect(title);
    };

    return (
        <div className="w-64 bg-gray-800 p-4">
            <ul className="space-y-2">
                {tableOfContents.map((item, index) => (
                    <li key={index}>
                        <button
                            data-title={item.title}
                            onClick={_onSelect}
                            className="cursor-pointer w-full text-left text-blue-400 hover:underline"
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