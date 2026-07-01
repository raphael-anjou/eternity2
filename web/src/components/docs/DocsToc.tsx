// On-page table of contents with scrollspy: h2/h3 anchors from the manifest,
// the one currently in view highlighted. Desktop-only right column.

import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";
import type { TocItem } from "@/lib/research/types";

const T = {
  en: { onThisPage: "On this page" },
  fr: { onThisPage: "Sur cette page" },
};

export function DocsToc({ toc }: { toc: TocItem[] }) {
  const t = useT(T);
  const [activeId, setActiveId] = useState<string>("");

  useEffect(() => {
    if (toc.length === 0) return;
    const headings = toc
      .map((i) => document.getElementById(i.id))
      .filter((el): el is HTMLElement => el !== null);
    if (headings.length === 0) return;
    const observer = new IntersectionObserver(
      (entries) => {
        // Highlight the last heading that has scrolled above the trigger zone.
        for (const e of entries) {
          if (e.isIntersecting) {
            setActiveId(e.target.id);
            return;
          }
        }
      },
      { rootMargin: "-80px 0px -70% 0px" },
    );
    for (const h of headings) observer.observe(h);
    return () => observer.disconnect();
  }, [toc]);

  if (toc.length === 0) return null;

  return (
    <aside className="hidden xl:block">
      <div className="sticky top-20 max-h-[calc(100vh-6rem)] overflow-y-auto">
        <div className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">
          {t.onThisPage}
        </div>
        <ul className="mt-3 space-y-1.5 border-l text-sm">
          {toc.map((item) => (
            <li key={item.id}>
              <a
                href={`#${item.id}`}
                className={cn(
                  "-ml-px block border-l pl-3 leading-snug transition-colors",
                  item.depth === 3 && "pl-6",
                  activeId === item.id
                    ? "border-foreground font-medium text-foreground"
                    : "border-transparent text-muted-foreground hover:text-foreground",
                )}
              >
                {item.text}
              </a>
            </li>
          ))}
        </ul>
      </div>
    </aside>
  );
}
