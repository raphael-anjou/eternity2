// Build-time glossary auto-linker (rehype plugin, runs inside the MDX pipeline
// in web/vite.config.ts).
//
// For each research article it wraps the FIRST prose mention of each glossary
// term in a <Term termId="..."> element (registered in
// web/src/components/docs/mdx-map.ts). The wrapped text stays as the element's
// children, so it prerenders into the static HTML verbatim; the tooltip popup
// only mounts on hover/focus at runtime.
//
// Rules:
//   - Case-insensitive match on prose text nodes only.
//   - Skip headings (h1..h6), existing links (<a>), code (<code>/<pre>), math
//     (KaTeX / remark-math output), existing <Term> wraps, and the glossary
//     page itself. Frontmatter is already an export by the time this rehype
//     pass runs, so it is never a text node here.
//   - Page language comes from the filename suffix (.fr.mdx / .es.mdx / else en)
//     so the right localized term strings are matched and the right localized
//     anchor id is produced.
//   - At most MAX_TERMS_PER_PAGE terms per article, first mention only, so long
//     articles do not become tooltip soup.
//   - On overlapping matches the longest matcher wins ("beam width" over
//     "beam"), and the earliest position in the text is taken first.
//
// It emits mdxJsxTextElement nodes (not plain hast elements): hast-util-to-estree
// turns those into component references, so <Term> resolves against the MDX
// component map. A plain lowercase/uppercase hast element would become a literal
// DOM tag instead.

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";
import GithubSlugger from "github-slugger";

// ---- types (minimal local hast / mdast-jsx shapes) ----
interface Text {
  type: "text";
  value: string;
}
interface ElementNode {
  type: "element";
  tagName: string;
  properties?: Record<string, unknown>;
  children: Node[];
}
interface MdxJsxElement {
  type: "mdxJsxTextElement" | "mdxJsxFlowElement";
  name: string | null;
  attributes: unknown[];
  children: Node[];
}
interface Parent {
  type: string;
  children: Node[];
}
type Node = Text | ElementNode | MdxJsxElement | Parent | { type: string };

interface GlossaryEntry {
  term: { en: string; fr: string; es: string };
  see?: string;
}
type Lang = "en" | "fr" | "es";

const MAX_TERMS_PER_PAGE = 10;

// ---- glossary data (read once at plugin construction) ----
const HERE = path.dirname(fileURLToPath(import.meta.url));
const GLOSSARY_PATH = path.resolve(HERE, "../content/research/glossary.json");
const ENTRIES: GlossaryEntry[] = (() => {
  try {
    const parsed = JSON.parse(readFileSync(GLOSSARY_PATH, "utf8")) as {
      terms?: GlossaryEntry[];
    };
    return parsed.terms ?? [];
  } catch {
    return [];
  }
})();

/** Same anchor id GlossaryPage and the Term component assign. A fresh slugger
 *  per call is deterministic; the glossary has no per-language slug collisions,
 *  so the -1 dedupe suffix never fires. */
function termSlug(term: string): string {
  return new GithubSlugger().slug(term);
}

interface Matcher {
  needle: string; // lower-cased phrase to find in prose
  termId: string; // localized glossary anchor id
}

/** Build the matcher table for one language. Each term yields its base phrase
 *  (with any trailing "(...)" gloss stripped) and, separately, the gloss inside
 *  the parentheses, both pointing at the same localized anchor id. Matchers are
 *  sorted longest-first so an overlap resolves to the longer phrase. */
function buildMatchers(lang: Lang): Matcher[] {
  const seen = new Set<string>();
  const matchers: Matcher[] = [];
  for (const entry of ENTRIES) {
    const term = entry.term[lang];
    if (!term) continue;
    const termId = termSlug(term);
    const phrases: string[] = [];
    const parenMatch = /^(.*?)\s*\(([^)]+)\)\s*$/.exec(term);
    if (parenMatch && parenMatch[1] !== undefined && parenMatch[2] !== undefined) {
      phrases.push(parenMatch[1].trim());
      phrases.push(parenMatch[2].trim());
    } else {
      phrases.push(term.trim());
    }
    for (const phrase of phrases) {
      const needle = phrase.toLowerCase();
      // Skip empty phrases and one-character glosses that would over-match.
      if (needle.length < 2) continue;
      if (seen.has(needle)) continue;
      seen.add(needle);
      matchers.push({ needle, termId });
    }
  }
  matchers.sort((a, b) => b.needle.length - a.needle.length);
  return matchers;
}

const MATCHERS: Record<Lang, Matcher[]> = {
  en: buildMatchers("en"),
  fr: buildMatchers("fr"),
  es: buildMatchers("es"),
};

/** Page language from the MDX filename: foo.fr.mdx -> fr, foo.es.mdx -> es,
 *  otherwise en. */
function langFromPath(filePath: string): Lang {
  const base = path.basename(filePath);
  if (/\.fr\.mdx?$/.test(base)) return "fr";
  if (/\.es\.mdx?$/.test(base)) return "es";
  return "en";
}

