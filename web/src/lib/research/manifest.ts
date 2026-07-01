// Client access to the research wiki manifest (built at compile time by
// plugins/research-content.ts from web/content/research frontmatter).

import { docs as docsEn } from "virtual:research-manifest-en";
import { docs as docsFr } from "virtual:research-manifest-fr";
import type { Lang } from "@/i18n";
import type { ResearchDoc } from "./types";

export function researchDocs(lang: Lang): ResearchDoc[] {
  return lang === "fr" ? docsFr : docsEn;
}

/** Look up a doc by its language-neutral site path ("/research/…"). */
export function researchDoc(lang: Lang, url: string): ResearchDoc | undefined {
  return researchDocs(lang).find((d) => d.url === url);
}
