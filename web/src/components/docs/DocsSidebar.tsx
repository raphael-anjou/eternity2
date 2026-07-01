// The research sidebar — scoped to the ACTIVE section only (the subnav is how
// a reader changes section, so this stays short and readable). On topic pages
// it lists the topic categories with page counts instead. Desktop: sticky
// left column. Mobile: a collapsible disclosure above the article.

import { useState, type ReactNode } from "react";
import { useLocation } from "react-router";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { KIND_DOT, topicMembers, type NavItem, type NavSection } from "@/lib/research/nav";
import { researchTopics, topicUrl } from "@/lib/research/manifest";

const T = {
  en: { browse: "Browse this section", topics: "Topics" },
  fr: { browse: "Parcourir cette section", topics: "Thèmes" },
};

function activePath(pathname: string): string {
  return pathname.replace(/^\/fr(?=\/|$)/, "").replace(/\/$/, "") || "/";
}

function SideLink({ item, depth, active }: { item: NavItem; depth: number; active: string }) {
  const isActive = active === item.url;
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
      {item.children.map((c) => (
        <SideLink key={c.url} item={c} depth={depth + 1} active={active} />
      ))}
    </>
  );
}

function SectionTree({ section }: { section: NavSection }) {
  const { pathname } = useLocation();
  const active = activePath(pathname);
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
      {section.items.map((i) => (
        <SideLink key={i.url} item={i} depth={0} active={active} />
      ))}
    </nav>
  );
}

function TopicsList() {
  const t = useT(T);
  const { lang } = useLang();
  const { pathname } = useLocation();
  const active = activePath(pathname);
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
        {t.topics}
      </LocalizedLink>
      {researchTopics(lang).map((topic) => {
        const url = topicUrl(topic.slug);
        const count = topicMembers(lang, topic.slug).length;
        const isActive = active === url;
        return (
          <LocalizedLink
            key={topic.slug}
            to={url}
            aria-current={isActive ? "page" : undefined}
            className={cn(
              "flex items-center justify-between gap-2 rounded-md px-2 py-1 transition-colors",
              isActive
                ? "bg-muted font-medium text-foreground"
                : "text-muted-foreground hover:bg-muted/60 hover:text-foreground",
            )}
          >
            <span className="truncate">{topic.label}</span>
            <span className="text-xs tabular-nums text-muted-foreground">{count}</span>
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

/** Sidebar scoped to a section; pass `section: null` for the topics list. */
export function DocsSidebar({ section }: { section: NavSection | null }) {
  const t = useT(T);
  return (
    <Collapsible label={section ? t.browse : t.topics}>
      {section ? <SectionTree section={section} /> : <TopicsList />}
    </Collapsible>
  );
}
