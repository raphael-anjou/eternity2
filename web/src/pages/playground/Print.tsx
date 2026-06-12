// Print real puzzles, PowerPoint-style: drop puzzle blocks onto A4 pages,
// drag them anywhere, resize them with the corner handle, then print
// double-sided and cut along the lines. The back of each piece carries its
// tray number and a short set code (size-colors-seed in base36), so mixed-up
// sets can be sorted and any set can be regenerated on the site.
//
// Duplex correctness: back sheets mirror every block's X position AND the
// columns inside each grid, so a long-edge duplex print puts every ID exactly
// behind its piece. Tray numbers are shuffled-order ids, never solution
// positions: the printed sheet does not spoil the puzzle.
//
// Printing goes through a body-level portal: ancestor layout (the app shell)
// would otherwise shift and scale the sheets in the print pipeline.

import { useEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useEngine } from "@/engine/useEngine";
import { getGeneratedPuzzle, getMaxColors } from "@/engine";
import { MotifDefs } from "@/components/board/MotifDefs";
import { DIRECTION_ROTATION } from "@/lib/motifs";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";

const PAGE_W = 210;
const PAGE_H = 297;
const MARGIN = 8; // blocks may not leave this safety border
const CAPTION_MM = 5;
const MIN_CELL_MM = 12;

const T = {
  en: {
    title: "Print and cut",
    intro:
      "Build real, hold-in-your-hands puzzles: add blocks, drag them anywhere on the page, resize them by their corner, then print double-sided and cut along the lines. The back of every piece shows its number and the set code, so mixed-up sets can be sorted out. The numbers never reveal the solution.",
    addCustom: "Add",
    sizeLabel: "Size",
    addPage: "+ Page",
    sheets: (n: number) => (n === 1 ? "1 sheet" : `${n} sheets`),
    duplexHint:
      "Print at 100% scale, double-sided, flip on long edge. Fronts and backs alternate so each piece gets its ID printed behind it.",
    dragHint: "Drag blocks to move them (even across pages); drag the blue corner to resize. Click selects.",
    print: "Print",
    empty: "Add a puzzle block to start.",
    colors: "Colors",
    duplicate: "Duplicate on next page",
    newSeed: "Reroll",
    set: "Set",
    answerKey: "Answer key:",
    cutHint: "cut along the lines",
    front: "front",
    back: "back (mirrored)",
    pageLabel: (i: number, n: number) => `page ${i} of ${n} (front + back)`,
    removePage: "Remove this page (its blocks move to the previous page)",
    loading: "Loading engine…",
  },
  fr: {
    title: "Imprimer et découper",
    intro:
      "Fabriquez de vrais puzzles à tenir en main : ajoutez des blocs, déplacez-les où vous voulez sur la page, redimensionnez-les par leur coin, puis imprimez en recto verso et découpez le long des lignes. Le dos de chaque pièce porte son numéro et le code du jeu, pour retrier des jeux mélangés. Les numéros ne révèlent jamais la solution.",
    addCustom: "Ajouter",
    sizeLabel: "Taille",
    addPage: "+ Page",
    sheets: (n: number) => (n === 1 ? "1 feuille" : `${n} feuilles`),
    duplexHint:
      "Imprimez à 100 %, en recto verso, retournement sur le bord long. Les rectos et versos alternent pour que chaque pièce reçoive son numéro au dos.",
    dragHint:
      "Faites glisser les blocs pour les déplacer (même d'une page à l'autre) ; tirez le coin bleu pour les redimensionner. Un clic sélectionne.",
    print: "Imprimer",
    empty: "Ajoutez un bloc de puzzle pour commencer.",
    colors: "Couleurs",
    duplicate: "Dupliquer sur la page suivante",
    newSeed: "Régénérer",
    set: "Jeu",
    answerKey: "Solution :",
    cutHint: "découpez le long des lignes",
    front: "recto",
    back: "verso (en miroir)",
    pageLabel: (i: number, n: number) => `page ${i} sur ${n} (recto + verso)`,
    removePage: "Supprimer cette page (ses blocs passent sur la page précédente)",
    loading: "Chargement du moteur…",
  },
};

