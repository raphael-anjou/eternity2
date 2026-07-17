import { describe, it, expect } from "vitest";
import { langFromPath, pathForLang } from "./index";

// These two pure functions are the whole i18n routing contract: the active
// language is derived from the URL, and switching language rewrites the URL.
// A regression here silently breaks every bilingual page and the FR/EN toggle.

describe("langFromPath", () => {
  it("treats the /fr prefix (and exactly /fr) as French", () => {
    expect(langFromPath("/fr")).toBe("fr");
    expect(langFromPath("/fr/")).toBe("fr");
    expect(langFromPath("/fr/research/records")).toBe("fr");
  });
  it("treats the root and every other path as English", () => {
    expect(langFromPath("/")).toBe("en");
    expect(langFromPath("/research/records")).toBe("en");
    expect(langFromPath("/puzzle")).toBe("en");
  });
  it("does not mistake a path that merely starts with 'fr' for French", () => {
    // "/french" or "/frames" must stay English — the boundary is /fr then / or end.
    expect(langFromPath("/french")).toBe("en");
    expect(langFromPath("/frames")).toBe("en");
  });
});

describe("pathForLang", () => {
  it("adds the /fr prefix when switching to French", () => {
    expect(pathForLang("/", "fr")).toBe("/fr");
    expect(pathForLang("/puzzle", "fr")).toBe("/fr/puzzle");
    expect(pathForLang("/research/records", "fr")).toBe("/fr/research/records");
  });
  it("strips the /fr prefix when switching to English", () => {
    expect(pathForLang("/fr", "en")).toBe("/");
    expect(pathForLang("/fr/puzzle", "en")).toBe("/puzzle");
    expect(pathForLang("/fr/research/records", "en")).toBe("/research/records");
  });
  it("is idempotent when the path is already in the target language", () => {
    expect(pathForLang("/puzzle", "en")).toBe("/puzzle");
    expect(pathForLang("/fr/puzzle", "fr")).toBe("/fr/puzzle");
  });
  it("round-trips en → fr → en", () => {
    const original = "/research/lab/experiments";
    const fr = pathForLang(original, "fr");
    expect(pathForLang(fr, "en")).toBe(original);
  });
  it("does not strip a leading 'fr' that is part of a real segment", () => {
    // "/frames" must not lose its "fr" when normalized to English.
    expect(pathForLang("/frames", "en")).toBe("/frames");
  });
});
