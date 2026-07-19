import { useEffect, useState } from "react";
import { Navigate, NavLink, Outlet, useLocation } from "react-router";
import { MotifDefs } from "@/components/board/MotifDefs";
import { FeedbackButton } from "@/components/FeedbackButton";
import { useLang, useT, pathForLang, preferredLang, langDef, LANGS } from "@/i18n";
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
    forAgents: "For AI agents and crawlers:",
    llmsLabel: "llms.txt",
    llmsFullLabel: "full text",
    sitemapLabel: "sitemap",
    langPicker: "Choose language",
    skipToContent: "Skip to content",
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
    forAgents: "Pour les agents IA et robots :",
    llmsLabel: "llms.txt",
    llmsFullLabel: "texte complet",
    sitemapLabel: "plan du site",
    langPicker: "Choisir la langue",
    skipToContent: "Aller au contenu",
  },
  es: {
    nav: [
      { to: "/", label: "Inicio" },
      { to: "/start", label: "Por dónde empezar" },
      { to: "/playground", label: "Zona de pruebas" },
      { to: "/algorithms", label: "Algoritmos" },
      { to: "/viewer", label: "Visor de tableros" },
      { to: "/research", label: "Investigación" },
    ],
    footerNav: [
      { to: "/puzzle", label: "El puzzle" },
      { to: "/status", label: "¿Se ha resuelto?" },
      { to: "/is-it-a-scam", label: "¿Es una estafa?" },
      { to: "/algorithms", label: "Algoritmos" },
      { to: "/playground", label: "Zona de pruebas" },
      { to: "/viewer", label: "Visor de tableros" },
      { to: "/convert", label: "Conversor de formatos" },
      { to: "/research", label: "Investigación" },
    ],
    footer:
      "Código abierto (MIT). Motivos de las piezas por Jef Bucas (GPL-3.0). Todo se ejecuta en tu navegador. No hay servidor.",
    builtBy: "Creado por Raphaël Anjou.",
    sourceCode: "Código fuente en GitHub",
    forAgents: "Para agentes de IA y rastreadores:",
    llmsLabel: "llms.txt",
    llmsFullLabel: "texto completo",
    sitemapLabel: "mapa del sitio",
    langPicker: "Elegir idioma",
    skipToContent: "Saltar al contenido",
  },
};

// GA4 page views per route (the root.tsx config disables automatic ones).
function PageTracking() {
  const { pathname } = useLocation();
  useEffect(() => {
    window.gtag?.("event", "page_view", {
      page_path: pathname,
      page_location: window.location.href,
      page_title: document.title,
    });
  }, [pathname]);
  return null;
}

// On a first visit to the bare English root "/", send visitors whose preferred
// language is not English to that language's tree (/fr, /es, …), honoring an
// earlier explicit choice or the browser language. This must run SYNCHRONOUSLY
// during render — not in a post-paint effect — because an effect can fire
// *after* the user has already clicked a nav link, and would then redirect away
// from the page they navigated to (the "click The Puzzle, flicker, bounce to
// home" bug). Rendering <Navigate> reacts to the live location, so a real
// navigation away from "/" simply makes this component render null.
function FirstVisitRedirect() {
  const { pathname, search } = useLocation();
  // Only the exact bare root with no query is eligible; any other path already
  // carries its own language/segments and is left untouched.
  const eligible = pathname === "/" && search.length === 0;
  if (eligible) {
    const preferred = preferredLang();
    const def = langDef(preferred);
    if (def.prefix) return <Navigate to={`/${def.prefix}`} replace />;
  }
  return null;
}

// Routes that actually use the WebAssembly engine (piece matching / solving).
// The read-only pages — the whole research wiki, Home, Algorithms — never touch
// it, so they must not pay its download + compile cost. Convert is deliberately
// engine-free (it only reshuffles cells), so it is not listed. initEngine() is
// idempotent, so warming again on navigation into one of these is a no-op.
// Any language prefix may precede the engine routes (/puzzle, /fr/puzzle,
// /es/puzzle, …), so a translated app page warms the engine too. The prefix
// group is built from the registry so a new language is covered automatically.
const ENGINE_PREFIX = LANGS.map((l) => l.prefix)
  .filter(Boolean)
  .join("|");
