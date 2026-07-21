// Vite plugin exposing the research wiki manifest as virtual modules:
//
//   virtual:research-manifest-en  →  export const docs: ResearchDoc[]
//   virtual:research-manifest-fr  →  export const docs: ResearchDoc[]
//
// The manifest is built by content.config.ts from web/content/research and
// drives the docs shell (sidebar, TOC, breadcrumbs, prev/next, related rail)
// and per-page SEO meta. Per-language modules keep the FR payload out of the
// EN chunk and vice versa. Content edits invalidate the modules in dev.

import path from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import type { Plugin, ViteDevServer } from "vite";
import {
  buildManifest,
  researchTopics,
  researchAuthors,
  researchPagePathsFor,
  scanResearchContent,
  searchEntries,
  pickLang,
  LANG_CODES,
  TRANSLATION_LANGS,
  CONTENT_DIR,
  type Lang,
} from "../content.config";
import type { ResearchDoc } from "../src/lib/research/types";

/** Strip ESM import lines and export blocks (multi-line, brace/bracket
 *  balanced) from an MDX body, then reduce the remaining JSX component islands
 *  to readable NOTES (not silence), leaving clean markdown. The docs shell
 *  renders those components as interactive figures/labs; the raw-markdown sibling
 *  can only point at them, so each becomes an explicit note naming the component.
 *  Keeping the note (rather than dropping it) is deliberate: a crawling agent
 *  then knows the canonical page shows MORE than the markdown carries — a
 *  leaderboard, a chart, a live lab — and can go look or screenshot it. Authored
 *  prose, inline links, tables and code fences pass through untouched. */
