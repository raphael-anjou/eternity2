import { pageMeta } from "@/seo";
// Watch a real DFS run live, in your browser. The Rust/WASM solver is
// stepped from an animation loop; speed controls how many solver steps
// (placements + backtracks) run per second.

import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useEngine } from "@/engine/useEngine";
import {
  createSolver,
  getGeneratedPuzzleFramed,
  getMaxColors,
  getOfficialPuzzle,
  getPath,
  getPathKinds,
} from "@/engine";
import type { SolverHandle } from "@/engine";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { useT } from "@/i18n";
import type { Puzzle, SolverReport } from "@/lib/types";
import { boardFromEngine } from "@/lib/bucas";
import { formatCompact, formatInt, formatSeconds } from "@/lib/format";

const SIZES = [3, 4, 5, 6, 7, 8, 10, 12, 14, 16];

const T = {
  en: {
    title: "Watch the machine think",
    intro:
      "This is a real depth-first search running in your browser. Watch it place pieces, hit dead ends, and backtrack. Crank the speed to see why even millions of nodes per second are not enough for the 16×16 puzzle.",
    buildingSolver: "Building solver…",
    loadingEngine: "Loading WASM engine…",
    puzzleCard: "Puzzle",
    official: "Official Eternity II (16×16)",
    sizeLabel: (s: number) => `Size: ${s}×${s}`,
    colorsLabel: (n: number) => `Colors: ${n}`,
    framed: "Frame-restricted colors",
    framedHint:
      "Like the real Eternity II: confine some colors to the border band, the rest to the deep interior (needs size ≥ 4).",
    seedLabel: (n: number) => `Seed: ${n}`,
    newPuzzle: "New puzzle",
    searchPath: "Search path",
    runCard: "Run",
    speedFull: "Speed: everything your machine can do",
    speedLabel: (steps: string) => `Speed: ${steps} steps/s`,
    fullSpeed: "Full speed",
    pause: "Pause",
    run: "Run",
    step: "Step",
    reset: "Reset",
    showBest: "Show deepest board reached",
    liveStats: "Live stats",
    solved: "SOLVED",
    noSolution: "no solution",
    placed: "Placed",
    deepestEver: "Deepest ever",
    placements: "Nodes",
    placementsTip:
      "One node = one piece placed (the community's standard solver metric).",
    undoneNote: (n: string) =>
      `${n} of those placements were later undone. A hard search un-builds almost everything it builds; what survives is the "Placed" count.`,
    nodesPerSecond: "Nodes / s",
    elapsed: "Run time",
    boardCompletion: "Board completion",
    whyNotFast: (
      <>
        Notice the node counter races while the depth bar barely moves? That's the whole story:
        on the 16×16 puzzle, no reachable speed is enough. Raw speed only divides the work by a
        constant; what actually moves the record is <em>pruning</em> the search.{" "}
        <Link className="underline" to="/research/why/prune-vs-speed">
          Why pruning beats speed →
        </Link>
      </>
    ),
  },
  fr: {
    title: "Regardez la machine réfléchir",
    intro:
      "Voici une vraie recherche en profondeur (DFS), en direct dans votre navigateur. Regardez-la poser des pièces, buter sur des impasses et revenir en arrière. Poussez la vitesse à fond : vous verrez pourquoi même des millions de nœuds par seconde ne suffisent pas face au puzzle 16×16.",
    buildingSolver: "Préparation du solveur…",
    loadingEngine: "Chargement du moteur WASM…",
    puzzleCard: "Puzzle",
    official: "Eternity II officiel (16×16)",
    sizeLabel: (s: number) => `Taille : ${s}×${s}`,
    colorsLabel: (n: number) => `Couleurs : ${n}`,
    framed: "Couleurs réservées au cadre",
    framedHint:
      "Comme le vrai Eternity II : confine certaines couleurs à la bande de bordure, les autres à l'intérieur profond (nécessite taille ≥ 4).",
    seedLabel: (n: number) => `Graine : ${n}`,
    newPuzzle: "Nouveau tirage",
    searchPath: "Parcours",
    runCard: "Exécution",
    speedFull: "Vitesse : tout ce dont votre machine est capable",
    speedLabel: (steps: string) => `Vitesse : ${steps} étapes/s`,
    fullSpeed: "Pleine vitesse",
    pause: "Pause",
    run: "Lecture",
    step: "Pas à pas",
    reset: "Réinitialiser",
    showBest: "Montrer le plateau le plus rempli atteint",
    liveStats: "Statistiques en direct",
    solved: "RÉSOLU",
    noSolution: "aucune solution",
    placed: "Placées",
    deepestEver: "Meilleur résultat",
    placements: "Nœuds",
    placementsTip:
      "Un nœud = une pièce posée (la mesure de référence des solveurs dans la communauté).",
    undoneNote: (n: string) =>
      `Sur ces placements, ${n} ont fini par être annulés. Une recherche difficile défait presque tout ce qu'elle construit ; ne subsiste que le compteur « Placées ».`,
    nodesPerSecond: "Nœuds / s",
    elapsed: "Temps écoulé",
    boardCompletion: "Remplissage du plateau",
    whyNotFast: (
      <>
        Vous voyez le compteur de nœuds s'emballer pendant que la barre de profondeur bouge à
        peine ? Tout est là : sur le puzzle 16×16, aucune vitesse atteignable ne suffit. La
        vitesse brute ne fait que diviser le travail par une constante ; ce qui fait réellement
        avancer le record, c'est l'<em>élagage</em> de la recherche.{" "}
        <Link className="underline" to="/research/why/prune-vs-speed">
          Pourquoi élaguer l'emporte sur la vitesse →
        </Link>
      </>
    ),
  },
  es: {
    title: "Observa pensar a la máquina",
    intro:
      "Esta es una verdadera búsqueda en profundidad ejecutándose en tu navegador. Míra cómo coloca piezas, choca con callejones sin salida y vuelve atrás. Sube la velocidad al máximo para ver por qué ni siquiera millones de nodos por segundo bastan frente al puzzle de 16×16.",
    buildingSolver: "Construyendo el solucionador…",
    loadingEngine: "Cargando el motor WASM…",
    puzzleCard: "Puzzle",
    official: "Eternity II oficial (16×16)",
    sizeLabel: (s: number) => `Tamaño: ${s}×${s}`,
    colorsLabel: (n: number) => `Colores: ${n}`,
    framed: "Colores reservados al marco",
    framedHint:
      "Como el Eternity II real: confina algunos colores a la banda del borde y el resto al interior profundo (requiere tamaño ≥ 4).",
    seedLabel: (n: number) => `Semilla: ${n}`,
    newPuzzle: "Nuevo puzzle",
    searchPath: "Orden de recorrido",
    runCard: "Ejecución",
    speedFull: "Velocidad: todo lo que tu máquina es capaz de dar",
    speedLabel: (steps: string) => `Velocidad: ${steps} pasos/s`,
    fullSpeed: "Máxima velocidad",
    pause: "Pausar",
    run: "Ejecutar",
    step: "Paso a paso",
    reset: "Reiniciar",
    showBest: "Mostrar el tablero más lleno alcanzado",
    liveStats: "Estadísticas en vivo",
    solved: "RESUELTO",
    noSolution: "sin solución",
    placed: "Colocadas",
    deepestEver: "Mejor resultado",
    placements: "Nodos",
    placementsTip:
      "Un nodo = una pieza colocada (la métrica de referencia de los solucionadores en la comunidad).",
    undoneNote: (n: string) =>
      `De esas colocaciones, ${n} acabaron deshaciéndose. Una búsqueda difícil desmonta casi todo lo que construye; lo único que perdura es el recuento de «Colocadas».`,
    nodesPerSecond: "Nodos / s",
    elapsed: "Tiempo transcurrido",
    boardCompletion: "Llenado del tablero",
    whyNotFast: (
      <>
        ¿Ves cómo el contador de nodos se dispara mientras la barra de profundidad apenas se
        mueve? Ahí está toda la historia: en el puzzle de 16×16, ninguna velocidad alcanzable
        basta. La velocidad bruta solo divide el trabajo por una constante; lo que de verdad hace
        avanzar el récord es la <em>poda</em> de la búsqueda.{" "}
        <Link className="underline" to="/research/why/prune-vs-speed">
          Por qué podar gana a la velocidad →
        </Link>
      </>
    ),
  },
};

