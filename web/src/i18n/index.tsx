// Tiny i18n: a language context plus colocated per-page dictionaries.
// English is the default; the choice persists in localStorage.
//
// Pattern, in any component:
//   const t = useT({ en: { title: "Hello" }, fr: { title: "Bonjour" } });
//   <h1>{t.title}</h1>
// Dictionaries may contain strings or JSX fragments.

import { createContext, useContext, useEffect, useState } from "react";
import type { ReactNode } from "react";

export type Lang = "en" | "fr";

const STORAGE_KEY = "e2-lang";

const LangContext = createContext<{ lang: Lang; setLang: (l: Lang) => void }>({
  lang: "en",
  setLang: () => {},
});

export function LangProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState<Lang>(() => {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored === "fr" ? "fr" : "en";
  });
  const setLang = (l: Lang) => {
    localStorage.setItem(STORAGE_KEY, l);
    setLangState(l);
  };
  useEffect(() => {
    document.documentElement.lang = lang;
  }, [lang]);
  return <LangContext.Provider value={{ lang, setLang }}>{children}</LangContext.Provider>;
}

export function useLang(): { lang: Lang; setLang: (l: Lang) => void } {
  return useContext(LangContext);
}

/** Pick the active language's dictionary. */
export function useT<T>(dict: { en: T; fr: T }): T {
  const { lang } = useContext(LangContext);
  return dict[lang];
}
