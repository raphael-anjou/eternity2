import { pageMeta } from "@/seo";
import { useMemo, useState } from "react";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Textarea } from "@/components/ui/textarea";
import { KNOWN_BOARDS } from "@/data/known-boards";
import { decodeBucas, parseParams, scoreSummary } from "@/lib/bucas";
import type { BucasBoard, Edges } from "@/lib/bucas";
import { useT } from "@/i18n";

// Re-encode decoded cells into the default bucas board_edges letters, so we can
// echo a clean string for any input (URL, hash, or bare board_edges alike).
// Mirrors the private encoder in bucas.ts but works straight off BucasBoard
// cells, which is all a converter needs (no engine, no piece matching).
function cellsToEdges(cells: (Edges | null)[]): string {
  let out = "";
  for (const cell of cells) {
    if (!cell) out += "aaaa";
    else out += cell.map((c) => String.fromCharCode(97 + c)).join("");
  }
  return out;
}

function cellsToPieces(pieceNumbers: number[] | null): string | null {
  if (!pieceNumbers) return null;
  return pieceNumbers.map((n) => String(n).padStart(3, "0")).join("");
}

// e2pieces.txt: one line per placed cell, in reading order (row by row), four
// whitespace-separated edge integers in top,right,bottom,left order, 0 = grey
// border. A board_edges string carries pieces already rotated into place, so
// this lists them AS PLACED, not as a canonical rotation-0 piece set. We can't
// recover the original catalogue order or canonical rotation from a board alone,
// so we do not claim to; this is the honest, useful export.
function cellsToE2Pieces(cells: (Edges | null)[]): string {
  const lines: string[] = [];
  for (const cell of cells) {
    if (!cell) continue;
    lines.push(cell.join(" "));
  }
  return lines.join("\n");
}

// Sanitize a puzzle name to the URL-safe subset, matching e2-io's sanitize_name
// so a board carries the same name string through every tool on the site.
function safeName(board: BucasBoard): string {
  const raw = (board.puzzleName ?? "eternity2-board").replace(/[^A-Za-z0-9_]+/g, "_");
  return raw.length > 0 ? raw : "eternity2_board";
}

// The canonical eternity2.dev viewer URL — the format every algorithm on the
// site now emits. Boards are square on the site, so we emit a single
// `puzzle_size`; the viewer also reads bucas's board_w/board_h. Built straight
// from cells so the converter works for any size (no engine, no piece matching).
function viewerUrl(board: BucasBoard): string {
  const name = safeName(board);
  const edges = cellsToEdges(board.cells);
  const pieces = cellsToPieces(board.pieceNumbers);
  let params = `puzzle=${name}&puzzle_size=${board.width}&board_edges=${edges}`;
  if (pieces) params += `&board_pieces=${pieces}`;
  return `https://eternity2.dev/viewer?${params}`;
}

// The legacy e2.bucas.name viewer URL, kept as a derived converter for interop
// with the community viewer. No longer the default output.
function bucasUrl(board: BucasBoard): string {
  const name = safeName(board);
  const edges = cellsToEdges(board.cells);
  const pieces = cellsToPieces(board.pieceNumbers);
  let params = `puzzle=${name}&board_w=${board.width}&board_h=${board.height}&board_edges=${edges}`;
  if (pieces) params += `&board_pieces=${pieces}`;
  return `https://e2.bucas.name/#${params}`;
}

// The canonical board JSON — byte-for-byte the schema the Rust `e2-io` crate's
// BoardDoc emits, so a board pasted here and a board written by any solver are
// the same document. `board` is the row-major piece*4+rot vector (-1 empty);
// without explicit piece numbers we can't recover piece identity from a board
// alone, so those cells are left as -1 and board_pieces is all-000.
function canonicalJson(board: BucasBoard): string {
  const n = board.width * board.height;
  const edges = cellsToEdges(board.cells);
  const pieces = cellsToPieces(board.pieceNumbers) ?? "0".repeat(n * 3);
  const codes: number[] = [];
  for (let i = 0; i < n; i++) {
    const pn = board.pieceNumbers?.[i] ?? 0;
    // We only know the placement code when a real piece number is present; a
    // board carries rotated edges, not a rotation, so rot is unknown here.
    codes.push(pn >= 1 ? (pn - 1) * 4 : -1);
  }
  const summary = scoreSummary(board);
  const doc = {
    name: safeName(board),
    size: board.width,
    score: summary.score,
    breaks: summary.max - summary.score,
    board: codes,
    board_edges: edges,
    board_pieces: pieces,
    url: viewerUrl(board),
  };
  return JSON.stringify(doc, null, 2);
}

