// Canonicalize the internal links written inside MDX PROSE for the machine-
// readable markdown exports (the `.md` siblings and llms-full.txt).
//
// The rendered HTML pages already do this at render time: <LocalizedLink> runs
// every in-content path through pathForLang() then canonicalPath(). The markdown
// export has no such layer, it emits the authored body verbatim, so a prose link
// authored as `](/research/x)` ships two defects:
//
//   1. No trailing slash. The host canonicalizes to the slash form, so a crawler
//      following the link pays a 301 hop on every single one.
//   2. No language. Inside a French or Spanish export that link points into the
//      ENGLISH tree, dropping the reader out of their language mid-document.
//
// Both are fixed here, on the export only. The MDX sources are never touched.
//
// WHY THIS IS NOT A REGEX OVER THE WHOLE BODY
// -------------------------------------------
// Site paths legitimately appear inside code, where they are literals and must
// survive byte-identical: fenced blocks (```), indented blocks (4 spaces or a
// tab), and inline spans (`/viewer?${b.params}` is real, authored content). A
// blanket regex corrupts all three. So the body is first split into code and
// prose regions by a small block-then-inline scanner, and only the prose regions
// are rewritten.

/** The site's top-level application paths. A prose link is rewritten only when
 *  its first segment is one of these, so a path that merely looks site-internal
 *  (a filesystem path, an API route in prose) is left alone. */
const APP_ROOTS: readonly string[] = [
  "research",
  "puzzle",
  "status",
  "is-it-a-scam",
  "playground",
  "algorithms",
  "viewer",
  "convert",
  "start",
];

/** Language prefixes the site mirrors itself under. Kept local (rather than
 *  imported from the i18n registry) because this module runs in the Node build
 *  and must stay dependency-free, and because it only needs to RECOGNIZE an
 *  already-prefixed path so it is not prefixed twice. */
const LANG_PREFIX_RE = /^\/(fr|es)(\/|$)/;

/** Split a path into its pathname and its `?query` / `#anchor` suffix, so the
 *  trailing slash can be inserted before the suffix rather than after it. */
