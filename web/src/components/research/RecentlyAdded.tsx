import { useMemo } from "react";
import { LocalizedLink } from "@/components/LocalizedLink";
import { pick, useLang, type Dict, type Lang } from "@/i18n";
import { authorDocs } from "@/lib/research/manifest";
import type { ResearchDoc } from "@/lib/research/types";

// A small "Recently added" strip for a researcher hub: the newest pages in the
// notebook, derived from the manifest rather than a hand-maintained list, so a
// fresh page appears here the day it ships and the strip never drifts across
// languages. Titles come from each page's own localized frontmatter and the
// `updated` field supplies the date, so the per-shipment cost is zero: no MDX
// edit, no translation. The doors and the scores table stay the canonical
// curated layer, re-cut at leisure; this strip only absorbs recency.

const LABEL: Dict<string> = {
  en: "Recently added",
  fr: "Ajouts récents",
  es: "Añadidos recientemente",
};

/** Hub/index pages are the landing surfaces themselves, not new work. */
function isIndexFile(file: string): boolean {
  return /(^|\/)index(\.[a-z]{2})?\.mdx$/.test(file);
}

/** "2026-07-22" in the page's locale, e.g. "22 Jul 2026" / "22 juil. 2026". */
function formatUpdated(iso: string, lang: Lang): string {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(iso);
  if (!m) return iso;
  const dt = new Date(Date.UTC(Number(m[1]), Number(m[2]) - 1, Number(m[3])));
  const locale = lang === "fr" ? "fr-FR" : lang === "es" ? "es-ES" : "en-GB";
  return dt.toLocaleDateString(locale, {
    year: "numeric",
    month: "short",
    day: "numeric",
    timeZone: "UTC",
  });
}

export function RecentlyAdded({
  author,
  count = 5,
}: {
  /** Registry slug of the researcher whose hub hosts the strip. */
  author: string;
  /** How many pages to list (a fixed small slot, not a changelog). */
  count?: number;
}) {
  const { lang } = useLang();

  const items = useMemo<ResearchDoc[]>(
    () =>
      authorDocs(lang, author)
        .filter((d) => !isIndexFile(d.file) && d.updated !== undefined)
        .sort((a, b) => (b.updated ?? "").localeCompare(a.updated ?? ""))
        .slice(0, count),
    [lang, author, count],
  );

  if (items.length === 0) return null;

  return (
    <div className="my-6 rounded-lg border p-4">
      <div className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
        {pick(LABEL, lang)}
      </div>
      <ul className="mt-2 space-y-1.5">
        {items.map((d) => (
          <li key={d.url} className="flex items-baseline justify-between gap-4 text-sm">
            <LocalizedLink to={d.url} className="font-medium underline-offset-2 hover:underline">
              {d.title}
            </LocalizedLink>
            <span className="whitespace-nowrap text-xs tabular-nums text-muted-foreground">
              {d.updated ? formatUpdated(d.updated, lang) : ""}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
}