// The legacy puzzle CSV the standalone community engines read: a size header,
// then one `top,right,bottom,left` row per filled cell in reading order, each
// color a 16-bit zero-padded binary word (border 0 = 1111111111111111).
function canonicalCsv(board: BucasBoard): string {
  const colorWord = (c: number): string =>
    c === 0 ? "1".repeat(16) : c.toString(2).padStart(16, "0");
  const lines: string[] = [String(board.width)];
  for (const cell of board.cells) {
    if (!cell) continue;
    lines.push(cell.map(colorWord).join(","));
  }
  return lines.join("\n");
}

const EXAMPLE_ID = "Joshua_Blackwood_470";

const T = {
  en: {
    title: "Format converter",
    intro: (
      <>
        The community's board formats never quite line up: viewer URLs,{" "}
        <code>board_edges</code> letter strings, numeric <code>e2pieces.txt</code>{" "}
        files, and CSV puzzles all say the same thing in different alphabets.
        Paste any one of them here and read back the others — including the
        canonical <code>eternity2.dev</code> URL and the one board <code>JSON</code>{" "}
        every solver on this site now emits — with a preview so you can see the
        file is right. New to the formats?{" "}
        <Link to="/research/build/formats" className="font-medium text-primary underline-offset-4 hover:underline">
          Read the format reference
        </Link>
        .
      </>
    ),
    inputTitle: "Paste a board",
    inputHelp: (
      <>
        A full <code>e2.bucas.name</code> link, a bare params string, or just a{" "}
        <code>board_edges</code> letter string on its own. For a lone{" "}
        <code>board_edges</code> string, set the size below.
      </>
    ),
    placeholder:
      "https://e2.bucas.name/#puzzle=…&board_w=16&board_h=16&board_edges=… (or paste board_edges letters alone)",
    sizeLabel: "Board size (for a bare board_edges string)",
    convert: "Convert",
    loadExample: "Load an example (Blackwood 470)",
    badInput: "Could not read that input. Check it is a bucas link, a params string, or a board_edges letter string with the matching size.",
    previewTitle: "Preview",
    previewEmpty: "Convert a board to see it here.",
    scoreTitle: "Score",
    matchedEdges: "matched edges",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} cells filled`,
    outEdgesTitle: "board_edges",
    outEdgesHelp: "Four lowercase letters per cell, row by row (URDL). 'a' is the grey border; \"aaaa\" is an empty cell.",
    outPiecesTitle: "board_pieces",
    outPiecesHelp: "Three digits per cell, the 1-based piece number, 000 for an empty cell.",
    outPiecesNone: "This board carries no piece numbers, so there is nothing to derive here. board_edges above is enough to render and score it.",
    outUrlTitle: "eternity2.dev URL",
    outUrlHelp: "The canonical viewer link, rebuilt from the board above. This is the format every solver on this site emits. Open it to view or share.",
    outJsonTitle: "Canonical board JSON",
    outJsonHelp: "The one board document every solver on this site writes: score, breaks, the placement vector, both letter blobs, and the eternity2.dev URL. Byte-compatible with the e2-io BoardDoc schema.",
    outCsvTitle: "Puzzle CSV",
    outCsvHelp: "The legacy row-per-cell format the standalone community engines read: a size header, then top,right,bottom,left per filled cell as 16-bit binary color words (border = 1111111111111111).",
    outBucasTitle: "bucas.name URL (community viewer)",
    outBucasHelp: "A legacy e2.bucas.name link, kept for interop with Jef Bucas's original viewer. Derived from the same board.",
    outE2Title: "e2pieces.txt (pieces as placed)",
    outE2Help: "One line per filled cell, in reading order: four edge numbers (top, right, bottom, left), 0 for grey. These are the pieces as placed and rotated, not a canonical rotation-0 catalogue; a board alone cannot recover the original piece order.",
    copy: "Copy",
    copied: "Copied",
    viewerLink: "Want to inspect and share a board instead?",
    viewerLinkCta: "Open the viewer",
  },
  fr: {
    title: "Convertisseur de formats",
    intro: (
      <>
        Les formats de plateau de la communauté ne collent jamais tout à fait :
        liens du visualiseur, chaînes de lettres <code>board_edges</code>,
        fichiers numériques <code>e2pieces.txt</code> et puzzles CSV disent la
        même chose dans des alphabets différents. Collez-en un ici et récupérez
        les autres — dont l'URL canonique <code>eternity2.dev</code> et le{" "}
        <code>JSON</code> unique que produit désormais chaque solveur du site —
        avec un aperçu pour vérifier d'un coup d'œil que le fichier est bon. Vous
        découvrez les formats ?{" "}
        <Link to="/research/build/formats" className="font-medium text-primary underline-offset-4 hover:underline">
          Lisez la référence des formats
        </Link>
        .
      </>
    ),
    inputTitle: "Collez un plateau",
    inputHelp: (
      <>
        Un lien <code>e2.bucas.name</code> complet, une chaîne de paramètres, ou
        simplement une chaîne de lettres <code>board_edges</code> seule. Pour une
        chaîne <code>board_edges</code> isolée, indiquez la taille ci-dessous.
      </>
    ),
    placeholder:
      "https://e2.bucas.name/#puzzle=…&board_w=16&board_h=16&board_edges=… (ou collez seulement les lettres board_edges)",
    sizeLabel: "Taille du plateau (pour une chaîne board_edges seule)",
    convert: "Convertir",
    loadExample: "Charger un exemple (Blackwood 470)",
    badInput: "Lecture impossible. Vérifiez qu'il s'agit d'un lien bucas, d'une chaîne de paramètres ou d'une chaîne board_edges à la bonne taille.",
    previewTitle: "Aperçu",
    previewEmpty: "Convertissez un plateau pour le voir ici.",
    scoreTitle: "Score",
    matchedEdges: "côtés appariés",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} cases remplies`,
    outEdgesTitle: "board_edges",
    outEdgesHelp: "Quatre lettres minuscules par case, ligne par ligne (URDL). « a » est la bordure grise ; « aaaa » est une case vide.",
    outPiecesTitle: "board_pieces",
    outPiecesHelp: "Trois chiffres par case, le numéro de pièce (base 1), 000 pour une case vide.",
    outPiecesNone: "Ce plateau ne porte aucun numéro de pièce : rien à déduire ici. Le board_edges ci-dessus suffit à le dessiner et à le noter.",
    outUrlTitle: "URL eternity2.dev",
    outUrlHelp: "Le lien canonique du visualiseur, reconstruit à partir du plateau ci-dessus. C'est le format que produit chaque solveur du site. Ouvrez-le pour le voir ou le partager.",
    outJsonTitle: "JSON de plateau canonique",
    outJsonHelp: "Le document unique que chaque solveur du site écrit : score, ruptures, le vecteur de placement, les deux chaînes de lettres et l'URL eternity2.dev. Compatible octet pour octet avec le schéma BoardDoc d'e2-io.",
    outCsvTitle: "Puzzle CSV",
    outCsvHelp: "Le format historique une ligne par case que lisent les moteurs de la communauté : un en-tête de taille, puis haut,droite,bas,gauche par case remplie en mots de couleur binaires 16 bits (bordure = 1111111111111111).",
    outBucasTitle: "URL bucas.name (visualiseur communautaire)",
    outBucasHelp: "Un lien e2.bucas.name historique, conservé pour l'interopérabilité avec le visualiseur d'origine de Jef Bucas. Dérivé du même plateau.",
    outE2Title: "e2pieces.txt (pièces telles que posées)",
    outE2Help: "Une ligne par case remplie, dans l'ordre de lecture : quatre numéros de côté (haut, droite, bas, gauche), 0 pour le gris. Ce sont les pièces telles que posées et tournées, pas un catalogue canonique en rotation 0 ; un plateau seul ne permet pas de retrouver l'ordre d'origine des pièces.",
    copy: "Copier",
    copied: "Copié",
    viewerLink: "Vous préférez inspecter et partager un plateau ?",
    viewerLinkCta: "Ouvrir le visualiseur",
  },
};

