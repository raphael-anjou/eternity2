// The research wiki sidebar: the whole territory in one tree, grouped by the
// three doors, with the active page highlighted. Desktop: sticky left column.
// Mobile: a collapsible "Browse the research" disclosure above the article.

import { useState } from "react";
import { useLocation } from "react-router";
import { useLang, useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { cn } from "@/lib/utils";
import { researchNav, KIND_DOT, type NavItem } from "@/lib/research/nav";

const T = {
  en: { overview: "Research overview", browse: "Browse the research" },
  fr: { overview: "Vue d'ensemble", browse: "Parcourir la recherche" },
};

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

function Tree() {
  const t = useT(T);
  const { lang } = useLang();
  const { pathname } = useLocation();
  // Active matching is language-neutral: strip the /fr prefix.
  const active = pathname.replace(/^\/fr(?=\/|$)/, "");
  const sections = researchNav(lang);

  return (
    <nav className="space-y-6 pb-4 text-sm">
      <LocalizedLink
        to="/research"
        className={cn(
          "block rounded-md px-2 py-1 font-semibold tracking-tight",
          active === "/research" ? "bg-muted" : "hover:bg-muted/60",
        )}
      >
        {t.overview}
      </LocalizedLink>
      {sections.map((s) => (
        <div key={s.key}>
          <LocalizedLink
            to={s.url}
            className={cn(
              "mb-1.5 block px-2 text-xs font-semibold uppercase tracking-wide",
              active === s.url ? "text-foreground" : "text-muted-foreground hover:text-foreground",
            )}
          >
            {s.label}
          </LocalizedLink>
          <div className="space-y-0.5">
            {s.items.map((i) => (
              <SideLink key={i.url} item={i} depth={0} active={active} />
            ))}
          </div>
        </div>
      ))}
    </nav>
  );
}

export function DocsSidebar() {
  const t = useT(T);
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
          {t.browse}
          <span aria-hidden>{open ? "−" : "+"}</span>
        </button>
        {open && (
          <div className="mt-2 max-h-96 overflow-y-auto rounded-lg border p-3">
            <Tree />
          </div>
        )}
      </div>
      {/* Desktop: sticky column */}
      <aside className="hidden lg:block">
        <div className="sticky top-20 max-h-[calc(100vh-6rem)] overflow-y-auto pr-2">
          <Tree />
        </div>
      </aside>
    </>
  );
}
