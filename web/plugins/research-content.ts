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
  scanResearchContent,
  searchEntries,
  CONTENT_DIR,
} from "../content.config";

/** Strip ESM import lines and export blocks (multi-line, brace/bracket
 *  balanced) from an MDX body, leaving readable markdown + JSX islands. */
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
  return out.join("\n").replace(/\n{3,}/g, "\n\n").trim();
}

const IDS = {
  "virtual:research-manifest-en": "en",
  "virtual:research-manifest-fr": "fr",
} as const;

// Full-text index payloads, split from the manifest so they only load when
// the search dialog opens (they carry every page's body text).
const SEARCH_IDS = {
  "virtual:research-search-en": "en",
  "virtual:research-search-fr": "fr",
} as const;

export function researchContent(): Plugin {
  let devServer: ViteDevServer | undefined;
  let isBuild = false;

  return {
    name: "research-content",

    configResolved(config) {
      isBuild = config.command === "build";
    },

    resolveId(id) {
      if (id in IDS || id in SEARCH_IDS) return "\0" + id;
      return null;
    },

    load(id) {
      if (!id.startsWith("\0")) return null;
      const bare = id.slice(1);
      if (bare in SEARCH_IDS) {
        const lang = SEARCH_IDS[bare as keyof typeof SEARCH_IDS];
        return `export const entries = ${JSON.stringify(searchEntries(lang))};`;
      }
      if (!(bare in IDS)) return null;
      const lang = IDS[bare as keyof typeof IDS];
      // Drafts stay reachable in dev so they can be written/previewed, but are
      // stripped from production builds (matching the prerender list).
      const docs = buildManifest(lang, { includeDrafts: !isBuild });
      // Topic registry, labels resolved to this language.
      const topics = researchTopics().map((t) => ({
        slug: t.slug,
        label: t.label[lang],
        description: t.description[lang],
      }));
      return `export const docs = ${JSON.stringify(docs)};\nexport const topics = ${JSON.stringify(topics)};`;
    },

    // Researcher/LLM affordance: every research page ships a raw-markdown
    // sibling (…/research/<slug>.md, FR under /fr/…) built from the MDX
    // source — frontmatter header + body with the ESM plumbing stripped.
    writeBundle(options) {
      const outDir = options.dir ?? "";
      if (!outDir.includes("client")) return; // client pass only (see sitemap plugin)
      const origin = (process.env["VITE_SITE_ORIGIN"] || "https://eternity2.dev").replace(/\/$/, "");
      const base = (process.env["BASE_PATH"] ?? "").replace(/\/$/, "");
      for (const lang of ["en", "fr"] as const) {
        for (const doc of buildManifest(lang)) {
          const raw = scanResearchContent().find((e) => e.file === doc.file);
          if (!raw) continue;
          const urlPath = (lang === "fr" ? "/fr" : "") + doc.url;
          const relFile =
            (lang === "fr" ? "fr/" : "") +
            (doc.slug === "" ? "research/index" : `research/${doc.slug}`) +
            ".md";
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
          const target = path.join(outDir, relFile);
          mkdirSync(path.dirname(target), { recursive: true });
          writeFileSync(target, header + stripMdxEsm(raw.body) + "\n");
        }
      }
    },

    configureServer(server) {
      devServer = server;
      server.watcher.add(CONTENT_DIR);
      const onContentChange = (file: string) => {
        if (!file.startsWith(CONTENT_DIR)) return;
        researchTopics(true); // bust both caches
        scanResearchContent(true);
        for (const bare of [...Object.keys(IDS), ...Object.keys(SEARCH_IDS)]) {
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