// A copyable, format-labelled output block: a monospaced value plus its own
// copy button. Value can wrap freely; long strings scroll rather than push the
// page wide.
function OutputBlock({
  title,
  help,
  value,
  copyLabel,
  copiedLabel,
}: {
  title: string;
  help: string;
  value: string;
  copyLabel: string;
  copiedLabel: string;
}) {
  const [copied, setCopied] = useState(false);
  const copy = () => {
    void navigator.clipboard.writeText(value);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };
  return (
    <Card>
      <CardHeader className="flex flex-row items-start justify-between gap-2">
        <div>
          <CardTitle className="text-base font-mono">{title}</CardTitle>
          <p className="mt-1 text-xs text-muted-foreground">{help}</p>
        </div>
        <Button variant="outline" size="sm" onClick={copy} className="shrink-0">
          {copied ? copiedLabel : copyLabel}
        </Button>
      </CardHeader>
      <CardContent>
        <pre className="max-h-56 overflow-auto rounded-md border bg-muted/40 p-2.5 font-mono text-xs break-all whitespace-pre-wrap">
          {value}
        </pre>
      </CardContent>
    </Card>
  );
}

export default function Convert() {
  const t = useT(T);
  const [input, setInput] = useState("");
  const [size, setSize] = useState(16);
  const [board, setBoard] = useState<BucasBoard | null>(null);
  const [error, setError] = useState<string | null>(null);

  const convert = (raw: string, boardSize: number) => {
    const text = raw.trim();
    if (!text) return;
    try {
      // A bare board_edges string (only lowercase letters, no key=value) has no
      // params wrapper, so hand decodeBucas an explicit param map with the size.
      const looksLikeBareEdges = /^[a-z]+$/.test(text) && !text.includes("=");
      const decoded = looksLikeBareEdges
        ? decodeBucas({
            board_edges: text,
            puzzle_size: String(boardSize),
          })
        : decodeBucas(parseParams(text));
      setBoard(decoded);
      setError(null);
    } catch {
      setBoard(null);
      setError(t.badInput);
    }
  };

  const loadExample = () => {
    const kb = KNOWN_BOARDS.find((b) => b.id === EXAMPLE_ID);
    if (!kb) return;
    setInput(kb.params);
    convert(kb.params, size);
  };

  const summary = useMemo(() => (board ? scoreSummary(board) : null), [board]);
  const edges = useMemo(() => (board ? cellsToEdges(board.cells) : ""), [board]);
  const pieces = useMemo(() => (board ? cellsToPieces(board.pieceNumbers) : null), [board]);
  const url = useMemo(() => (board ? viewerUrl(board) : ""), [board]);
  const json = useMemo(() => (board ? canonicalJson(board) : ""), [board]);
  const csv = useMemo(() => (board ? canonicalCsv(board) : ""), [board]);
  const bucas = useMemo(() => (board ? bucasUrl(board) : ""), [board]);
  const e2 = useMemo(() => (board ? cellsToE2Pieces(board.cells) : ""), [board]);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-1 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t.inputTitle}</CardTitle>
          <p className="mt-1 text-xs text-muted-foreground">{t.inputHelp}</p>
        </CardHeader>
        <CardContent className="space-y-3">
          <Textarea
            placeholder={t.placeholder}
            value={input}
            onChange={(e) => setInput(e.target.value)}
            className="min-h-[90px] font-mono text-xs"
          />
          <div className="flex flex-wrap items-center gap-2">
            <label className="flex items-center gap-2 text-sm text-muted-foreground">
              {t.sizeLabel}
              <input
                type="number"
                min={1}
                max={32}
                value={size}
                onChange={(e) => setSize(Math.max(1, Math.min(32, Number(e.target.value) || 1)))}
                className="h-8 w-16 rounded-lg border border-input bg-transparent px-2 text-sm outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
              />
            </label>
            <div className="flex-1" />
            <Button variant="outline" size="sm" onClick={loadExample}>
              {t.loadExample}
            </Button>
            <Button onClick={() => convert(input, size)} disabled={!input.trim()}>
              {t.convert}
            </Button>
          </div>
          {error && <p className="text-sm text-red-500">{error}</p>}
        </CardContent>
      </Card>

      <div className="grid gap-6 lg:grid-cols-[minmax(0,340px)_1fr]">
        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="text-base">{t.previewTitle}</CardTitle>
            </CardHeader>
            <CardContent>
              {board ? (
                <BoardSvg
                  width={board.width}
                  height={board.height}
                  cells={board.cells}
                  className="max-w-full"
                />
              ) : (
                <div className="flex aspect-square items-center justify-center rounded-lg border border-dashed text-sm text-muted-foreground">
                  {t.previewEmpty}
                </div>
              )}
            </CardContent>
          </Card>

          {board && summary && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base">{t.scoreTitle}</CardTitle>
              </CardHeader>
              <CardContent className="space-y-2">
                <div className="flex items-center gap-2">
                  <Badge
                    variant={summary.score === summary.max ? "default" : "secondary"}
                    className="text-base"
                  >
                    {summary.score} / {summary.max}
                  </Badge>
                  <span className="text-sm text-muted-foreground">{t.matchedEdges}</span>
                </div>
                <p className="text-sm text-muted-foreground">
                  {t.piecesPlaced(summary.placed, board.width * board.height)}
                </p>
              </CardContent>
            </Card>
          )}
        </div>

        <div className="space-y-4">
          {board ? (
            <>
              <OutputBlock
                title={t.outEdgesTitle}
                help={t.outEdgesHelp}
                value={edges}
                copyLabel={t.copy}
                copiedLabel={t.copied}
              />
              {pieces ? (
                <OutputBlock
                  title={t.outPiecesTitle}
                  help={t.outPiecesHelp}
                  value={pieces}
                  copyLabel={t.copy}
                  copiedLabel={t.copied}
                />
              ) : (
                <Card>
                  <CardHeader>
                    <CardTitle className="text-base font-mono">{t.outPiecesTitle}</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <p className="text-sm text-muted-foreground">{t.outPiecesNone}</p>
                  </CardContent>
                </Card>
              )}
              <OutputBlock
                title={t.outUrlTitle}
                help={t.outUrlHelp}
                value={url}
                copyLabel={t.copy}
                copiedLabel={t.copied}
              />
              <OutputBlock
                title={t.outJsonTitle}
                help={t.outJsonHelp}
                value={json}
                copyLabel={t.copy}
                copiedLabel={t.copied}
              />
              <OutputBlock
                title={t.outCsvTitle}
                help={t.outCsvHelp}
                value={csv}
                copyLabel={t.copy}
                copiedLabel={t.copied}
              />
              <OutputBlock
                title={t.outE2Title}
                help={t.outE2Help}
                value={e2}
                copyLabel={t.copy}
                copiedLabel={t.copied}
              />
              <OutputBlock
                title={t.outBucasTitle}
                help={t.outBucasHelp}
                value={bucas}
                copyLabel={t.copy}
                copiedLabel={t.copied}
              />
            </>
          ) : (
            <Card>
              <CardContent className="py-8 text-center text-sm text-muted-foreground">
                {t.previewEmpty}
              </CardContent>
            </Card>
          )}
        </div>
      </div>

      <p className="text-sm text-muted-foreground">
        {t.viewerLink}{" "}
        <Link to="/viewer" className="font-medium text-primary underline-offset-4 hover:underline">
          {t.viewerLinkCta}
        </Link>
      </p>
    </div>
  );
}

export const meta = pageMeta("convert");
