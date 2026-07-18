// Which URLs have a genuine translated rendering — used by the root shell to
// decide whether to advertise hreflang="<lang>" for a page. Kept in its own
// module (rather than manifest.ts) so it pulls in only the tiny per-language
// `translated` lists, not the full ~400KB research manifest, on every page.

import { translated as translatedFr } from "virtual:research-translated-fr";
import { translated as translatedEs } from "virtual:research-translated-es";
import type { Lang } from "@/i18n";

// One set per non-English language. Adding a language means importing its
// `virtual:research-translated-<lang>` module and adding it here.
const TRANSLATED: Partial<Record<Lang, Set<string>>> = {
  fr: new Set(translatedFr),
  es: new Set(translatedEs),
};

/** True if the given language-neutral path has a real page in `lang`.
 *
 *  English is the canonical tree, so it always returns true. Non-research pages
 *  mirror under every language prefix, so they return true too. Research pages
 *  return true only when a `<slug>.<lang>.mdx` sidecar exists (or the page is a
 *  registry-backed hub — topics/people/glossary — which is always reachable in
 *  every language); an untranslated research page returns false, so the English
 *  page does not advertise a translated alternate that was never prerendered. */
export function hasTwin(neutralPath: string, lang: Lang): boolean {
  if (lang === "en") return true;
  if (!/^\/research(\/|$)/.test(neutralPath)) return true;
  const set = TRANSLATED[lang];
  if (!set) return false;
  return set.has(neutralPath === "/research/" ? "/research" : neutralPath);
}
