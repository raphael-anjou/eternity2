import { useEffect, useState } from "react";
import { useT } from "@/i18n";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useRunWhileVisible } from "@/lib/useRunWhileVisible";

// Régin's all-different filter, made watchable on a fixed 6-cell / 6-piece
// instance. Phase 1 animates Kuhn's augmenting-path construction of a maximum
// matching (conflicts, re-routing, path flips). Phase 2 orients the graph
// around the matching, colours its strongly connected components, and deletes
// every unmatched edge that crosses two components — exactly the edges that
// belong to no maximum matching (Berge / Régin 1994). The instance is chosen
// so the filter forces two cells that plain crossing-off would never catch.
// Fully precomputed and deterministic: the component just replays frames.

const NN = 6;

/** Cell index -> feasible piece indices. Crafted so C1,C2 form a Hall set on
 *  {P1,P2}, which makes C3-P2, C4-P3 and C6-P5 removable (in no matching). */
const CELL_DOMAINS: number[][] = [
  [0, 1], // C1: P1 P2
  [0, 1], // C2: P1 P2
  [1, 2], // C3: P2 P3
  [2, 3, 4], // C4: P3 P4 P5
  [3, 4], // C5: P4 P5
  [4, 5], // C6: P5 P6
];

type Note =
  | { k: "start" }
  | { k: "consider"; c: number }
  | { k: "tryFree"; c: number; p: number }
  | { k: "conflict"; c: number; p: number; owner: number }
  | { k: "deadEdge"; c: number; p: number }
  | { k: "augment"; c: number; len: number }
  | { k: "matchedAll" }
  | { k: "orient" }
  | { k: "scc"; count: number; size: number }
  | { k: "singles" }
  | { k: "keep"; c: number; p: number }
  | { k: "drop"; c: number; p: number }
  | { k: "filtered"; removed: number; forced: number };

interface RFrame {
  phase: 0 | 1;
  matched: number[]; // cell -> piece or -1
  active: [number, number] | null;
  path: [number, number][];
  removed: [number, number][];
  /** node -> scc display id (cells 0..5, pieces 6..11), -1 = not yet revealed */
  scc: number[] | null;
  note: Note;
}

function tarjan(adj: number[][]): number[] {
  const n = adj.length;
  const index: number[] = Array.from({ length: n }, () => -1);
  const low: number[] = Array.from({ length: n }, () => 0);
  const onStack: boolean[] = Array.from({ length: n }, () => false);
  const stack: number[] = [];
  const comp: number[] = Array.from({ length: n }, () => -1);
  let counter = 0;
  let comps = 0;

  const strongconnect = (v: number): void => {
    index[v] = counter;
    low[v] = counter;
    counter++;
    stack.push(v);
    onStack[v] = true;
    for (const w of adj[v] ?? []) {
      if ((index[w] ?? -1) === -1) {
        strongconnect(w);
        low[v] = Math.min(low[v] ?? 0, low[w] ?? 0);
      } else if (onStack[w] ?? false) {
        low[v] = Math.min(low[v] ?? 0, index[w] ?? 0);
      }
    }
    if (low[v] === index[v]) {
      for (;;) {
        const w = stack.pop();
        if (w === undefined) break;
        onStack[w] = false;
        comp[w] = comps;
        if (w === v) break;
      }
      comps++;
    }
  };

  for (let v = 0; v < n; v++) if ((index[v] ?? -1) === -1) strongconnect(v);
  return comp;
}

