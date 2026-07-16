#!/usr/bin/env node
// Ping IndexNow with every URL in the built sitemap, so Bing (and the other
// IndexNow participants, which share submissions) re-crawl changed pages fast.
// This targets the one gap classic SEO leaves open for this site: Bing's index,
// which ChatGPT search leans on. Google does not use IndexNow, so this changes
// nothing there; it is purely the Bing/answer-engine channel.
//
// It is a notification, not a ranking lever: it gets pages *considered* sooner,
// nothing more. Ownership is proved by a static key file at the site root
// (web/public/<key>.txt), which the build copies to the deployed root.
//
// Run after a deploy, from the web/ directory:
//   node scripts/indexnow-submit.mjs
// Reads the URL list from build/client/sitemap.xml (the single source of truth,
// emitted by the sitemap plugin), so it never drifts from what actually shipped.
//
// Config (all optional):
//   INDEXNOW_KEY        the key (defaults to the committed key below)
//   VITE_SITE_ORIGIN    site origin, no trailing slash (default eternity2.dev)
//   INDEXNOW_DRY_RUN=1  print what would be sent, do not POST
//   INDEXNOW_SITEMAP    path to the sitemap (default ../build/client/sitemap.xml
//                       relative to this script, i.e. web/build/client)

import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// The IndexNow key. Not a secret (it is published at the site root to prove
// ownership), so committing it is fine and keeps CI configuration-free. Must
// match the filename hosted at web/public/<key>.txt.
const DEFAULT_KEY = "c7259bc712de2f2d86298cf28e768b82";

const key = process.env["INDEXNOW_KEY"] || DEFAULT_KEY;
const origin = (process.env["VITE_SITE_ORIGIN"] || "https://eternity2.dev").replace(/\/$/, "");
const dryRun = process.env["INDEXNOW_DRY_RUN"] === "1";
const sitemapPath =
  process.env["INDEXNOW_SITEMAP"] || path.join(__dirname, "..", "build", "client", "sitemap.xml");

const host = new URL(origin).host;
const keyLocation = `${origin}/${key}.txt`;

function readSitemapUrls(file) {
  let xml;
  try {
    xml = readFileSync(file, "utf8");
  } catch {
    console.error(`IndexNow: sitemap not found at ${file}. Build the site first.`);
    process.exit(1);
  }
  const urls = [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) => m[1].trim());
  // Only submit URLs on this host (IndexNow rejects a batch whose URLs are not
  // all under the declared host with a 422).
  const onHost = urls.filter((u) => {
    try {
      return new URL(u).host === host;
    } catch {
      return false;
    }
  });
  return [...new Set(onHost)];
}

async function submit(urlList) {
  // IndexNow shares a submission across all participating engines, so one POST
  // to the shared endpoint reaches Bing, Yandex, and the rest.
  const endpoint = "https://api.indexnow.org/indexnow";
  const body = JSON.stringify({ host, key, keyLocation, urlList });
  const res = await fetch(endpoint, {
    method: "POST",
    headers: { "Content-Type": "application/json; charset=utf-8" },
    body,
  });
  return res;
}

const urls = readSitemapUrls(sitemapPath);
if (urls.length === 0) {
  console.error("IndexNow: no URLs found in the sitemap; nothing to submit.");
  process.exit(1);
}

// IndexNow allows up to 10,000 URLs per POST; this site is ~150, so one batch
// is enough. Chunk anyway so the script stays correct if the site grows.
const CHUNK = 10000;
const chunks = [];
for (let i = 0; i < urls.length; i += CHUNK) chunks.push(urls.slice(i, i + CHUNK));

console.log(
  `IndexNow: ${urls.length} URL(s) for ${host}, key ${key.slice(0, 8)}…, keyLocation ${keyLocation}`,
);

if (dryRun) {
  console.log("IndexNow: DRY RUN, not submitting. First few URLs:");
  for (const u of urls.slice(0, 5)) console.log("  " + u);
  process.exit(0);
}

let ok = true;
for (const [i, chunk] of chunks.entries()) {
  try {
    const res = await submit(chunk);
    const text = await res.text().catch(() => "");
    // 200 OK and 202 Accepted are both success (202 = key validation pending).
    if (res.status === 200 || res.status === 202) {
      console.log(`IndexNow: batch ${i + 1}/${chunks.length} (${chunk.length} URLs) -> ${res.status}`);
    } else {
      ok = false;
      console.error(
        `IndexNow: batch ${i + 1}/${chunks.length} -> ${res.status} ${res.statusText}. ${text}`.trim(),
      );
    }
  } catch (err) {
    ok = false;
    console.error(`IndexNow: batch ${i + 1}/${chunks.length} failed: ${err?.message ?? err}`);
  }
}

// A submission failure should not fail the deploy: the site is already live and
// this is a best-effort ping. Exit 0 either way, but log clearly. Set
// INDEXNOW_STRICT=1 to make a failure non-zero (useful when testing).
if (!ok && process.env["INDEXNOW_STRICT"] === "1") process.exit(1);
process.exit(0);