const ENGINE_ROUTES = new RegExp(
  `^/((${ENGINE_PREFIX})/)?(puzzle|start|viewer|playground(/|$))`,
);

export default function Layout() {
  const [menuOpen, setMenuOpen] = useState(false);
  const { lang, setLang } = useLang();
  const t = useT(T);
  const { pathname } = useLocation();

  useEffect(() => {
    // Load analytics once idle, keeping it off the critical render path.
    return loadAnalyticsWhenIdle();
  }, []);

  useEffect(() => {
    // Warm the WebAssembly engine in the background, but only on the routes that
    // use it, so a research reader never downloads the solver for nothing. The
    // engine module is imported dynamically here (not at the top of the file) so
    // its .wasm asset is never even pulled into a read-only page's module graph.
    if (ENGINE_ROUTES.test(pathname)) {
      void import("@/engine").then((m) => m.initEngine()).catch(() => {});
    }
  }, [pathname]);

  // Localize a nav target to the active language (/puzzle ↔ /fr/puzzle).
  const link = (to: string) => pathForLang(to, lang);

  return (
    <div className="min-h-screen bg-background text-foreground">
      <PageTracking />
      <FirstVisitRedirect />
      <MotifDefs />
      {/* Skip link: the first focusable element, hidden until focused, so a
          keyboard/SR user can jump past the header nav + research sidebar
          straight to the page content (WCAG 2.4.1). */}
      <a
        href="#main"
        className="sr-only rounded-md border bg-background px-4 py-2 text-sm font-medium focus:not-sr-only focus:absolute focus:left-4 focus:top-4 focus:z-50 focus:ring-2 focus:ring-ring"
      >
        {t.skipToContent}
      </a>
      <header className="sticky top-0 z-40 border-b bg-background/90 backdrop-blur">
        <div className="mx-auto flex max-w-[90rem] items-center gap-3 px-3 py-3 sm:gap-6 sm:px-4">
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
          {/* Language picker: a segmented control, one segment per registry
              language, the active one highlighted. Switching navigates to the
              same page under the chosen language's URL (pathForLang). */}
          <div
            className="flex shrink-0 overflow-hidden rounded-md border text-xs font-semibold"
            role="group"
            aria-label={t.langPicker}
          >
            {LANGS.map((l) => {
              const active = l.code === lang;
              return (
                <button
                  key={l.code}
                  onClick={() => setLang(l.code)}
                  aria-current={active ? "true" : undefined}
                  title={l.native}
                  className={cn(
                    "px-2 py-1 transition-colors",
                    active
                      ? "bg-primary text-primary-foreground"
                      : "text-muted-foreground hover:bg-muted hover:text-foreground",
                  )}
                >
                  {l.label}
                </button>
              );
            })}
          </div>
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
            <div className="mx-auto flex max-w-[90rem] flex-col gap-1 px-3 py-2">
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

      <main id="main" tabIndex={-1} className="mx-auto max-w-[90rem] px-4 py-8 focus:outline-none">
        <Outlet />
      </main>

      <footer className="space-y-4 border-t py-8 text-center text-sm text-muted-foreground">
        {/* One line, all links: no wrap; on a viewport too narrow to fit the
            full row it scrolls horizontally rather than breaking onto a second
            line. Every link stays in the DOM, so the internal-link SEO job holds. */}
        <nav className="flex justify-center gap-x-4 overflow-x-auto px-4">
          {t.footerNav.map((item) => (
            <NavLink
              key={item.to}
              to={link(item.to)}
              className="whitespace-nowrap hover:text-foreground hover:underline"
            >
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
        {/* A visible, machine-followable pointer to the site's agent-readable
            index. These are static build-emitted files, not React routes, so
            they use plain anchors. */}
        <p className="text-xs">
          {t.forAgents}{" "}
          <a className="underline hover:text-foreground" href="/llms.txt">
            {t.llmsLabel}
          </a>
          {" · "}
          <a className="underline hover:text-foreground" href="/llms-full.txt">
            {t.llmsFullLabel}
          </a>
          {" · "}
          <a className="underline hover:text-foreground" href="/sitemap.xml">
            {t.sitemapLabel}
          </a>
        </p>
      </footer>

      <FeedbackButton />
    </div>
  );
}