interface Block {
  uid: number;
  size: number;
  colors: number;
  seed: number;
  cellMm: number;
  page: number; // 0-based
  xMm: number;
  yMm: number;
}

const blockW = (b: Block) => b.size * b.cellMm;
const blockH = (b: Block) => b.size * b.cellMm + CAPTION_MM;
const maxCellMm = (size: number) =>
  Math.floor(Math.min((PAGE_W - 2 * MARGIN) / size, (PAGE_H - 2 * MARGIN - CAPTION_MM) / size));

function setCode(b: { size: number; colors: number; seed: number }): string {
  return `${b.size}x${b.size}-${b.colors}c-${b.seed.toString(36).toUpperCase()}`;
}

function answerKeyUrl(b: Block): string {
  return `https://eternity2.dev/#/viewer?g=${b.size}.${b.colors}.${b.seed}`;
}

function clampToPage(b: Block): Block {
  const x = Math.min(Math.max(b.xMm, MARGIN), Math.max(MARGIN, PAGE_W - MARGIN - blockW(b)));
  const y = Math.min(Math.max(b.yMm, MARGIN), Math.max(MARGIN, PAGE_H - MARGIN - blockH(b)));
  return { ...b, xMm: x, yMm: y };
}

function PrintGrid({ block, back }: { block: Block; back: boolean }) {
  const engineReady = useEngine();
  const { size, cellMm } = block;
  const C = 100;
  const code = setCode(block);
  const puzzle = useMemo(
    () => (engineReady ? getGeneratedPuzzle(block.size, block.colors, block.seed) : null),
    [engineReady, block.size, block.colors, block.seed],
  );
  if (!puzzle) return null;
  return (
    <svg
      width={`${size * cellMm}mm`}
      height={`${size * cellMm}mm`}
      viewBox={`0 0 ${size * C} ${size * C}`}
      style={{ display: "block" }}
    >
      {Array.from({ length: size * size }, (_, i) => {
        const row = Math.floor(i / size);
        const col = i % size;
        // Back sheets mirror columns so duplex printing aligns IDs behind
        // their pieces.
        const srcIndex = back ? row * size + (size - 1 - col) : i;
        const cx = col * C + C / 2;
        const cy = row * C + C / 2;
        if (back) {
          return (
            <g key={i}>
              <text x={cx} y={cy} textAnchor="middle" fontSize={C * 0.3} fontWeight={700} fontFamily="monospace">
                #{srcIndex + 1}
              </text>
              <text x={cx} y={cy + C * 0.24} textAnchor="middle" fontSize={C * 0.11} fontFamily="monospace" fill="#444">
                {code}
              </text>
            </g>
          );
        }
        const edges = puzzle.pieces[srcIndex];
        return (
          <g key={i} transform={`translate(${cx} ${cy}) scale(${C / 256})`}>
            {edges.map((c, dir) => (
              <use key={dir} href={`#pe2m-${c}`} transform={`rotate(${DIRECTION_ROTATION[dir]})`} />
            ))}
          </g>
        );
      })}
      {Array.from({ length: size + 1 }, (_, i) => (
        <g key={i} stroke="#000" strokeWidth={back ? 0.6 : 1.2}>
          <line x1={i * C} y1={0} x2={i * C} y2={size * C} />
          <line x1={0} y1={i * C} x2={size * C} y2={i * C} />
        </g>
      ))}
    </svg>
  );
}

