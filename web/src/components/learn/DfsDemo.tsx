// A 3×3 DFS, one decision at a time, narrated. Now a thin wrapper over the shared
// StepThroughSolver (which precomputes a real engine run into scrubable snapshots
// and highlights the legal candidates at each step) — this keeps the original
// backtracking story while the same machinery backs the research-side explainers.

import { StepThroughSolver, type Snapshot } from "@/components/research/StepThroughSolver";
import { useT } from "@/i18n";

const SIZE = 3;
const SEED = 15; // 21 placements, 12 backtracks: a story worth watching

const T = {
  en: {
    title: "Backtracking on a 3×3, in slow motion",
    fresh: "A fresh board. First stop: the top-left square.",
    solved: (nodes: number, backtracks: number) =>
      `Solved! ${nodes} placements, ${backtracks} backtracks. Restarting…`,
    deadEnd: "Nothing fits here. Dead end: remove the previous piece and try its next option.",
    placed: (tries: number) =>
      `A piece fits (found after checking ${tries} candidate${tries === 1 ? "" : "s"}). On to the next square.`,
    footer:
      "The red square is where the solver is working. Watch what happens when no piece fits: it un-places the previous piece and continues from there. That undo is the whole secret of backtracking.",
  },
  fr: {
    title: "Le retour en arrière (backtracking) sur un 3×3, au ralenti",
    fresh: "Un plateau vierge. On commence par la case en haut à gauche.",
    solved: (nodes: number, backtracks: number) =>
      `Résolu ! ${nodes} placements, ${backtracks} retours en arrière. On recommence…`,
    deadEnd:
      "Rien ne convient ici : c'est une impasse. On retire alors la dernière pièce posée et on tente l'option suivante.",
    placed: (tries: number) =>
      `Une pièce convient (trouvée au bout de ${tries} essai${tries > 1 ? "s" : ""}). Et on passe à la case suivante.`,
    footer:
      "La case rouge, c'est là où le solveur travaille. Regardez bien ce qui se passe quand plus aucune pièce ne convient : il défait la dernière pièce posée et repart de là. Ce simple retour en arrière, c'est tout le principe du backtracking.",
  },
};

export function DfsDemo() {
  const t = useT(T);

  const narrate = (snap: Snapshot, prev: Snapshot | null) => {
    if (!prev) return { text: t.fresh, mood: "start" as const };
    if (snap.solved) return { text: t.solved(snap.nodes, snap.backtracks), mood: "solved" as const };
    if (snap.backtracks > prev.backtracks) return { text: t.deadEnd, mood: "back" as const };
    return { text: t.placed(snap.attempts - prev.attempts), mood: "place" as const };
  };

  return (
    <div className="max-w-2xl">
      <StepThroughSolver
        title={t.title}
        size={SIZE}
        colors={3}
        seed={SEED}
        pathKind="row-major"
        stepMs={900}
        narrate={narrate}
        footer={t.footer}
      />
    </div>
  );
}
