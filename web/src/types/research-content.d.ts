// Module declarations for the research wiki content pipeline: MDX pages and
// the virtual manifest modules served by plugins/research-content.ts.

declare module "*.mdx" {
  import type { MDXContent } from "mdx/types";
  const Content: MDXContent;
  export default Content;
  export const frontmatter: Record<string, unknown> | undefined;
}

declare module "virtual:research-manifest-en" {
  import type { ResearchDoc, Topic } from "@/lib/research/types";
  export const docs: ResearchDoc[];
  export const topics: Topic[];
}

declare module "virtual:research-manifest-fr" {
  import type { ResearchDoc, Topic } from "@/lib/research/types";
  export const docs: ResearchDoc[];
  export const topics: Topic[];
}
