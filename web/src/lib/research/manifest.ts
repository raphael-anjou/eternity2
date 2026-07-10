// Client access to the research wiki manifest (built at compile time by
// plugins/research-content.ts from web/content/research frontmatter).

import {
  docs as docsEn,
  topics as topicsEn,
  authors as authorsEn,
} from "virtual:research-manifest-en";
import {
  docs as docsFr,
  topics as topicsFr,
  authors as authorsFr,
} from "virtual:research-manifest-fr";
import type { Lang } from "@/i18n";
import type { ResearchDoc, Topic, Author } from "./types";

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

/** The author registry, profile fields in the given language. */
export function researchAuthors(lang: Lang): Author[] {
  return lang === "fr" ? authorsFr : authorsEn;
}

export function researchAuthor(lang: Lang, slug: string): Author | undefined {
  return researchAuthors(lang).find((a) => a.slug === slug);
}

export function authorUrl(slug: string): string {
  return `/research/people/${slug}`;
}

/** Docs authored by one researcher, in global reading order. */
export function authorDocs(lang: Lang, slug: string): ResearchDoc[] {
  return researchDocs(lang).filter((d) => d.author === slug);
}
