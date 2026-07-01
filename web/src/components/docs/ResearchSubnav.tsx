// The research section's own navigation bar, under the site header: the three
// doors as tabs, then the big cross-cutting topic categories as chips. The
// left sidebar only ever shows the ACTIVE section's tree — this bar is how a
// reader changes territory.

import { useLocation } from "react-router";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { researchNav } from "@/lib/research/nav";
import { SearchDialog } from "./SearchDialog";

const T = {
  en: { overview: "Overview", topics: "Topics" },
  fr: { overview: "Vue d'ensemble", topics: "Thèmes" },
};

export function ResearchSubnav() {
  const t = useT(T);
  const { lang } = useLang();
  const { pathname } = useLocation();
  const active = pathname.replace(/^\/fr(?=\/|$)/, "").replace(/\/$/, "") || "/";
  const sections = researchNav(lang);
  const onTopics = active === "/research/topics" || active.startsWith("/research/topics/");

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
      <nav className="no-scrollbar flex items-center gap-1 overflow-x-auto" aria-label="Research sections">
        {tab("/research", t.overview, active === "/research")}
        {sections.map((s) =>
          tab(s.url, s.label, !onTopics && (active === s.url || active.startsWith(s.url + "/")) ||
            // The re-homed flat pages (reference/papers/records) highlight Build.
            (s.key === "build" && /^\/research\/(reference|papers|records)$/.test(active))),
        )}
        {tab("/research/topics", t.topics, onTopics)}
        <SearchDialog />
      </nav>
    </div>
  );
}