function splitSuffix(target: string): { pathname: string; suffix: string } {
  const cut = target.search(/[?#]/);
  if (cut < 0) return { pathname: target, suffix: "" };
  return { pathname: target.slice(0, cut), suffix: target.slice(cut) };
}

/** True when the pathname ends in a file extension (".md", ".txt", ".svg", …).
 *  Those are files the host serves directly, never directories, so they must
 *  NOT gain a trailing slash. Mirrors canonicalPath() in src/site.ts. */
function hasExtension(pathname: string): boolean {
  return /\.[a-z0-9]+$/i.test(pathname);
}

/** The first path segment of a site-absolute path ("/research/a/b" → "research"). */
function firstSegment(pathname: string): string {
  return pathname.split("/")[1] ?? "";
}

/** Options for rewriting one export's worth of prose links. */
export interface RewriteOptions {
  /** Language prefix to apply, "" for English, "fr" / "es" otherwise. */
  readonly langPrefix: string;
  /** Predicate: does the language-neutral path have a real page in this
   *  language? Only consulted for a non-empty langPrefix. A path that has no
   *  twin keeps its English URL, which is a valid page, rather than gaining a
   *  prefix that would 404. */
  readonly hasTranslation: (neutralPath: string) => boolean;
}

/** Rewrite one link target. Returns the target unchanged when it is not a
 *  site-internal app path (external URL, mailto:, protocol-relative, bare
 *  fragment, relative path) or when it already carries its canonical form. */
export function rewriteLinkTarget(target: string, opts: RewriteOptions): string {
  // Only site-absolute paths. "//host/x" is protocol-relative (external), and
  // anything not starting with "/" is external, relative, or a bare fragment.
  if (!target.startsWith("/") || target.startsWith("//")) return target;

  const { pathname, suffix } = splitSuffix(target);
  // A language-prefixed path is already localized; strip the prefix to get the
  // neutral path, then re-apply the EXPORT's language below. This keeps an
  // authored "/fr/research/x" correct in the FR export and, more importantly,
  // stops "/fr" being prefixed a second time.
  const langPrefix = LANG_PREFIX_RE.exec(pathname)?.[1] ?? "";
  const neutral = langPrefix ? pathname.slice(langPrefix.length + 1) || "/" : pathname;

  if (!APP_ROOTS.includes(firstSegment(neutral))) return target;

  // Language: prefix only when this export is non-English AND the target has a
  // genuine page in that language. Otherwise the English URL is kept, which is
  // a real 200 rather than a translated 404.
  const localized =
    opts.langPrefix && opts.hasTranslation(stripTrailingSlash(neutral))
      ? `/${opts.langPrefix}${neutral}`
      : neutral;

  // Trailing slash, unless the path is a file or already ends with one.
  const slashed = localized.endsWith("/") || hasExtension(localized) ? localized : localized + "/";
  return slashed + suffix;
}

/** "/research/x/" → "/research/x", "/research" → "/research". The translation
 *  predicate is keyed on manifest URLs, which carry no trailing slash. */
function stripTrailingSlash(pathname: string): string {
  return pathname.length > 1 ? pathname.replace(/\/+$/, "") : pathname;
}

// ---- Markdown region scanning --------------------------------------------
//
// Two levels, in order. First BLOCK level: a line-by-line walk that classifies
// each line as fenced code, indented code, or prose. Then INLINE level, on the
// prose lines only: backtick spans are masked out.

/** True when the line opens or closes a fence. Returns the fence marker (the run
 *  of backticks or tildes) or null. Only a run of 3+ counts, per CommonMark. */
function fenceMarker(line: string): string | null {
  const m = /^ {0,3}(`{3,}|~{3,})/.exec(line);
  return m ? (m[1] ?? null) : null;
}

/** Apply `fn` to every PROSE line of a markdown body, leaving fenced and
 *  indented code blocks byte-identical.
 *
 *  Indented-code detection follows CommonMark closely enough for this corpus: a
 *  line indented 4+ spaces (or a tab) is code only when it cannot be a lazy
 *  continuation of a paragraph, i.e. the previous line was blank or itself
 *  indented code. That guard matters because deeply nested list items are also
 *  indented 4+ spaces and are very much prose. */
function mapProseLines(body: string, fn: (line: string) => string): string {
  const lines = body.split("\n");
  const out: string[] = [];
  let openFence: string | null = null;
  let prevBlank = true;
  let inIndentedCode = false;

  for (const line of lines) {
    if (openFence !== null) {
      // Inside a fence: a closing fence is the same character, at least as long.
      const marker = fenceMarker(line);
      if (marker && marker[0] === openFence[0] && marker.length >= openFence.length) {
        openFence = null;
      }
      out.push(line);
      prevBlank = false;
      continue;
    }

    const marker = fenceMarker(line);
    if (marker) {
      openFence = marker;
      out.push(line);
      prevBlank = false;
      inIndentedCode = false;
      continue;
    }

    const blank = line.trim() === "";
    if (blank) {
      // A blank line neither opens nor closes indented code on its own; it keeps
      // an open indented block open and permits a new one to start after it.
      out.push(line);
      prevBlank = true;
      continue;
    }

    const indented = /^(?: {4}|\t)/.test(line);
    if (indented && (inIndentedCode || prevBlank)) {
      inIndentedCode = true;
      out.push(line);
      prevBlank = false;
      continue;
    }

    inIndentedCode = false;
    out.push(fn(line));
    prevBlank = false;
  }

  return out.join("\n");
}

/** Apply `fn` to the parts of a prose line that are NOT inside a backtick code
 *  span. A span opens on a run of N backticks and closes on the next run of
 *  exactly N; an unclosed run is literal text, so it stays prose. */
function mapOutsideInlineCode(line: string, fn: (chunk: string) => string): string {
  const runs = [...line.matchAll(/`+/g)];
  if (runs.length === 0) return fn(line);

  let out = "";
  let cursor = 0;
  // Index of the run that opened the span currently being scanned for a closer,
  // or -1 while outside a span.
  let openIdx = -1;
  for (let i = 0; i < runs.length; i++) {
    const run = runs[i];
    if (run === undefined) continue;
    if (openIdx < 0) {
      openIdx = i;
      continue;
    }
    const open = runs[openIdx];
    // A closer is a run of exactly the opener's length; anything else is span
    // content and is skipped.
    if (open === undefined || run[0].length !== open[0].length) continue;
    const end = run.index + run[0].length;
    out += fn(line.slice(cursor, open.index));
    out += line.slice(open.index, end);
    cursor = end;
    openIdx = -1;
  }
  // An unclosed opener is literal text, so the tail (including it) is prose.
  out += fn(line.slice(cursor));
  return out;
}

// Link shapes rewritten in prose. Two authored forms survive stripMdxEsm into
// the export:
//   - markdown inline links / images:  [text](/research/x)  or  [text](/x "title")
//   - JSX island attributes:           href="/research/x"  to="/research/x"
// Reference definitions ([id]: /path) are matched too, for completeness; the
// corpus has none today but the shape is cheap to cover.
// The destination is everything up to the first whitespace or the closing ")",
// which covers both the plain `[t](/x)` and the titled `[t](/x "Title")` forms,
// and the pointy-bracket form `[t](</x>)`.
const MD_LINK_RE = /(\]\(\s*)(<?)([^)<>\s]+)(>?)/g;
const ATTR_LINK_RE = /((?:href|to)=)(["'])([^"']+)\2/g;
const REF_DEF_RE = /^(\s{0,3}\[[^\]]+\]:\s+)(\S+)/;

/** Rewrite every internal link target in one prose chunk. */
function rewriteChunk(chunk: string, opts: RewriteOptions): string {
  return chunk
    .replace(MD_LINK_RE, (_m, open: string, lt: string, target: string, gt: string) =>
      `${open}${lt}${rewriteLinkTarget(target, opts)}${gt}`,
    )
    .replace(ATTR_LINK_RE, (_m, head: string, quote: string, target: string) =>
      `${head}${quote}${rewriteLinkTarget(target, opts)}${quote}`,
    )
    .replace(REF_DEF_RE, (_m, head: string, target: string) =>
      `${head}${rewriteLinkTarget(target, opts)}`,
    );
}

/** Canonicalize the internal links inside a markdown body, skipping every code
 *  region (fenced, indented, inline). This is the entry point the export
 *  pipeline calls; everything above it is machinery.
 *
 *  Idempotent: running it twice is the same as running it once, because an
 *  already-canonical target rewrites to itself. */
export function canonicalizeExportLinks(body: string, opts: RewriteOptions): string {
  return mapProseLines(body, (line) =>
    mapOutsideInlineCode(line, (chunk) => rewriteChunk(chunk, opts)),
  );
}
