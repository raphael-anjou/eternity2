import { describe, it, expect } from "vitest";
import { canonicalizeExportLinks, rewriteLinkTarget } from "./export-links";

// The English export: no language prefix, so hasTranslation is never consulted.
const en = { langPrefix: "", hasTranslation: () => false };

// A French export where only two pages have a real French twin.
const FR_TWINS = new Set(["/research/why", "/research/build/known-facts"]);
const fr = { langPrefix: "fr", hasTranslation: (p: string) => FR_TWINS.has(p) };

describe("rewriteLinkTarget", () => {
  it("adds a trailing slash to a slashless internal path", () => {
    expect(rewriteLinkTarget("/research/why", en)).toBe("/research/why/");
  });

  it("leaves an already-slashed path alone", () => {
    expect(rewriteLinkTarget("/research/why/", en)).toBe("/research/why/");
  });

  it("never adds a slash after a file extension", () => {
    expect(rewriteLinkTarget("/research/why.md", en)).toBe("/research/why.md");
    expect(rewriteLinkTarget("/research/data.json", en)).toBe("/research/data.json");
  });

  it("puts the slash before an anchor", () => {
    expect(rewriteLinkTarget("/research/why#hard", en)).toBe("/research/why/#hard");
  });

  it("puts the slash before a query string", () => {
    expect(rewriteLinkTarget("/viewer?puzzle=x&size=16", en)).toBe("/viewer/?puzzle=x&size=16");
  });

  it("handles a non-ASCII anchor", () => {
    expect(rewriteLinkTarget("/research/why#conçu-pour-résister", en)).toBe(
      "/research/why/#conçu-pour-résister",
    );
  });

  it("covers the other top-level app roots", () => {
    for (const p of [
      "/puzzle",
      "/status",
      "/is-it-a-scam",
      "/playground",
      "/algorithms",
      "/viewer",
      "/convert",
      "/start",
    ]) {
      expect(rewriteLinkTarget(p, en)).toBe(`${p}/`);
    }
  });

  it("leaves external URLs untouched", () => {
    for (const u of [
      "https://e2.bucas.name/x",
      "http://example.com/research/why",
      "mailto:a@b.dev",
      "//cdn.example.com/research/why",
      "#section",
      "./sibling.md",
      "../up",
    ]) {
      expect(rewriteLinkTarget(u, en)).toBe(u);
    }
  });

  it("leaves site-absolute paths that are not app roots untouched", () => {
    expect(rewriteLinkTarget("/robots.txt", en)).toBe("/robots.txt");
    expect(rewriteLinkTarget("/some-other-thing", en)).toBe("/some-other-thing");
  });

  it("prefixes the language when a twin exists", () => {
    expect(rewriteLinkTarget("/research/why", fr)).toBe("/fr/research/why/");
    expect(rewriteLinkTarget("/research/build/known-facts#x", fr)).toBe(
      "/fr/research/build/known-facts/#x",
    );
  });

  it("keeps the English URL when no twin exists", () => {
    expect(rewriteLinkTarget("/research/lab/untranslated", fr)).toBe(
      "/research/lab/untranslated/",
    );
  });

  it("consults the twin predicate with the trailing slash stripped", () => {
    const seen: string[] = [];
    rewriteLinkTarget("/research/why/", {
      langPrefix: "fr",
      hasTranslation: (p) => {
        seen.push(p);
        return false;
      },
    });
    expect(seen).toEqual(["/research/why"]);
  });

  it("does not double-prefix an already-localized path", () => {
    expect(rewriteLinkTarget("/fr/research/why", fr)).toBe("/fr/research/why/");
    // An authored /fr link inside the ES export retargets to ES, not /es/fr/.
    const es = { langPrefix: "es", hasTranslation: () => true };
    expect(rewriteLinkTarget("/fr/research/why", es)).toBe("/es/research/why/");
    // And it degrades to the neutral English URL when ES has no twin.
    const esNone = { langPrefix: "es", hasTranslation: () => false };
    expect(rewriteLinkTarget("/fr/research/why", esNone)).toBe("/research/why/");
  });

  it("is idempotent", () => {
    const once = rewriteLinkTarget("/research/why#a", fr);
    expect(rewriteLinkTarget(once, fr)).toBe(once);
  });
});

describe("canonicalizeExportLinks: prose", () => {
  it("rewrites a plain markdown link", () => {
    expect(canonicalizeExportLinks("See [why](/research/why) for more.", en)).toBe(
      "See [why](/research/why/) for more.",
    );
  });

  it("rewrites several links on one line", () => {
    expect(
      canonicalizeExportLinks("[a](/research/a) and [b](/research/b#z) and [x](https://x.dev)", en),
    ).toBe("[a](/research/a/) and [b](/research/b/#z) and [x](https://x.dev)");
  });

  it("rewrites a link carrying a title", () => {
    expect(canonicalizeExportLinks('[a](/research/a "Why it is hard")', en)).toBe(
      '[a](/research/a/ "Why it is hard")',
    );
  });

  it("rewrites a pointy-bracket destination", () => {
    expect(canonicalizeExportLinks("[a](</research/a>)", en)).toBe("[a](</research/a/>)");
  });

  it("rewrites an image link", () => {
    expect(canonicalizeExportLinks("![alt](/research/a)", en)).toBe("![alt](/research/a/)");
  });

  it("rewrites href and to attributes on surviving JSX islands", () => {
    expect(canonicalizeExportLinks('<Door to="/research/why" title="Why" />', en)).toBe(
      '<Door to="/research/why/" title="Why" />',
    );
    expect(canonicalizeExportLinks('<a href="/viewer?x=1">v</a>', en)).toBe(
      '<a href="/viewer/?x=1">v</a>',
    );
  });

  it("rewrites a reference definition", () => {
    expect(canonicalizeExportLinks("[why]: /research/why", en)).toBe("[why]: /research/why/");
  });

  it("applies the language prefix through the full pipeline", () => {
    expect(
      canonicalizeExportLinks("Voir [pourquoi](/research/why) et [autre](/research/other).", fr),
    ).toBe("Voir [pourquoi](/fr/research/why/) et [autre](/research/other/).");
  });
});

