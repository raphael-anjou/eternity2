// Three small self-running demos that make "computers think in binary"
// concrete: motif → number, edge comparison bit by bit, rotation as an
// array shift. Each runs on its own clock, no interaction needed.

import { useEffect, useMemo, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { MotifSwatch } from "@/components/board/MotifSwatch";
import { PieceSvg } from "@/components/board/PieceSvg";
import { colorToLetter } from "@/lib/motifs";
import { rotateEdges } from "@/lib/types";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";

const T = {
  en: {
    motif: {
      title: "1 · A motif is just a number",
      label: (letter: string) => `motif '${letter}'`,
      explainer:
        "The computer never sees the artwork. Each of the 22 motifs gets a number, and 5 binary digits (bits) are enough to store any of them. A whole piece = 4 numbers.",
    },
    compare: {
      title: '2 · "Does it fit?" = "are the numbers equal?"',
      fits: "all bits equal → the piece FITS ✓",
      rejected: "a bit differs → rejected, try the next piece",
      comparing: "comparing…",
      explainer:
        'This is one "fit check", the operation our stats pages count by the million. A CPU compares all the bits in a single instruction, in under a nanosecond.',
    },
    rotation: {
      title: "3 · Rotating = shifting the list",
      dirLabels: ["U", "R", "D", "L"],
      order: "[up, right, down, left]",
      explainer:
        "A quarter-turn clockwise just slides every number one slot to the right (the last wraps to the front). No drawing is ever rotated: four list shifts give all four orientations, basically for free.",
    },
  },
  fr: {
    motif: {
      title: "1 · Un motif n'est qu'un nombre",
      label: (letter: string) => `motif « ${letter} »`,
      explainer:
        "L'ordinateur ne voit jamais le dessin. Chacun des 22 motifs reçoit un nombre, et 5 chiffres binaires (bits) suffisent pour stocker n'importe lequel. Une pièce entière = 4 nombres.",
    },
    compare: {
      title: "2 · « Ça s'emboîte ? » = « les nombres sont-ils égaux ? »",
      fits: "tous les bits sont égaux → la pièce CONVIENT ✓",
      rejected: "un bit diffère → rejetée, on essaie la pièce suivante",
      comparing: "comparaison…",
      explainer:
        "Voici un « test de compatibilité », l'opération que nos pages de statistiques comptent par millions. Un processeur compare tous les bits en une seule instruction, en moins d'une nanoseconde.",
    },
    rotation: {
      title: "3 · Tourner = décaler la liste",
      dirLabels: ["H", "D", "B", "G"],
      order: "[haut, droite, bas, gauche]",
      explainer:
        "Un quart de tour dans le sens des aiguilles d'une montre fait simplement glisser chaque nombre d'une case vers la droite (le dernier revient au début). Aucun dessin n'est jamais tourné : quatre décalages de liste donnent les quatre orientations, quasiment gratuitement.",
    },
  },
};

function bits5(n: number): string[] {
  return n.toString(2).padStart(5, "0").split("");
}

function BitBox({ bit, state }: { bit: string; state?: "match" | "diff" | null }) {
  return (
    <span
      className={cn(
        "inline-flex h-7 w-6 items-center justify-center rounded border font-mono text-sm font-bold transition-colors duration-300",
        state === "match" && "border-emerald-500 bg-emerald-500/15 text-emerald-600",
        state === "diff" && "border-red-500 bg-red-500/15 text-red-600",
        !state && "bg-muted/50",
      )}
    >
      {bit}
    </span>
  );
}

// --- demo 1: a motif is just a number -------------------------------------

function MotifAsNumber() {
  const t = useT(T);
  const [color, setColor] = useState(1);
  useEffect(() => {
    const id = setInterval(() => setColor((c) => (c % 22) + 1), 2000);
    return () => clearInterval(id);
  }, []);
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">{t.motif.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex items-center justify-center gap-4">
          <MotifSwatch color={color} width={88} />
          <span className="text-2xl text-muted-foreground">=</span>
          <div className="text-center">
            <div className="text-2xl font-bold tabular-nums">{color}</div>
            <div className="text-xs text-muted-foreground">{t.motif.label(colorToLetter(color))}</div>
          </div>
          <span className="text-2xl text-muted-foreground">=</span>
          <div className="flex gap-1">
            {bits5(color).map((b, i) => (
              <BitBox key={i} bit={b} />
            ))}
          </div>
        </div>
        <p className="text-xs text-muted-foreground">{t.motif.explainer}</p>
      </CardContent>
    </Card>
  );
}

// --- demo 2: does it fit? compare two numbers ------------------------------

