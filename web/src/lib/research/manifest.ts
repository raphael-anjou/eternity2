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

/** The string for a doc's <meta name="description">/og:description.
 *
 *  Prefers the page's explicit `metaDescription` (a hand-tuned short form).
 *  Otherwise falls back to `description`, truncated at a word boundary to keep
 *  the SERP snippet from being cut mid-word: Google shows ~155–160 chars, so we
 *  aim for ~155 and append an ellipsis only when we actually cut. Short
 *  descriptions pass through untouched. */
export function metaDescriptionFor(doc: Pick<ResearchDoc, "description" | "metaDescription">): string {
  if (doc.metaDescription) return doc.metaDescription;
  const full = doc.description.trim();
  const LIMIT = 157; // 155 target + a little slack; ellipsis makes it 158 max
  if (full.length <= LIMIT) return full;
  // Cut at the last word boundary at or before the limit, then trim trailing
  // punctuation/space so "… ," or "…." doesn't happen.
  const slice = full.slice(0, LIMIT);
  const lastSpace = slice.lastIndexOf(" ");
  const cut = (lastSpace > 80 ? slice.slice(0, lastSpace) : slice).replace(/[\s,;:.\-–—]+$/, "");
  return cut + "…";
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