export default function Watch() {
  const t = useT(T);
  const engineReady = useEngine();

  const [official, setOfficial] = useState(false);
  const [size, setSize] = useState(6);
  const [colors, setColors] = useState(6);
  // Frame-restricted colours: like the real Eternity II, confine some colours to
  // the border band and the rest to the deep interior (needs size ≥ 4).
  const [framed, setFramed] = useState(false);
  const [seed, setSeed] = useState(1);
  const [pathKind, setPathKind] = useState("row-major");
  const [speedExp, setSpeedExp] = useState(2); // 10^x steps per second
  const [fullSpeed, setFullSpeed] = useState(false);
  const [running, setRunning] = useState(false);
  const [showBest, setShowBest] = useState(false);

  const [report, setReport] = useState<SolverReport | null>(null);
  const [boardCells, setBoardCells] = useState<Int32Array | null>(null);
  const [measuredRate, setMeasuredRate] = useState(0);
  const [elapsedMs, setElapsedMs] = useState(0);
  // The active puzzle is render state, not a ref: `rebuild` swaps it together
  // with the board/report it produced, and the JSX reads it every render.
  const [puzzle, setPuzzle] = useState<Puzzle | null>(null);
  const elapsedRef = useRef(0);

  const solverRef = useRef<SolverHandle | null>(null);
  const rafRef = useRef(0);
  const lastTickRef = useRef(0);
  // Pause the solver loop while the board is scrolled offscreen: it keeps
  // stepping WASM and re-rendering otherwise (background *tabs* already pause
  // rAF, but same-tab-offscreen does not). The run resumes on scroll-back.
  const { ref: boardRef, visible: boardVisible } = useRunWhileVisible();

  const stepsPerSecond = Math.round(10 ** speedExp);

  const rebuild = useCallback(() => {
    if (!engineReady) return;
    solverRef.current?.free();
    const puzzle = official
      ? getOfficialPuzzle()
      : getGeneratedPuzzleFramed(size, colors, seed, framed);
    const width = puzzle.width;
    const path = getPath(pathKind, width, puzzle.height, seed);
    const solver = createSolver(puzzle, path, { useHints: true });
    setPuzzle(puzzle);
    solverRef.current = solver;
    setReport(solver.report());
    setBoardCells(solver.board());
    setRunning(false);
    elapsedRef.current = 0;
    setElapsedMs(0);
  }, [engineReady, official, size, colors, seed, framed, pathKind]);

  useEffect(() => {
    // rebuild() constructs the WASM solver (an external system) and seeds the
    // board/report state from it — a legitimate effect, not a render cascade.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    rebuild();
    return () => {
      solverRef.current?.free();
      solverRef.current = null;
    };
  }, [rebuild]);

  useEffect(() => {
    if (!running || !boardVisible) return;
    lastTickRef.current = performance.now();
    let nodesAtLast = report?.nodes ?? 0;
    let lastRateUpdate = performance.now();
    let stepDebt = 0; // fractional steps carried between frames (for slow speeds)

    const tick = () => {
      const solver = solverRef.current;
      if (!solver) return;
      const now = performance.now();
      const dt = Math.min(now - lastTickRef.current, 250) / 1000;
      lastTickRef.current = now;
      elapsedRef.current += dt * 1000;
      setElapsedMs(elapsedRef.current);
      stepDebt += stepsPerSecond * dt;

      // Run in small chunks inside a ~12ms frame budget so huge speeds
      // degrade gracefully instead of freezing the tab. "Full speed" ignores
      // the slider and simply uses the whole frame budget.
      let r = solver.report();
      const frameStart = performance.now();
      while ((fullSpeed || stepDebt >= 1) && performance.now() - frameStart < 12) {
        const chunk = fullSpeed ? 10_000 : Math.min(Math.floor(stepDebt), 5000);
        r = solver.step(chunk);
        stepDebt -= chunk;
        if (r.status !== "running") break;
      }

      // Don't bank more than ~1s of unmet work when the machine can't keep up.
      stepDebt = Math.max(0, Math.min(stepDebt, stepsPerSecond));

      setReport(r);
      setBoardCells(showBest ? solver.bestBoard() : solver.board());
      if (now - lastRateUpdate > 500) {
        setMeasuredRate(((r.nodes - nodesAtLast) * 1000) / (now - lastRateUpdate));
        nodesAtLast = r.nodes;
        lastRateUpdate = now;
      }
      if (r.status !== "running") {
        setRunning(false);
        return;
      }
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [running, boardVisible, stepsPerSecond, showBest, fullSpeed]);

  const cells = useMemo(() => {
    if (!puzzle || !boardCells) return null;
    return boardFromEngine(puzzle, boardCells).cells;
  }, [puzzle, boardCells]);

  const totalCells = puzzle ? puzzle.width * puzzle.height : 0;
  // "Placed" and the completion bar track the displayed board: the live one,
  // or the deepest-ever snapshot when that toggle is on.
  const shownPlaced = report ? (showBest ? report.bestPlaced : report.placed) : 0;
  const maxColorsForSize = engineReady && !official ? getMaxColors(size) : 22;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-1 text-muted-foreground">{t.intro}</p>
      </div>

      <div className="grid gap-6 lg:grid-cols-[1fr_320px]">
        <div>
          {cells && puzzle ? (
            <div ref={boardRef} className="max-w-3xl space-y-2">
              <BoardSvg
                width={puzzle.width}
                height={puzzle.height}
                cells={cells}
                highlight={official ? puzzle.hints.map((h) => h.pos) : undefined}
              />
            </div>
          ) : (
            <div className="flex aspect-square max-w-3xl items-center justify-center rounded-lg border text-muted-foreground">
              {engineReady ? t.buildingSolver : t.loadingEngine}
            </div>
          )}
        </div>

        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-base">{t.puzzleCard}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <Label htmlFor="w-official">{t.official}</Label>
                <Switch id="w-official" checked={official} onCheckedChange={setOfficial} />
              </div>
              {!official && (
                <>
                  <div className="space-y-1.5">
                    <Label>{t.sizeLabel(size)}</Label>
                    <Select value={String(size)} onValueChange={(v) => {
                      if (!v) return;
                      const s = parseInt(v, 10);
                      setSize(s);
                      setColors((c) => Math.min(c, getMaxColors(s)));
                    }}>
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {SIZES.map((s) => (
                          <SelectItem key={s} value={String(s)}>
                            {s}×{s}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                  <div className="space-y-1.5">
                    <Label>{t.colorsLabel(colors)}</Label>
                    <Slider
                      aria-label={t.colorsLabel(colors)}
                      min={2}
                      max={maxColorsForSize}
                      step={1}
                      value={colors}
                      onValueChange={(v) => setColors(singleSliderValue(v))}
                    />
                  </div>
                  <div className="space-y-1">
                    <div className="flex items-center justify-between">
                      <Label htmlFor="w-framed">{t.framed}</Label>
                      <Switch
                        id="w-framed"
                        checked={framed}
                        disabled={size < 4 || colors < 2}
                        onCheckedChange={setFramed}
                      />
                    </div>
                    <p className="text-xs text-muted-foreground">{t.framedHint}</p>
                  </div>
                  <div className="flex items-center gap-2">
                    <Label className="flex-1">{t.seedLabel(seed)}</Label>
                    <Button variant="outline" size="sm" onClick={() => setSeed(Math.floor(Math.random() * 100000))}>
                      {t.newPuzzle}
                    </Button>
                  </div>
                </>
              )}
              <div className="space-y-1.5">
                <Label>{t.searchPath}</Label>
                <Select value={pathKind} onValueChange={(v) => v && setPathKind(v)}>
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {(engineReady ? getPathKinds() : ["row-major"]).map((k) => (
                      <SelectItem key={k} value={k}>
                        {k}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="text-base">{t.runCard}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-1.5">
                <div className="flex items-center justify-between">
                  <Label>
                    {fullSpeed ? t.speedFull : t.speedLabel(formatCompact(stepsPerSecond))}
                  </Label>
                  <Button
                    variant={fullSpeed ? "default" : "outline"}
                    size="xs"
                    aria-pressed={fullSpeed}
                    onClick={() => setFullSpeed((f) => !f)}
                  >
                    {t.fullSpeed}
                  </Button>
                </div>
                <Slider
                  aria-label={t.speedLabel(formatCompact(stepsPerSecond))}
                  min={0}
                  max={6}
                  step={0.1}
                  value={speedExp}
                  disabled={fullSpeed}
                  onValueChange={(v) => setSpeedExp(singleSliderValue(v))}
                />
              </div>
              <div className="flex gap-2">
                <Button
                  className="flex-1"
                  onClick={() => setRunning((r) => !r)}
                  disabled={!engineReady || report?.status !== "running"}
                >
                  {running ? t.pause : t.run}
                </Button>
                <Button
                  variant="outline"
                  onClick={() => {
                    const s = solverRef.current;
                    if (!s) return;
                    setReport(s.step(1));
                    setBoardCells(showBest ? s.bestBoard() : s.board());
                  }}
                  disabled={!engineReady || running || report?.status !== "running"}
                >
                  {t.step}
                </Button>
                <Button variant="outline" onClick={rebuild} disabled={!engineReady}>
                  {t.reset}
                </Button>
              </div>
              <div className="flex items-center justify-between">
                <Label htmlFor="w-best">{t.showBest}</Label>
                <Switch id="w-best" checked={showBest} onCheckedChange={setShowBest} />
              </div>
            </CardContent>
          </Card>

          {report && (
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center justify-between text-base">
                  {t.liveStats}
                  {report.status === "solved" && <Badge className="bg-emerald-600">{t.solved}</Badge>}
                  {report.status === "exhausted" && <Badge variant="destructive">{t.noSolution}</Badge>}
                </CardTitle>
              </CardHeader>
              <CardContent>
                <dl className="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
                  <dt className="text-muted-foreground">{t.placed}</dt>
                  <dd className="text-right font-mono">
                    {shownPlaced} / {totalCells}
                  </dd>
                  <dt className="text-muted-foreground">{t.deepestEver}</dt>
                  <dd className="text-right font-mono">{report.bestPlaced}</dd>
                  <dt className="text-muted-foreground" title={t.placementsTip}>
                    {t.placements}
                  </dt>
                  <dd className="text-right font-mono">{formatInt(report.nodes)}</dd>
                  <dt className="text-muted-foreground">{t.elapsed}</dt>
                  <dd className="text-right font-mono">{formatSeconds(elapsedMs / 1000)}</dd>
                  {running && (
                    <>
                      <dt className="text-muted-foreground">{t.nodesPerSecond}</dt>
                      <dd className="text-right font-mono">{formatCompact(measuredRate)}</dd>
                    </>
                  )}
                </dl>
                <div className="mt-3">
                  <div className="mb-1 flex justify-between text-xs text-muted-foreground">
                    <span>{t.boardCompletion}</span>
                    <span>{totalCells ? Math.round((shownPlaced / totalCells) * 100) : 0}%</span>
                  </div>
                  <div className="h-2 overflow-hidden rounded bg-muted">
                    <div
                      className="h-full bg-primary transition-[width]"
                      style={{ width: `${totalCells ? (shownPlaced / totalCells) * 100 : 0}%` }}
                    />
                  </div>
                </div>
                {report.backtracks > 0 && (
                  <p className="mt-3 text-xs text-muted-foreground">
                    {t.undoneNote(formatInt(report.backtracks))}
                  </p>
                )}
              </CardContent>
            </Card>
          )}
        </div>
      </div>

      <p className="max-w-3xl border-t pt-4 text-sm text-muted-foreground">{t.whyNotFast}</p>
    </div>
  );
}

export const meta = pageMeta("watch");
