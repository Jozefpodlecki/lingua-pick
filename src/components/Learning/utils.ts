import { lazy } from "react";

interface ContentItem {
    title: string;
    component: React.LazyExoticComponent<React.FC>;
}

export const getTableOfContents = (isoCode: string): ContentItem[] => [
    {
        title: "Vocabulary",
        component: lazy(() =>
            import(`./content/${isoCode}/VocabularyContent.tsx`)
        ),
    },
    {
        title: "Grammar",
        component: lazy(() =>
            import(`./content/${isoCode}/GrammarContent.tsx`)
        ),
    },
];