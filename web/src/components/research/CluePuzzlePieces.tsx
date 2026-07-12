// Download buttons for the recovered clue-puzzle piece sets, mirroring the main
// puzzle's "download pieces" affordance (/puzzle). Each button zips one SVG per
// piece via the shared pieceSvg renderer, so the output matches the site's
// motif art. The pieces come from Le Bail's SHORTER solver; see
// research/topics/clue-puzzle-pieces for provenance.

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { downloadPiecesZip } from "@/lib/pieceSvg";
import { CLUE1_PIECES, CLUE2_PIECES } from "@/data/clue-puzzle-pieces";
import type { Edges } from "@/lib/bucas";

function DownloadButton({
  label,
  pieces,
  filename,
}: {
  label: string;
  pieces: Edges[];
  filename: string;
}) {
  const [zipping, setZipping] = useState(false);
  return (
    <Button
      variant="outline"
      disabled={zipping}
      onClick={() => {
        setZipping(true);
        // Defer so the label repaints before the synchronous zip blocks the thread.
        setTimeout(() => {
          try {
            downloadPiecesZip(pieces, filename);
          } finally {
            setZipping(false);
          }
        }, 0);
      }}
    >
      {zipping ? "Preparing…" : label}
    </Button>
  );
}

/** Two buttons: download each recovered clue puzzle as a zip of per-piece SVGs. */
export function CluePuzzlePieces() {
  return (
    <div className="flex flex-col gap-3 sm:flex-row">
      <DownloadButton
        label="Download Clue Puzzle 1 (36 pieces)"
        pieces={CLUE1_PIECES}
        filename="eternity2-clue-puzzle-1.svg.zip"
      />
      <DownloadButton
        label="Download Clue Puzzle 2 (72 pieces)"
        pieces={CLUE2_PIECES}
        filename="eternity2-clue-puzzle-2.svg.zip"
      />
    </div>
  );
}
