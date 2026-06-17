import { pageMeta } from "@/seo";
import { useEffect, useMemo, useRef, useState } from "react";
import { useSearchParams } from "react-router";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Slider, singleSliderValue } from "@/components/ui/slider";
import { Textarea } from "@/components/ui/textarea";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { ChevronDownIcon } from "lucide-react";
import { KNOWN_BOARDS } from "@/data/known-boards";
import {
  decodeBucas,
  parseParams,
  toOurParams,
  conflictEdges,
  scoreSummary,
  matchToPieces,
  boardFromEngine,
  ourParams,
  encodeBucasUrl,
} from "@/lib/bucas";
import type { BucasBoard } from "@/lib/bucas";
import { getOfficialPuzzle, getGeneratedSolvedPuzzle, getMaxColors } from "@/engine";
import { useEngine } from "@/engine/useEngine";
import { useT } from "@/i18n";
import { auditBoard } from "@/lib/audit";
import { downloadSvg, downloadPng } from "@/lib/svg-export";
import type { Puzzle } from "@/lib/types";

const HINT_POSITIONS = [34, 45, 135, 210, 221];

const T = {
  en: {
    title: "Board viewer",
    intro: (
      <>
        Paste any e2.bucas.name link, or pick a famous board. Fully compatible with the
        community URL format, including <code>motifs_order</code> translation.
      </>
    ),
    credit: (
      <>
        Motif artwork and board URL format from{" "}
        <a className="underline" href="https://e2.bucas.name" target="_blank" rel="noreferrer">
          e2.bucas.name
        </a>{" "}
        (© Jef Bucas,{" "}
        <a
          className="underline"
          href="https://github.com/jfbucas/eternityII-viewer"
          target="_blank"
          rel="noreferrer"
        >
          GPL-3.0
        </a>
        ).
      </>
    ),
    officialById: "Official pieces by ID",
    generatedName: (s: number) => `generated ${s}×${s}`,
    placeholder: "https://e2.bucas.name/#puzzle=…&board_w=16&board_h=16&board_edges=…",
    load: "Load",
    emptyState: "Pick a board above, paste a link, or generate one →",
    importedBoard: "Imported board",
    matchedEdges: "matched edges",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} pieces placed`,
    showConflicts: "Show conflicts",
    pieceNumbers: "Piece numbers",
    coordinates: "Coordinates",
    cluePositions: "Clue positions",
    verification: "Verification",
    officialSet: "Official piece set",
    verified: "✓ verified",
    notOfficial: "✗ no",
    duplicates: "Duplicate pieces",
    noDuplicates: "✓ none",
    cluesRespected: "Clues respected",
    hintTitle: (n: number, cell: string) => `piece ${n} at ${cell}`,
    share: "Share",
    copyLink: "Copy link",
    downloadImage: "Download image",
    copied: "Copied!",
    generator: "Board generator",
    generatorIntro:
      "Build a brand-new solvable puzzle and see its solution. (Colors cap at 22: that's how many motifs exist.)",
    sizeLabel: (n: number) => `Size: ${n}×${n}`,
    colorsLabel: (n: number) => `Colors: ${n}`,
    generate: "Generate new board",
  },
  fr: {
    title: "Visualiseur",
    intro: (
      <>
        Collez un lien e2.bucas.name ou choisissez un plateau célèbre. Le visualiseur
        comprend le format de lien de la communauté, conversion des{" "}
        <code>motifs_order</code> comprise.
      </>
    ),
    credit: (
      <>
        Motifs et format de lien des plateaux signés{" "}
        <a className="underline" href="https://e2.bucas.name" target="_blank" rel="noreferrer">
          e2.bucas.name
        </a>{" "}
        (© Jef Bucas,{" "}
        <a
          className="underline"
          href="https://github.com/jfbucas/eternityII-viewer"
          target="_blank"
          rel="noreferrer"
        >
          GPL-3.0
        </a>
        ).
      </>
    ),
    officialById: "Pièces officielles, par numéro",
    generatedName: (s: number) => `généré ${s}×${s}`,
    placeholder: "https://e2.bucas.name/#puzzle=…&board_w=16&board_h=16&board_edges=…",
    load: "Afficher",
    emptyState: "Choisissez un plateau, collez un lien ou générez-en un →",
    importedBoard: "Plateau importé",
    matchedEdges: "côtés appariés",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} pièces posées`,
    showConflicts: "Afficher les conflits",
    pieceNumbers: "Numéros des pièces",
    coordinates: "Coordonnées",
    cluePositions: "Position des indices",
    verification: "Vérification",
    officialSet: "Jeu de pièces officiel",
    verified: "✓ vérifié",
    notOfficial: "✗ non",
    duplicates: "Pièces en double",
    noDuplicates: "✓ aucune",
    cluesRespected: "Indices respectés",
    hintTitle: (n: number, cell: string) => `pièce ${n} en ${cell}`,
    share: "Partager",
    copyLink: "Copier le lien",
    downloadImage: "Télécharger l'image",
    copied: "Copié !",
    generator: "Générateur de plateaux",
    generatorIntro:
      "Fabriquez un puzzle inédit qui a bien une solution, puis admirez-la. (Pas plus de 22 couleurs : c'est le nombre de motifs disponibles.)",
    sizeLabel: (n: number) => `Taille : ${n}×${n}`,
    colorsLabel: (n: number) => `Couleurs : ${n}`,
    generate: "Générer un plateau",
  },
};

