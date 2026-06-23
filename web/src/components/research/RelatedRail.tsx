import { useT, useLang } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { relatedFor, kindLabel, type LinkKind } from "@/data/research-links";

// The "keep going" rail at the foot of a research page. It reads the page's own
// path from the central research-links graph and renders the handful of pages a
// reader would naturally jump to next — the finding that explains an invention,
// the invention that probes a finding, the log behind it all. Every research
// page that opts in becomes self-supporting: you can always step sideways.

const T = {
  en: { title: "Keep exploring", empty: "" },
  fr: { title: "Continuer l'exploration", empty: "" },
};

const KIND_DOT: Record<LinkKind, string> = {
  why: "bg-violet-400",
  invention: "bg-emerald-400",
  finding: "bg-sky-400",
  tool: "bg-amber-400",
  reference: "bg-stone-400",
};

export function RelatedRail({ path }: { path: string }) {
  const t = useT(T);
  const { lang } = useLang();
  const items = relatedFor(path);
  if (items.length === 0) return null;

  return (
    <section className="mx-auto max-w-3xl border-t pt-8">
      <h2 className="text-sm font-semibold uppercase tracking-wide text-muted-foreground">
        {t.title}
      </h2>
      <div className="mt-4 grid gap-3 sm:grid-cols-2">
        {items.map((n) => (
          <LocalizedLink
            key={n.path}
            to={n.path}
            className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
          >
            <div className="flex items-center gap-2">
              <span className={`h-2 w-2 shrink-0 rounded-full ${KIND_DOT[n.kind]}`} />
              <span className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
                {kindLabel(n.kind, lang)}
              </span>
            </div>
            <div className="mt-1.5 text-sm font-semibold tracking-tight group-hover:underline">
              {n.title[lang]}
            </div>
            <p className="mt-1 text-xs leading-relaxed text-muted-foreground">{n.blurb[lang]}</p>
          </LocalizedLink>
        ))}
      </div>
    </section>
  );
}