function buildFrames(): { frames: RFrame[]; matchEnd: number } {
  const frames: RFrame[] = [];
  const matched: number[] = Array.from({ length: NN }, () => -1);
  const owner: number[] = Array.from({ length: NN }, () => -1);
  const removed: [number, number][] = [];

  const snap = (over: Partial<RFrame> & { note: Note }): void => {
    frames.push({
      phase: 0,
      matched: [...matched],
      active: null,
      path: [],
      removed: [...removed],
      scc: null,
      ...over,
    });
  };

  snap({ note: { k: "start" } });

  // Phase 0: Kuhn's augmenting-path matching, cell by cell. Returns the full
  // augmenting path (root to free piece) on success, for the path highlight.
  const tryCell = (
    c: number,
    visited: Set<number>,
    path: [number, number][],
  ): [number, number][] | null => {
    for (const p of CELL_DOMAINS[c] ?? []) {
      if (visited.has(p)) continue;
      visited.add(p);
      const edge: [number, number] = [c, p];
      const own = owner[p] ?? -1;
      if (own === -1) {
        snap({ note: { k: "tryFree", c, p }, active: edge, path: [...path, edge] });
        matched[c] = p;
        owner[p] = c;
        return [...path, edge];
      }
      snap({ note: { k: "conflict", c, p, owner: own }, active: edge, path: [...path, edge] });
      const deeper = tryCell(own, visited, [...path, edge]);
      if (deeper !== null) {
        matched[c] = p;
        owner[p] = c;
        return deeper;
      }
      snap({ note: { k: "deadEdge", c, p }, active: edge, path: [...path] });
    }
    return null;
  };

  for (let c = 0; c < NN; c++) {
    snap({ note: { k: "consider", c } });
    const successPath = tryCell(c, new Set(), []) ?? [];
    snap({ note: { k: "augment", c, len: successPath.length }, path: successPath });
  }
  snap({ note: { k: "matchedAll" } });
  const matchEnd = frames.length - 1;

  // Phase 1: orient around the matching, SCCs, delete cross edges.
  snap({ phase: 1, note: { k: "orient" } });

  // Digraph on 12 nodes: cells 0..5, pieces 6..11. Matched: cell -> piece;
  // unmatched: piece -> cell.
  const adj: number[][] = Array.from({ length: 2 * NN }, () => []);
  for (let c = 0; c < NN; c++) {
    for (const p of CELL_DOMAINS[c] ?? []) {
      if (matched[c] === p) adj[c]?.push(NN + p);
      else adj[NN + p]?.push(c);
    }
  }
  const comp = tarjan(adj);

  // Reveal SCCs: multi-node components one at a time, singles in one frame.
  const bySize = new Map<number, number[]>();
  comp.forEach((id, node) => {
    const list = bySize.get(id) ?? [];
    list.push(node);
    bySize.set(id, list);
  });
  const multi = [...bySize.entries()]
    .filter(([, nodes]) => nodes.length > 1)
    .sort((a, b) => Math.min(...a[1]) - Math.min(...b[1]));
  const reveal: number[] = Array.from({ length: 2 * NN }, () => -1);
  let display = 0;
  for (const [, nodes] of multi) {
    for (const node of nodes) reveal[node] = display;
    snap({ phase: 1, scc: [...reveal], note: { k: "scc", count: display + 1, size: nodes.length } });
    display++;
  }
  const SINGLE = 98;
  for (let node = 0; node < 2 * NN; node++) if ((reveal[node] ?? -1) === -1) reveal[node] = SINGLE;
  snap({ phase: 1, scc: [...reveal], note: { k: "singles" } });

  // Check every unmatched edge: same SCC keeps it, crossing deletes it.
  for (let c = 0; c < NN; c++) {
    for (const p of CELL_DOMAINS[c] ?? []) {
      if (matched[c] === p) continue;
      const same = comp[c] === comp[NN + p];
      if (same) {
        snap({ phase: 1, scc: [...reveal], active: [c, p], note: { k: "keep", c, p } });
      } else {
        removed.push([c, p]);
        snap({ phase: 1, scc: [...reveal], active: [c, p], note: { k: "drop", c, p } });
      }
    }
  }
  const forced = CELL_DOMAINS.filter(
    (dom, c) => dom.length > 1 && dom.filter((p) => !removed.some(([rc, rp]) => rc === c && rp === p)).length === 1,
  ).length;
  snap({ phase: 1, scc: [...reveal], note: { k: "filtered", removed: removed.length, forced } });

  return { frames, matchEnd };
}