/** One physical face (front or back) with absolutely positioned blocks. */
function Sheet({
  blocks,
  back,
  label,
  t,
  interactive,
}: {
  blocks: Block[];
  back: boolean;
  label: string;
  t: (typeof T)["en"];
  interactive?: {
    selected: number | null;
    onSelect: (uid: number) => void;
    onDragStart: (uid: number, e: React.PointerEvent) => void;
    onResizeStart: (uid: number, e: React.PointerEvent) => void;
  };
}) {
  return (
    <div
      className="print-sheet relative bg-white text-black shadow-md"
      style={{ width: `${PAGE_W}mm`, height: `${PAGE_H}mm` }}
    >
      {blocks.map((b) => {
        const w = blockW(b);
        const x = back ? PAGE_W - b.xMm - w : b.xMm;
        const isSel = interactive?.selected === b.uid;
        return (
          <div
            key={b.uid}
            className={cn(
              "absolute",
              interactive && "cursor-grab active:cursor-grabbing",
              isSel && "outline-2 outline-offset-2 outline-sky-500",
            )}
            style={{ left: `${x}mm`, top: `${b.yMm}mm`, width: `${w}mm` }}
            onPointerDown={
              interactive
                ? (e) => {
                    interactive.onSelect(b.uid);
                    interactive.onDragStart(b.uid, e);
                  }
                : undefined
            }
          >
            <div
              className="flex items-baseline justify-between font-mono"
              style={{ fontSize: "2.6mm", height: `${CAPTION_MM}mm` }}
            >
              <span>
                {t.set} {setCode(b)} · {back ? t.back : t.front}
              </span>
              {!back && <span>{t.cutHint}</span>}
            </div>
            <PrintGrid block={b} back={back} />
            {!back && (
              <div className="truncate font-mono" style={{ fontSize: "2.2mm", marginTop: "0.8mm" }}>
                {t.answerKey} {answerKeyUrl(b)}
              </div>
            )}
            {interactive && isSel && !back && (
              <div
                data-resize-handle
                className="absolute -right-2 -bottom-2 h-4 w-4 cursor-nwse-resize rounded-sm border border-white bg-sky-500"
                onPointerDown={(e) => {
                  e.stopPropagation();
                  interactive.onResizeStart(b.uid, e);
                }}
              />
            )}
          </div>
        );
      })}
      <div
        className="absolute font-mono"
        style={{ right: `${MARGIN}mm`, bottom: "4mm", fontSize: "2.6mm" }}
      >
        eternity2.dev · {label}
      </div>
    </div>
  );
}

