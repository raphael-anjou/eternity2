// The research sidebar — scoped to the ACTIVE section only (the subnav is how
// a reader changes section, so this stays short and readable). On topic pages
// it lists the topic categories with page counts instead. Desktop: sticky
// left column. Mobile: a collapsible disclosure above the article.

import { useState, type ReactNode } from "react";
import { useLocation } from "react-router";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { KIND_DOT, groupedItems, topicMembers, type NavItem, type NavSection } from "@/lib/research/nav";
import { researchTopics, topicUrl } from "@/lib/research/manifest";
import { THEME_ROOTS, NON_PATH_THEMES } from "@/lib/research/theme-roots";

const T = {
  en: { browse: "Browse this section", topics: "Topics", roads: "The nine roads", more: "More", glossary: "Glossary" },
  fr: { browse: "Parcourir cette section", topics: "Thèmes", roads: "Les neuf voies", more: "Plus", glossary: "Glossaire" },
};

function activePath(pathname: string): string {
  return pathname.replace(/^\/fr(?=\/|$)/, "").replace(/\/$/, "") || "/";
}

function SideLink({
  item,
  depth,
  active,
  flat = false,
}: {
  item: NavItem;
  depth: number;
  active: string;
  /** When true, render only this item — its children are placed elsewhere
   *  (e.g. flattened into sidebar groups), so don't recurse. */
  flat?: boolean;
}) {
  const isActive = active === item.url;
  // A per-author hub (its own experiments nested beneath) renders as a
  // first-class heading — bold, no bullet, a touch of top space — so the lab
  // reads as "these people, each with their runs" rather than a uniform list.
  const isAuthorHub = Boolean(item.author) && item.children.length > 0;
  if (isAuthorHub) {
    return (
      <>
        <LocalizedLink
          to={item.url}
          aria-current={isActive ? "page" : undefined}
          className={cn(
            "mt-2 mb-0.5 flex items-center gap-2 rounded-md px-2 py-1 text-sm font-semibold transition-colors",
            isActive ? "text-foreground" : "text-foreground/90 hover:text-foreground",
          )}
        >
          <span className="truncate">{item.title}</span>
        </LocalizedLink>
        {!flat &&
          item.children.map((c) => (
            <SideLink key={c.url} item={c} depth={depth + 1} active={active} />
          ))}
      </>
    );
  }
  return (
    <>
      <LocalizedLink
        to={item.url}
        aria-current={isActive ? "page" : undefined}
        className={cn(
          "group flex items-center gap-2 rounded-md px-2 py-1 text-sm transition-colors",
          depth > 0 && "ml-3 border-l pl-3",
          isActive
            ? "bg-muted font-medium text-foreground"
            : "text-muted-foreground hover:bg-muted/60 hover:text-foreground",
        )}
      >
        <span
          className={cn("h-1.5 w-1.5 shrink-0 rounded-full opacity-70", KIND_DOT[item.kind])}
          aria-hidden
        />
        <span className="truncate">{item.title}</span>
      </LocalizedLink>
      {!flat &&
        item.children.map((c) => (
          <SideLink key={c.url} item={c} depth={depth + 1} active={active} />
        ))}
    </>
  );
}

function SectionTree({ section }: { section: NavSection }) {
  const { pathname } = useLocation();
  const active = activePath(pathname);
  const groups = groupedItems(section);
  // A grouped section is flattened (each page placed by its own group), so its
  // links must not re-render their children; an ungrouped section keeps nesting.
  const isGrouped = groups.some((g) => g.label !== null);
  return (
    <nav className="space-y-0.5 pb-4 text-sm">
      <LocalizedLink
        to={section.url}
        className={cn(
          "mb-1.5 block px-2 text-xs font-semibold uppercase tracking-wide",
          active === section.url
            ? "text-foreground"
            : "text-muted-foreground hover:text-foreground",
        )}
      >
        {section.label}
      </LocalizedLink>
      {groups.map((g, gi) => (
        <div key={g.label ?? "_"} className={gi > 0 ? "pt-2" : undefined}>
          {g.label && (
            <div className="mt-1 mb-0.5 px-2 text-[11px] font-medium uppercase tracking-wide text-muted-foreground/80">
              {g.label}
            </div>
          )}
          {g.items.map((i) => (
            <SideLink key={i.url} item={i} depth={0} active={active} flat={isGrouped} />
          ))}
        </div>
      ))}
    </nav>
  );
}

