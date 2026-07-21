// The inline glossary term. The build-time auto-linker (plugins/
// research-glossary-autolink.ts) wraps the first prose mention of each glossary
// term per article in <Term termId="..." />; the wrapped prose text is the
// element's children, so it lands verbatim in the prerendered HTML (the tooltip
// popup only mounts on hover/focus, never at prerender time).
//
// The term reads as a subtly underlined span, not a loud link. Hovering (mouse)
// or focusing (keyboard) reveals a small popup with the localized one-line
// definition and a link into /research/glossary#<termId> for the full entry.
// The trigger itself is that link, so a keyboard or touch reader activating it
// jumps straight to the glossary, and the whole control is one interactive
// element (no nested-interactive trap).

import { useMemo, type ReactNode } from "react";
import GithubSlugger from "github-slugger";
import { useLang, pathForLang, type Lang } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import glossary from "../../../content/research/glossary.json";

type Localized = { en: string; fr: string; es: string };
interface GlossaryEntry {
  term: Localized;
  def: Localized;
}

const ENTRIES = (glossary.terms as GlossaryEntry[]) ?? [];

/** Same anchor id GlossaryPage assigns: a fresh slugger per call is
 *  deterministic, and the glossary carries no per-language slug collisions, so
 *  the -1 dedupe suffix never fires. */
function termSlug(term: string): string {
  return new GithubSlugger().slug(term);
}

/** id (in a given language) -> localized definition, so a Term resolves its
 *  popup text from the reader's language without the plugin baking it in. */
function buildIndex(lang: Lang): Map<string, { def: string }> {
  const map = new Map<string, { def: string }>();
  for (const entry of ENTRIES) {
    map.set(termSlug(entry.term[lang]), { def: entry.def[lang] });
  }
  return map;
}

const TRIGGER_CLASS =
  "cursor-help underline decoration-dotted decoration-muted-foreground/50 underline-offset-2 transition-colors hover:decoration-foreground focus-visible:outline-none focus-visible:rounded-sm focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-1 focus-visible:ring-offset-background";

export function Term({ termId, children }: { termId: string; children: ReactNode }) {
  const { lang } = useLang();
  const index = useMemo(() => buildIndex(lang), [lang]);
  const entry = index.get(termId);
  // The glossary anchor, localized to the reader's language prefix. A native
  // anchor (not <LocalizedLink>) is used as the tooltip trigger because base-ui
  // merges a ref onto the rendered element, which a plain function component
  // like LocalizedLink cannot receive; an <a> takes the ref cleanly and still
  // prerenders the term text into the static HTML.
  const href = pathForLang(`/research/glossary#${termId}`, lang);

  // If the id does not resolve (a stale wrap after a term was renamed), fall
  // back to a plain link into the glossary so nothing breaks in prose. This
  // path keeps SPA navigation via LocalizedLink since there is no tooltip ref
  // to satisfy.
  if (!entry) {
    return (
      <LocalizedLink
        to={`/research/glossary#${termId}`}
        className="underline decoration-dotted decoration-muted-foreground/50 underline-offset-2 hover:decoration-foreground"
      >
        {children}
      </LocalizedLink>
    );
  }

  return (
    <TooltipProvider delay={120}>
      <Tooltip>
        <TooltipTrigger render={<a href={href} className={TRIGGER_CLASS} />}>
          {children}
        </TooltipTrigger>
        <TooltipContent className="max-w-xs text-left leading-relaxed">
          {entry.def}
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
}
