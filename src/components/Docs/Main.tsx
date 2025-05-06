import React, { useState, useEffect } from "react";
import ReactMarkdown from "react-markdown";
import Breadcrumbs from "./Breadcrumbs";
import TableOfContents from "./TableOfContents";
import MarkdownComponents from "./MarkdownComponents";
import { loadMarkdownFiles, MarkdownFile } from "../../utils/loadMarkdownFiles";
import { ContentItem } from "../../models";

interface State {
    selectedTitle: string;
    selectedContent: string;
    tableOfContents: MarkdownFile[];
}

const Docs: React.FC = () => {
    const [state, setState] = useState<State>({
        selectedTitle: "Loading...",
        selectedContent: "Loading...",
        tableOfContents: [],
    });

    useEffect(() => {
        const fetchMarkdownFiles = async () => {
            const contents = await loadMarkdownFiles();
            const content = contents[0];

            setState({
                selectedTitle: content?.title || "Untitled",
                selectedContent: content?.content || "No content available.",
                tableOfContents: contents,
            });
        };

        fetchMarkdownFiles();
    }, []);

    const onContentSelection = (item: ContentItem) => {
        setState((prevState) => ({
            ...prevState,
            selectedTitle: item.title,
            selectedContent: item.content,
        }));
    };

    return (
        <div className="flex flex-col min-h-screen bg-black text-white">
            <div className="p-4">
                <Breadcrumbs selectedTitle={state.selectedTitle} />
            </div>
            <div className="flex flex-1 h-full">
                <TableOfContents
                    tableOfContents={state.tableOfContents}
                    onSelect={onContentSelection}
                />
                <div className="border-l border-gray-700"></div>
                <div className="flex-1 bg-gray-800 p-6 overflow-y-auto">
                    <div className="max-w-3xl p-6 rounded shadow-lg">
                        <ReactMarkdown components={MarkdownComponents}>
                            {state.selectedContent}
                        </ReactMarkdown>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Docs;