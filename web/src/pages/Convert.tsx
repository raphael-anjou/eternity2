import { pageMeta } from "@/seo";
import { useMemo, useState } from "react";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { BoardEditor, type BoardEditorLabels } from "@/components/BoardEditor";
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

// Decode a 16-bit color word from the puzzle CSV back to a color number, the
// inverse of `canonicalCsv`'s encoding: all-ones is the grey border (0), any
// other word is the plain binary value of the interior color.
function colorFromWord(word: string): number | null {
  const w = word.trim();
  if (!/^[01]{1,16}$/.test(w)) return null;
  if (/^1+$/.test(w) && w.length === 16) return 0; // border sentinel 1111111111111111
  return parseInt(w, 2);
}

// The converter accepts any format the page also emits, auto-detected, so a
// board can enter as a URL, a bare edge string, a binary puzzle CSV, an
// e2pieces list, a board_pieces digit blob, or the canonical JSON — and read
// back as all the others. Each branch returns decoded cells + size, or throws.
// `size` seeds a size when the format does not carry one (bare edges, e2pieces).
function parseAnyFormat(raw: string, size: number): BucasBoard {
  const text = raw.trim();
  if (!text) throw new Error("empty");

  // 1. Canonical board JSON (or any JSON carrying board_edges / a viewer url).
  if (text.startsWith("{")) {
    const doc = JSON.parse(text) as {
      board_edges?: string;
      url?: string;
      size?: number;
      name?: string;
      board_pieces?: string;
    };
    if (doc.board_edges) {
      return decodeBucas({
        board_edges: doc.board_edges,
        puzzle_size: String(doc.size ?? size),
        ...(doc.name ? { puzzle: doc.name } : {}),
        ...(doc.board_pieces ? { board_pieces: doc.board_pieces } : {}),
      });
    }
    if (doc.url) return decodeBucas(parseParams(doc.url));
    throw new Error("json has no board_edges or url");
  }

  // 2. Anything with key=value params (a full URL, a hash, a params string).
  if (text.includes("board_edges=") || text.includes("=")) {
    return decodeBucas(parseParams(text));
  }

  // 3. A bare board_edges letter string: only lowercase letters, no separators.
  if (/^[a-z]+$/.test(text)) {
    return decodeBucas({ board_edges: text, puzzle_size: String(size) });
  }

  // 4. A board_pieces digit blob: only digits, length a multiple of 3. Piece
  //    numbers alone can't render edges, but we surface them as the placement.
  if (/^\d+$/.test(text) && text.length % 3 === 0) {
    const n = text.length / 3;
    const w = Math.round(Math.sqrt(n));
    if (w * w !== n) throw new Error("board_pieces length is not a square board");
    const pieceNumbers = Array.from({ length: n }, (_, i) => parseInt(text.slice(i * 3, i * 3 + 3), 10));
    return { width: w, height: w, cells: Array.from({ length: n }, () => null), pieceNumbers, puzzleName: null };
  }

  // 5. Line-oriented: either the binary puzzle CSV (comma-separated 16-bit
  //    words, optional size header) or e2pieces.txt (whitespace-separated small
  //    integers, one cell per line). Detect by the separator inside a data row.
  const lines = text.split(/\r?\n/).map((l) => l.trim()).filter((l) => l.length > 0);
  const first = lines[0];
  if (first !== undefined) {
    // Drop a lone leading size header (a single integer on the first line).
    let header = size;
    let rows = lines;
    if (/^\d+$/.test(first) && !first.includes(",")) {
      header = parseInt(first, 10);
      rows = lines.slice(1);
    }
    const isBinaryCsv = rows.some((l) => l.includes(",") && /[01]{4,}/.test(l));
    const cellsList: (Edges | null)[] = [];
    for (const line of rows) {
      const parts = isBinaryCsv ? line.split(",") : line.split(/[\s,]+/);
      if (parts.length !== 4) throw new Error(`expected 4 edges per cell, got "${line}"`);
      const quad = parts.map((p) => (isBinaryCsv ? colorFromWord(p) : (/^\d+$/.test(p) ? parseInt(p, 10) : null)));
      if (quad.some((c) => c == null)) throw new Error(`bad edge in "${line}"`);
      const e = quad as Edges;
      cellsList.push(e.every((c) => c === 0) ? null : e);
    }
    const n = cellsList.length;
    // These formats list only filled cells in reading order. Size = the header,
    // else the square that fits, else pad to the requested size.
    const w = header && header * header >= n ? header : Math.max(size, Math.ceil(Math.sqrt(n)));
    while (cellsList.length < w * w) cellsList.push(null);
    return { width: w, height: w, cells: cellsList, pieceNumbers: null, puzzleName: null };
  }

  throw new Error("unrecognised format");
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
    inputTitle: "Paste a board — any format",
    inputHelp: (
      <>
        Any format this page also emits, auto-detected: a viewer or{" "}
        <code>e2.bucas.name</code> link, a params string, a bare{" "}
        <code>board_edges</code> letter string, a binary puzzle <code>CSV</code>,
        an <code>e2pieces.txt</code> list, a <code>board_pieces</code> digit
        string, or the canonical board <code>JSON</code>. For a format that
        carries no size (a bare edge string, an e2pieces list), set the size
        below. Then click any cell to edit it.
      </>
    ),
    placeholder:
      "Paste any format: a viewer/bucas URL, board_edges letters, a binary CSV, an e2pieces list, a board_pieces digit string, or board JSON",
    sizeLabel: "Board size (for a format without one)",
    convert: "Convert",
    loadExample: "Load an example (Blackwood 470)",
    badInput: "Could not read that input. It should be a viewer/bucas URL, a board_edges string, a binary puzzle CSV, an e2pieces list, a board_pieces digit string, or board JSON (set the size for formats that lack one).",
    previewEmpty: "Convert a board to see it here.",
    scoreTitle: "Score",
    matchedEdges: "matched edges",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} cells filled`,
    outEdgesTitle: "board_edges",
    outEdgesHelp: "Four lowercase letters per cell, row by row (URDL: up, right, down, left). 'a' is the grey border; \"aaaa\" is an empty cell.",
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
    editorTitle: "Edit a cell",
    editHint: "Click any cell to edit its four edge colours. Invalid values are flagged in red and left out of the outputs until fixed.",
    selectPrompt: "Click a cell on the board to edit its edges.",
    cellLabel: (row: string, col: number) => `Cell ${row}${col} — edges (U, R, D, L)`,
    dirU: "Up",
    dirR: "Right",
    dirD: "Down",
    dirL: "Left",
    clearCell: "Clear cell",
    faultNan: "whole number only",
    faultRange: (max: number) => `0 to ${max}`,
    faultsSummary: (n: number) => `${n} cell${n === 1 ? "" : "s"} with an out-of-range edge.`,
    noFaults: "Every filled cell is valid.",
    resetBoard: "Clear board",
    startBlank: "Start a blank board",
    editTab: "Edit the board",
    fieldInvalid: "Could not read a board from this. It stays as you typed it; the fields below still show the last valid board.",
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
    inputTitle: "Collez un plateau — n'importe quel format",
    inputHelp: (
      <>
        Tout format que cette page produit aussi, détecté automatiquement : un
        lien du visualiseur ou <code>e2.bucas.name</code>, une chaîne de
        paramètres, une chaîne de lettres <code>board_edges</code>, un{" "}
        <code>CSV</code> binaire, une liste <code>e2pieces.txt</code>, une chaîne
        de chiffres <code>board_pieces</code>, ou le <code>JSON</code> canonique.
        Pour un format sans taille (chaîne de côtés seule, liste e2pieces),
        indiquez la taille ci-dessous. Cliquez ensuite sur une case pour la
        modifier.
      </>
    ),
    placeholder:
      "Collez n'importe quel format : une URL visualiseur/bucas, des lettres board_edges, un CSV binaire, une liste e2pieces, une chaîne board_pieces, ou un JSON de plateau",
    sizeLabel: "Taille du plateau (pour un format qui n'en porte pas)",
    convert: "Convertir",
    loadExample: "Charger un exemple (Blackwood 470)",
    badInput: "Lecture impossible. Attendu : une URL visualiseur/bucas, une chaîne board_edges, un CSV binaire, une liste e2pieces, une chaîne board_pieces, ou un JSON de plateau (indiquez la taille pour les formats qui n'en portent pas).",
    previewEmpty: "Convertissez un plateau pour le voir ici.",
    scoreTitle: "Score",
    matchedEdges: "côtés appariés",
    piecesPlaced: (placed: number, total: number) => `${placed} / ${total} cases remplies`,
    outEdgesTitle: "board_edges",
    outEdgesHelp: "Quatre lettres minuscules par case, ligne par ligne (URDL : haut, droite, bas, gauche). « a » est la bordure grise ; « aaaa » est une case vide.",
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
    editorTitle: "Modifier une case",
    editHint: "Cliquez sur une case pour modifier ses quatre couleurs de côté. Les valeurs invalides sont signalées en rouge et exclues des sorties jusqu'à correction.",
    selectPrompt: "Cliquez sur une case du plateau pour modifier ses côtés.",
    cellLabel: (row: string, col: number) => `Case ${row}${col} — côtés (H, D, B, G)`,
    dirU: "Haut",
    dirR: "Droite",
    dirD: "Bas",
    dirL: "Gauche",
    clearCell: "Vider la case",
    faultNan: "entier uniquement",
    faultRange: (max: number) => `de 0 à ${max}`,
    faultsSummary: (n: number) => `${n} case${n === 1 ? "" : "s"} avec un côté hors plage.`,
    noFaults: "Chaque case remplie est valide.",
    resetBoard: "Vider le plateau",
    startBlank: "Partir d'un plateau vide",
    editTab: "Modifier le plateau",
    fieldInvalid: "Lecture impossible ici. Le texte reste tel quel ; les champs ci-dessous montrent toujours le dernier plateau valide.",
  },
};

// A two-way, format-labelled field: it shows the board rendered into this
// format, and it is editable — typing a board in this format re-derives every
// other field. Editing keeps the raw text; when it parses, the board updates
// and this field snaps to the canonical rendering; when it does not, a red
// message names the fault and the board is left untouched. A field can also be
// read-only (the derived URLs), in which case it is a copyable value with no
// parsing.
function FormatField({
  title,
  help,
  value,
  onCommit,
  readOnly,
  invalidLabel,
  copyLabel,
  copiedLabel,
}: {
  title: string;
  help: string;
  value: string;
  onCommit?: (text: string) => boolean; // returns true if the text parsed
  readOnly?: boolean;
  invalidLabel: string;
  copyLabel: string;
  copiedLabel: string;
}) {
  const [copied, setCopied] = useState(false);
  // While the user is typing, the field is "dirty": it shows their draft, not
  // the derived value. On a parse it goes clean and follows the board again.
  const [draft, setDraft] = useState<string | null>(null);
  const [invalid, setInvalid] = useState(false);

  const shown = draft ?? value;

  const copy = () => {
    void navigator.clipboard.writeText(shown);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  const onType = (text: string) => {
    setDraft(text);
    if (!onCommit) return;
    if (text.trim() === "") {
      setInvalid(false);
      return;
    }
    const ok = onCommit(text);
    setInvalid(!ok);
    if (ok) setDraft(null); // parsed: follow the board again
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
      <CardContent className="space-y-1.5">
        {readOnly ? (
          <pre className="max-h-56 overflow-auto rounded-md border bg-muted/40 p-2.5 font-mono text-xs break-all whitespace-pre-wrap">
            {shown}
          </pre>
        ) : (
          <textarea
            value={shown}
            onChange={(e) => onType(e.target.value)}
            spellCheck={false}
            className={
              "block max-h-56 min-h-[72px] w-full overflow-auto rounded-md border bg-muted/40 p-2.5 font-mono text-xs break-all whitespace-pre-wrap outline-none focus-visible:ring-3 focus-visible:ring-ring/50 " +
              (invalid ? "border-red-500 focus-visible:border-red-500" : "border-input focus-visible:border-ring")
            }
          />
        )}
        {invalid && <p className="text-xs text-red-500">{invalidLabel}</p>}
      </CardContent>
    </Card>
  );
}

export default function Convert() {
  const t = useT(T);
  const [input, setInput] = useState("");
  const [size, setSize] = useState(16);
  // The board is held as its editable cells plus the metadata a decoded board
  // carries (name, piece numbers). Editing a cell, pasting a board, or starting
  // blank all funnel through here, and every output below is derived from it, so
  // a hand-edited board round-trips through the same formats as a pasted one.
  const [cells, setCells] = useState<(Edges | null)[] | null>(null);
  const [meta, setMeta] = useState<{ width: number; height: number; puzzleName: string | null; pieceNumbers: number[] | null }>(
    { width: 16, height: 16, puzzleName: null, pieceNumbers: null },
  );
  const [error, setError] = useState<string | null>(null);

  // The live board object every output consumes. Editing drops piece numbers,
  // because a hand-changed edge no longer corresponds to the pasted piece id.
  const board: BucasBoard | null = useMemo(
    () =>
      cells
        ? {
            width: meta.width,
            height: meta.height,
            cells,
            pieceNumbers: meta.pieceNumbers,
            puzzleName: meta.puzzleName,
          }
        : null,
    [cells, meta],
  );

  const convert = (raw: string, boardSize: number) => {
    const text = raw.trim();
    if (!text) return;
    try {
      // Auto-detect the input format: a URL/params string, a bare edge string, a
      // binary puzzle CSV, an e2pieces list, a board_pieces blob, or board JSON.
      const decoded = parseAnyFormat(text, boardSize);
      setCells(decoded.cells);
      setMeta({
        width: decoded.width,
        height: decoded.height,
        puzzleName: decoded.puzzleName,
        pieceNumbers: decoded.pieceNumbers,
      });
      setError(null);
    } catch {
      setCells(null);
      setError(t.badInput);
    }
  };

  const loadExample = () => {
    const kb = KNOWN_BOARDS.find((b) => b.id === EXAMPLE_ID);
    if (!kb) return;
    setInput(kb.params);
    convert(kb.params, size);
  };

  // Start (or reset to) a blank board at the current size, ready to hand-build.
  const startBlank = () => {
    const n = size * size;
    setCells(Array.from({ length: n }, () => null));
    setMeta({ width: size, height: size, puzzleName: null, pieceNumbers: null });
    setError(null);
  };

  // An edit invalidates the pasted piece numbers, so drop them on first change.
  const onEdit = (next: (Edges | null)[]) => {
    setCells(next);
    setMeta((m) => (m.pieceNumbers ? { ...m, pieceNumbers: null } : m));
  };

  // Commit a board typed into one of the format fields. Every format is
  // auto-detected by the same parser the paste box uses, so a field for any one
  // format accepts a board in that format and re-derives all the others. Returns
  // whether the text parsed, so the field can flag itself red on a bad edit.
  const commitFrom = (text: string): boolean => {
    try {
      const decoded = parseAnyFormat(text, size);
      setCells(decoded.cells);
      setMeta({
        width: decoded.width,
        height: decoded.height,
        puzzleName: decoded.puzzleName,
        pieceNumbers: decoded.pieceNumbers,
      });
      setError(null);
      return true;
    } catch {
      return false;
    }
  };

  const editorLabels: BoardEditorLabels = {
    editorTitle: t.editorTitle,
    editHint: t.editHint,
    selectPrompt: t.selectPrompt,
    cellLabel: t.cellLabel,
    dir: { u: t.dirU, r: t.dirR, d: t.dirD, l: t.dirL },
    clearCell: t.clearCell,
    faultNan: t.faultNan,
    faultRange: t.faultRange,
    faultsSummary: t.faultsSummary,
    noFaults: t.noFaults,
    reset: t.resetBoard,
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
            <Button variant="ghost" size="sm" onClick={startBlank}>
              {t.startBlank}
            </Button>
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

      <Card>
        <CardHeader className="flex flex-row items-center justify-between gap-2">
          <CardTitle className="text-base">{t.editTab}</CardTitle>
          {!board && (
            <Button variant="outline" size="sm" onClick={startBlank}>
              {t.startBlank}
            </Button>
          )}
        </CardHeader>
        <CardContent>
          {board ? (
            <BoardEditor
              width={board.width}
              height={board.height}
              cells={board.cells}
              onChange={onEdit}
              onReset={startBlank}
              labels={editorLabels}
            />
          ) : (
            <div className="flex aspect-[2/1] items-center justify-center rounded-lg border border-dashed text-sm text-muted-foreground">
              {t.previewEmpty}
            </div>
          )}
        </CardContent>
      </Card>

      <div className="grid gap-6 lg:grid-cols-[minmax(0,300px)_1fr]">
        <div className="space-y-4">
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
          {/* Every field is editable and empty until a board exists. Typing a
              board in any one format re-derives all the others; the derived
              URLs are copy-only since they are not a canonical input format. */}
          <FormatField
            title={t.outEdgesTitle}
            help={t.outEdgesHelp}
            value={edges}
            onCommit={commitFrom}
            invalidLabel={t.fieldInvalid}
            copyLabel={t.copy}
            copiedLabel={t.copied}
          />
          <FormatField
            title={t.outPiecesTitle}
            help={pieces ? t.outPiecesHelp : t.outPiecesNone}
            value={pieces ?? ""}
            onCommit={commitFrom}
            invalidLabel={t.fieldInvalid}
            copyLabel={t.copy}
            copiedLabel={t.copied}
          />
          <FormatField
            title={t.outJsonTitle}
            help={t.outJsonHelp}
            value={json}
            onCommit={commitFrom}
            invalidLabel={t.fieldInvalid}
            copyLabel={t.copy}
            copiedLabel={t.copied}
          />
          <FormatField
            title={t.outCsvTitle}
            help={t.outCsvHelp}
            value={csv}
            onCommit={commitFrom}
            invalidLabel={t.fieldInvalid}
            copyLabel={t.copy}
            copiedLabel={t.copied}
          />
          <FormatField
            title={t.outE2Title}
            help={t.outE2Help}
            value={e2}
            onCommit={commitFrom}
            invalidLabel={t.fieldInvalid}
            copyLabel={t.copy}
            copiedLabel={t.copied}
          />
          <FormatField
            title={t.outUrlTitle}
            help={t.outUrlHelp}
            value={url}
            onCommit={commitFrom}
            invalidLabel={t.fieldInvalid}
            copyLabel={t.copy}
            copiedLabel={t.copied}
          />
          <FormatField
            title={t.outBucasTitle}
            help={t.outBucasHelp}
            value={bucas}
            onCommit={commitFrom}
            invalidLabel={t.fieldInvalid}
            copyLabel={t.copy}
            copiedLabel={t.copied}
          />
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