const BUILT = buildFrames();
const FRAMES: RFrame[] = BUILT.frames;
const MATCH_END = BUILT.matchEnd;
const END = FRAMES.length - 1;

// --- Layout -------------------------------------------------------------------

const PX = 78; // pieces column x
const CX = 262; // cells column x
const Y0 = 30;
const DY = 44;
const H = Y0 + (NN - 1) * DY + 30;

const SCC_NODE = ["fill-sky-500/25 stroke-sky-500", "fill-emerald-500/25 stroke-emerald-500"];

function pieceName(p: number): string {
  return `P${p + 1}`;
}
function cName(c: number): string {
  return `C${c + 1}`;
}
function edgeKey(c: number, p: number): string {
  return `${c}:${p}`;
}

const T = {
  en: {
    title: "Régin's filter, live: matching, components, deletions",
    intro:
      "Six pieces, six cells, feasible edges in between. First build a maximum matching by augmenting paths — watch a taken piece get re-routed. Then Régin's insight: orient the graph around the matching, compute its strongly connected components, and every unmatched edge that crosses two components belongs to no maximum matching. Delete them, soundly — here that forces two cells that simple crossing-off never notices.",
    findMatching: "1 · Find maximum matching",
    runFilter: "2 · Run Régin's filter",
    step: "Step",
    reset: "Reset",
    matchingSize: "matching",
    removedCount: "edges deleted",
    pieces: "pieces",
    cells: "cells",
    domains: "Domains",
    forced: "forced!",
    nStart: "The bipartite graph: an edge wherever a piece is still in a cell's candidate list.",
    nConsider: (c: string) => `${c} needs a piece — start a search for an augmenting path.`,
    nTryFree: (c: string, p: string) => `${c} tries ${p}: free! Match them.`,
    nConflict: (c: string, p: string, o: string) =>
      `${c} wants ${p}, but ${p} is taken by ${o}. Try to re-route ${o} to another piece…`,
    nDeadEdge: (c: string, p: string) => `No re-route works through ${p} — back up and try ${c}'s next edge.`,
    nAugmentShort: (c: string) => `${c} matched directly.`,
    nAugmentLong: (c: string, len: number) =>
      `Augmenting path found (${len} edges): flip it — unmatched edges become matched and vice versa. ${c} is served and nobody lost their piece.`,
    nMatchedAll: "Maximum matching found: all 6 cells covered. If this had failed, the position would already be dead.",
    nOrient: "Now orient every edge: matched edges run cell → piece, unmatched edges piece → cell. Alternating paths become directed paths.",
    nScc: (i: number, size: number) =>
      `Strongly connected component ${i}: ${size} nodes trade pieces among themselves along an alternating cycle.`,
    nSingles: "Every remaining node is its own tiny component — no cycle passes through it.",
    nKeep: (c: string, p: string) =>
      `${c}–${p} stays: both ends sit in the same component, so an alternating cycle can swap it into some maximum matching.`,
    nDrop: (c: string, p: string) =>
      `${c}–${p} crosses two components and is unmatched: it belongs to no maximum matching. Deleted — soundly.`,
    nFiltered: (r: number, f: number) =>
      `Filter done: ${r} edges deleted in one matching + one SCC pass, ${f} cells now forced. Pairwise crossing-off would have found none of this.`,
  },
  fr: {
    title: "Le filtre de Régin, en direct : couplage, composantes, suppressions",
    intro:
      "Six pièces, six cases, et les arêtes faisables entre les deux. D'abord un couplage maximum par chemins augmentants — regardez une pièce déjà prise se faire re-router. Puis l'idée de Régin : orienter le graphe autour du couplage, calculer ses composantes fortement connexes, et toute arête non couplée qui traverse deux composantes n'appartient à aucun couplage maximum. On les supprime, en toute correction — ici, cela force deux cases que le simple rayage ne remarquerait jamais.",
    findMatching: "1 · Trouver le couplage maximum",
    runFilter: "2 · Lancer le filtre de Régin",
    step: "Pas",
    reset: "Réinitialiser",
    matchingSize: "couplage",
    removedCount: "arêtes supprimées",
    pieces: "pièces",
    cells: "cases",
    domains: "Domaines",
    forced: "forcée !",
    nStart: "Le graphe biparti : une arête partout où une pièce figure encore dans la liste de candidats d'une case.",
    nConsider: (c: string) => `${c} a besoin d'une pièce — on cherche un chemin augmentant.`,
    nTryFree: (c: string, p: string) => `${c} essaie ${p} : libre ! On les couple.`,
    nConflict: (c: string, p: string, o: string) =>
      `${c} veut ${p}, mais ${p} est prise par ${o}. On tente de re-router ${o} vers une autre pièce…`,
    nDeadEdge: (c: string, p: string) => `Aucun re-routage ne passe par ${p} — on recule et on essaie l'arête suivante de ${c}.`,
    nAugmentShort: (c: string) => `${c} couplée directement.`,
    nAugmentLong: (c: string, len: number) =>
      `Chemin augmentant trouvé (${len} arêtes) : on le bascule — les arêtes libres deviennent couplées et inversement. ${c} est servie et personne n'a perdu sa pièce.`,
    nMatchedAll: "Couplage maximum trouvé : les 6 cases sont couvertes. S'il avait échoué, la position serait déjà morte.",
    nOrient: "On oriente maintenant chaque arête : les arêtes couplées vont de la case vers la pièce, les autres de la pièce vers la case. Les chemins alternants deviennent des chemins orientés.",
    nScc: (i: number, size: number) =>
      `Composante fortement connexe ${i} : ${size} nœuds s'échangent leurs pièces le long d'un cycle alternant.`,
    nSingles: "Chaque nœud restant est sa propre petite composante — aucun cycle ne passe par lui.",
    nKeep: (c: string, p: string) =>
      `${c}–${p} reste : ses deux extrémités sont dans la même composante, un cycle alternant peut donc la faire entrer dans un couplage maximum.`,
    nDrop: (c: string, p: string) =>
      `${c}–${p} traverse deux composantes sans être couplée : elle n'appartient à aucun couplage maximum. Supprimée — en toute correction.`,
    nFiltered: (r: number, f: number) =>
      `Filtre terminé : ${r} arêtes supprimées en un couplage + une passe de composantes, ${f} cases désormais forcées. Le rayage par paires n'en aurait trouvé aucune.`,
  },
};

