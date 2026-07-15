import { useEffect, useState } from "react";
import { Navigate, NavLink, Outlet, useLocation } from "react-router";
import { MotifDefs } from "@/components/board/MotifDefs";
import { FeedbackButton } from "@/components/FeedbackButton";
import { initEngine } from "@/engine";
import { useLang, useT, pathForLang, preferredLang, type Lang } from "@/i18n";
import { cn } from "@/lib/utils";
import { loadAnalyticsWhenIdle } from "@/lib/analytics";

const T = {
  en: {
    nav: [
      { to: "/", label: "Home" },
      { to: "/start", label: "Start here" },
      { to: "/playground", label: "Playground" },
      { to: "/algorithms", label: "Algorithms" },
      { to: "/viewer", label: "Board Viewer" },
      { to: "/research", label: "Research" },
    ],
    // A flat footer link row. Its main SEO job is to give the two "money"
    // pages that aren't in the header nav — /status and /is-it-a-scam — a
    // site-wide internal link (they were previously reachable only from a
    // couple of in-body links), and to spread link equity to the cornerstone
    // pages from every page on the site.
    footerNav: [
      { to: "/puzzle", label: "The puzzle" },
      { to: "/status", label: "Has it been solved?" },
      { to: "/is-it-a-scam", label: "Is it a scam?" },
      { to: "/algorithms", label: "Algorithms" },
      { to: "/playground", label: "Playground" },
      { to: "/viewer", label: "Board Viewer" },
      { to: "/convert", label: "Format converter" },
      { to: "/research", label: "Research" },
    ],
    footer:
      "Open source (MIT). Piece motifs by Jef Bucas (GPL-3.0). Everything runs in your browser. There is no server.",
    builtBy: "Built by Raphaël Anjou.",
    sourceCode: "Source code on GitHub",
  },
  fr: {
    nav: [
      { to: "/", label: "Accueil" },
      { to: "/start", label: "Par où commencer" },
      { to: "/playground", label: "Aire de jeu" },
      { to: "/algorithms", label: "Algorithmes" },
      { to: "/viewer", label: "Visualiseur" },
      { to: "/research", label: "Recherche" },
    ],
    footerNav: [
      { to: "/puzzle", label: "Le puzzle" },
      { to: "/status", label: "A-t-il été résolu ?" },
      { to: "/is-it-a-scam", label: "Une arnaque ?" },
      { to: "/algorithms", label: "Algorithmes" },
      { to: "/playground", label: "Aire de jeu" },
      { to: "/viewer", label: "Visualiseur" },
      { to: "/convert", label: "Convertisseur" },
      { to: "/research", label: "Recherche" },
    ],
    footer:
      "Logiciel libre (MIT). Motifs des pièces par Jef Bucas (GPL-3.0). Tout se passe dans votre navigateur : aucun serveur n'est sollicité.",
    builtBy: "Réalisé par Raphaël Anjou.",
    sourceCode: "Code source sur GitHub",
  },
};

// GA4 page views per route (the root.tsx config disables automatic ones).
function PageTracking() {
  const { pathname } = useLocation();
  useEffect(() => {
    const gtag = (window as unknown as { gtag?: (...args: unknown[]) => void }).gtag;
    gtag?.("event", "page_view", {
      page_path: pathname,
      page_location: window.location.href,
      page_title: document.title,
    });
  }, [pathname]);
  return null;
}

// On a first visit to the bare English root "/", send French visitors to /fr
// (honoring an earlier explicit choice or the browser language). This must run
// SYNCHRONOUSLY during render — not in a post-paint effect — because an effect
// can fire *after* the user has already clicked a nav link, and would then
// redirect away from the page they navigated to (the "click The Puzzle, flicker,
// bounce to home" bug). Rendering <Navigate> reacts to the live location, so a
// real navigation away from "/" simply makes this component render null.
function FirstVisitRedirect() {
  const { pathname, search } = useLocation();
  // Only the exact bare root with no query is eligible; any other path already
  // carries its own language/segments and is left untouched.
  const eligible = pathname === "/" && search.length === 0;
  if (eligible && preferredLang() === "fr") {
    return <Navigate to="/fr" replace />;
  }
  return null;
}

