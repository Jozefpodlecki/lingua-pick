import React, { useState, Suspense } from "react";
import Breadcrumbs from "./Breadcrumbs";
import TableOfContents from "./TableOfContents";
import { useLanguage } from "../../context/LanguageContext.tsx";
import { getTableOfContents } from "./utils";

interface ContentItem {
    title: string;
    component: React.LazyExoticComponent<React.FC>;
}

const Main: React.FC = () => {
    const { selectedLanguage } = useLanguage();
    const tableOfContents = getTableOfContents(selectedLanguage!.isoCode);
    const [selectedContent, setSelectedContent] = useState<ContentItem | null>(tableOfContents[0]);

    const onContentSelection = (title: String) => {
        const contentItem = tableOfContents.find((item) => item.title === title);
        setSelectedContent(contentItem!);
    };

    return (
        <main className="flex flex-col min-h-screen bg-black text-white">
            <div className="p-4">
                <Breadcrumbs selectedTitle={selectedContent?.title || "Learning"} />
            </div>

            <div className="flex flex-1 h-full">
                <TableOfContents
                    tableOfContents={tableOfContents}
                    onSelect={onContentSelection}
                />

                <div className="border-l border-gray-700"></div>

                <div className="flex-1 bg-gray-800 p-6 overflow-y-auto">
                    <div className="max-w-3xl p-6 rounded shadow-lg">
                        {selectedContent ? (
                            <Suspense fallback={<div>Loading...</div>}>
                                <selectedContent.component />
                            </Suspense>
                        ) : (
                            <p>Select a topic to start learning.</p>
                        )}
                    </div>
                </div>
            </div>
        </main>
    );
};

export default Main;