describe("canonicalizeExportLinks: code regions are untouched", () => {
  it("leaves a fenced code block byte-identical", () => {
    const body = [
      "Prose [a](/research/a).",
      "",
      "```js",
      'fetch("/research/a");',
      'const u = "/viewer?x=1";',
      "```",
      "",
      "More [b](/research/b).",
    ].join("\n");
    const out = canonicalizeExportLinks(body, en);
    expect(out).toContain('fetch("/research/a");');
    expect(out).toContain('const u = "/viewer?x=1";');
    expect(out).toContain("[a](/research/a/)");
    expect(out).toContain("[b](/research/b/)");
  });

  it("leaves a tilde fence byte-identical", () => {
    const body = ["~~~", "[a](/research/a)", "~~~"].join("\n");
    expect(canonicalizeExportLinks(body, en)).toBe(body);
  });

  it("does not close a backtick fence on a tilde fence, or vice versa", () => {
    const body = ["```", "~~~", "[a](/research/a)", "```"].join("\n");
    expect(canonicalizeExportLinks(body, en)).toBe(body);
  });

  it("treats a longer closing fence as closing, a shorter one as content", () => {
    const body = ["````", "```", "[a](/research/a)", "````", "", "[b](/research/b)"].join("\n");
    const out = canonicalizeExportLinks(body, en);
    expect(out).toContain("[a](/research/a)\n");
    expect(out).toContain("[b](/research/b/)");
  });

  it("leaves an indented code block untouched", () => {
    const body = ["Prose:", "", "    curl /research/a", "", "Then [b](/research/b)."].join("\n");
    const out = canonicalizeExportLinks(body, en);
    expect(out).toContain("    curl /research/a");
    expect(out).toContain("[b](/research/b/)");
  });

  it("leaves a tab-indented code block untouched", () => {
    const body = ["Prose:", "", "\tcurl /research/a", ""].join("\n");
    expect(canonicalizeExportLinks(body, en)).toBe(body);
  });

  it("still rewrites a deeply nested list item, which is also indented", () => {
    const body = ["- top", "  - mid", "    - deep [a](/research/a)"].join("\n");
    expect(canonicalizeExportLinks(body, en)).toContain("deep [a](/research/a/)");
  });

  it("leaves an inline code span untouched", () => {
    const body = "Build the URL as `/viewer?${b.params}` then open [it](/viewer).";
    expect(canonicalizeExportLinks(body, en)).toBe(
      "Build the URL as `/viewer?${b.params}` then open [it](/viewer/).",
    );
  });

  it("leaves a link INSIDE an inline code span untouched", () => {
    const body = "Write `[a](/research/a)` verbatim, then [a](/research/a).";
    expect(canonicalizeExportLinks(body, en)).toBe(
      "Write `[a](/research/a)` verbatim, then [a](/research/a/).",
    );
  });

  it("handles a double-backtick span containing a backtick", () => {
    const body = "``code /research/a ` here`` and [b](/research/b)";
    expect(canonicalizeExportLinks(body, en)).toBe(
      "``code /research/a ` here`` and [b](/research/b/)",
    );
  });

  it("treats an unclosed backtick run as prose, not as an open span", () => {
    const body = "A stray ` tick and [b](/research/b).";
    expect(canonicalizeExportLinks(body, en)).toBe("A stray ` tick and [b](/research/b/).");
  });

  it("rewrites nothing in a body that is entirely a fence", () => {
    const body = ["```bash", "curl https://eternity2.dev/research/why", "```"].join("\n");
    expect(canonicalizeExportLinks(body, en)).toBe(body);
  });

  it("preserves the exact line count and trailing newline", () => {
    const body = "a\n\n```\n/research/x\n```\n\nb [c](/research/c)\n";
    const out = canonicalizeExportLinks(body, en);
    expect(out.split("\n").length).toBe(body.split("\n").length);
    expect(out.endsWith("\n")).toBe(true);
  });

  it("is idempotent over a mixed body", () => {
    const body = [
      "[a](/research/a) and `/viewer?x`",
      "",
      "```",
      "/research/b",
      "```",
      "",
      "[c](/research/c#s) [d](/research/d.md)",
    ].join("\n");
    const once = canonicalizeExportLinks(body, fr);
    expect(canonicalizeExportLinks(once, fr)).toBe(once);
  });
});
