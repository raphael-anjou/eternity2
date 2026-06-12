import { Suspense, lazy, useEffect, useState } from "react";
import { NavLink, Route, Routes, useLocation } from "react-router-dom";
import { MotifDefs } from "@/components/board/MotifDefs";
import { initEngine } from "@/engine";
import { useLang, useT } from "@/i18n";
import { cn } from "@/lib/utils";

const Home = lazy(() => import("@/pages/Home"));
const PuzzlePage = lazy(() => import("@/pages/Puzzle"));
const PlaygroundHub = lazy(() => import("@/pages/playground/Hub"));
const Watch = lazy(() => import("@/pages/playground/Watch"));
const Solve = lazy(() => import("@/pages/playground/Solve"));
const Paths = lazy(() => import("@/pages/playground/Paths"));
const Algorithms = lazy(() => import("@/pages/Algorithms"));
const Research = lazy(() => import("@/pages/Research"));
const Viewer = lazy(() => import("@/pages/Viewer"));

const T = {
  en: {
    nav: [
      { to: "/", label: "Home" },
      { to: "/puzzle", label: "The Puzzle" },
      { to: "/playground", label: "Playground" },
      { to: "/algorithms", label: "Algorithms" },
      { to: "/viewer", label: "Board Viewer" },
      { to: "/research", label: "Research" },
    ],
    engineReady: "engine ready",
    engineLoading: "loading engine",
    engineFailed: "engine failed",
    engineTitle: "Rust engine compiled to WebAssembly, running in your browser",
    footer:
      "Open source (MIT, motif artwork GPL-3.0). Everything runs in your browser. There is no server.",
    builtBy: "Built by Raphaël Anjou.",
  },
  fr: {
    nav: [
      { to: "/", label: "Accueil" },
      { to: "/puzzle", label: "Le Puzzle" },
      { to: "/playground", label: "Aire de jeu" },
      { to: "/algorithms", label: "Algorithmes" },
      { to: "/viewer", label: "Visualiseur" },
      { to: "/research", label: "Recherche" },
    ],
    engineReady: "moteur prêt",
    engineLoading: "chargement du moteur",
    engineFailed: "échec du moteur",
    engineTitle: "Moteur Rust compilé en WebAssembly, qui tourne dans votre navigateur",
    footer:
      "Open source (MIT, motifs graphiques GPL-3.0). Tout tourne dans votre navigateur. Il n'y a aucun serveur.",
    builtBy: "Créé par Raphaël Anjou.",
  },
};

function ScrollToTop() {
  const { pathname } = useLocation();
  useEffect(() => {
    window.scrollTo(0, 0);
  }, [pathname]);
  return null;
}

export default function App() {
  const [engineReady, setEngineReady] = useState(false);
  const [engineError, setEngineError] = useState<string | null>(null);
  const { lang, setLang } = useLang();
  const t = useT(T);

  useEffect(() => {
    initEngine()
      .then(() => setEngineReady(true))
      .catch((e) => setEngineError(String(e)));
  }, []);

  return (
    <div className="min-h-screen bg-background text-foreground">
      <ScrollToTop />
      <MotifDefs />
      <header className="sticky top-0 z-40 border-b bg-background/90 backdrop-blur">
        <div className="mx-auto flex max-w-6xl items-center gap-3 px-3 py-3 sm:gap-6 sm:px-4">
          <NavLink to="/" className="flex shrink-0 items-center gap-2 font-bold tracking-tight">
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
          <nav className="flex flex-1 items-center gap-1 overflow-x-auto [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
            {t.nav.map((item) => (
              <NavLink
                key={item.to}
                to={item.to}
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
          <span
            className={cn(
              "hidden items-center gap-1.5 text-xs sm:flex",
              engineReady ? "text-emerald-600" : "text-muted-foreground",
            )}
            title={t.engineTitle}
          >
            <span
              className={cn(
                "inline-block h-2 w-2 rounded-full",
                engineReady ? "bg-emerald-500" : engineError ? "bg-red-500" : "bg-amber-400 animate-pulse",
              )}
            />
            {engineError ? t.engineFailed : engineReady ? t.engineReady : t.engineLoading}
          </span>
          <button
            onClick={() => setLang(lang === "en" ? "fr" : "en")}
            className="shrink-0 rounded-md border px-2 py-1 text-xs font-semibold text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
            title={lang === "en" ? "Passer en français" : "Switch to English"}
          >
            {lang === "en" ? "FR" : "EN"}
          </button>
        </div>
      </header>

      <main className="mx-auto max-w-6xl px-4 py-8">
        <Suspense
          fallback={<div className="py-24 text-center text-muted-foreground">Loading…</div>}
        >
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/puzzle" element={<PuzzlePage />} />
            <Route path="/playground" element={<PlaygroundHub />} />
            <Route path="/playground/watch" element={<Watch />} />
            <Route path="/playground/solve" element={<Solve />} />
            <Route path="/playground/paths" element={<Paths />} />
            <Route path="/algorithms" element={<Algorithms />} />
            <Route path="/research" element={<Research />} />
            <Route path="/viewer" element={<Viewer />} />
          </Routes>
        </Suspense>
      </main>

      <footer className="space-y-1 border-t py-8 text-center text-sm text-muted-foreground">
        <p>{t.footer}</p>
        <p>{t.builtBy}</p>
      </footer>
    </div>
  );
}