function noteText(note: Note, t: (typeof T)["en"]): string {
  switch (note.k) {
    case "start":
      return t.nStart;
    case "consider":
      return t.nConsider(cName(note.c));
    case "tryFree":
      return t.nTryFree(cName(note.c), pieceName(note.p));
    case "conflict":
      return t.nConflict(cName(note.c), pieceName(note.p), cName(note.owner));
    case "deadEdge":
      return t.nDeadEdge(cName(note.c), pieceName(note.p));
    case "augment":
      return note.len <= 1 ? t.nAugmentShort(cName(note.c)) : t.nAugmentLong(cName(note.c), note.len);
    case "matchedAll":
      return t.nMatchedAll;
    case "orient":
      return t.nOrient;
    case "scc":
      return t.nScc(note.count, note.size);
    case "singles":
      return t.nSingles;
    case "keep":
      return t.nKeep(cName(note.c), pieceName(note.p));
    case "drop":
      return t.nDrop(cName(note.c), pieceName(note.p));
    case "filtered":
      return t.nFiltered(note.removed, note.forced);
  }
}

function MidArrow({ x1, y1, x2, y2 }: { x1: number; y1: number; x2: number; y2: number }) {
  const mx = x1 + (x2 - x1) * 0.5;
  const my = y1 + (y2 - y1) * 0.5;
  const deg = (Math.atan2(y2 - y1, x2 - x1) * 180) / Math.PI;
  return (
    <polygon
      points="-1,-4 7,0 -1,4"
      transform={`translate(${mx},${my}) rotate(${deg})`}
      className="fill-muted-foreground"
    />
  );
}

