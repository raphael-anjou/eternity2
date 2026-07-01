// Client access to the research wiki manifest (built at compile time by
// plugins/research-content.ts from web/content/research frontmatter).

import { docs as docsEn, topics as topicsEn } from "virtual:research-manifest-en";
import { docs as docsFr, topics as topicsFr } from "virtual:research-manifest-fr";
import type { Lang } from "@/i18n";
import type { ResearchDoc, Topic } from "./types";

export function researchDocs(lang: Lang): ResearchDoc[] {
  return lang === "fr" ? docsFr : docsEn;
}

/** Look up a doc by its language-neutral site path ("/research/…"). */
export function researchDoc(lang: Lang, url: string): ResearchDoc | undefined {
  return researchDocs(lang).find((d) => d.url === url);
}

/** The topic registry, labels in the given language (registry order). */
export function researchTopics(lang: Lang): Topic[] {
  return lang === "fr" ? topicsFr : topicsEn;
}

export function researchTopic(lang: Lang, slug: string): Topic | undefined {
  return researchTopics(lang).find((t) => t.slug === slug);
}

export function topicUrl(slug: string): string {
  return `/research/topics/${slug}`;
}
