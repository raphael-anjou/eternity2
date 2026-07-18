// Tiny i18n: the active language is derived from the URL (English at the root,
// French under /fr, Spanish under /es), so every page has its own crawlable,
// shareable URL. A context exposes it plus colocated per-page dictionaries.
//
// Pattern, in any component:
//   const t = useT({ en: { title: "Hello" }, fr: { title: "Bonjour" }, es: { title: "Hola" } });
//   <h1>{t.title}</h1>
// Dictionaries may contain strings or JSX fragments. Only `en` is required — a
// dictionary that omits a language falls back to English for that language, so a
// new language can be added incrementally (the UI reads English until each
// dictionary gains its branch) without a flag-day edit across every component.

import { createContext, useContext } from "react";
import type { ReactNode } from "react";
import { useLocation, useNavigate } from "react-router";

// ---- Language registry ---------------------------------------------------
//
// The single source of truth for what languages the site speaks. Adding a
// language is ideally a one-line change here plus its content: everything that
// iterates languages (sitemap, hreflang, the picker, the manifest builder)
// derives from LANGS, and every dictionary that hasn't been translated yet
// falls back to English. English MUST stay first — it is the canonical tree
// (served at the URL root) and the fallback every other language resolves to.

/** A language definition. `label` is what the picker button shows; `hint` is
 *  the picker's title/tooltip in that language; `path` is the URL prefix (empty
 *  for English at the root). */
export interface LangDef {
  code: Lang;
  /** URL prefix without a leading slash's trailing content, e.g. "fr" → "/fr".
   *  Empty string for English, which lives at the root. */
  prefix: string;
  /** Short code shown on the picker button (e.g. "EN"). */
  label: string;
  /** Native language name, for the picker menu + the switch tooltip. */
  native: string;
}

export type Lang = "en" | "fr" | "es";

/** English — the canonical language, at the URL root, and the fallback every
 *  other language resolves to. Named so it can be the default without a
 *  non-null assertion on LANGS[0]. */
const EN: LangDef = { code: "en", prefix: "", label: "EN", native: "English" };

export const LANGS: readonly LangDef[] = [
  EN,
  { code: "fr", prefix: "fr", label: "FR", native: "Français" },
  { code: "es", prefix: "es", label: "ES", native: "Español" },
] as const;

/** All language codes, English first. */
export const LANG_CODES: readonly Lang[] = LANGS.map((l) => l.code);

/** The non-English languages, in registry order — the ones that carry a URL
 *  prefix and a translation tree. */
export const PREFIXED_LANGS: readonly LangDef[] = LANGS.filter((l) => l.prefix !== "");

const BY_CODE = new Map<Lang, LangDef>(LANGS.map((l) => [l.code, l]));

/** The definition for a language code (English if somehow unknown). */
export function langDef(code: Lang): LangDef {
  return BY_CODE.get(code) ?? EN;
}

// A regexp fragment matching any non-English prefix: "fr|es". Built from the
// registry so a new language is covered automatically.
const PREFIX_ALT = PREFIXED_LANGS.map((l) => l.prefix).join("|");
const LEADING_PREFIX_RE = new RegExp(`^/(${PREFIX_ALT})(?=/|$)`);

export const STORAGE_KEY = "e2-lang";

/** Language encoded in a pathname: "/fr/..." → "fr", "/es/..." → "es",
 *  everything else → "en". */
export function langFromPath(pathname: string): Lang {
  const m = LEADING_PREFIX_RE.exec(pathname);
  if (!m) return "en";
  const found = PREFIXED_LANGS.find((l) => l.prefix === m[1]);
  return found ? found.code : "en";
}

/** Strip any known language prefix from a pathname, yielding the language-neutral
 *  (English) path. "/es/puzzle" → "/puzzle", "/fr" → "/". */
export function neutralPath(pathname: string): string {
  return pathname.replace(LEADING_PREFIX_RE, "") || "/";
}

/** Rewrite a pathname to the given language, preserving the page + query. */
export function pathForLang(pathname: string, lang: Lang): string {
  const bare = neutralPath(pathname);
  const def = langDef(lang);
  if (def.prefix === "") return bare;
  return bare === "/" ? `/${def.prefix}` : `/${def.prefix}${bare}`;
}

const LangContext = createContext<{ lang: Lang }>({ lang: "en" });

export function LangProvider({ children }: { children: ReactNode }) {
  const { pathname } = useLocation();
  const lang = langFromPath(pathname);
  return <LangContext.Provider value={{ lang }}>{children}</LangContext.Provider>;
}

/**
 * Active language plus a setter that navigates to the localized URL (the URL
 * is the source of truth) and remembers the choice for the next first visit.
 */
export function useLang(): { lang: Lang; setLang: (l: Lang) => void } {
  const { lang } = useContext(LangContext);
  const { pathname, search } = useLocation();
  const navigate = useNavigate();
  const setLang = (l: Lang) => {
    if (typeof window !== "undefined") localStorage.setItem(STORAGE_KEY, l);
    void navigate(pathForLang(pathname, l) + search);
  };
  return { lang, setLang };
}

// A dictionary supplies English (required) and any subset of the other
// languages. A missing language reads English — so a partially-translated
// component still renders, in English, for that language.
export type Dict<T> = { en: T } & Partial<Record<Lang, T>>;

/** Pick the active language's dictionary, falling back to English when the
 *  active language has no entry (incremental translation). */
export function useT<T>(dict: Dict<T>): T {
  const { lang } = useContext(LangContext);
  return dict[lang] ?? dict.en;
}

/** The non-hook form of useT: resolve a dictionary to a language with the same
 *  English fallback, when the active language is already known (a route `meta`
 *  function, a lookup-table cell, a `.map` callback — anywhere outside a
 *  component render where useContext isn't available). English is required on
 *  the dictionary and is the fallback for any untranslated language. */
export function pick<T>(dict: Dict<T>, lang: Lang): T {
  return dict[lang] ?? dict.en;
}

/**
 * First-visit preference for redirecting a bare "/" path: an earlier explicit
 * choice, else the browser's ordered languages, else English. Browser-only;
 * returns "en" during prerender (no navigator).
 */
export function preferredLang(): Lang {
  if (typeof window === "undefined") return "en";
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored && (LANG_CODES as readonly string[]).includes(stored)) return stored as Lang;
  const prefs = navigator.languages ?? [navigator.language];
  for (const p of prefs) {
    const base = p.toLowerCase().split("-")[0];
    const match = LANG_CODES.find((c) => c === base);
    if (match) return match;
  }
  return "en";
}
