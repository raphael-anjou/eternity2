import { LocalizedLink } from "@/components/LocalizedLink";
import { useLang } from "@/i18n";
import { researchDocs, researchAuthors } from "@/lib/research/manifest";

// The experiment gallery's author grid, discovered from the manifest rather than
// hand-listed: any researcher who has an experiment page under
// lab/experiments/<slug>/ gets a tile linking to their section, followed by the
// standing "your section here" invite. Adding an author is then adding their
// pages (with `author:` frontmatter) and a registry entry — no edit here.
//
// "Has an experiment page" = a doc of kind `experiment` under the author's own
// /research/lab/experiments/<slug>/ folder. That excludes the gallery hubs
// (kind: page) and the shared engine explainers (kind: concept), so the count is
// the researcher's actual named experiments, not their infrastructure pages.

const EXPERIMENTS_PREFIX = "/research/lab/experiments/";

type AuthorTile = {
  slug: string;
  name: string;
  /** The author's experiments hub, e.g. /research/lab/experiments/raphael-anjou. */
  url: string;
  count: number;
};

const T = {
  en: {
    experiments: (n: number) => `${n} experiment${n === 1 ? "" : "s"}`,
    inviteTitle: "Your section here",
    inviteBody:
      "The notebook is open. If you have a search worth writing down, it gets its own section, credited to you, sitting beside the rest.",
  },
  fr: {
    experiments: (n: number) => `${n} expérience${n === 1 ? "" : "s"}`,
    inviteTitle: "Votre section ici",
    inviteBody:
      "Le carnet est ouvert. Si vous avez une recherche à consigner, elle aura sa propre section, créditée à votre nom, aux côtés des autres.",
  },
};

export function ExperimentAuthors() {
  const { lang } = useLang();
  const t = T[lang];
  const docs = researchDocs(lang);
  const authors = researchAuthors(lang);

  // Count named experiment pages per author: kind === "experiment" under the
  // author's own folder. Hubs (kind: page) and engine explainers (kind: concept)
  // don't count; the number is the researcher's real experiments.
  const counts = new Map<string, number>();
  for (const d of docs) {
    if (!d.author) continue;
    if (d.kind !== "experiment") continue;
    // Must live under the author's own folder, not the shared benchmark root.
    if (!d.url.startsWith(`${EXPERIMENTS_PREFIX}${d.author}/`)) continue;
    counts.set(d.author, (counts.get(d.author) ?? 0) + 1);
  }

  const tiles: AuthorTile[] = [];
  for (const [slug, count] of counts) {
    const author = authors.find((a) => a.slug === slug);
    if (!author) continue;
    tiles.push({
      slug,
      name: author.name,
      url: `${EXPERIMENTS_PREFIX}${slug}`,
      count,
    });
  }
  // Most experiments first, then alphabetical — a stable, sensible order.
  tiles.sort((a, b) => b.count - a.count || a.name.localeCompare(b.name));

  return (
    <div className="not-prose grid gap-4 md:grid-cols-2">
      {tiles.map((tile) => (
        <LocalizedLink
          key={tile.slug}
          to={tile.url}
          className="group block rounded-xl border p-5 no-underline transition-shadow hover:shadow-md"
        >
          <div className="text-base font-semibold tracking-tight group-hover:underline">
            {tile.name}
          </div>
          <div className="mt-1.5 text-sm leading-relaxed text-muted-foreground">
            {t.experiments(tile.count)}
          </div>
        </LocalizedLink>
      ))}
      <LocalizedLink
        to="/research/contribute"
        className="group block rounded-xl border border-dashed p-5 no-underline transition-shadow hover:shadow-md"
      >
        <div className="text-base font-semibold tracking-tight group-hover:underline">
          {t.inviteTitle}
        </div>
        <div className="mt-1.5 text-sm leading-relaxed text-muted-foreground">
          {t.inviteBody}
        </div>
      </LocalizedLink>
    </div>
  );
}
