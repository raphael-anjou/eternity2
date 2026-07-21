// The research glossary at /research/glossary: every community and domain term
// the wiki uses, defined once, alphabetised, each linking to the page that goes
// deep. Data-driven from content/research/glossary.json (no MDX), rendered in
// the docs shell like the topic hubs. Each term gets a slug id so it can be
// linked directly (e.g. /research/glossary#break-index).

import { useMemo } from "react";
import GithubSlugger from "github-slugger";
import { useT, useLang } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { DocsSidebar } from "./DocsSidebar";
import { ResearchSubnav } from "./ResearchSubnav";
import glossary from "../../../content/research/glossary.json";

// term/def are localized {en, fr, es}; the auto-linker (plugins/
// research-glossary-autolink.ts) computes the anchor from the same localized
// term string with the same slugger, so a Term popup's link lands on the right
// entry here.
type Localized = { en: string; fr: string; es: string };
interface Term {
  term: Localized;
  def: Localized;
  tier?: number;
  see?: string;
}

const TERMS = (glossary.terms as Term[]) ?? [];

/** Stable per-term anchor id: a fresh slugger per call is deterministic and (as
 *  the glossary carries no per-language slug collisions) never needs the -1
 *  dedupe suffix, so the plugin can reproduce this id for a single term in
 *  isolation. */
function termSlug(term: string): string {
  return new GithubSlugger().slug(term);
}

const T = {
  en: {
    research: "Research",
    title: "Glossary",
    lede: "The words the wiki leans on, defined once: community jargon, the loaded computer-science terms that mean something specific here, and the notation the boards are written in, each with a link to the page that goes deep.",
    see: "See",
    jump: "Jump to",
  },
  fr: {
    research: "Recherche",
    title: "Glossaire",
    lede: "Les mots sur lesquels le wiki s'appuie, définis une bonne fois. Le jargon de la communauté, les termes d'informatique au sens précis ici, et la notation dans laquelle les plateaux s'écrivent ; chacun avec un lien vers la page qui approfondit.",
    see: "Voir",
    jump: "Aller à",
  },
  es: {
    research: "Investigación",
    title: "Glosario",
    lede: "Las palabras en las que se apoya el wiki, definidas de una vez por todas: la jerga de la comunidad, los términos informáticos que aquí tienen un sentido preciso y la notación con la que se escriben los tableros, cada uno con un enlace a la página que lo profundiza.",
    see: "Ver",
    jump: "Ir a",
  },
};

function Crumbs({ current }: { current: string }) {
  const t = useT(T);
  return (
    <nav className="flex flex-wrap items-center gap-1.5 text-sm text-muted-foreground">
      <LocalizedLink to="/research" className="hover:text-foreground">
        {t.research}
      </LocalizedLink>
      <span aria-hidden>/</span>
      <span className="text-foreground">{current}</span>
    </nav>
  );
}

export function GlossaryPage() {
  const t = useT(T);
  const { lang } = useLang();

  // Group alphabetically in the reader's language; assign a stable slug id per
  // term (computed from the localized term, matching the auto-linker).
  const { letters, byLetter } = useMemo(() => {
    const localized = TERMS.map((entry) => ({
      term: entry.term[lang],
      def: entry.def[lang],
      see: entry.see,
      id: termSlug(entry.term[lang]),
    }));
    const sorted = localized.sort((a, b) =>
      a.term.localeCompare(b.term, lang, { sensitivity: "base" }),
    );
    const map = new Map<string, Array<(typeof sorted)[number]>>();
    for (const term of sorted) {
      const letter = term.term[0]?.toUpperCase() ?? "#";
      const bucket = map.get(letter) ?? [];
      bucket.push(term);
      map.set(letter, bucket);
    }
    return { letters: [...map.keys()], byLetter: map };
  }, [lang]);

  return (
    <div>
      <ResearchSubnav />
      <div className="lg:grid lg:grid-cols-[15rem_minmax(0,1fr)] lg:gap-10">
        <DocsSidebar section={null} />
        <div className="min-w-0">
          <Crumbs current={t.title} />
          <header className="mt-4 space-y-3">
            <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{t.title}</h1>
            <p className="text-lg text-muted-foreground">{t.lede}</p>
          </header>

          {/* A–Z jump bar */}
          <nav
            className="mt-6 flex flex-wrap gap-1.5 border-y py-3 text-sm"
            aria-label={t.jump}
          >
            {letters.map((l) => (
              <a
                key={l}
                href={`#letter-${l}`}
                className="rounded px-2 py-0.5 font-medium text-muted-foreground hover:bg-muted hover:text-foreground"
              >
                {l}
              </a>
            ))}
          </nav>

          <div className="mt-8 space-y-10">
            {letters.map((l) => (
              <section key={l} id={`letter-${l}`} className="scroll-mt-24">
                <h2 className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
                  {l}
                </h2>
                <dl className="mt-3 space-y-4">
                  {(byLetter.get(l) ?? []).map((term) => (
                    <div key={term.id} id={term.id} className="scroll-mt-24">
                      <dt className="text-sm font-semibold tracking-tight">{term.term}</dt>
                      <dd className="mt-0.5 text-sm leading-relaxed text-muted-foreground">
                        {term.def}
                        {term.see && (
                          <>
                            {" "}
                            <LocalizedLink
                              to={term.see}
                              className="font-medium text-foreground underline-offset-2 hover:underline"
                            >
                              {t.see} →
                            </LocalizedLink>
                          </>
                        )}
                      </dd>
                    </div>
                  ))}
                </dl>
              </section>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