/** One topic row: accent dot (path themes) + label + page count. */
function TopicRow({
  slug,
  label,
  color,
  count,
  active,
}: {
  slug: string;
  label: string;
  color?: string | undefined;
  count: number;
  active: string;
}) {
  const url = topicUrl(slug);
  const isActive = active === url;
  return (
    <LocalizedLink
      to={url}
      aria-current={isActive ? "page" : undefined}
      className={cn(
        "flex items-center gap-2 rounded-md px-2 py-1 transition-colors",
        isActive
          ? "bg-muted font-medium text-foreground"
          : "text-muted-foreground hover:bg-muted/60 hover:text-foreground",
      )}
    >
      {color ? (
        <span
          className="h-2 w-2 shrink-0 rounded-full"
          style={{ backgroundColor: color }}
          aria-hidden
        />
      ) : (
        <span className="h-2 w-2 shrink-0" aria-hidden />
      )}
      <span className="truncate">{label}</span>
      <span className="ml-auto text-xs tabular-nums text-muted-foreground">{count}</span>
    </LocalizedLink>
  );
}

// The overview/topic-hub left rail: the nine solving paths, styled to match the
// homepage tree (short labels + accent dots + counts), then the meta themes
// (records) under a divider so their hubs stay reachable.
function TopicsList() {
  const t = useT(T);
  const { lang } = useLang();
  const { pathname } = useLocation();
  const active = activePath(pathname);
  const topics = researchTopics(lang);
  const paths = topics.filter((x) => !NON_PATH_THEMES.has(x.slug) && THEME_ROOTS[x.slug]);
  const meta = topics.filter((x) => NON_PATH_THEMES.has(x.slug) || !THEME_ROOTS[x.slug]);
  return (
    <nav className="space-y-0.5 pb-4 text-sm">
      <LocalizedLink
        to="/research/topics"
        className={cn(
          "mb-1.5 block px-2 text-xs font-semibold uppercase tracking-wide",
          active === "/research/topics"
            ? "text-foreground"
            : "text-muted-foreground hover:text-foreground",
        )}
      >
        {t.roads}
      </LocalizedLink>
      {paths.map((topic) => (
        <TopicRow
          key={topic.slug}
          slug={topic.slug}
          label={topic.label}
          color={THEME_ROOTS[topic.slug]?.color}
          count={topicMembers(lang, topic.slug).length}
          active={active}
        />
      ))}
      <div className="mt-3 mb-1.5 px-2 text-xs font-semibold uppercase tracking-wide text-muted-foreground">
        {t.more}
      </div>
      {meta.map((topic) => (
        <TopicRow
          key={topic.slug}
          slug={topic.slug}
          label={topic.label}
          count={topicMembers(lang, topic.slug).length}
          active={active}
        />
      ))}
      <LocalizedLink
        to="/research/glossary"
        aria-current={active === "/research/glossary" ? "page" : undefined}
        className={cn(
          "flex items-center gap-2 rounded-md px-2 py-1 transition-colors",
          active === "/research/glossary"
            ? "bg-muted font-medium text-foreground"
            : "text-muted-foreground hover:bg-muted/60 hover:text-foreground",
        )}
      >
        <span className="h-2 w-2 shrink-0" aria-hidden />
        <span className="truncate">{t.glossary}</span>
      </LocalizedLink>
    </nav>
  );
}

function Collapsible({ children, label }: { children: ReactNode; label: string }) {
  const [open, setOpen] = useState(false);
  return (
    <>
      {/* Mobile: disclosure above the article */}
      <div className="mb-6 lg:hidden">
        <button
          onClick={() => setOpen((o) => !o)}
          aria-expanded={open}
          className="flex w-full items-center justify-between rounded-lg border px-3 py-2 text-sm font-medium"
        >
          {label}
          <span aria-hidden>{open ? "−" : "+"}</span>
        </button>
        {open && <div className="mt-2 max-h-96 overflow-y-auto rounded-lg border p-3">{children}</div>}
      </div>
      {/* Desktop: sticky column */}
      <aside className="hidden lg:block">
        <div className="sticky top-20 max-h-[calc(100vh-6rem)] overflow-y-auto pr-2">
          {children}
        </div>
      </aside>
    </>
  );
}

/** Sidebar scoped to a section; pass `section: null` for the topics list. */
export function DocsSidebar({ section }: { section: NavSection | null }) {
  const t = useT(T);
  return (
    <Collapsible label={section ? t.browse : t.topics}>
      {section ? <SectionTree section={section} /> : <TopicsList />}
    </Collapsible>
  );
}