/** True for a node whose subtree must not be linked. */
function isSkippedContainer(node: Node): boolean {
  if (node.type === "element") {
    const tag = (node as ElementNode).tagName;
    if (/^h[1-6]$/.test(tag)) return true;
    if (tag === "a" || tag === "code" || tag === "pre") return true;
    // KaTeX / remark-math output carries a `math` class.
    const cls = (node as ElementNode).properties?.["className"];
    const classList = Array.isArray(cls) ? cls.map(String) : typeof cls === "string" ? [cls] : [];
    if (classList.some((c) => c === "math" || c.startsWith("math-") || c.startsWith("katex")))
      return true;
  }
  if (node.type === "mdxJsxTextElement" || node.type === "mdxJsxFlowElement") {
    if ((node as MdxJsxElement).name === "Term") return true;
  }
  return false;
}

/** A word-boundary check on the raw text: the match must not sit inside a longer
 *  word (so "cell" does not fire inside "cellar"). Accent letters count as word
 *  characters. */
const WORD = /[\p{L}\p{N}]/u;
function boundedAt(text: string, start: number, end: number): boolean {
  const before = start > 0 ? text[start - 1] : "";
  const after = end < text.length ? text[end] : "";
  if (before && WORD.test(before)) return false;
  if (after && WORD.test(after)) return false;
  return true;
}

/** Make a <Term termId> mdx JSX element wrapping a single text child. */
function makeTerm(termId: string, value: string): MdxJsxElement {
  return {
    type: "mdxJsxTextElement",
    name: "Term",
    attributes: [
      {
        type: "mdxJsxAttribute",
        name: "termId",
        value: termId,
      },
    ],
    children: [{ type: "text", value }],
  };
}

export function rehypeGlossaryAutolink() {
  return function transform(tree: Node, file: { path?: string }): void {
    const filePath = file.path ?? "";
    // Never link on the glossary page itself (it is a generated component, not
    // MDX, but guard anyway) and only touch research MDX.
    if (/\/glossary(\.[a-z]{2})?\.mdx?$/.test(filePath)) return;
    if (!/\/content\/research\//.test(filePath.replace(/\\/g, "/"))) return;

    const lang = langFromPath(filePath);
    const matchers = MATCHERS[lang];
    if (matchers.length === 0) return;

    const usedTermIds = new Set<string>();
    let linkedCount = 0;

    /** Rewrite one text node into a sequence of text + Term nodes. Returns the
     *  replacement children (empty array means "no change"). */
    function rewriteText(value: string): Node[] | null {
      const out: Node[] = [];
      let cursor = 0;
      let changed = false;
      const lower = value.toLowerCase();

      while (cursor < value.length && linkedCount < MAX_TERMS_PER_PAGE) {
        // Find the earliest match among still-unused matchers; on a tie, the
        // longest matcher (matchers are pre-sorted longest-first, so the first
        // hit at the min index is already the longest).
        let bestStart = -1;
        let bestMatcher: Matcher | null = null;
        for (const m of matchers) {
          if (usedTermIds.has(m.termId)) continue;
          const idx = lower.indexOf(m.needle, cursor);
          if (idx === -1) continue;
          if (!boundedAt(value, idx, idx + m.needle.length)) {
            // Try later occurrences of this matcher within the node.
            let next = idx;
            let ok = -1;
            while (next !== -1) {
              if (boundedAt(value, next, next + m.needle.length)) {
                ok = next;
                break;
              }
              next = lower.indexOf(m.needle, next + 1);
            }
            if (ok === -1) continue;
            if (bestStart === -1 || ok < bestStart) {
              bestStart = ok;
              bestMatcher = m;
            }
            continue;
          }
          if (bestStart === -1 || idx < bestStart) {
            bestStart = idx;
            bestMatcher = m;
          }
        }

        if (bestStart === -1 || !bestMatcher) break;

        const matchEnd = bestStart + bestMatcher.needle.length;
        if (bestStart > cursor) {
          out.push({ type: "text", value: value.slice(cursor, bestStart) });
        }
        // Preserve the original casing of the matched text.
        out.push(makeTerm(bestMatcher.termId, value.slice(bestStart, matchEnd)));
        usedTermIds.add(bestMatcher.termId);
        linkedCount++;
        changed = true;
        cursor = matchEnd;
      }

      if (!changed) return null;
      if (cursor < value.length) {
        out.push({ type: "text", value: value.slice(cursor) });
      }
      return out;
    }

    function walk(node: Node): void {
      if (linkedCount >= MAX_TERMS_PER_PAGE) return;
      const parent = node as Parent;
      if (!Array.isArray(parent.children)) return;

      for (let i = 0; i < parent.children.length; i++) {
        const child = parent.children[i];
        if (!child) continue;
        if (child.type === "text") {
          if (linkedCount >= MAX_TERMS_PER_PAGE) continue;
          const replacement = rewriteText((child as Text).value);
          if (replacement) {
            parent.children.splice(i, 1, ...replacement);
            i += replacement.length - 1;
          }
          continue;
        }
        if (isSkippedContainer(child)) continue;
        walk(child);
      }
    }

    walk(tree);
  };
}

export default rehypeGlossaryAutolink;
