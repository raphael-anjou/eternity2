// Tiny i18n: the active language is derived from the URL (English at the root,
// French under /fr), so every page has its own crawlable, shareable URL. A
// context exposes it plus colocated per-page dictionaries.
//
// Pattern, in any component (unchanged):
//   const t = useT({ en: { title: "Hello" }, fr: { title: "Bonjour" } });
//   <h1>{t.title}</h1>
// Dictionaries may contain strings or JSX fragments.

import { createContext, useContext } from "react";
import type { ReactNode } from "react";
import { useLocation, useNavigate } from "react-router";

export type Lang = "en" | "fr";

export const STORAGE_KEY = "e2-lang";

/** Language encoded in a pathname: "/fr/..." → "fr", everything else → "en". */
export function langFromPath(pathname: string): Lang {
  return /^\/fr(\/|$)/.test(pathname) ? "fr" : "en";
}

/** Rewrite a pathname to the given language, preserving the page + query. */
export function pathForLang(pathname: string, lang: Lang): string {
  const bare = pathname.replace(/^\/fr(?=\/|$)/, "") || "/";
  if (lang === "en") return bare;
  return bare === "/" ? "/fr" : "/fr" + bare;
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

/** Pick the active language's dictionary. */
export function useT<T>(dict: { en: T; fr: T }): T {
  const { lang } = useContext(LangContext);
  return dict[lang];
}

/**
 * First-visit preference for redirecting a bare "/" path: an earlier explicit
 * choice, else the browser's ordered languages, else English. Browser-only;
 * returns "en" during prerender (no navigator).
 */
export function preferredLang(): Lang {
  if (typeof window === "undefined") return "en";
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === "fr" || stored === "en") return stored;
  const prefs = navigator.languages ?? [navigator.language];
  for (const p of prefs) {
    const base = p.toLowerCase().split("-")[0];
    if (base === "fr") return "fr";
    if (base === "en") return "en";
  }
  return "en";
}