export function ReginMatchingLab() {
  const t = useT(T);
  const { ref: rootRef, visible } = useRunWhileVisible();
  const [idx, setIdx] = useState(0);
  const [target, setTarget] = useState<number | null>(null);

  const frame = FRAMES[Math.min(idx, END)];

  useEffect(() => {
    if (target === null || !visible) return;
    const id = setInterval(() => {
      setIdx((cur) => (cur < target ? cur + 1 : cur));
    }, 750);
    return () => clearInterval(id);
  }, [target, visible]);

  useEffect(() => {
    // Clear the play target once reached (auto-stop at the phase boundary).
    if (target !== null && idx >= target) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setTarget(null);
    }
  }, [idx, target]);

  if (!frame) return null;

  const removedSet = new Set(frame.removed.map(([c, p]) => edgeKey(c, p)));
  const pathSet = new Set(frame.path.map(([c, p]) => edgeKey(c, p)));
  const activeKey = frame.active ? edgeKey(frame.active[0], frame.active[1]) : null;
  const activeDrop = frame.note.k === "drop";
  const matchingSize = frame.matched.filter((p) => p >= 0).length;

  const nodeY = (i: number): number => Y0 + i * DY;

  return (
    <div ref={rootRef} className="space-y-4 rounded-lg border bg-card p-4">
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{t.title}</h3>
        <p className="text-xs leading-relaxed text-muted-foreground">{t.intro}</p>
      </div>

      <div className="flex flex-wrap items-start justify-center gap-6">
        <div className="w-full max-w-80">
          <svg viewBox={`0 0 340 ${H}`} className="w-full" role="img" aria-label="Bipartite graph of six cells and six pieces with feasible edges, showing the maximum matching, strongly connected components, and the cross-component edges Régin's filter deletes">
            <text x={PX} y={12} fontSize={9} textAnchor="middle" className="fill-muted-foreground">
              {t.pieces}
            </text>
            <text x={CX} y={12} fontSize={9} textAnchor="middle" className="fill-muted-foreground">
              {t.cells}
            </text>

            {/* edges */}
            {CELL_DOMAINS.map((dom, c) =>
              dom.map((p) => {
                const key = edgeKey(c, p);
                const y1 = nodeY(p);
                const y2 = nodeY(c);
                const isMatched = frame.matched[c] === p;
                const isRemoved = removedSet.has(key);
                const isActive = activeKey === key;
                const onPath = pathSet.has(key);
                const cls = isActive
                  ? activeDrop
                    ? "stroke-rose-500"
                    : "stroke-amber-500"
                  : onPath
                    ? "stroke-amber-500"
                    : isRemoved
                      ? "stroke-rose-500"
                      : isMatched
                        ? "stroke-primary"
                        : "stroke-muted-foreground";
                return (
                  <g key={key} opacity={isRemoved && !isActive ? 0.18 : 1}>
                    <line
                      x1={PX + 18} y1={y1} x2={CX - 24} y2={y2}
                      className={cls}
                      strokeWidth={isActive || onPath ? 3 : isMatched ? 3 : 1.2}
                      strokeDasharray={isRemoved ? "4 3" : undefined}
                      opacity={isMatched || isActive || onPath || isRemoved ? 1 : 0.45}
                      style={{ transition: "stroke 200ms, opacity 200ms" }}
                    />
                    {/* orientation arrows during the filter phase */}
                    {frame.phase === 1 && !isRemoved &&
                      (isMatched ? (
                        <MidArrow x1={CX - 24} y1={y2} x2={PX + 18} y2={y1} />
                      ) : (
                        <MidArrow x1={PX + 18} y1={y1} x2={CX - 24} y2={y2} />
                      ))}
                  </g>
                );
              }),
            )}

            {/* piece nodes */}
            {Array.from({ length: NN }, (_, p) => {
              const sccId = frame.scc?.[NN + p] ?? -1;
              const cls =
                sccId >= 0 && sccId < SCC_NODE.length
                  ? SCC_NODE[sccId]
                  : sccId >= SCC_NODE.length
                    ? "fill-muted stroke-border"
                    : "fill-card stroke-border";
              return (
                <g key={p}>
                  <circle cx={PX} cy={nodeY(p)} r={15} className={cls} strokeWidth={1.5} />
                  <text x={PX} y={nodeY(p) + 4} fontSize={11} fontWeight={600} textAnchor="middle" className="fill-foreground">
                    {pieceName(p)}
                  </text>
                </g>
              );
            })}

            {/* cell nodes */}
            {Array.from({ length: NN }, (_, c) => {
              const sccId = frame.scc?.[c] ?? -1;
              const cls =
                sccId >= 0 && sccId < SCC_NODE.length
                  ? SCC_NODE[sccId]
                  : sccId >= SCC_NODE.length
                    ? "fill-muted stroke-border"
                    : "fill-card stroke-border";
              return (
                <g key={c}>
                  <rect x={CX - 22} y={nodeY(c) - 13} width={44} height={26} rx={7} className={cls} strokeWidth={1.5} />
                  <text x={CX} y={nodeY(c) + 4} fontSize={11} fontWeight={600} textAnchor="middle" className="fill-foreground">
                    {cName(c)}
                  </text>
                </g>
              );
            })}
          </svg>
        </div>

        <div className="min-w-56 flex-1 space-y-3">
          <div className="flex flex-wrap gap-2 text-[11px]">
            <span className="rounded-md border px-2 py-1 tabular-nums">
              {t.matchingSize} {matchingSize}/6
            </span>
            <span
              className={cn(
                "rounded-md border px-2 py-1 tabular-nums",
                frame.removed.length > 0 && "border-rose-300 bg-rose-500/10",
              )}
            >
              {frame.removed.length} {t.removedCount}
            </span>
          </div>

          <p className="min-h-16 rounded-md border bg-muted/40 px-3 py-2 text-xs leading-relaxed text-muted-foreground">
            {noteText(frame.note, t)}
          </p>

          <div className="space-y-1">
            <div className="text-[10px] font-medium uppercase tracking-wide text-muted-foreground">
              {t.domains}
            </div>
            {CELL_DOMAINS.map((dom, c) => {
              const left = dom.filter((p) => !removedSet.has(edgeKey(c, p)));
              return (
                <div key={c} className="flex items-center gap-2 text-xs">
                  <span className="w-6 font-medium">{cName(c)}</span>
                  <span className="flex gap-1">
                    {dom.map((p) => {
                      const gone = removedSet.has(edgeKey(c, p));
                      const isMatch = frame.matched[c] === p;
                      return (
                        <span
                          key={p}
                          className={cn(
                            "rounded border px-1.5 py-0.5 tabular-nums",
                            gone && "border-rose-300 text-rose-500 line-through opacity-60",
                            !gone && isMatch && "border-primary font-semibold",
                          )}
                        >
                          {pieceName(p)}
                        </span>
                      );
                    })}
                  </span>
                  {left.length === 1 && dom.length > 1 && frame.removed.length > 0 && (
                    <span className="rounded bg-emerald-500/10 px-1.5 py-0.5 text-[10px] font-medium text-emerald-600 dark:text-emerald-400">
                      {t.forced}
                    </span>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-2">
        <Button
          size="sm"
          onClick={() => setTarget(MATCH_END)}
          disabled={idx >= MATCH_END}
        >
          {t.findMatching}
        </Button>
        <Button
          size="sm"
          onClick={() => setTarget(END)}
          disabled={idx < MATCH_END || idx >= END}
        >
          {t.runFilter}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setTarget(null);
            setIdx((cur) => Math.min(cur + 1, END));
          }}
          disabled={idx >= END}
        >
          {t.step}
        </Button>
        <Button
          size="sm"
          variant="outline"
          onClick={() => {
            setTarget(null);
            setIdx(0);
          }}
        >
          {t.reset}
        </Button>
      </div>
    </div>
  );
}