/** Identity board: piece i at cell i, rotation 0. */
function identityBoard(puzzle: Puzzle): number[] {
  return Array.from({ length: puzzle.width * puzzle.height }, (_, i) => i * 4);
}

export default function Viewer() {
  const t = useT(T);
  const engineReady = useEngine();
  const [searchParams, setSearchParams] = useSearchParams();
  const [input, setInput] = useState("");
  const [board, setBoard] = useState<BucasBoard | null>(null);
  /** Set when the board came from the engine side (generator / pieces-by-ID),
   *  which gives us exact piece ids without matching. */
  const [enginePair, setEnginePair] = useState<{ puzzle: Puzzle; board: number[] } | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showConflicts, setShowConflicts] = useState(true);
  const [showNumbers, setShowNumbers] = useState(false);
  const [showCoords, setShowCoords] = useState(false);
  const [showHints, setShowHints] = useState(false);

  const [genSize, setGenSize] = useState(8);
  const [genColors, setGenColors] = useState(8);
  const [genSeed, setGenSeed] = useState(1);
  const [copied, setCopied] = useState(false);

  const boardRef = useRef<HTMLDivElement>(null);
  const boardName = () => board?.puzzleName?.replaceAll(" ", "_") ?? "eternity2-board";

  const copyUrl = (url: string) => {
    void navigator.clipboard.writeText(url);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  const saveImage = (kind: "svg" | "png") => {
    const svg = boardRef.current?.querySelector("svg");
    if (!svg) return;
    if (kind === "svg") downloadSvg(svg, boardName());
    else void downloadPng(svg, boardName());
  };

  const load = (params: string) => {
    try {
      const decoded = decodeBucas(params);
      setBoard(decoded);
      setEnginePair(null);
      setError(null);
      setSearchParams(toOurParams(parseParams(params)), { replace: true });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  };

  const loadEngine = (
    puzzle: Puzzle,
    cells: number[],
    name: string,
    { numbered = false, preventScroll = false } = {},
  ) => {
    const b = boardFromEngine(puzzle, cells);
    b.puzzleName = name;
    if (numbered) b.pieceNumbers = cells.map((v) => (v >= 0 ? (v >> 2) + 1 : 0));
    setBoard(b);
    setEnginePair({ puzzle, board: cells });
    setError(null);
    setSearchParams(ourParams(puzzle, cells, name), {
      replace: true,
      preventScrollReset: preventScroll,
    });
  };

  // Generate (or regenerate) a solvable board. Pass a seed to keep the same
  // board while only size/colors change; omit it to roll a fresh one.
  const generate = (size: number, colors: number, seed = genSeed) => {
    if (!engineReady) return;
    const puzzle = getGeneratedSolvedPuzzle(size, colors, seed);
    loadEngine(puzzle, identityBoard(puzzle), t.generatedName(size), { preventScroll: true });
  };

  // Live regeneration as a slider drags: each tick can fire many times per
  // frame, and generating a solved board is a synchronous WASM search, so we
  // coalesce to at most one generation per animation frame using the latest
  // params. The label still updates instantly off the slider's own value.
  const genFrame = useRef<{ id: number; size: number; colors: number } | null>(null);
  const generateLive = (size: number, colors: number) => {
    if (!engineReady) return;
    if (genFrame.current) {
      genFrame.current.size = size;
      genFrame.current.colors = colors;
      return;
    }
    const pending = { id: 0, size, colors };
    pending.id = requestAnimationFrame(() => {
      genFrame.current = null;
      generate(pending.size, pending.colors);
    });
    genFrame.current = pending;
  };

  // Shared links: the board params (puzzle_size, board_edges, …) sit directly
  // in the query string. A legacy single `b=<bucas blob>` is still honored.
  useEffect(() => {
    if (board) return;
    const legacy = searchParams.get("b");
    const map = legacy ? parseParams(legacy) : Object.fromEntries(searchParams);
    if (!map["board_edges"]) return;
    try {
      // Initialize state from an external system (the URL) on mount.
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setBoard(decodeBucas(map));
    } catch {
      /* bad shared link; ignore */
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Answer keys from printed sheets: #/viewer?g=size.colors.seed shows the
  // solved board of that generated set.
  useEffect(() => {
    const g = searchParams.get("g");
    if (!g || !engineReady || board) return;
    const m = /^(\d+)\.(\d+)\.(\d+)$/.exec(g);
    if (!m) return;
    const [, sizeStr, colorsStr, seedStr] = m;
    if (sizeStr === undefined || colorsStr === undefined || seedStr === undefined) return;
    const puzzle = getGeneratedSolvedPuzzle(
      parseInt(sizeStr, 10),
      parseInt(colorsStr, 10),
      parseInt(seedStr, 10),
    );
    // Initialize the view from a URL param (external system) once the engine
    // is ready; loadEngine seeds board/engine state.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    loadEngine(puzzle, identityBoard(puzzle), `${sizeStr}x${sizeStr}-${colorsStr}c answer key`);
    // Run once when the engine becomes ready; re-reading the URL on every
    // searchParams/board change would clobber user edits.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [engineReady]);

  const summary = useMemo(() => (board ? scoreSummary(board) : null), [board]);
  const audit = useMemo(
    () => (board && engineReady ? auditBoard(board, getOfficialPuzzle()) : null),
    [board, engineReady],
  );
  const conflicts = useMemo(
    () => (board && showConflicts ? conflictEdges(board) : undefined),
    [board, showConflicts],
  );

  const is16 = board?.width === 16 && board?.height === 16;

  const pieceNumbers = useMemo(() => {
    if (!board || !showNumbers) return undefined;
    if (board.pieceNumbers) {
      // Bucas convention is 1-based with 000 = empty, but some bundled
      // boards (Sample_4x4) number occupied cells from 0. Detect that and
      // shift to 1-based; a 0 only means "no number" on an empty cell.
      const zeroBased = board.pieceNumbers.some((n, i) => n === 0 && board.cells[i] !== null);
      const offset = zeroBased ? 1 : 0;
      return board.pieceNumbers.map((n, i) => (board.cells[i] !== null ? n + offset : null));
    }
    if (!is16 || !engineReady) return undefined;
    const matched = matchToPieces(getOfficialPuzzle(), board);
    return Array.from(matched, (v) => (v >= 0 ? (v >> 2) + 1 : null));
  }, [board, showNumbers, is16, engineReady]);

  // Every displayed board can go back out as a bucas URL.
  const exportPair = useMemo(() => {
    if (enginePair) return enginePair;
    if (!board || !is16 || !engineReady) return null;
    const puzzle = getOfficialPuzzle();
    return { puzzle, board: Array.from(matchToPieces(puzzle, board)) };
  }, [enginePair, board, is16, engineReady]);

  const bucasUrl = () =>
    exportPair ? encodeBucasUrl(exportPair.puzzle, exportPair.board, board?.puzzleName ?? undefined) : "";

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-1 text-muted-foreground">{t.intro}</p>
        <p className="mt-1 text-xs text-muted-foreground">{t.credit}</p>
      </div>

      <div className="flex flex-wrap gap-2">
        {KNOWN_BOARDS.map((kb) => (
          <Button key={kb.id} variant="outline" size="sm" onClick={() => load(kb.params)}>
            {kb.label}
          </Button>
        ))}
        <Button
          variant="outline"
          size="sm"
          disabled={!engineReady}
          onClick={() => {
            const puzzle = getOfficialPuzzle();
            loadEngine(puzzle, identityBoard(puzzle), t.officialById, { numbered: true });
            setShowNumbers(true);
            setShowConflicts(false);
          }}
        >
          {t.officialById}
        </Button>
      </div>

      <div className="flex flex-col gap-2 sm:flex-row">
        <Textarea
          placeholder={t.placeholder}
          value={input}
          onChange={(e) => setInput(e.target.value)}
          className="min-h-[60px] font-mono text-xs"
        />
        <Button
          className="self-end sm:self-auto"
          onClick={() => load(input)}
          disabled={!input.trim()}
        >
          {t.load}
        </Button>
      </div>
      {error && <p className="text-sm text-red-500">{error}</p>}

      <div className="grid gap-6 lg:grid-cols-[1fr_340px]">
        <div ref={boardRef}>
          {board && (
            <BoardSvg
              width={board.width}
              height={board.height}
              cells={board.cells}
              conflicts={conflicts}
              pieceNumbers={pieceNumbers}
              highlight={showHints && is16 ? HINT_POSITIONS : undefined}
              coordinates={showCoords}
              className="max-w-3xl"
            />
          )}
          {!board && (
            <div className="flex aspect-square max-w-3xl items-center justify-center rounded-lg border border-dashed text-muted-foreground">
              {t.emptyState}
            </div>
          )}
        </div>
        <div className="space-y-4">
          {board && summary && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base">
                  {board.puzzleName?.replaceAll("_", " ") ?? t.importedBoard}
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-3">
                <div className="flex items-center gap-2">
                  <Badge variant={summary.score === summary.max ? "default" : "secondary"} className="text-base">
                    {summary.score} / {summary.max}
                  </Badge>
                  <span className="text-sm text-muted-foreground">{t.matchedEdges}</span>
                </div>
                <p className="text-sm text-muted-foreground">
                  {t.piecesPlaced(summary.placed, board.width * board.height)}
                </p>
                <div className="space-y-2 pt-2">
                  <div className="flex items-center justify-between">
                    <Label htmlFor="v-conf">{t.showConflicts}</Label>
                    <Switch id="v-conf" checked={showConflicts} onCheckedChange={setShowConflicts} />
                  </div>
                  <div className="flex items-center justify-between">
                    <Label htmlFor="v-num">{t.pieceNumbers}</Label>
                    <Switch id="v-num" checked={showNumbers} onCheckedChange={setShowNumbers} />
                  </div>
                  <div className="flex items-center justify-between">
                    <Label htmlFor="v-coord">{t.coordinates}</Label>
                    <Switch id="v-coord" checked={showCoords} onCheckedChange={setShowCoords} />
                  </div>
                  {is16 && (
                    <div className="flex items-center justify-between">
                      <Label htmlFor="v-hint">{t.cluePositions}</Label>
                      <Switch id="v-hint" checked={showHints} onCheckedChange={setShowHints} />
                    </div>
                  )}
                </div>
              </CardContent>
            </Card>
          )}

          {board && audit?.applicable && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base">{t.verification}</CardTitle>
              </CardHeader>
              <CardContent className="space-y-1.5 text-sm">
                <div className="flex items-center justify-between">
                  <span>{t.officialSet}</span>
                  <span className={audit.officialSet ? "text-emerald-600" : "text-red-500"}>
                    {audit.officialSet ? t.verified : t.notOfficial}
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <span>{t.duplicates}</span>
                  <span className={audit.duplicates === 0 ? "text-emerald-600" : "text-red-500"}>
                    {audit.duplicates === 0 ? t.noDuplicates : `✗ ${audit.duplicates}`}
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <span>{t.cluesRespected}</span>
                  <span
                    className={
                      audit.hintsRespected === 5
                        ? "text-emerald-600"
                        : audit.hintsRespected > 0
                          ? "text-amber-600"
                          : "text-red-500"
                    }
                  >
                    {audit.hintsRespected} / 5
                  </span>
                </div>
                <div className="flex flex-wrap gap-1 pt-1">
                  {audit.hints.map((h) => (
                    <Badge
                      key={h.pos}
                      variant={h.respected ? "secondary" : "outline"}
                      className={h.respected ? "text-emerald-700" : "text-muted-foreground"}
                      title={t.hintTitle(h.pieceNumber, h.cellLabel)}
                    >
                      {h.respected ? "✓" : "✗"} {h.cellLabel}
                    </Badge>
                  ))}
                </div>
              </CardContent>
            </Card>
          )}

          {board && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base">{t.share}</CardTitle>
              </CardHeader>
              <CardContent className="flex flex-wrap gap-2">
                <DropdownMenu>
                  <DropdownMenuTrigger
                    render={
                      <Button variant="outline" size="sm">
                        {copied ? t.copied : t.copyLink}
                        <ChevronDownIcon className="size-3.5 text-muted-foreground" />
                      </Button>
                    }
                  />
                  <DropdownMenuContent>
                    <DropdownMenuItem onClick={() => copyUrl(window.location.href)}>
                      eternity2.dev
                    </DropdownMenuItem>
                    <DropdownMenuItem
                      disabled={!exportPair}
                      onClick={() => copyUrl(bucasUrl())}
                    >
                      e2.bucas.name
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>

                <DropdownMenu>
                  <DropdownMenuTrigger
                    render={
                      <Button variant="outline" size="sm">
                        {t.downloadImage}
                        <ChevronDownIcon className="size-3.5 text-muted-foreground" />
                      </Button>
                    }
                  />
                  <DropdownMenuContent>
                    <DropdownMenuItem onClick={() => saveImage("svg")}>SVG</DropdownMenuItem>
                    <DropdownMenuItem onClick={() => saveImage("png")}>PNG</DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </CardContent>
            </Card>
          )}

          <Card>
            <CardHeader>
              <CardTitle className="text-base">{t.generator}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-xs text-muted-foreground">{t.generatorIntro}</p>
              <div className="space-y-1.5">
                <Label>{t.sizeLabel(genSize)}</Label>
                <Slider
                  min={2}
                  max={16}
                  step={1}
                  value={genSize}
                  onValueChange={(v) => {
                    const s = singleSliderValue(v);
                    const c = Math.min(genColors, engineReady ? getMaxColors(s) : 4);
                    setGenSize(s);
                    setGenColors(c);
                    generateLive(s, c);
                  }}
                />
              </div>
              <div className="space-y-1.5">
                <Label>{t.colorsLabel(genColors)}</Label>
                <Slider
                  min={2}
                  max={engineReady ? getMaxColors(genSize) : 22}
                  step={1}
                  value={genColors}
                  onValueChange={(v) => {
                    const c = singleSliderValue(v);
                    setGenColors(c);
                    generateLive(genSize, c);
                  }}
                />
              </div>
              <Button
                className="w-full"
                disabled={!engineReady}
                onClick={() => {
                  const seed = Math.floor(Math.random() * 1_000_000);
                  setGenSeed(seed);
                  generate(genSize, genColors, seed);
                }}
              >
                {t.generate}
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export const meta = pageMeta("viewer");
