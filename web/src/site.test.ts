import { describe, it, expect } from "vitest";
import { canonicalPath } from "./site";

// canonicalPath decides the exact URL form every internal link and canonical
// tag emits. GitHub Pages serves an extensionless path only at its trailing-
// slash variant and 301-redirects the bare form, so the slash must land on the
// path portion — never inside a ?query or #hash — and never on the root or a
// file with an extension.
describe("canonicalPath", () => {
  it("adds a trailing slash to an extensionless page path", () => {
    expect(canonicalPath("/algorithms")).toBe("/algorithms/");
    expect(canonicalPath("/research/build/known-facts")).toBe("/research/build/known-facts/");
    expect(canonicalPath("/fr/puzzle")).toBe("/fr/puzzle/");
  });

  it("leaves the root and already-slashed paths untouched", () => {
    expect(canonicalPath("/")).toBe("/");
    expect(canonicalPath("/research/")).toBe("/research/");
    expect(canonicalPath("/fr")).toBe("/fr/");
  });

  it("never slashes a path that ends in a file extension", () => {
    expect(canonicalPath("/sitemap.xml")).toBe("/sitemap.xml");
    expect(canonicalPath("/llms-full.txt")).toBe("/llms-full.txt");
    expect(canonicalPath("/research/build/known-facts.md")).toBe("/research/build/known-facts.md");
  });

  it("slashes the path portion but preserves a query string", () => {
    expect(canonicalPath("/viewer?board=abc")).toBe("/viewer/?board=abc");
    expect(canonicalPath("/fr/viewer?board=abc&x=1")).toBe("/fr/viewer/?board=abc&x=1");
    // An already-slashed path with a query is left as-is.
    expect(canonicalPath("/viewer/?board=abc")).toBe("/viewer/?board=abc");
  });

  it("slashes the path portion but preserves a hash", () => {
    expect(canonicalPath("/research/glossary#clue")).toBe("/research/glossary/#clue");
    expect(canonicalPath("/status#faq")).toBe("/status/#faq");
  });
});
