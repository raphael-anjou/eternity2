// "Copy / open on e2.bucas.name" actions, usable next to any board, since
// every board we render can be encoded as a bucas URL.

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { useT } from "@/i18n";
import { encodeBucasUrl } from "@/lib/bucas";
import type { BoardCells, Puzzle } from "@/lib/types";

const T = {
  en: {
    copied: "Copied!",
    copyLink: "Copy e2.bucas.name link",
    open: "open ↗",
  },
  fr: {
    copied: "Copié !",
    copyLink: "Copier le lien e2.bucas.name",
    open: "ouvrir ↗",
  },
};

export function BucasActions({
  puzzle,
  board,
  name,
  size = "sm",
}: {
  puzzle: Puzzle;
  board: BoardCells;
  name?: string;
  size?: "sm" | "xs";
}) {
  const t = useT(T);
  const [copied, setCopied] = useState(false);
  const url = () => encodeBucasUrl(puzzle, board, name ?? puzzle.name);
  return (
    <div className="flex flex-wrap items-center gap-2">
      <Button
        variant="outline"
        size={size}
        onClick={() => {
          navigator.clipboard.writeText(url());
          setCopied(true);
          setTimeout(() => setCopied(false), 1500);
        }}
      >
        {copied ? t.copied : t.copyLink}
      </Button>
      <Button
        variant="ghost"
        size={size}
        render={<a href={url()} target="_blank" rel="noreferrer" />}
      >
        {t.open}
      </Button>
    </div>
  );
}