export default function Layout() {
  const [menuOpen, setMenuOpen] = useState(false);
  const { lang, setLang } = useLang();
  const t = useT(T);
  const { pathname } = useLocation();
  // The research wiki is English-only, so the language toggle has nothing to
  // switch to there — hide it on /research (and the /fr/research redirect).
  const onResearch = /^\/(fr\/)?research(\/|$)/.test(pathname);

  useEffect(() => {
    // Warm the WebAssembly engine in the background so interactive pages
    // (playground, solver, convert) don't pay the compile cost on first use.
    void initEngine().catch(() => {});
    // Load analytics once idle, keeping it off the critical render path.
    return loadAnalyticsWhenIdle();
  }, []);

  // Localize a nav target to the active language (/puzzle ↔ /fr/puzzle).
  const link = (to: string) => pathForLang(to, lang);
  const other: Lang = lang === "en" ? "fr" : "en";

  return (
    <div className="min-h-screen bg-background text-foreground">
      <PageTracking />
      <FirstVisitRedirect />
      <MotifDefs />
      <header className="sticky top-0 z-40 border-b bg-background/90 backdrop-blur">
        <div className="mx-auto flex max-w-6xl items-center gap-3 px-3 py-3 sm:gap-6 sm:px-4">
          <NavLink to={link("/")} className="flex shrink-0 items-center gap-2 font-bold tracking-tight">
            <svg viewBox="-130 -130 260 260" width="28" height="28" aria-hidden>
              <use href="#e2m-9" transform="rotate(90)" />
              <use href="#e2m-12" transform="rotate(180)" />
              <use href="#e2m-5" transform="rotate(-90)" />
              <use href="#e2m-17" />
            </svg>
            <span>
              Eternity&nbsp;II
              <span className="hidden text-muted-foreground md:inline"> · community</span>
            </span>
          </NavLink>
          <nav className="hidden flex-1 items-center gap-1 md:flex">
            {t.nav.map((item) => (
              <NavLink
                key={item.to}
                to={link(item.to)}
                end={item.to === "/"}
                className={({ isActive }) =>
                  cn(
                    "rounded-md px-3 py-1.5 text-sm font-medium whitespace-nowrap transition-colors",
                    isActive
                      ? "bg-primary text-primary-foreground"
                      : "text-muted-foreground hover:bg-muted hover:text-foreground",
                  )
                }
              >
                {item.label}
              </NavLink>
            ))}
          </nav>
          <div className="flex-1 md:hidden" />
          {!onResearch && (
            <button
              onClick={() => setLang(other)}
              className="shrink-0 rounded-md border px-2 py-1 text-xs font-semibold text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
              title={lang === "en" ? "Passer en français" : "Switch to English"}
            >
              {lang === "en" ? "FR" : "EN"}
            </button>
          )}
          <button
            onClick={() => setMenuOpen((o) => !o)}
            className="shrink-0 rounded-md border px-2.5 py-1 text-base leading-none md:hidden"
            aria-label={menuOpen ? "Close menu" : "Open menu"}
            aria-expanded={menuOpen}
          >
            {menuOpen ? "✕" : "☰"}
          </button>
        </div>
        {menuOpen && (
          <nav className="border-t md:hidden">
            <div className="mx-auto flex max-w-6xl flex-col gap-1 px-3 py-2">
              {t.nav.map((item) => (
                <NavLink
                  key={item.to}
                  to={link(item.to)}
                  end={item.to === "/"}
                  onClick={() => setMenuOpen(false)}
                  className={({ isActive }) =>
                    cn(
                      "rounded-md px-3 py-2 text-sm font-medium transition-colors",
                      isActive
                        ? "bg-primary text-primary-foreground"
                        : "text-muted-foreground hover:bg-muted hover:text-foreground",
                    )
                  }
                >
                  {item.label}
                </NavLink>
              ))}
            </div>
          </nav>
        )}
      </header>

      <main className="mx-auto max-w-6xl px-4 py-8">
        <Outlet />
      </main>

      <footer className="space-y-4 border-t py-8 text-center text-sm text-muted-foreground">
        <nav className="mx-auto flex max-w-3xl flex-wrap justify-center gap-x-4 gap-y-2">
          {t.footerNav.map((item) => (
            <NavLink key={item.to} to={link(item.to)} className="hover:text-foreground hover:underline">
              {item.label}
            </NavLink>
          ))}
        </nav>
        <p>{t.footer}</p>
        <p>
          {t.builtBy}{" "}
          <a
            className="underline hover:text-foreground"
            href="https://github.com/raphael-anjou/eternity2"
            target="_blank"
            rel="noreferrer"
          >
            {t.sourceCode}
          </a>
        </p>
      </footer>

      <FeedbackButton />
    </div>
  );
}
