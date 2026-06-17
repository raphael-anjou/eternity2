// Self-running demos that make "computers think in binary" concrete:
// (1) a whole piece packed into 4x5 bits, (2) "does it fit" as a bit compare,
// (3) rotation as a real bit-rotate of that packed word, (4) "is this piece
// still free?" as one bit in a 256-bit availability mask. Each runs on its
// own clock, no interaction needed.

import { useEffect, useMemo, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { PieceSvg } from "@/components/board/PieceSvg";
import { rotateEdges } from "@/lib/types";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";

const T = {
  en: {
    motif: {
      title: "1 · A whole piece is just 20 bits",
      pieceWord: "one 20-bit number",
      explainer:
        "The computer never sees the artwork. Each of the 22 motifs gets a number that fits in 5 bits, and a piece is four of them: 4 × 5 = 20 bits, one small number. The whole official set is about 1 KB.",
      dirLabels: ["up", "right", "down", "left"],
    },
    compare: {
      title: '2 · "Does it fit?" = "are the numbers equal?"',
      fits: "all bits equal → the piece FITS ✓",
      rejected: "a bit differs → rejected, try the next piece",
      comparing: "comparing…",
      explainer:
        'This is one "fit check". A CPU compares all the bits in a single instruction, in under a nanosecond.',
    },
    rotation: {
      title: "3 · Rotating = one bit-rotate",
      explainer:
        "Because a piece is one 20-bit word, a quarter-turn is a single CPU rotate: slide the bits 5 places, the edge that falls off the end wraps back to the front. No drawing is ever turned, and all four orientations cost almost nothing.",
      before: "before",
      after: "after ↻",
    },
    mask: {
      title: "4 · 256 pieces = 256 bits of “still free?”",
      explainer:
        "Fast solvers track which pieces are still unused as one 256-bit number, one bit each. “Is piece #137 free?” is a single bit-test; “mark it used” flips one bit. So checking and updating availability stays almost instant even with 256 pieces. Our own engine does exactly this.",
      placing: (n: number) => `placing piece #${n}…`,
      free: "free",
      used: "used",
    },
  },
  fr: {
    motif: {
      title: "1 · Une pièce entière tient sur 20 bits",
      pieceWord: "un seul nombre de 20 bits",
      explainer:
        "L'ordinateur ne voit jamais le dessin. À chacun des 22 motifs correspond un nombre qui tient sur 5 bits, et comme une pièce en réunit quatre, cela fait 4 × 5 = 20 bits : un tout petit nombre. Le jeu officiel complet pèse à peine 1 Ko.",
      dirLabels: ["haut", "droite", "bas", "gauche"],
    },
    compare: {
      title: "2 · « Ça s'emboîte ? » revient à « les nombres sont-ils égaux ? »",
      fits: "tous les bits coïncident → la pièce CONVIENT ✓",
      rejected: "un bit diffère → on écarte la pièce et on passe à la suivante",
      comparing: "comparaison en cours…",
      explainer:
        "C'est un « test de compatibilité ». Le processeur compare tous les bits d'un coup, en une seule instruction, en moins d'une nanoseconde.",
    },
    rotation: {
      title: "3 · Pivoter une pièce = une seule rotation de bits",
      explainer:
        "Une pièce n'étant qu'un mot de 20 bits, un quart de tour se résume à une rotation du processeur : on fait glisser les bits de 5 crans, et ceux qui débordent par la fin réapparaissent au début. Aucun dessin n'est réellement tourné, et passer d'une orientation à l'autre ne coûte presque rien.",
      before: "avant",
      after: "après ↻",
    },
    mask: {
      title: "4 · 256 pièces = 256 bits pour dire « encore libre ? »",
      explainer:
        "Les solveurs rapides suivent les pièces encore disponibles dans un unique nombre de 256 bits, à raison d'un bit par pièce. « La pièce n°137 est-elle libre ? » se réduit à la lecture d'un seul bit ; la « marquer comme utilisée » revient à en basculer un. Consulter et mettre à jour les disponibilités reste donc quasi instantané, même avec 256 pièces. C'est précisément ainsi que procède notre propre moteur.",
      placing: (n: number) => `pose de la pièce n°${n}…`,
      free: "libre",
      used: "utilisée",
    },
  },
};

function bits5(n: number): string[] {
  return n.toString(2).padStart(5, "0").split("");
}

function BitBox({
  bit,
  state,
  size = "md",
}: {
  bit: string;
  state?: "match" | "diff" | "group" | null;
  size?: "md" | "sm";
}) {
  return (
    <span
      className={cn(
        "inline-flex items-center justify-center rounded border font-mono font-bold transition-colors duration-300",
        size === "md" ? "h-7 w-6 text-sm" : "h-5 w-4 text-xs",
        state === "match" && "border-emerald-500 bg-emerald-500/15 text-emerald-600",
        state === "diff" && "border-red-500 bg-red-500/15 text-red-600",
        state === "group" && "border-sky-400 bg-sky-400/10",
        !state && "bg-muted/50",
      )}
    >
      {bit}
    </span>
  );
}

// --- demo 1: a whole piece packed into 4x5 bits ----------------------------

const PIECE1: [number, number, number, number] = [8, 9, 12, 17];

function PieceAsBits() {
  const t = useT(T);
  const [seed, setSeed] = useState(0);
  // cycle through a few real-looking pieces
  const piece = useMemo<[number, number, number, number]>(() => {
    const pieces: [number, number, number, number][] = [
      PIECE1,
      [3, 21, 14, 5],
      [19, 2, 11, 22],
      [7, 16, 1, 13],
    ];
    return pieces[seed % pieces.length] ?? PIECE1;
  }, [seed]);
  useEffect(() => {
    const id = setInterval(() => setSeed((s) => s + 1), 2600);
    return () => clearInterval(id);
  }, []);
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">{t.motif.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex items-center justify-center gap-4">
          <PieceSvg edges={piece} size={84} />
          <span className="text-2xl text-muted-foreground">=</span>
          <div className="space-y-1">
            {piece.map((c, i) => (
              <div key={i} className="flex items-center gap-2">
                <span className="w-12 text-right text-[10px] text-muted-foreground">
                  {t.motif.dirLabels[i]}
                </span>
                <span className="w-5 text-right font-mono text-xs font-bold tabular-nums">{c}</span>
                <span className="flex gap-0.5">
                  {bits5(c).map((bit, j) => (
                    <BitBox key={j} bit={bit} state="group" size="sm" />
                  ))}
                </span>
              </div>
            ))}
          </div>
        </div>
        <p className="text-center font-mono text-xs text-muted-foreground">
          {piece.map((c) => bits5(c).join("")).join(" ")} · {t.motif.pieceWord}
        </p>
        <p className="text-xs text-muted-foreground">{t.motif.explainer}</p>
      </CardContent>
    </Card>
  );
}

