// Research search: a ⌘K dialog over a build-time full-text index. The index
// (per language) ships as its own chunk and loads the first time the dialog
// opens — page loads pay nothing. MDX pages are indexed full-text; legacy TSX
// pages contribute title+description until they migrate.

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useNavigate } from "react-router";
import { Search } from "lucide-react";
import type MiniSearch from "minisearch";
import { useLang, useT, pathForLang } from "@/i18n";
import { cn } from "@/lib/utils";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog";
import { allNavItems, kindLabel, KIND_DOT } from "@/lib/research/nav";
import type { ResearchKind, SearchEntry } from "@/lib/research/types";

const T = {
  en: {
    button: "Search",
    placeholder: "Search the research wiki…",
    empty: "No result — try another word.",
    hint: "Type to search every research page.",
  },
  fr: {
    button: "Rechercher",
    placeholder: "Chercher dans le wiki…",
    empty: "Aucun résultat — essayez un autre mot.",
    hint: "Tapez pour chercher dans toutes les pages de recherche.",
  },
  es: {
    button: "Buscar",
    placeholder: "Buscar en el wiki…",
    empty: "Sin resultados — prueba con otra palabra.",
    hint: "Escribe para buscar en todas las páginas de investigación.",
  },
};

interface Hit extends SearchEntry {
  id: string;
}

function snippet(hit: Hit, query: string): string {
  const hay = hit.text || hit.description;
  const terms = query.toLowerCase().split(/\s+/).filter(Boolean);
  const lower = hay.toLowerCase();
  for (const t of terms) {
    const i = lower.indexOf(t);
    if (i >= 0) {
      const start = Math.max(0, i - 50);
      const end = Math.min(hay.length, i + t.length + 90);
      return (start > 0 ? "…" : "") + hay.slice(start, end) + (end < hay.length ? "…" : "");
    }
  }
  return hit.description;
}

export function SearchDialog() {
  const t = useT(T);
  const { lang } = useLang();
  const navigate = useNavigate();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState(0);
  const [index, setIndex] = useState<MiniSearch<Hit> | null>(null);
  const indexLang = useRef<string>("");
  const inputRef = useRef<HTMLInputElement>(null);

  // ⌘K / Ctrl+K toggles the dialog anywhere in the research section.
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setOpen((o) => !o);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  // Build the index on first open (and rebuild if the language changed).
  useEffect(() => {
    if (!open || indexLang.current === lang) return;
    let cancelled = false;
    void (async () => {
      // Per-language search index, loaded only when the dialog opens. Each
      // specifier is static so the bundler can code-split it; the switch picks
      // the active language (English is the fallback for any other).
      const loadIndex = () => {
        switch (lang) {
          case "fr":
            return import("virtual:research-search-fr");
          case "es":
            return import("virtual:research-search-es");
          default:
            return import("virtual:research-search-en");
        }
      };
      const [{ default: MiniSearchCtor }, mod] = await Promise.all([
        import("minisearch"),
        loadIndex(),
      ]);
      const mini = new MiniSearchCtor<Hit>({
        fields: ["title", "description", "text"],
        storeFields: ["url", "title", "description", "kind", "text"],
        searchOptions: { boost: { title: 3, description: 2 }, prefix: true, fuzzy: 0.2 },
      });
      const docs: Hit[] = mod.entries.map((e) => ({ ...e, id: e.url }));
      const covered = new Set(docs.map((d) => d.url));
      // Legacy TSX research pages: searchable by title/description.
      for (const item of allNavItems(lang)) {
        if (!covered.has(item.url)) {
          docs.push({
            id: item.url,
            url: item.url,
            title: item.title,
            description: item.description,
            kind: item.kind,
            text: "",
          });
        }
      }
      mini.addAll(docs);
      if (!cancelled) {
        indexLang.current = lang;
        setIndex(mini);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [open, lang]);

  const results = useMemo(() => {
    if (!index || query.trim().length < 2) return [];
    return index.search(query).slice(0, 12) as unknown as (Hit & { score: number })[];
  }, [index, query]);

  const go = useCallback(
    (url: string) => {
      setOpen(false);
      setQuery("");
      void navigate(pathForLang(url, lang));
    },
    [navigate, lang],
  );

  const onInputKey = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setSelected((s) => Math.min(s + 1, results.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setSelected((s) => Math.max(s - 1, 0));
    } else if (e.key === "Enter") {
      const hit = results[selected];
      if (hit) go(hit.url);
    }
  };

  return (
    <>
      <button
        onClick={() => setOpen(true)}
        className="ml-auto flex shrink-0 items-center gap-2 rounded-md border px-2.5 py-1 text-sm text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
      >
        <Search className="h-3.5 w-3.5" aria-hidden />
        <span className="hidden sm:inline">{t.button}</span>
        <kbd className="hidden rounded border bg-muted px-1 font-mono text-[10px] sm:inline">
          ⌘K
        </kbd>
      </button>
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="top-[15%] max-w-xl translate-y-0 gap-0 p-0" showCloseButton={false}>
          <DialogTitle className="sr-only">{t.button}</DialogTitle>
          <div className="flex items-center gap-2 border-b px-3">
            <Search className="h-4 w-4 shrink-0 text-muted-foreground" aria-hidden />
            <input
              ref={inputRef}
              autoFocus
              value={query}
              onChange={(e) => {
                setQuery(e.target.value);
                setSelected(0);
              }}
              onKeyDown={onInputKey}
              placeholder={t.placeholder}
              className="w-full bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground"
              // Combobox pattern: the input owns the results listbox and points
              // at the active option, so arrow-key navigation is announced.
              role="combobox"
              aria-expanded={results.length > 0}
              aria-controls="search-results"
              aria-activedescendant={
                results.length > 0 && results[selected] ? `search-opt-${selected}` : undefined
              }
              aria-autocomplete="list"
              aria-label={t.button}
            />
          </div>
          <div
            id="search-results"
            role="listbox"
            aria-label={t.button}
            className="max-h-80 overflow-y-auto p-2"
          >
            {query.trim().length < 2 ? (
              <p className="px-2 py-6 text-center text-sm text-muted-foreground">{t.hint}</p>
            ) : results.length === 0 ? (
              <p className="px-2 py-6 text-center text-sm text-muted-foreground">{t.empty}</p>
            ) : (
              results.map((hit, i) => (
                <button
                  key={hit.url}
                  id={`search-opt-${i}`}
                  role="option"
                  aria-selected={i === selected}
                  onClick={() => go(hit.url)}
                  onMouseEnter={() => setSelected(i)}
                  className={cn(
                    "block w-full rounded-md px-3 py-2 text-left transition-colors",
                    i === selected ? "bg-muted" : "hover:bg-muted/60",
                  )}
                >
                  <span className="flex items-center gap-2">
                    <span
                      className={cn(
                        "h-1.5 w-1.5 shrink-0 rounded-full",
                        KIND_DOT[hit.kind as ResearchKind] ?? "bg-stone-400",
                      )}
                      aria-hidden
                    />
                    <span className="truncate text-sm font-medium">{hit.title}</span>
                    <span className="ml-auto shrink-0 text-[10px] uppercase tracking-wide text-muted-foreground">
                      {kindLabel(hit.kind as ResearchKind, lang)}
                    </span>
                  </span>
                  <span className="mt-0.5 line-clamp-2 block text-xs leading-relaxed text-muted-foreground">
                    {snippet(hit, query)}
                  </span>
                </button>
              ))
            )}
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}
