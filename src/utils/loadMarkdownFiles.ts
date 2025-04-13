export interface MarkdownFile {
    title: string;
    content: string;
}

export const loadMarkdownFiles = async (): Promise<MarkdownFile[]> => {
    const markdownFiles = import.meta.glob("../components/Docs/markdowns/*.md");

    const contents = await Promise.all(
        Object.entries(markdownFiles).map(async ([path, loader]) => {
            const module = (await loader()) as { default: string };
            const title = path.split("/").pop()?.replace(".md", "") || "Untitled";
            return { title, content: module.default };
        })
    );

    return contents;
};