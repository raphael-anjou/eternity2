// Which URLs have a genuine French rendering — used by the root shell to decide
// whether to advertise hreflang="fr" for a page. Kept in its own module (rather
// than manifest.ts) so it pulls in only the tiny `translatedFr` list, not the
// full ~400KB research manifest, on every page.

import { translatedFr } from "virtual:research-translated-fr";

const TRANSLATED_FR = new Set(translatedFr);

/** True if the given language-neutral path has a real French page.
 *
 *  Non-research pages always mirror under /fr, so they return true. Research
 *  pages return true only when a `<slug>.fr.mdx` sidecar exists (or the page is
 *  a registry-backed hub — topics/people/glossary — which is always bilingual);
 *  an untranslated research page returns false, so the EN page does not
 *  advertise a French alternate that was never prerendered. */
export function hasFrenchTwin(neutralPath: string): boolean {
  if (!/^\/research(\/|$)/.test(neutralPath)) return true;
  return TRANSLATED_FR.has(neutralPath === "/research/" ? "/research" : neutralPath);
}