function stripMdxEsm(body: string): string {
  const out: string[] = [];
  let depth = 0;
  let inExport = false;
  for (const line of body.split("\n")) {
    if (!inExport && /^import\s/.test(line)) continue;
    if (!inExport && /^export\s/.test(line)) {
      depth = 0;
      inExport = true;
    }
    if (inExport) {
      for (const ch of line) {
        if (ch === "{" || ch === "[" || ch === "(") depth++;
        else if (ch === "}" || ch === "]" || ch === ")") depth--;
      }
      if (depth <= 0 && !/[{[(]\s*$/.test(line)) inExport = false;
      continue;
    }
    out.push(line);
  }
  return stripJsxIslands(out.join("\n")).replace(/\n{3,}/g, "\n\n").trim();
}

/** Reduce the JSX component islands in an MDX body to clean markdown, in two
 *  classes:
 *
 *  1. CONTENT-bearing wrappers — `<Callout>` and `<Door>` — carry authored prose
 *     that must survive. A Callout is unwrapped into a titled blockquote note; a
 *     Door (a navigation card) becomes a markdown link with its blurb.
 *  2. FIGURE / LAB components — `<Figure>` and every other PascalCase island —
 *     render a chart, board, leaderboard or live lab the markdown cannot carry.
 *     These become an explicit note NAMING the component, so a crawling agent
 *     knows the canonical page shows more than the export and can go look or
 *     screenshot it. (Deliberately not silent — the note is the signal.)
 *
 *  Lowercase HTML tags, inline `<ProseLink>`-free links, tables and code fences
 *  are left untouched. */
function stripJsxIslands(md: string): string {
  const attr = (attrs: string, name: string): string =>
    new RegExp(`\\b${name}="([^"]*)"`).exec(attrs)?.[1] ?? "";
  const compNames = (s: string): string => {
    const names = [...s.matchAll(/<([A-Z][A-Za-z0-9]*)\b/g)].map((m) => m[1]);
    return names.length ? [...new Set(names)].join(", ") : "";
  };
  const tidy = (s: string): string => s.replace(/\s+/g, " ").trim();
  return (
    md
      // <Door to="/x" title="X"> blurb </Door> → a markdown link with its blurb.
      .replace(/<Door\b([^>]*)>([\s\S]*?)<\/Door>/g, (_m, attrs: string, body: string) => {
        const to = attr(attrs, "to") || attr(attrs, "href");
        const title = attr(attrs, "title");
        const blurb = tidy(body.replace(/<[^>]+>/g, ""));
        const label = title || blurb || to;
        return to ? `- [${label}](${to})${title && blurb ? ` — ${blurb}` : ""}` : blurb;
      })
      // <Callout kind="note" title="X"> prose </Callout> → a titled blockquote.
      .replace(/<Callout\b([^>]*)>([\s\S]*?)<\/Callout>/g, (_m, attrs: string, body: string) => {
        const title = attr(attrs, "title");
        const kind = attr(attrs, "kind") || "note";
        const prose = tidy(body);
        const head = title ? `**${title}**` : `**${kind[0]?.toUpperCase()}${kind.slice(1)}**`;
        return `> ${head}\n>\n> ${prose}`;
      })
      // <ProseLink href="/x"> text </ProseLink> → inline markdown link.
      .replace(/<ProseLink\b([^>]*)>([\s\S]*?)<\/ProseLink>/g, (_m, attrs: string, body: string) => {
        const href = attr(attrs, "href") || attr(attrs, "to");
        const text = tidy(body.replace(/<[^>]+>/g, ""));
        return href ? `[${text}](${href})` : text;
      })
      // <Figure ... title="X" ...> component(s) </Figure> → a titled note naming
      // the interactive component(s) it wraps.
      .replace(/<Figure\b([^>]*)>([\s\S]*?)<\/Figure>/g, (_m, attrs: string, body: string) => {
        const title = attr(attrs, "title");
        const comps = compNames(body);
        const what = comps ? `interactive: ${comps}` : "interactive figure";
        const named = title ? `${title} — ${what}` : what;
        return `> **[Figure]** ${named}. Rendered on the canonical page (link above); not shown in this markdown export.`;
      })
      // A standalone self-closing island: <PascalCase ... />
      .replace(
        /^\s*<([A-Z][A-Za-z0-9]*)\b[^>]*\/>\s*$/gm,
        (_m, name: string) =>
          `> **[Interactive: ${name}]** Rendered on the canonical page (link above); not shown in this markdown export.`,
      )
      // Any remaining stray open/close PascalCase tag on its own line.
      .replace(/^\s*<\/?[A-Z][A-Za-z0-9]*\b[^>]*>\s*$/gm, "")
  );
}

/** A markdown link-list of docs (title, url, one-line blurb) under a `##`
 *  heading, or "" if there are none. Used to fold the two big blocks the docs
 *  shell renders around the MDX body — the hub's child pages and the related
 *  rail — into the raw-markdown export, which otherwise only carries the intro
 *  prose. Links are absolute site paths so the .md stays navigable on its own. */
function mdLinkList(
  heading: string,
  items: ResearchDoc[],
  origin: string,
  base: string,
): string {
  if (items.length === 0) return "";
  const lines = items.map((d) => {
    const url = `${origin}${base}${d.url}`;
    const blurb = d.description ? ` — ${d.description.trim().replace(/\s+/g, " ")}` : "";
    return `- [${d.title}](${url})${blurb}`;
  });
  return `\n\n## ${heading}\n\n${lines.join("\n")}`;
}

/** The child pages the docs shell renders as cards under a hub page, derived by
 *  URL nesting: a hub at `/research/a/b` lists the docs at `/research/a/b/<x>`
 *  (exactly one segment deeper), in the manifest's order. Empty for a leaf page.
 *  This mirrors HubCards without needing the browser-only nav module. */
function hubChildren(doc: ResearchDoc, all: ResearchDoc[]): ResearchDoc[] {
  const prefix = doc.url.replace(/\/$/, "") + "/";
  return all.filter(
    (d) => d.url !== doc.url && d.url.startsWith(prefix) && !d.url.slice(prefix.length).includes("/"),
  );
}

/** The related-rail docs for a page, resolving its `related` url list against
 *  the manifest, in the order the page listed them. */
function relatedDocs(doc: ResearchDoc, all: ResearchDoc[]): ResearchDoc[] {
  return doc.related
    .map((u) => all.find((d) => d.url === u))
    .filter((d): d is ResearchDoc => d !== undefined);
}

// ---- Auto-backlinks ("Referenced by") ------------------------------------
//
// A per-page reverse map of INBOUND prose links: for each research page, which
// other research pages link to it in their body. This is the reverse of the
// links check-research-links.mjs validates; the extraction below reuses that
// script's link-shapes (markdown links + href/to attributes) so the two stay in
// step, but deliberately DROP the frontmatter `- /research/...` list shape
// (those are the curated `related[]` entries the related rail already renders).
// The map keys and values are language-neutral site URLs ("/research/…"); the
// docs shell resolves each source URL's title/kind from the per-language nav, so
// one map serves all three languages.

/** Language-neutral URL of a raw entry's page ("/research/<slug>", or
 *  "/research" for the root hub). Mirrors buildManifest's url derivation so a
 *  source page maps to the same URL the manifest exposes. */
function entryUrl(slug: string): string {
  return slug === "" ? "/research" : `/research/${slug}`;
}

/** Inline prose links to internal /research pages found in an MDX body: the two
 *  authored shapes (markdown `[text](/research/…)` and JSX `href=/`to=` on a
 *  ProseLink or other island). Anchors/query strings are stripped and the path
 *  is de-trailing-slashed so it matches a manifest URL. Frontmatter `related`
 *  list items are NOT here (they are matched by their own `- /research/…` line
 *  shape in the links check and belong to the related rail, not backlinks). */
function proseLinkTargets(body: string): string[] {
  const raw = [
    ...[...body.matchAll(/\]\((\/research\/[^)\s#?]*)[^)]*\)/g)].map((m) => m[1]),
    ...[...body.matchAll(/(?:href|to)=["'{]+(\/research\/[^"'}\s#?]*)/g)].map((m) => m[1]),
  ];
  return raw.map((l) => (l ?? "").replace(/\/$/, "")).filter((l) => l.length > 0);
}

/** Build the reverse backlink map from every scanned entry (any language body),
 *  keyed by the target's language-neutral URL, value = the source page URLs that
 *  link to it, deduped and in manifest reading order. A page never lists itself.
 *  Only links that resolve to a real, non-draft page are kept (the same known-URL
 *  set the docs shell can render), so a typo'd or removed target contributes
 *  nothing rather than a dead "Referenced by" row. */
function buildBacklinks(): Record<string, string[]> {
  const docs = buildManifest("en");
  const known = new Set(docs.map((d) => d.url));
  // Source ordering: index sources by their position in the manifest so the
  // rendered list is stable (manifest reading order), not filesystem order.
  const orderOf = new Map(docs.map((d, i) => [d.url, i]));
  // target URL → set of source URLs.
  const rev = new Map<string, Set<string>>();
  for (const entry of scanResearchContent()) {
    const sourceUrl = entryUrl(entry.slug);
    if (!known.has(sourceUrl)) continue; // draft/unknown source page
    for (const target of proseLinkTargets(entry.body)) {
      if (!known.has(target) || target === sourceUrl) continue;
      const set = rev.get(target) ?? new Set<string>();
      set.add(sourceUrl);
      rev.set(target, set);
    }
  }
  const out: Record<string, string[]> = {};
  for (const [target, sources] of rev) {
    out[target] = [...sources].sort(
      (a, b) => (orderOf.get(a) ?? 1e9) - (orderOf.get(b) ?? 1e9),
    );
  }
  return out;
}

// The curated top of llms.txt: the framing and the top-level (non-research)
// pages, hand-written because those TSX pages carry no frontmatter description.
// The complete research map is generated below it, so this header stays short
// and the page list never goes stale. `{ORIGIN}` is substituted at build time.
const LLMS_HEADER = `# Eternity II community site (eternity2.dev)

> An open-source, static, multilingual (English / French / Spanish) educational hub for the Eternity II edge-matching puzzle. You can play it, watch real solvers run in your browser, learn the backtracking algorithms behind it, import/export/score community boards, and read the research. The solver is a Rust engine compiled to WebAssembly, so there is no server and no user data.

Key facts an assistant should know when answering about this site:

- **The puzzle.** Eternity II is a 16x16 edge-matching puzzle: 256 square tiles, each edge bearing one of 22 colors/motifs, must tile a board so every shared edge matches and the border is grey. It carried a US $2,000,000 prize (unclaimed; expired 2010). The best public board matches 470 of 480 edges; a perfect solution (480) has never been found. The search space is ~1.115 x 10^557 raw arrangements.
- **What's verifiable here.** Every claim on the site (record boards, piece set, clues, scores) is checked by the same Rust/WASM engine the playground runs, and cross-validated against real e2.bucas.name boards.
- **License / reuse.** Open source. Content is meant to be indexed, learned from, and cited freely. When citing, link to the relevant page.
- **No server.** Pages are pre-rendered to static HTML, so every URL is directly fetchable. The French tree mirrors the English tree under /fr, the Spanish tree under /es.
- **Markdown for machines.** Every research page has a raw-markdown sibling at the same URL with \`.md\` appended (e.g. {ORIGIN}/research/build/known-facts.md); prefer those when quoting or ingesting. The whole research corpus in one file is at {ORIGIN}/llms-full.txt.

## Top-level pages

- [Home]({ORIGIN}/): overview of the puzzle and the site's sections.
- [The Puzzle]({ORIGIN}/puzzle): history, piece-set anatomy, all 256 pieces, the 22 motifs and their rarity, the 5 official clues, the record table, and complexity numbers.
- [Algorithms]({ORIGIN}/algorithms): DFS and backtracking from scratch, with a scrubbable slow-motion demo, the exponential wall, live binary demos, and difficulty charts measured by the engine.
- [Board Viewer]({ORIGIN}/viewer): import/export of e2.bucas.name URLs, live scoring, conflict marks, a verification card, famous boards, and a solvable-board generator.
- [Converter]({ORIGIN}/convert): paste any board format (URL, board_edges, params) and read back every other format with a live preview and score.
- [Playground]({ORIGIN}/playground): interactive in-browser solver demos ([solve]({ORIGIN}/playground/solve), [watch]({ORIGIN}/playground/watch), [paths]({ORIGIN}/playground/paths)).
- [Status]({ORIGIN}/status) and [Is it a scam?]({ORIGIN}/is-it-a-scam): the plain answers to "is it solved" and "is the prize real".
- [Repository]({{REPO}}): the Rust to WASM engine, the static site, the contribution guide.
`;

const REPO_URL = "https://github.com/raphael-anjou/eternity2";

/** The complete llms.txt: the curated header, then every research page grouped
 *  by section with its description and its .md sibling. Generated from the
 *  manifest so the map can never fall out of sync with the pages. */
function buildLlmsTxt(
  docs: ResearchDoc[],
  origin: string,
  base: string,
  canonical: (p: string) => string,
): string {
  const SECTIONS: { key: string; label: string }[] = [
    { key: "why", label: "Research: why the puzzle is hard" },
    { key: "build", label: "Research: how to build a solver" },
    { key: "lab", label: "Research: the lab notebook (findings & experiments)" },
    { key: "community", label: "Research: history & community" },
  ];
  const md = (url: string) => `${origin}${base}${canonical(url).replace(/\/$/, "")}.md`;
  const parts: string[] = [
    LLMS_HEADER.replace(/\{ORIGIN\}/g, `${origin}${base}`).replace("{{REPO}}", REPO_URL),
  ];
  for (const { key, label } of SECTIONS) {
    const inSection = docs
      .filter((d) => d.section === key)
      .sort((a, b) => a.url.localeCompare(b.url));
    if (!inSection.length) continue;
    parts.push(`\n## ${label}\n`);
    for (const d of inSection) {
      const desc = (d.metaDescription ?? d.description).replace(/\s+/g, " ").trim();
      parts.push(`- [${d.title}](${origin}${base}${canonical(d.url)}) (md: ${md(d.url)}): ${desc}`);
    }
  }
  parts.push(
    `\n## Optional\n`,
    `- [French home](${origin}${base}/fr): the whole site mirrored under /fr; skip unless answering in French.`,
    `- [Spanish home](${origin}${base}/es): the whole site mirrored under /es; skip unless answering in Spanish.`,
    `- [Sitemap](${origin}${base}/sitemap.xml): machine-readable list of every page.`,
    "",
  );
  return parts.join("\n");
}

/** llms-full.txt: the entire research corpus as one markdown file, so an agent
 *  can ingest everything in a single fetch. Uses the same clean-markdown bodies
 *  as the per-page .md siblings. */
function buildLlmsFull(
  bodies: { doc: ResearchDoc; markdown: string }[],
  origin: string,
  base: string,
): string {
  const head = [
    `# Eternity II research corpus (eternity2.dev) — full text`,
    "",
    `> Every research page on eternity2.dev, concatenated. This is the machine-ingestible mirror of the wiki at ${origin}${base}/research. The map with links is at ${origin}${base}/llms.txt.`,
    "",
    "---",
    "",
  ].join("\n");
  const ordered = [...bodies].sort((a, b) => a.doc.url.localeCompare(b.doc.url));
  return head + ordered.map((b) => b.markdown).join("\n\n---\n\n") + "\n";
}

// The per-language virtual modules, derived from the language registry so a new
// language wires itself up. For each language:
//   virtual:research-manifest-<lang>    → docs/topics/authors, resolved to <lang>
//   virtual:research-search-<lang>      → the full-text index for <lang>
// and for each NON-English language:
//   virtual:research-translated-<lang>  → the URLs with a genuine <lang> rendering
const MANIFEST_ID = (lang: Lang) => `virtual:research-manifest-${lang}`;
const SEARCH_ID = (lang: Lang) => `virtual:research-search-${lang}`;
// A tiny module (just a string[] of language-neutral research URLs that have a
// genuine <lang> rendering) so the root shell can decide whether to advertise
// hreflang="<lang>" for a research page WITHOUT pulling the full ~400KB manifest
// into every page's chunk. Non-research pages always have a /<lang> twin, so they
// are not listed here — the root shell only consults this for /research paths.
const TRANSLATED_ID = (lang: Lang) => `virtual:research-translated-${lang}`;
// The reverse-backlink map ("Referenced by"). Language-neutral (keys + values
// are neutral /research URLs; the shell resolves display metadata per language),
// so a single id, not one per language.
const BACKLINKS_ID = "virtual:research-backlinks";

// Reverse lookup: bare virtual id → parsed descriptor, or null if not ours. The
// backlinks map has no language (kind "backlinks", lang omitted).
function parseId(
  bare: string,
): { kind: "manifest" | "search" | "translated"; lang: Lang } | { kind: "backlinks" } | null {
  if (bare === BACKLINKS_ID) return { kind: "backlinks" };
  for (const lang of LANG_CODES) {
    if (bare === MANIFEST_ID(lang)) return { kind: "manifest", lang };
    if (bare === SEARCH_ID(lang)) return { kind: "search", lang };
  }
  for (const lang of TRANSLATION_LANGS) {
    if (bare === TRANSLATED_ID(lang)) return { kind: "translated", lang };
  }
  return null;
}

export function researchContent(): Plugin {
  let devServer: ViteDevServer | undefined;
  let isBuild = false;

  return {
    name: "research-content",

    configResolved(config) {
      isBuild = config.command === "build";
    },

    resolveId(id) {
      if (parseId(id)) return "\0" + id;
      return null;
    },

    load(id) {
      if (!id.startsWith("\0")) return null;
      const parsed = parseId(id.slice(1));
      if (!parsed) return null;
      if (parsed.kind === "backlinks") {
        // Reverse map of inbound prose links: targetUrl → sourceUrl[]. Regenerated
        // on every load, so a dev content edit reflects immediately (the module is
        // invalidated in configureServer). Never absent while the plugin is
        // installed; the client still guards against an empty payload.
        return `export const backlinks = ${JSON.stringify(buildBacklinks())};`;
      }
      const { kind, lang } = parsed;
      if (kind === "translated") {
        // Language-neutral URLs ("/research/…") that have a real <lang>
        // rendering. researchPagePathsFor() returns basename-relative paths
        // ("research/…"); expose them as absolute "/research/…" URLs.
        const urls = researchPagePathsFor(lang).map((p) => `/${p}`);
        return `export const translated = ${JSON.stringify(urls)};`;
      }
      if (kind === "search") {
        return `export const entries = ${JSON.stringify(searchEntries(lang))};`;
      }
      // kind === "manifest"
      // Drafts stay reachable in dev so they can be written/previewed, but are
      // stripped from production builds (matching the prerender list).
      const docs = buildManifest(lang, { includeDrafts: !isBuild });
      // Topic registry, labels resolved to this language (EN fallback per field).
      const topics = researchTopics().map((t) => ({
        slug: t.slug,
        label: pickLang(t.label, lang),
        description: pickLang(t.description, lang),
      }));
      // Author registry, profile fields resolved to this language (EN fallback).
      const authors = researchAuthors().map((a) => ({
        slug: a.slug,
        name: a.name,
        ...(a.tagline ? { tagline: pickLang(a.tagline, lang) } : {}),
        ...(a.affiliation ? { affiliation: pickLang(a.affiliation, lang) } : {}),
        ...(a.bio ? { bio: pickLang(a.bio, lang) } : {}),
        links: a.links,
      }));
      return (
        `export const docs = ${JSON.stringify(docs)};\n` +
        `export const topics = ${JSON.stringify(topics)};\n` +
        `export const authors = ${JSON.stringify(authors)};`
      );
    },

    // Researcher/LLM affordance: every research page ships a raw-markdown
    // sibling (…/research/<slug>.md) built from the MDX source — frontmatter
    // header + body with the ESM plumbing stripped. The research wiki is
    // English-only, so only EN siblings are emitted.
    writeBundle(options) {
      const outDir = options.dir ?? "";
      if (!outDir.includes("client")) return; // client pass only (see sitemap plugin)
      const origin = (process.env["VITE_SITE_ORIGIN"] || "https://eternity2.dev").replace(/\/$/, "");
      const base = (process.env["BASE_PATH"] ?? "").replace(/\/$/, "");
      // Trailing-slash form the host serves at 200 (see the sitemap plugin).
      const canonical = (p: string) =>
        p === "/" || /\.[a-z0-9]+$/i.test(p) || p.endsWith("/") ? p : p + "/";
      {
        const allDocs = buildManifest("en");
        // Collected as we write each .md, then folded into llms-full.txt so the
        // "everything in one fetch" file uses the exact same clean-markdown
        // rendering as the per-page siblings.
        const fullBodies: { doc: ResearchDoc; markdown: string }[] = [];
        for (const doc of allDocs) {
          const raw = scanResearchContent().find((e) => e.file === doc.file);
          if (!raw) continue;
          const urlPath = canonical(doc.url);
          const relFile =
            (doc.slug === "" ? "research/index" : `research/${doc.slug}`) + ".md";
          const header = [
            `# ${doc.title}`,
            "",
            `> ${doc.description}`,
            "",
            `- Canonical page (with interactive figures/demos): ${origin}${base}${urlPath}`,
            ...(doc.updated ? [`- Updated: ${doc.updated}`] : []),
            ...(doc.topics.length ? [`- Topics: ${doc.topics.join(", ")}`] : []),
            ...(doc.repro?.cmd ? [`- Reproduce: \`${doc.repro.cmd}\``] : []),
            ...doc.sources.map((s) => `- Source: ${s.label} — ${s.url}`),
            "",
            "---",
            "",
          ].join("\n");
          // The docs shell renders two link blocks around the MDX body that the
          // source prose never contains: a hub's child pages (as cards) and the
          // related rail. Fold both into the export so the .md carries the same
          // navigation an agent sees on the page, not just the intro prose.
          const children = mdLinkList("Pages in this section", hubChildren(doc, allDocs), origin, base);
          const related = mdLinkList("Related", relatedDocs(doc, allDocs), origin, base);
          const body = header + stripMdxEsm(raw.body) + children + related + "\n";
          const target = path.join(outDir, relFile);
          mkdirSync(path.dirname(target), { recursive: true });
          writeFileSync(target, body);
          fullBodies.push({ doc, markdown: body });
        }

        // llms.txt (the map) and llms-full.txt (the whole corpus in one file),
        // generated from the same manifest so they never drift from the pages.
        // The curated header (the top-level pages + framing) is hand-written; the
        // complete research map below it is generated, every page with its
        // description and its .md sibling, so an agent sees ALL 100+ pages, not a
        // curated 10. See public/llms.txt for the header source.
        writeFileSync(path.join(outDir, "llms.txt"), buildLlmsTxt(allDocs, origin, base, canonical));
        writeFileSync(path.join(outDir, "llms-full.txt"), buildLlmsFull(fullBodies, origin, base));
      }
    },

    configureServer(server) {
      devServer = server;
      server.watcher.add(CONTENT_DIR);
      const onContentChange = (file: string) => {
        if (!file.startsWith(CONTENT_DIR)) return;
        researchTopics(true); // bust all caches
        researchAuthors(true);
        scanResearchContent(true);
        // Invalidate every per-language virtual module (manifest + search +
        // translated list) so a content edit reloads in dev.
        const bareIds = [
          ...LANG_CODES.map(MANIFEST_ID),
          ...LANG_CODES.map(SEARCH_ID),
          ...TRANSLATION_LANGS.map(TRANSLATED_ID),
          BACKLINKS_ID,
        ];
        for (const bare of bareIds) {
          const mod = devServer?.moduleGraph.getModuleById("\0" + bare);
          if (mod) devServer?.moduleGraph.invalidateModule(mod);
        }
        devServer?.ws.send({ type: "full-reload" });
      };
      server.watcher.on("change", onContentChange);
      server.watcher.on("add", onContentChange);
      server.watcher.on("unlink", onContentChange);
    },
  };
}
