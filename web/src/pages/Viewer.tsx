import { useEffect, useMemo, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { BoardSvg } from "@/components/board/BoardSvg";
import { BucasActions } from "@/components/board/BucasActions";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Slider } from "@/components/ui/slider";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { KNOWN_BOARDS } from "@/data/known-boards";
import {
  decodeBucas,
  conflictEdges,
  scoreSummary,
  matchToPieces,
  boardFromEngine,
  bucasParams,
} from "@/lib/bucas";
import type { BucasBoard } from "@/lib/bucas";
import { getOfficialPuzzle, getGeneratedSolvedPuzzle, getMaxColors } from "@/engine";
import { useEngine } from "@/engine/useEngine";
import { useT } from "@/i18n";
import { auditBoard } from "@/lib/audit";
import type { Puzzle } from "@/lib/types";

const HINT_POSITIONS = [34, 45, 135, 210, 221];

const T = {
  en: {
    title: "Board viewer",
    intro: (
      <>
        Paste any <code>e2.bucas.name</code> link, or pick a famous board. Fully compatible
        with the community URL format, including <code>motifs_order</code> translation.
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
    copyPageLink: "Copy link to this page",
    generator: "Board generator",
    generatorIntro:
      "Build a brand-new solvable puzzle and see its solution. (Colors cap at 22: that's how many motifs exist.)",
    sizeLabel: "Size",
    colorsLabel: (n: number) => `Colors: ${n}`,
    generate: "Generate board",
    seedTitle: "Regenerate with the same seed",
    seedBtn: (n: number) => `Seed ${n}`,
  },
  fr: {
    title: "Visualiseur de plateaux",
    intro: (
      <>
        Collez n'importe quel lien <code>e2.bucas.name</code>, ou choisissez un plateau
        célèbre. Entièrement compatible avec le format d'URL de la communauté, y compris la
        conversion <code>motifs_order</code>.
      </>
    ),
    credit: (
      <>
        Motifs graphiques et format d'URL des plateaux issus de{" "}
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
    officialById: "Pièces officielles par numéro",
    generatedName: (s: number) => `généré ${s}×${s}`,
    placeholder: "https://e2.bucas.name/#puzzle=…&board_w=16&board_h=16&board_edges=…",
    load: "Charger",
    emptyState: "Choisissez un plateau ci-dessus, collez un lien, ou générez-en un →",
    importedBoard: "Plateau importé",
    matchedEdges: "côtés appariés",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} pièces placées`,
    showConflicts: "Afficher les conflits",
    pieceNumbers: "Numéros des pièces",
    cluePositions: "Positions des indices",
    verification: "Vérification",
    officialSet: "Jeu de pièces officiel",
    verified: "✓ vérifié",
    notOfficial: "✗ non",
    duplicates: "Pièces en double",
    noDuplicates: "✓ aucune",
    cluesRespected: "Indices respectés",
    hintTitle: (n: number, cell: string) => `pièce ${n} en ${cell}`,
    share: "Partager",
    copyPageLink: "Copier le lien de cette page",
    generator: "Générateur de plateaux",
    generatorIntro:
      "Créez un tout nouveau puzzle qui a une solution et regardez-la. (Maximum 22 couleurs : c'est le nombre de motifs qui existent.)",
    sizeLabel: "Taille",
    colorsLabel: (n: number) => `Couleurs : ${n}`,
    generate: "Générer un plateau",
    seedTitle: "Régénérer avec la même graine",
    seedBtn: (n: number) => `Graine ${n}`,
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
  const [showHints, setShowHints] = useState(false);

  const [genSize, setGenSize] = useState(8);
  const [genColors, setGenColors] = useState(8);
  const [genSeed, setGenSeed] = useState(1);

  const load = (params: string) => {
    try {
      const decoded = decodeBucas(params);
      setBoard(decoded);
      setEnginePair(null);
      setError(null);
      setSearchParams({ b: params }, { replace: true });
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  };

  const loadEngine = (puzzle: Puzzle, cells: number[], name: string, numbered = false) => {
    const b = boardFromEngine(puzzle, cells);
    b.puzzleName = name;
    if (numbered) b.pieceNumbers = cells.map((v) => (v >= 0 ? (v >> 2) + 1 : 0));
    setBoard(b);
    setEnginePair({ puzzle, board: cells });
    setError(null);
    setSearchParams({ b: bucasParams(puzzle, cells, name) }, { replace: true });
  };

  // Shared links: #/viewer?b=<bucas params>
  useEffect(() => {
    const b = searchParams.get("b");
    if (b && !board) {
      try {
        setBoard(decodeBucas(b));
      } catch {
        /* bad shared link; ignore */
      }
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
    const puzzle = getGeneratedSolvedPuzzle(
      parseInt(m[1], 10),
      parseInt(m[2], 10),
      parseInt(m[3], 10),
    );
    loadEngine(puzzle, identityBoard(puzzle), `${m[1]}x${m[1]}-${m[2]}c answer key`);
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
    if (board.pieceNumbers) return board.pieceNumbers.map((n) => (n > 0 ? n : null));
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
            loadEngine(puzzle, identityBoard(puzzle), t.officialById, true);
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

      <div className="grid gap-6 lg:grid-cols-[1fr_300px]">
        <div>
          {board && (
            <BoardSvg
              width={board.width}
              height={board.height}
              cells={board.cells}
              conflicts={conflicts}
              pieceNumbers={pieceNumbers}
              highlight={showHints && is16 ? HINT_POSITIONS : undefined}
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
              <CardContent className="space-y-2">
                <Button
                  variant="outline"
                  size="sm"
                  className="w-full"
                  onClick={() => navigator.clipboard.writeText(window.location.href)}
                >
                  {t.copyPageLink}
                </Button>
                {exportPair && (
                  <BucasActions
                    puzzle={exportPair.puzzle}
                    board={exportPair.board}
                    name={board.puzzleName ?? "shared-board"}
                  />
                )}
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
                <Label>{t.sizeLabel}</Label>
                <Select
                  value={String(genSize)}
                  onValueChange={(v) => {
                    if (!v) return;
                    const s = parseInt(v, 10);
                    setGenSize(s);
                    setGenColors((c) => Math.min(c, engineReady ? getMaxColors(s) : 4));
                  }}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {Array.from({ length: 15 }, (_, i) => i + 2).map((s) => (
                      <SelectItem key={s} value={String(s)}>
                        {s}×{s}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              <div className="space-y-1.5">
                <Label>{t.colorsLabel(genColors)}</Label>
                <Slider
                  min={2}
                  max={engineReady ? getMaxColors(genSize) : 22}
                  step={1}
                  value={genColors}
                  onValueChange={(v) => setGenColors(Array.isArray(v) ? v[0] : v)}
                />
              </div>
              <div className="flex gap-2">
                <Button
                  className="flex-1"
                  disabled={!engineReady}
                  onClick={() => {
                    const seed = Math.floor(Math.random() * 1_000_000);
                    setGenSeed(seed);
                    const puzzle = getGeneratedSolvedPuzzle(genSize, genColors, seed);
                    loadEngine(puzzle, identityBoard(puzzle), t.generatedName(genSize));
                  }}
                >
                  {t.generate}
                </Button>
                <Button
                  variant="outline"
                  disabled={!engineReady}
                  title={t.seedTitle}
                  onClick={() => {
                    const puzzle = getGeneratedSolvedPuzzle(genSize, genColors, genSeed);
                    loadEngine(puzzle, identityBoard(puzzle), t.generatedName(genSize));
                  }}
                >
                  {t.seedBtn(genSeed)}
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
