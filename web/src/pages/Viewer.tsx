import { pageMeta } from "@/seo";
import { useEffect, useMemo, useRef, useState } from "react";
import { useSearchParams } from "react-router";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "@/components/ui/dropdown-menu";
import { ChevronDownIcon } from "lucide-react";
import { KNOWN_BOARDS } from "@/data/known-boards";
import type { KnownBoard } from "@/data/known-boards";
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
import { getOfficialPuzzle, getGeneratedSolvedPuzzle } from "@/engine";
import { useEngine } from "@/engine/useEngine";
import { useT } from "@/i18n";
import { auditBoard } from "@/lib/audit";
import { downloadSvg, downloadPng } from "@/lib/svg-export";
import type { Puzzle } from "@/lib/types";

const HINT_POSITIONS = [34, 45, 135, 210, 221];

// Group the bundled boards in the picker dropdown by record tier, so the 18
// boards (records + Bucas/McGavin/Verhaard variants) read as an organised list
// rather than a wall of buttons.
type BoardGroupKey = "records" | "class469" | "class467" | "linear" | "other";
const BOARD_GROUPS: { key: BoardGroupKey; filter: (b: KnownBoard) => boolean }[] = [
  { key: "records", filter: (b) => b.tag !== "linear" && b.score !== null && b.score >= 470 },
  { key: "class469", filter: (b) => b.tag !== "linear" && b.score === 469 },
  {
    key: "class467",
    filter: (b) => b.tag !== "linear" && b.score !== null && b.score >= 460 && b.score <= 468,
  },
  { key: "linear", filter: (b) => b.tag === "linear" },
  { key: "other", filter: (b) => b.tag !== "linear" && (b.score === null || b.score < 460) },
];

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
    officialClues: "Official clues",
    officialCluesName: "Official clues",
    pickBoard: "Pick a board…",
    groups: {
      records: "Records",
      class469: "469 boards",
      class467: "460–468 boards",
      linear: "Linear",
      other: "Other",
    } satisfies Record<BoardGroupKey, string>,
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
    genMovedIntro:
      "The board generator now lives in the builder's toolkit, where you can also make a whole batch at once. Generate there, then paste any board's link back here to score it.",
    genMovedCta: "Open the toolkit generator →",
    generatorIntro:
      "Build a brand-new solvable puzzle and see its solution. (Colors cap at 22: that's how many motifs exist.)",
    sizeLabel: (n: number) => `Size: ${n}×${n}`,
    colorsLabel: (n: number) => `Colors: ${n}`,
    framed: "Frame-restricted colors",
    framedHint:
      "Like the real Eternity II: confine some colors to the border band, the rest to the deep interior (needs size ≥ 4, colors ≥ 2).",
    generate: "Generate new board",
    convertLink: "Need to convert between board_edges, board_pieces and bucas URLs?",
    convertLinkCta: "Open the format converter",
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
    officialClues: "Indices officiels",
    officialCluesName: "Indices officiels",
    pickBoard: "Choisir un plateau…",
    groups: {
      records: "Records",
      class469: "Plateaux 469",
      class467: "Plateaux 460–468",
      linear: "Linéaires",
      other: "Autres",
    } satisfies Record<BoardGroupKey, string>,
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
    genMovedIntro:
      "Le générateur de plateaux se trouve désormais dans la boîte à outils, où vous pouvez aussi en produire tout un lot d'un coup. Générez là-bas, puis collez le lien d'un plateau ici pour le scorer.",
    genMovedCta: "Ouvrir le générateur de la boîte à outils →",
    generatorIntro:
      "Fabriquez un puzzle inédit qui a bien une solution, puis admirez-la. (Pas plus de 22 couleurs : c'est le nombre de motifs disponibles.)",
    sizeLabel: (n: number) => `Taille : ${n}×${n}`,
    colorsLabel: (n: number) => `Couleurs : ${n}`,
    framed: "Couleurs réservées au cadre",
    framedHint:
      "Comme le vrai Eternity II : confine certaines couleurs à la bande de bordure, les autres à l'intérieur profond (nécessite taille ≥ 4, couleurs ≥ 2).",
    generate: "Générer un plateau",
    convertLink: "Besoin de passer entre board_edges, board_pieces et les liens bucas ?",
    convertLinkCta: "Ouvrir le convertisseur de formats",
  },
  es: {
    title: "Visor de tableros",
    intro: (
      <>
        Pega cualquier enlace de e2.bucas.name o elige un tablero célebre. Totalmente
        compatible con el formato de URL de la comunidad, incluida la traducción de{" "}
        <code>motifs_order</code>.
      </>
    ),
    credit: (
      <>
        Ilustración de los motivos y formato de URL de los tableros de{" "}
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
    officialById: "Piezas oficiales, por número",
    officialClues: "Pistas oficiales",
    officialCluesName: "Pistas oficiales",
    pickBoard: "Elige un tablero…",
    groups: {
      records: "Récords",
      class469: "Tableros 469",
      class467: "Tableros 460–468",
      linear: "Lineales",
      other: "Otros",
    } satisfies Record<BoardGroupKey, string>,
    generatedName: (s: number) => `generado ${s}×${s}`,
    placeholder: "https://e2.bucas.name/#puzzle=…&board_w=16&board_h=16&board_edges=…",
    load: "Mostrar",
    emptyState: "Elige un tablero arriba, pega un enlace o genera uno →",
    importedBoard: "Tablero importado",
    matchedEdges: "aristas coincidentes",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} piezas colocadas`,
    showConflicts: "Mostrar conflictos",
    pieceNumbers: "Números de las piezas",
    coordinates: "Coordenadas",
    cluePositions: "Posición de las pistas",
    verification: "Verificación",
    officialSet: "Juego de piezas oficial",
    verified: "✓ verificado",
    notOfficial: "✗ no",
    duplicates: "Piezas duplicadas",
    noDuplicates: "✓ ninguna",
    cluesRespected: "Pistas respetadas",
    hintTitle: (n: number, cell: string) => `pieza ${n} en ${cell}`,
    share: "Compartir",
    copyLink: "Copiar el enlace",
    downloadImage: "Descargar la imagen",
    copied: "¡Copiado!",
    generator: "Generador de tableros",
    genMovedIntro:
      "El generador de tableros ahora está en la caja de herramientas, donde también puedes crear un lote entero de una vez. Genera allí y luego pega aquí el enlace de cualquier tablero para puntuarlo.",
    genMovedCta: "Abrir el generador de la caja de herramientas →",
    generatorIntro:
      "Crea un puzzle inédito con solución garantizada y contempla su solución. (Máximo 22 colores: es la cantidad de motivos disponibles.)",
    sizeLabel: (n: number) => `Tamaño: ${n}×${n}`,
    colorsLabel: (n: number) => `Colores: ${n}`,
    framed: "Colores reservados al marco",
    framedHint:
      "Como el auténtico Eternity II: confina algunos colores a la banda del borde y el resto al interior profundo (requiere tamaño ≥ 4, colores ≥ 2).",
    generate: "Generar tablero",
    convertLink: "¿Necesitas pasar entre board_edges, board_pieces y los enlaces bucas?",
    convertLinkCta: "Abrir el conversor de formatos",
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
    // A puzzle's pinned clues ride in the URL's `hints`, so a clue set stays
    // shareable and the overlay can mark exactly those cells.
    const hints = puzzle.hints.map((h) => ({ pos: h.pos, rot: h.rot }));
    b.hints = hints.length ? hints.map((h) => h.pos) : null;
    setBoard(b);
    setEnginePair({ puzzle, board: cells });
    setError(null);
    setSearchParams(ourParams(puzzle, cells, name, hints), {
      replace: true,
      preventScrollReset: preventScroll,
    });
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
        <p className="mt-2 text-sm text-muted-foreground">
          {t.convertLink}{" "}
          <Link
            to="/convert"
            className="font-medium text-primary underline-offset-4 hover:underline"
          >
            {t.convertLinkCta}
          </Link>
        </p>
      </div>

      <div className="flex flex-wrap items-center gap-2">
        <Select
          value=""
          onValueChange={(id) => {
            const kb = KNOWN_BOARDS.find((b) => b.id === id);
            if (kb) load(kb.params);
          }}
        >
          <SelectTrigger className="w-72">
            <SelectValue placeholder={t.pickBoard} />
          </SelectTrigger>
          <SelectContent>
            {BOARD_GROUPS.map((group) => {
              const items = KNOWN_BOARDS.filter(group.filter);
              if (items.length === 0) return null;
              return (
                <SelectGroup key={group.key}>
                  <SelectLabel>{t.groups[group.key]}</SelectLabel>
                  {items.map((kb) => (
                    <SelectItem key={kb.id} value={kb.id}>
                      {kb.label}
                      {kb.score !== null ? ` · ${kb.score}/480` : ""}
                    </SelectItem>
                  ))}
                </SelectGroup>
              );
            })}
          </SelectContent>
        </Select>
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
        <Button
          variant="outline"
          size="sm"
          disabled={!engineReady}
          onClick={() => {
            // A clue-only board: every cell empty (-1) except the five pinned
            // hint pieces, placed at their official positions and rotations.
            const puzzle = getOfficialPuzzle();
            const cells = new Array<number>(puzzle.width * puzzle.height).fill(-1);
            for (const h of puzzle.hints) cells[h.pos] = h.piece * 4 + h.rot;
            loadEngine(puzzle, cells, t.officialCluesName, { numbered: true });
            setShowNumbers(true);
            setShowConflicts(false);
            setShowHints(true);
          }}
        >
          {t.officialClues}
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
              highlight={
                showHints ? (board.hints ?? (is16 ? HINT_POSITIONS : undefined)) : undefined
              }
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
            <CardContent className="space-y-3">
              <p className="text-xs text-muted-foreground">{t.genMovedIntro}</p>
              <Button
                variant="outline"
                className="w-full"
                render={<Link to="/research/build/toolkit" />}
              >
                {t.genMovedCta}
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export const meta = pageMeta("viewer");