// Half-pieces meeting like on a real board: the left piece shows its
// right-edge triangle, the right piece its left-edge triangle, bases facing.
// When the motifs match, the two halves complete the full diamond.
function FacingEdge({ color, side, size = 96 }: { color: number; side: "left" | "right"; size?: number }) {
  return (
    <svg
      viewBox={side === "right" ? "0 -128 128 256" : "-128 -128 128 256"}
      width={size / 2}
      height={size}
      aria-hidden
    >
      <use href={`#e2m-${color}`} transform={side === "right" ? "rotate(180)" : undefined} />
    </svg>
  );
}

function EdgeCompare() {
  const t = useT(T);
  const [pair, setPair] = useState<[number, number]>([7, 7]);
  const [revealed, setRevealed] = useState(0);

  useEffect(() => {
    let bit = 0;
    const newRound = () => {
      const a = 1 + Math.floor(Math.random() * 22);
      const equal = Math.random() < 0.5;
      let b = a;
      if (!equal) {
        do {
          b = 1 + Math.floor(Math.random() * 22);
        } while (b === a);
      }
      setPair([a, b]);
      setRevealed(0);
      bit = 0;
    };
    newRound();
    const timer = setInterval(() => {
      bit += 1;
      if (bit <= 5) {
        setRevealed(bit);
      } else if (bit >= 9) {
        newRound();
      }
    }, 350);
    return () => clearInterval(timer);
  }, []);

  const [a, b] = pair;
  const bitsA = bits5(a);
  const bitsB = bits5(b);
  const done = revealed >= 5;
  const same = a === b;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">{t.compare.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex items-center justify-center">
          <FacingEdge color={a} side="right" />
          <div className="w-1" />
          <FacingEdge color={b} side="left" />
        </div>
        <div className="flex items-center justify-center gap-5">
          <div className="flex gap-1">
            {bitsA.map((bit, i) => (
              <BitBox key={i} bit={bit} state={i < revealed ? (bitsA[i] === bitsB[i] ? "match" : "diff") : null} />
            ))}
          </div>
          <div className="flex gap-1">
            {bitsB.map((bit, i) => (
              <BitBox key={i} bit={bit} state={i < revealed ? (bitsA[i] === bitsB[i] ? "match" : "diff") : null} />
            ))}
          </div>
        </div>
        <div className="h-6 text-center text-sm font-semibold">
          {done ? (
            same ? (
              <span className="text-emerald-600">{t.compare.fits}</span>
            ) : (
              <span className="text-red-500">{t.compare.rejected}</span>
            )
          ) : (
            <span className="text-muted-foreground">{t.compare.comparing}</span>
          )}
        </div>
        <p className="text-xs text-muted-foreground">{t.compare.explainer}</p>
      </CardContent>
    </Card>
  );
}

// --- demo 3: rotating a piece = shifting a list ----------------------------

const DEMO_PIECE: [number, number, number, number] = [8, 9, 12, 17];

function RotationDemo() {
  const t = useT(T);
  const [rot, setRot] = useState(0);
  useEffect(() => {
    const id = setInterval(() => setRot((r) => r + 1), 2600);
    return () => clearInterval(id);
  }, []);
  const edges = useMemo(() => rotateEdges(DEMO_PIECE, rot & 3), [rot]);
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">{t.rotation.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex items-center justify-center gap-6">
          <div
            style={{
              transform: `rotate(${rot * 90}deg)`,
              transition: "transform 600ms cubic-bezier(.4,0,.2,1)",
            }}
          >
            <PieceSvg edges={DEMO_PIECE} size={96} />
          </div>
          <div className="space-y-1.5">
            <div className="flex gap-1">
              {t.rotation.dirLabels.map((d) => (
                <span key={d} className="inline-flex w-9 justify-center text-xs font-semibold text-muted-foreground">
                  {d}
                </span>
              ))}
            </div>
            <div className="flex gap-1">
              {edges.map((c, i) => (
                <span
                  key={i}
                  className="inline-flex h-9 w-9 items-center justify-center rounded border bg-muted/50 font-mono text-sm font-bold transition-all duration-500"
                >
                  {c}
                </span>
              ))}
            </div>
            <div className="text-center text-xs text-muted-foreground">{t.rotation.order}</div>
          </div>
        </div>
        <p className="text-xs text-muted-foreground">{t.rotation.explainer}</p>
      </CardContent>
    </Card>
  );
}

export function BinaryPlayground() {
  return (
    <div className="grid gap-4 lg:grid-cols-3">
      <MotifAsNumber />
      <EdgeCompare />
      <RotationDemo />
    </div>
  );
}
