// Vite plugin exposing the research wiki manifest as virtual modules:
//
//   virtual:research-manifest-en  →  export const docs: ResearchDoc[]
//   virtual:research-manifest-fr  →  export const docs: ResearchDoc[]
//
// The manifest is built by content.config.ts from web/content/research and
// drives the docs shell (sidebar, TOC, breadcrumbs, prev/next, related rail)
// and per-page SEO meta. Per-language modules keep the FR payload out of the
// EN chunk and vice versa. Content edits invalidate the modules in dev.

import type { Plugin, ViteDevServer } from "vite";
import {
  buildManifest,
  researchTopics,
  scanResearchContent,
  CONTENT_DIR,
} from "../content.config";

const IDS = {
  "virtual:research-manifest-en": "en",
  "virtual:research-manifest-fr": "fr",
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
      if (id in IDS) return "\0" + id;
      return null;
    },

    load(id) {
      if (!id.startsWith("\0")) return null;
      const bare = id.slice(1);
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

    configureServer(server) {
      devServer = server;
      server.watcher.add(CONTENT_DIR);
      const onContentChange = (file: string) => {
        if (!file.startsWith(CONTENT_DIR)) return;
        researchTopics(true); // bust both caches
        scanResearchContent(true);
        for (const bare of Object.keys(IDS)) {
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
