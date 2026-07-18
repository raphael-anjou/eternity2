// The research sidebar — scoped to the ACTIVE section only (the subnav is how
// a reader changes section, so this stays short and readable). On topic pages
// it lists the topic categories with page counts instead. Desktop: sticky
// left column. Mobile: a collapsible disclosure above the article.

import { useState, type ReactNode } from "react";
import { useLocation } from "react-router";
import { useLang, useT, neutralPath } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { KIND_DOT, groupedItems, topicMembers, type NavItem, type NavSection } from "@/lib/research/nav";
import { researchTopics, topicUrl, researchAuthors, authorDocs, authorUrl } from "@/lib/research/manifest";
import { THEME_ROOTS, NON_PATH_THEMES } from "@/lib/research/theme-roots";

const T = {
  en: { browse: "Browse this section", topics: "Topics", roads: "The nine roads", more: "More", glossary: "Glossary", people: "People", allPeople: "Who's who" },
  fr: { browse: "Parcourir cette section", topics: "Thèmes", roads: "Les neuf voies", more: "Plus", glossary: "Glossaire", people: "Contributeurs", allPeople: "Qui est qui" },
  es: { browse: "Explorar esta sección", topics: "Temas", roads: "Las nueve vías", more: "Más", glossary: "Glosario", people: "Contribuidores", allPeople: "Quién es quién" },
};

function activePath(pathname: string): string {
  return neutralPath(pathname).replace(/\/$/, "") || "/";
}

/** Indent for a nested row. Each level adds a rule + padding, so a page inside
 *  a sub-hub reads as deeper than its group's siblings (Tailwind needs literal
 *  class names, so this is a lookup, not an interpolation). */
const DEPTH_INDENT: Record<number, string> = {
  0: "",
  1: "ml-3 border-l pl-3",
  2: "ml-6 border-l pl-3",
  3: "ml-9 border-l pl-3",
};
const indentAt = (depth: number) => DEPTH_INDENT[Math.min(depth, 3)] ?? DEPTH_INDENT[3];

/** Does this subtree contain the active page? Used to auto-open a sub-hub so a
 *  reader never lands on a page whose own group is collapsed shut. */
function containsActive(item: NavItem, active: string): boolean {
  return item.url === active || item.children.some((c) => containsActive(c, active));
}

/** A collapsible group of pages nested INSIDE an author hub (the engines, the
 *  runs, the analyses). The author hub itself stays a flat heading — this is
 *  the level below it, where a long list needs folding. Open by default when it
 *  holds the active page; otherwise remembers whatever the reader last chose. */
function SubHub({ item, depth, active }: { item: NavItem; depth: number; active: string }) {
  const hasActive = containsActive(item, active);
  const [open, setOpen] = useState(hasActive);
  // Following a link into a collapsed group must reveal it; without this the
  // reader clicks a search result and lands behind a shut door.
  const [lastActive, setLastActive] = useState(active);
  if (active !== lastActive) {
    setLastActive(active);
    if (hasActive && !open) setOpen(true);
  }
  const isActive = active === item.url;
  return (
    <>
      <div className={cn("flex items-center gap-1", indentAt(depth))}>
        <button
          type="button"
          onClick={() => setOpen((o) => !o)}
          aria-expanded={open}
          aria-label={`${open ? "Collapse" : "Expand"} ${item.title}`}
          className="shrink-0 rounded p-0.5 text-muted-foreground/70 transition-colors hover:bg-muted/60 hover:text-foreground"
        >
          <svg
            viewBox="0 0 12 12"
            className={cn("h-3 w-3 transition-transform", open && "rotate-90")}
            aria-hidden
          >
            <path d="M4 2.5 L8 6 L4 9.5" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
          </svg>
        </button>
        <LocalizedLink
          to={item.url}
          aria-current={isActive ? "page" : undefined}
          className={cn(
            "group flex min-w-0 flex-1 items-center gap-2 rounded-md px-1.5 py-1 text-sm transition-colors",
            isActive
              ? "bg-muted font-medium text-foreground"
              : "font-medium text-foreground/80 hover:bg-muted/60 hover:text-foreground",
          )}
        >
          <span className="truncate">{item.title}</span>
          <span className="ml-auto shrink-0 pl-1 text-[10px] tabular-nums text-muted-foreground/60">
            {item.children.length}
          </span>
        </LocalizedLink>
      </div>
      {open &&
        item.children.map((c) => (
          <SideLink key={c.url} item={c} depth={depth + 1} active={active} />
        ))}
    </>
  );
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
  // Only at depth 0: nav lifts author hubs to the top of their section, so a
  // deeper page carrying the same `author` is a sub-hub INSIDE one, not an
  // author. This branch ignores `depth`, so letting a sub-hub through here
  // would render it flush-left, level with the author it belongs to.
  const isAuthorHub = Boolean(item.author) && item.children.length > 0 && depth === 0;
  // Any other page with children nested under a hub is a collapsible group.
  if (!isAuthorHub && !flat && item.children.length > 0 && depth > 0) {
    return <SubHub item={item} depth={depth} active={active} />;
  }
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
          indentAt(depth),
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
        to="/research"
        className={cn(
          "mb-1.5 block px-2 text-xs font-semibold uppercase tracking-wide",
          active === "/research"
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
      {meta.length > 0 && (
        <div className="mt-3 mb-1.5 px-2 text-xs font-semibold uppercase tracking-wide text-muted-foreground">
          {t.more}
        </div>
      )}
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

// The people left rail: the gallery link on top, then every contributor with a
// page, alphabetical (by display name, locale-aware), each with the count of
// pages they wrote. Shown on /research/people and every /research/people/<slug>.
function PeopleList() {
  const t = useT(T);
  const { lang } = useLang();
  const { pathname } = useLocation();
  const active = activePath(pathname);
  const authors = [...researchAuthors(lang)].sort((a, b) =>
    a.name.localeCompare(b.name, lang),
  );
  return (
    <nav className="space-y-0.5 pb-4 text-sm">
      <LocalizedLink
        to="/research/people"
        className={cn(
          "mb-1.5 block px-2 text-xs font-semibold uppercase tracking-wide",
          active === "/research/people"
            ? "text-foreground"
            : "text-muted-foreground hover:text-foreground",
        )}
      >
        {t.allPeople}
      </LocalizedLink>
      {authors.map((a) => {
        const url = authorUrl(a.slug);
        const isActive = active === url;
        const count = authorDocs(lang, a.slug).length;
        return (
          <LocalizedLink
            key={a.slug}
            to={url}
            aria-current={isActive ? "page" : undefined}
            className={cn(
              "flex items-center gap-2 rounded-md px-2 py-1 transition-colors",
              isActive
                ? "bg-muted font-medium text-foreground"
                : "text-muted-foreground hover:bg-muted/60 hover:text-foreground",
            )}
          >
            <span className="truncate">{a.name}</span>
            {count > 0 && (
              <span className="ml-auto text-xs tabular-nums text-muted-foreground">{count}</span>
            )}
          </LocalizedLink>
        );
      })}
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

/** Sidebar scoped to a section; pass `section: null` for the topics list, or
 *  `variant: "people"` for the alphabetical contributor list. */
export function DocsSidebar({
  section,
  variant,
}: {
  section: NavSection | null;
  variant?: "people";
}) {
  const t = useT(T);
  if (variant === "people") {
    return (
      <Collapsible label={t.people}>
        <PeopleList />
      </Collapsible>
    );
  }
  return (
    <Collapsible label={section ? t.browse : t.topics}>
      {section ? <SectionTree section={section} /> : <TopicsList />}
    </Collapsible>
  );
}