export default function Print() {
  const t = useT(T);
  const engineReady = useEngine();
  const [blocks, setBlocks] = useState<Block[]>([]);
  const [pageCountState, setPageCountState] = useState(1);
  const [selected, setSelected] = useState<number | null>(null);
  const [nextUid, setNextUid] = useState(1);
  const [addSize, setAddSize] = useState(4);
  const [addColors, setAddColors] = useState(5);
  // Live drag/resize state; window-level pointermove updates blocks state.
  const gesture = useRef<{
    kind: "move" | "resize";
    uid: number;
    startX: number;
    startY: number;
    orig: Block;
    pxPerMm: number;
  } | null>(null);

  // Printing must hide the app shell and print only the body portal.
  useEffect(() => {
    document.body.classList.add("print-sheets-page");
    return () => document.body.classList.remove("print-sheets-page");
  }, []);

  const pageCount = Math.max(1, pageCountState, ...blocks.map((b) => b.page + 1));

  const removePage = (i: number) => {
    setBlocks((prev) =>
      prev.map((b) =>
        b.page === i ? { ...b, page: Math.max(0, i - 1) } : b.page > i ? { ...b, page: b.page - 1 } : b,
      ),
    );
    setPageCountState(Math.max(1, pageCount - 1));
  };

  const addBlock = (size: number, colors?: number) => {
    const cellMm = Math.min(30, maxCellMm(size));
    const onLast = blocks.filter((b) => b.page === pageCount - 1).length;
    setBlocks((prev) => [
      ...prev,
      clampToPage({
        uid: nextUid,
        size,
        colors: Math.min(colors ?? Math.min(size + 1, 8), getMaxColors(size)),
        seed: Math.floor(Math.random() * 1_000_000),
        cellMm,
        page: pageCount - 1,
        xMm: MARGIN + 6 * onLast,
        yMm: MARGIN + 6 * onLast,
      }),
    ]);
    setSelected(nextUid);
    setNextUid((u) => u + 1);
  };

  const update = (uid: number, patch: Partial<Block>) =>
    setBlocks((prev) => prev.map((b) => (b.uid === uid ? clampToPage({ ...b, ...patch }) : b)));

  const startGesture = (kind: "move" | "resize") => (uid: number, e: React.PointerEvent) => {
    const block = blocks.find((b) => b.uid === uid);
    const pageEl = (e.target as HTMLElement).closest(".print-sheet");
    if (!block || !pageEl) return;
    e.preventDefault();
    gesture.current = {
      kind,
      uid,
      startX: e.clientX,
      startY: e.clientY,
      orig: block,
      pxPerMm: pageEl.getBoundingClientRect().width / PAGE_W,
    };
  };

  useEffect(() => {
    const move = (e: PointerEvent) => {
      const g = gesture.current;
      if (!g) return;
      const dxMm = (e.clientX - g.startX) / g.pxPerMm;
      const dyMm = (e.clientY - g.startY) / g.pxPerMm;
      if (g.kind === "move") {
        // Reparent to whichever page the pointer is over.
        const el = document.elementFromPoint(e.clientX, e.clientY);
        const pageEl = el?.closest("[data-page-index]") as HTMLElement | null;
        const page = pageEl ? parseInt(pageEl.dataset["pageIndex"] ?? "0", 10) : g.orig.page;
        update(g.uid, { xMm: g.orig.xMm + dxMm, yMm: g.orig.yMm + dyMm, page });
      } else {
        const newW = Math.max(
          MIN_CELL_MM * g.orig.size,
          Math.min(blockW(g.orig) + dxMm, maxCellMm(g.orig.size) * g.orig.size),
        );
        update(g.uid, { cellMm: newW / g.orig.size });
      }
    };
    const up = () => {
      gesture.current = null;
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", up);
    return () => {
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", up);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [blocks]);

  const pages = useMemo(
    () => Array.from({ length: pageCount }, (_, i) => blocks.filter((b) => b.page === i)),
    [blocks, pageCount],
  );

  const selectedBlock = blocks.find((b) => b.uid === selected) ?? null;

  return (
    <div className="space-y-6">
      <div className="print:hidden">
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-1 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-center gap-2 print:hidden">
        <Label className="text-xs">{t.sizeLabel}</Label>
        <Select
          value={String(addSize)}
          onValueChange={(v) => {
            if (!v) return;
            const s = parseInt(v, 10);
            setAddSize(s);
            setAddColors((c) => Math.min(c, getMaxColors(s)));
          }}
        >
          <SelectTrigger className="w-20">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {Array.from({ length: 8 }, (_, i) => i + 2).map((s) => (
              <SelectItem key={s} value={String(s)}>
                {s}×{s}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Label className="text-xs">{t.colors}</Label>
        <Select value={String(addColors)} onValueChange={(v) => v && setAddColors(parseInt(v, 10))}>
          <SelectTrigger className="w-18">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {engineReady &&
              Array.from({ length: Math.max(1, getMaxColors(addSize) - 1) }, (_, i) => i + 2).map((c) => (
                <SelectItem key={c} value={String(c)}>
                  {c}
                </SelectItem>
              ))}
          </SelectContent>
        </Select>
        <Button variant="secondary" onClick={() => addBlock(addSize, addColors)} disabled={!engineReady}>
          {t.addCustom}
        </Button>
        <Button variant="outline" onClick={() => setPageCountState(pageCount + 1)}>
          {t.addPage}
        </Button>
        {blocks.length > 0 && (
          <>
            <span className="text-sm text-muted-foreground">{t.sheets(pageCount * 2)}</span>
            <Button className="ml-auto" onClick={() => window.print()}>
              🖨 {t.print}
            </Button>
          </>
        )}
      </div>

      {!engineReady ? (
        <p className="py-12 text-center text-muted-foreground print:hidden">{t.loading}</p>
      ) : blocks.length === 0 ? (
        <p className="py-12 text-center text-muted-foreground print:hidden">{t.empty}</p>
      ) : (
        <div className="space-y-4 print:hidden">
          {selectedBlock ? (
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="flex flex-wrap items-center gap-3 text-sm">
                  <span>
                    {selectedBlock.size}×{selectedBlock.size} · {setCode(selectedBlock)} ·{" "}
                    {Math.round(selectedBlock.cellMm)} mm
                  </span>
                  <span className="flex flex-wrap gap-1.5">
                    <Select
                      value={String(selectedBlock.colors)}
                      onValueChange={(v) => v && update(selectedBlock.uid, { colors: parseInt(v, 10) })}
                    >
                      <SelectTrigger className="h-7 w-24">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {Array.from(
                          { length: Math.max(1, getMaxColors(selectedBlock.size) - 1) },
                          (_, i) => i + 2,
                        ).map((c) => (
                          <SelectItem key={c} value={String(c)}>
                            {c} {t.colors.toLowerCase()}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <Button
                      variant="outline"
                      size="xs"
                      onClick={() =>
                        update(selectedBlock.uid, { seed: Math.floor(Math.random() * 1_000_000) })
                      }
                    >
                      🎲 {t.newSeed}
                    </Button>
                    <Button
                      variant="outline"
                      size="xs"
                      onClick={() => {
                        setBlocks((prev) => [
                          ...prev,
                          {
                            ...selectedBlock,
                            uid: nextUid,
                            page: selectedBlock.page + 1,
                            seed: Math.floor(Math.random() * 1_000_000),
                          },
                        ]);
                        setPageCountState((c) => Math.max(c, selectedBlock.page + 2));
                        setNextUid((u) => u + 1);
                      }}
                    >
                      ⧉ {t.duplicate}
                    </Button>
                    <Button
                      variant="ghost"
                      size="xs"
                      onClick={() => {
                        setBlocks((prev) => prev.filter((b) => b.uid !== selectedBlock.uid));
                        setSelected(null);
                      }}
                    >
                      ✕
                    </Button>
                  </span>
                </CardTitle>
              </CardHeader>
              <CardContent className="pt-0 text-xs text-muted-foreground">{t.dragHint}</CardContent>
            </Card>
          ) : (
            <p className="text-xs text-muted-foreground">{t.dragHint}</p>
          )}

          <MotifDefs prefix="pe2m" />
          <div className="space-y-6 overflow-x-auto pb-4">
            {pages.map((pageBlocks, i) => (
              <div key={i} className="space-y-1">
                <p className="flex items-center gap-2 text-xs text-muted-foreground">
                  {t.pageLabel(i + 1, pageCount)}
                  {pageCount > 1 && (
                    <button
                      onClick={() => removePage(i)}
                      className="rounded border px-1.5 leading-relaxed hover:bg-muted"
                      title={t.removePage}
                    >
                      ✕
                    </button>
                  )}
                </p>
                <div className="flex flex-wrap items-start gap-4">
                  <div className="print-sheet-scale touch-none select-none" data-page-index={i}>
                    <Sheet
                      blocks={pageBlocks}
                      back={false}
                      label={`${i * 2 + 1} / ${pageCount * 2}`}
                      t={t}
                      interactive={{
                        selected,
                        onSelect: setSelected,
                        onDragStart: startGesture("move"),
                        onResizeStart: startGesture("resize"),
                      }}
                    />
                  </div>
                  <div className="print-sheet-scale hidden 2xl:block">
                    <Sheet blocks={pageBlocks} back={true} label={`${i * 2 + 2} / ${pageCount * 2}`} t={t} />
                  </div>
                </div>
              </div>
            ))}
          </div>
          <p className="text-xs text-muted-foreground">{t.duplexHint}</p>
        </div>
      )}

      {blocks.length > 0 &&
        createPortal(
          <div className="print-portal">
            <MotifDefs prefix="pe2m" />
            {pages.map((pageBlocks, i) => (
              <div key={i}>
                <Sheet blocks={pageBlocks} back={false} label={`${i * 2 + 1} / ${pageCount * 2}`} t={t} />
                <Sheet blocks={pageBlocks} back={true} label={`${i * 2 + 2} / ${pageCount * 2}`} t={t} />
              </div>
            ))}
          </div>,
          document.body,
        )}
    </div>
  );
}