// --- demo 2: does it fit? compare two numbers ------------------------------

// Half-pieces meeting like on a real board: the left piece shows its
// right-edge triangle, the right piece its left-edge triangle, bases facing.
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

// --- demo 3: rotating a piece = one bit-rotate of the packed word ----------

const DEMO_PIECE: [number, number, number, number] = [8, 9, 12, 17];

/** The packed 20-bit word as a flat bit string (up,right,down,left). */
function packedBits(edges: readonly number[]): string {
  return edges.map((c) => bits5(c).join("")).join("");
}

function RotationDemo() {
  const t = useT(T);
  const [rot, setRot] = useState(0);
  useEffect(() => {
    const id = setInterval(() => setRot((r) => r + 1), 2600);
    return () => clearInterval(id);
  }, []);
  const before = useMemo(() => rotateEdges(DEMO_PIECE, rot & 3), [rot]);
  const after = useMemo(() => rotateEdges(DEMO_PIECE, (rot + 1) & 3), [rot]);
  const beforeBits = packedBits(before);
  const afterBits = packedBits(after);

  // The wrapped group: a clockwise turn rotates the 20-bit word right by 5,
  // so the last 5 bits (the 'left' edge) wrap to the front.
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">{t.rotation.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="flex items-center justify-center gap-4">
          <div
            style={{
              transform: `rotate(${rot * 90}deg)`,
              transition: "transform 600ms cubic-bezier(.4,0,.2,1)",
            }}
          >
            <PieceSvg edges={DEMO_PIECE} size={84} />
          </div>
          <div className="space-y-2">
            <div>
              <div className="mb-0.5 text-[10px] text-muted-foreground">{t.rotation.before}</div>
              <div className="flex gap-0.5">
                {beforeBits.split("").map((bit, i) => (
                  <BitBox key={i} bit={bit} state={i >= 15 ? "group" : null} size="sm" />
                ))}
              </div>
            </div>
            <div className="text-center text-sm text-sky-500">↻ rotate right by 5</div>
            <div>
              <div className="mb-0.5 text-[10px] text-muted-foreground">{t.rotation.after}</div>
              <div className="flex gap-0.5">
                {afterBits.split("").map((bit, i) => (
                  <BitBox key={i} bit={bit} state={i < 5 ? "group" : null} size="sm" />
                ))}
              </div>
            </div>
          </div>
        </div>
        <p className="text-xs text-muted-foreground">{t.rotation.explainer}</p>
      </CardContent>
    </Card>
  );
}

// --- demo 4: availability bitmask ------------------------------------------

function AvailabilityMask() {
  const t = useT(T);
  // A small 8x8 = 64-bit window of the real 256-bit mask, all-free to start.
  const N = 64;
  const [used, setUsed] = useState<Set<number>>(new Set());
  const [placing, setPlacing] = useState<number | null>(null);

  useEffect(() => {
    const id = setInterval(() => {
      setUsed((prev) => {
        if (prev.size >= N) return new Set();
        let i: number;
        do {
          i = Math.floor(Math.random() * N);
        } while (prev.has(i));
        setPlacing(i);
        const next = new Set(prev);
        next.add(i);
        return next;
      });
    }, 700);
    return () => clearInterval(id);
  }, []);

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">{t.mask.title}</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="mx-auto grid w-fit grid-cols-8 gap-0.5">
          {Array.from({ length: N }, (_, i) => {
            const isUsed = used.has(i);
            return (
              <span
                key={i}
                className={cn(
                  "inline-flex h-5 w-5 items-center justify-center rounded-sm font-mono text-[10px] font-bold transition-colors duration-200",
                  i === placing
                    ? "bg-sky-500 text-white"
                    : isUsed
                      ? "bg-muted text-muted-foreground/50"
                      : "bg-emerald-500/15 text-emerald-600",
                )}
              >
                {isUsed ? "0" : "1"}
              </span>
            );
          })}
        </div>
        <p className="h-4 text-center font-mono text-xs text-sky-600">
          {placing !== null ? t.mask.placing(placing + 1) : ""}
        </p>
        <p className="text-xs text-muted-foreground">
          <span className="font-semibold text-emerald-600">1 = {t.mask.free}</span> ·{" "}
          <span className="font-semibold text-muted-foreground">0 = {t.mask.used}</span>.{" "}
          {t.mask.explainer}
        </p>
      </CardContent>
    </Card>
  );
}

export function BinaryPlayground() {
  return (
    <div className="grid gap-4 lg:grid-cols-2">
      <PieceAsBits />
      <EdgeCompare />
      <RotationDemo />
      <AvailabilityMask />
    </div>
  );
}
