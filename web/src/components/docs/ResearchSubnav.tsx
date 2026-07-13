// The research section's own navigation bar, under the site header: the reading
// MODES (Overview + the three door sections) + an all-themes tab + search. The
// nine solving PATHS are not repeated here — they live in the homepage tree and
// in the persistent left rail ("The nine roads"), so a third navbar row would
// only duplicate them. The left sidebar shows the ACTIVE section's tree; this
// bar is how a reader changes territory.

import { useLocation } from "react-router";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { researchNav } from "@/lib/research/nav";
import { SearchDialog } from "./SearchDialog";

const T = {
  en: { overview: "Overview", themes: "All themes", people: "People" },
  fr: { overview: "Vue d'ensemble", themes: "Tous les thèmes", people: "Contributeurs" },
};

export function ResearchSubnav() {
  const t = useT(T);
  const { lang } = useLang();
  const { pathname } = useLocation();
  const active = pathname.replace(/^\/fr(?=\/|$)/, "").replace(/\/$/, "") || "/";
  const sections = researchNav(lang);
  const onTopics = active === "/research/topics" || active.startsWith("/research/topics/");
  const onPeople = active === "/research/people" || active.startsWith("/research/people/");

  const tab = (to: string, label: string, isActive: boolean) => (
    <LocalizedLink
      key={to}
      to={to}
      className={cn(
        "rounded-md px-2.5 py-1 text-sm font-medium whitespace-nowrap transition-colors",
        isActive
          ? "bg-primary text-primary-foreground"
          : "text-muted-foreground hover:bg-muted hover:text-foreground",
      )}
    >
      {label}
    </LocalizedLink>
  );

  return (
    <div className="mb-6 border-b pb-3">
      <nav
        className="no-scrollbar flex items-center gap-1 overflow-x-auto"
        aria-label="Research sections"
      >
        {tab("/research", t.overview, active === "/research")}
        {sections.map((s) =>
          tab(
            s.url,
            s.label,
            (!onTopics &&
              !onPeople &&
              (active === s.url || active.startsWith(s.url + "/"))) ||
              // The re-homed flat pages (reference/papers/records) highlight Build.
              (s.key === "build" && /^\/research\/(reference|papers|records)$/.test(active)),
          ),
        )}
        {tab("/research/people", t.people, onPeople)}
        {tab("/research/topics", t.themes, onTopics)}
        <SearchDialog />
      </nav>
    </div>
  );
}
