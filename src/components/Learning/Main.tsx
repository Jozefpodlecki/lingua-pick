import React, { useState, Suspense, lazy } from "react";
import Breadcrumbs from "./Breadcrumbs";
import TableOfContents from "./TableOfContents";

interface ContentItem {
    title: string;
    component: React.LazyExoticComponent<React.FC>;
}

const tableOfContents: ContentItem[] = [
    { title: "Vocabulary", component: lazy(() => import("./content/VocabularyContent.tsx")) },
    { title: "Grammar", component: lazy(() => import("./content/GrammarContent.tsx")) },
];

const Main: React.FC = () => {
    const [selectedContent, setSelectedContent] = useState<ContentItem>(tableOfContents[0]);

    const onContentSelection = (content: ContentItem) => {
        setSelectedContent(content);
    };

    return (
        <div className="flex flex-col min-h-screen bg-black text-white">
            <div className="p-4">
                <Breadcrumbs selectedTitle={selectedContent.title} />
            </div>

            <div className="flex flex-1 h-full">
                <TableOfContents
                    tableOfContents={tableOfContents}
                    onSelect={(title) =>
                        onContentSelection(
                            tableOfContents.find((item) => item.title === title)!
                        )
                    }
                />

                <div className="border-l border-gray-700"></div>

                <div className="flex-1 bg-gray-800 p-6 overflow-y-auto">
                    <div className="max-w-3xl p-6 rounded shadow-lg">
                        <Suspense fallback={<div>Loading...</div>}>
                            <selectedContent.component />
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Main;