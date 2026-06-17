import { pageMeta } from "@/seo";
import { useEffect, useMemo, useRef, useState } from "react";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useEngine } from "@/engine/useEngine";
import { createSolver, getGeneratedPuzzle, getPath } from "@/engine";
import type { SolverHandle } from "@/engine";
import { useT } from "@/i18n";
import type { Puzzle } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import type { Edges } from "@/lib/bucas";

// A small board solving itself forever: the site's heartbeat.
function HeroBoard() {
  const engineReady = useEngine();
  const [cells, setCells] = useState<ReturnType<typeof boardFromEngine>["cells"] | null>(null);
  const solverRef = useRef<SolverHandle | null>(null);
  const puzzleRef = useRef<Puzzle | null>(null);
  const seedRef = useRef(1);

  useEffect(() => {
    if (!engineReady) return;
    let raf = 0;
    const make = () => {
      solverRef.current?.free();
      const puzzle = getGeneratedPuzzle(8, 8, seedRef.current++);
      puzzleRef.current = puzzle;
      solverRef.current = createSolver(puzzle, getPath("snake", 8, 8, 0), { useHints: false });
    };
    make();
    // Solve at full speed behind the scenes, but repaint gently (every
    // ~600ms) so the hero doesn't flicker and steal attention.
    let solvedAt = 0;
    let lastPaint = 0;
    const tick = (t: number) => {
      const solver = solverRef.current;
      const puzzle = puzzleRef.current;
      if (solver && puzzle) {
        const r = solver.report();
        if (r.status === "solved") {
          if (!solvedAt) {
            solvedAt = t;
            setCells(boardFromEngine(puzzle, solver.board()).cells);
          }
          if (t - solvedAt > 4000) {
            solvedAt = 0;
            make();
            lastPaint = 0;
          }
        } else {
          solver.step(1500);
          if (t - lastPaint > 450) {
            lastPaint = t;
            setCells(boardFromEngine(puzzle, solver.board()).cells);
          }
        }
      }
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => {
      cancelAnimationFrame(raf);
      solverRef.current?.free();
      solverRef.current = null;
    };
  }, [engineReady]);

  const placeholder = useMemo<(Edges | null)[]>(() => Array<Edges | null>(64).fill(null), []);
  return <BoardSvg width={8} height={8} cells={cells ?? placeholder} className="w-full" />;
}

const T = {
  en: {
    heroTitleTop: "The puzzle that beat",
    heroTitleAccent: "everyone.",
    heroLede: (
      <>
        Eternity II looks like a children's jigsaw: 256 square pieces, match the colors. Launched
        in 2007 with a <strong>$2 million</strong> prize, it has never been solved: not by people,
        not by supercomputers. This site lets you <em>feel</em> why, right in your browser.
      </>
    ),
    ctaPlayground: "Try the playground",
    ctaPuzzle: "What is Eternity II?",
    heroCaption:
      "← that board is a live solver, restarting on a fresh puzzle each time it wins. The 16×16 one would not finish before the stars burn out.",
    cards: [
      {
        to: "/puzzle",
        title: "The Puzzle",
        text: "256 pieces, 22 motifs, a $2,000,000 prize nobody ever claimed. The full story and anatomy of Eternity II.",
      },
      {
        to: "/playground",
        title: "Playground",
        text: "Solve a small one yourself, watch a real solver run at a million steps per second, or design its search path.",
      },
      {
        to: "/algorithms",
        title: "Algorithms",
        text: "Backtracking, binary tricks and exponential walls: how computer scientists actually attack the puzzle, explained from scratch.",
      },
      {
        to: "/viewer",
        title: "Board Viewer",
        text: "Display and share any board. Fully compatible with e2.bucas.name links, including the famous 467/469/470 record boards.",
      },
      {
        to: "/research",
        title: "Research",
        text: "Papers, community records, solver internals, and an open lab notebook of attempts at the full 480.",
      },
    ],
    classroomTitle: "For classrooms",
    classroomText: (
      <>
        Built for{" "}
        <a className="underline" href="https://terra-numerica.org" target="_blank" rel="noreferrer">
          Terra Numerica
        </a>
        : a hands-on way to discover combinatorial explosion, algorithms and why "simple-looking"
        can mean "impossibly hard". No prerequisites needed.
      </>
    ),
  },
  fr: {
    heroTitleTop: "Le puzzle qui a battu",
    heroTitleAccent: "tout le monde.",
    heroLede: (
      <>
        Eternity II a tout d'un puzzle pour enfants : 256 pièces carrées, et des couleurs à faire
        coïncider. Lancé en 2007, doté d'un prix de{" "}
        <strong>2 millions de dollars</strong>, il n'a pourtant jamais été résolu : ni par l'homme,
        ni par les superordinateurs. Ce site vous fait <em>toucher du doigt</em> pourquoi, sans rien
        installer.
      </>
    ),
    ctaPlayground: "Essayer l'aire de jeu",
    ctaPuzzle: "C'est quoi, Eternity II ?",
    heroCaption:
      "← ce plateau est un vrai solveur en action : à chaque victoire, il repart sur un puzzle tout neuf. En 16×16, il n'aurait pas terminé que les étoiles se seraient déjà éteintes.",
    cards: [
      {
        to: "/puzzle",
        title: "Le Puzzle",
        text: "256 pièces, 22 motifs et un prix de 2 millions de dollars jamais réclamé. Toute l'histoire et l'anatomie d'Eternity II.",
      },
      {
        to: "/playground",
        title: "Aire de jeu",
        text: "Résolvez un petit puzzle à la main, regardez un vrai solveur filer à un million d'étapes par seconde, ou tracez vous-même son parcours.",
      },
      {
        to: "/algorithms",
        title: "Algorithmes",
        text: "Retour en arrière (backtracking), astuces binaires et murs exponentiels : comment les informaticiens s'y prennent réellement face au puzzle. Tout est expliqué depuis le début.",
      },
      {
        to: "/viewer",
        title: "Visualiseur",
        text: "Affichez et partagez n'importe quel plateau. Pleinement compatible avec les liens e2.bucas.name, jusqu'aux célèbres plateaux records de 467, 469 et 470 côtés appariés.",
      },
      {
        to: "/research",
        title: "Recherche",
        text: "Articles scientifiques, records de la communauté, dessous techniques du solveur, et un carnet de laboratoire ouvert sur la quête des 480 côtés appariés.",
      },
    ],
    classroomTitle: "Pour les classes",
    classroomText: (
      <>
        Pensé pour{" "}
        <a className="underline" href="https://terra-numerica.org" target="_blank" rel="noreferrer">
          Terra Numerica
        </a>
        : une manière concrète d'aborder l'explosion combinatoire et les algorithmes, et de
        comprendre pourquoi un jeu « tout simple » peut se révéler « impossiblement difficile ».
        Aucun prérequis.
      </>
    ),
  },
};

export default function Home() {
  const t = useT(T);
  return (
    <div className="space-y-14">
      <section className="grid items-center gap-10 pt-6 md:grid-cols-2">
        <div className="space-y-5">
          <h1 className="text-4xl font-extrabold leading-tight tracking-tight md:text-5xl">
            {t.heroTitleTop}
            <br />
            <span className="bg-gradient-to-r from-pink-500 via-amber-500 to-sky-500 bg-clip-text text-transparent">
              {t.heroTitleAccent}
            </span>
          </h1>
          <p className="max-w-prose text-lg text-muted-foreground">{t.heroLede}</p>
          <div className="flex flex-wrap gap-3">
            <Button size="lg" render={<Link to="/playground" />}>
              {t.ctaPlayground}
            </Button>
            <Button size="lg" variant="outline" render={<Link to="/puzzle" />}>
              {t.ctaPuzzle}
            </Button>
          </div>
          <p className="text-sm text-muted-foreground">{t.heroCaption}</p>
        </div>
        <HeroBoard />
      </section>

      <section className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {t.cards.map((c) => (
          <Link key={c.to} to={c.to} className="group">
            <Card className="h-full transition-shadow group-hover:shadow-lg">
              <CardHeader>
                <CardTitle>{c.title}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{c.text}</CardContent>
            </Card>
          </Link>
        ))}
        <Card className="h-full border-dashed">
          <CardHeader>
            <CardTitle>{t.classroomTitle}</CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-muted-foreground">{t.classroomText}</CardContent>
        </Card>
      </section>
    </div>
  );
}

export const meta = pageMeta("home");
