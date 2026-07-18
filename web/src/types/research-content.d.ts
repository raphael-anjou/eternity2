// Module declarations for the research wiki content pipeline: MDX pages and
// the virtual manifest modules served by plugins/research-content.ts.

declare module "*.mdx" {
  import type { MDXContent } from "mdx/types";
  const Content: MDXContent;
  export default Content;
  export const frontmatter: Record<string, unknown> | undefined;
}

// Per-language virtual modules served by plugins/research-content.ts, one set
// per registry language (see src/i18n LANGS). The plugin builds these by string
// id; these declarations give each a type.

declare module "virtual:research-manifest-en" {
  import type { ResearchDoc, Topic, Author } from "@/lib/research/types";
  export const docs: ResearchDoc[];
  export const topics: Topic[];
  export const authors: Author[];
}

declare module "virtual:research-manifest-fr" {
  import type { ResearchDoc, Topic, Author } from "@/lib/research/types";
  export const docs: ResearchDoc[];
  export const topics: Topic[];
  export const authors: Author[];
}

declare module "virtual:research-manifest-es" {
  import type { ResearchDoc, Topic, Author } from "@/lib/research/types";
  export const docs: ResearchDoc[];
  export const topics: Topic[];
  export const authors: Author[];
}

declare module "virtual:research-search-en" {
  import type { SearchEntry } from "@/lib/research/types";
  export const entries: SearchEntry[];
}

declare module "virtual:research-search-fr" {
  import type { SearchEntry } from "@/lib/research/types";
  export const entries: SearchEntry[];
}

declare module "virtual:research-search-es" {
  import type { SearchEntry } from "@/lib/research/types";
  export const entries: SearchEntry[];
}

declare module "virtual:research-translated-fr" {
  // Language-neutral URLs ("/research/…") that have a genuine French rendering.
  export const translated: string[];
}

declare module "virtual:research-translated-es" {
  // Language-neutral URLs ("/research/…") that have a genuine Spanish rendering.
  export const translated: string[];
}
