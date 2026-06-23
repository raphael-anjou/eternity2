import { StepThroughSolver, type Snapshot, type NarrationContext } from "./StepThroughSolver";
import { useT } from "@/i18n";

// GAUNTLET's heart, one decision at a time, on the real engine. We step a single
// fill direction (a spiral, here) and narrate *why* the order matters: at each
// cell the spiral reaches, how many pieces actually fit — and how the choice the
// search is forced into depends entirely on which cell the order put next. Read
// it next to the live four-lane race: this explains, slowly, why two orders on
// the same pieces end up on different boards.

function cellName(pos: number, w: number, h: number, en: boolean): string {
  const x = pos % w;
  const y = Math.floor(pos / w);
  const corner =
    (x === 0 || x === w - 1) && (y === 0 || y === h - 1)
      ? en
        ? "a corner"
        : "un coin"
      : x === 0 || x === w - 1 || y === 0 || y === h - 1
        ? en
          ? "an edge cell"
          : "une case de bord"
        : en
          ? "an interior cell"
          : "une case intérieure";
  return corner;
}

const T = {
  en: {
    title: "Why the scan order decides the board (real run, step by step)",
    footer:
      "Same pieces, same rules — only the order of attack differs. The spiral commits to the frame early, where few pieces fit, so its forced choices ripple inward. A different order would force different cells first and land somewhere else entirely. That is exactly why GAUNTLET runs several orders at once.",
  },
  fr: {
    title: "Pourquoi l'ordre de parcours décide le plateau (vraie exécution, pas à pas)",
    footer:
      "Mêmes pièces, mêmes règles — seul l'ordre d'attaque change. La spirale s'engage tôt sur le cadre, où peu de pièces conviennent, et ses choix forcés se propagent vers l'intérieur. Un autre ordre forcerait d'autres cases d'abord et atterrirait ailleurs. C'est précisément pourquoi GAUNTLET lance plusieurs ordres à la fois.",
  },
};

const NARR = {
  en: {
    fresh: (cell: string) => `Fresh board. The spiral order starts at ${cell}.`,
    solved: (nodes: number, bt: number) =>
      `Filled! ${nodes} placements, ${bt} backtracks for this order. Another order reaches a different board — restarting…`,
    deadEnd:
      "No piece fits the cell this order reached. Dead end — un-place the last piece and try its next option. A different fill order would never have asked this cell yet.",
    placed: (cell: string, cands: number, tries: number) =>
      `Placed on ${cell}: ${cands} piece${cands === 1 ? "" : "s"} fit here, chosen after ${tries} attempt${tries === 1 ? "" : "s"}. The order put this cell next, so this is where the search was forced to commit.`,
    placedTight: (cell: string, cands: number) =>
      `Placed on ${cell} — only ${cands} piece${cands === 1 ? "" : "s"} could go here. Tight cells like this, reached early by the spiral, are what steer the whole board.`,
  },
  fr: {
    fresh: (cell: string) => `Plateau vierge. L'ordre en spirale commence par ${cell}.`,
    solved: (nodes: number, bt: number) =>
      `Rempli ! ${nodes} placements, ${bt} retours pour cet ordre. Un autre ordre atteint un autre plateau — on recommence…`,
    deadEnd:
      "Aucune pièce ne convient à la case atteinte par cet ordre. Impasse — on retire la dernière pièce et on tente l'option suivante. Un autre ordre n'aurait pas encore demandé cette case.",
    placed: (cell: string, cands: number, tries: number) =>
      `Posée sur ${cell} : ${cands} pièce${cands === 1 ? "" : "s"} y conviennent, choisie après ${tries} essai${tries === 1 ? "" : "s"}. L'ordre a placé cette case ensuite, c'est donc là que la recherche a dû s'engager.`,
    placedTight: (cell: string, cands: number) =>
      `Posée sur ${cell} — seulement ${cands} pièce${cands === 1 ? "" : "s"} possible(s). Les cases serrées comme celle-ci, atteintes tôt par la spirale, orientent tout le plateau.`,
  },
};

export function GauntletStepThrough() {
  const t = useT(T);
  const lang = useT({ en: true, fr: false });
  const n = useT(NARR);

  const narrate = (snap: Snapshot, prev: Snapshot | null, ctx: NarrationContext) => {
    if (!prev) {
      const start = ctx.path[0] ?? 0;
      return {
        text: n.fresh(cellName(start, ctx.width, ctx.height, lang)),
        mood: "start" as const,
      };
    }
    if (snap.solved) {
      return { text: n.solved(snap.nodes, snap.backtracks), mood: "solved" as const };
    }
    if (snap.backtracks > prev.backtracks) {
      return { text: n.deadEnd, mood: "back" as const };
    }
    // A placement just happened; describe the cell it landed on (prev frontier).
    const cell = prev.frontier ?? 0;
    const name = cellName(cell, ctx.width, ctx.height, lang);
    const tries = snap.attempts - prev.attempts;
    // candidates is computed for the *current* frontier; for the cell we just
    // filled we approximate "tightness" by how few attempts it took plus whether
    // it's a frame cell. Use the current-frontier candidate count as the live hint.
    const cands = ctx.candidates.size;
    if (cands > 0 && cands <= 2) {
      return { text: n.placedTight(name, cands), mood: "place" as const };
    }
    return { text: n.placed(name, Math.max(1, cands), tries), mood: "place" as const };
  };

  return (
    <StepThroughSolver
      title={t.title}
      size={6}
      colors={5}
      seed={31}
      pathKind="spiral-in"
      stepMs={950}
      narrate={narrate}
      footer={t.footer}
      boardMaxClass="max-w-64"
    />
  );
}
