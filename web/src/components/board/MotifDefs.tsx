// Global SVG <defs> for the 22 motifs + grey border. Mounted once at the app
// root (zero-size svg); every board/piece references #e2m-<color> via <use>.
// Each motif fills the LEFT-edge triangle of a 256×256 cell centered at the
// origin; rotate 90/180/-90 to face up/right/down (bucas convention).

import { MOTIFS } from "@/lib/motifs";

const TRIANGLE = "M0,0 L-128,128 L-128,-128 Z";

export function MotifDefs({ prefix = "e2m" }: { prefix?: string }) {
  return (
    <svg width="0" height="0" style={{ position: "absolute" }} aria-hidden>
      <defs>
        <clipPath id={`${prefix}-clip`}>
          <path d={TRIANGLE} />
        </clipPath>
        {MOTIFS.map((m, i) => (
          <g key={i} id={`${prefix}-${i}`}>
            <path d={TRIANGLE} fill={m.bg} stroke="none" />
            {m.path && (
              <path
                d={m.path}
                fill={m.pathFill ?? "none"}
                stroke={m.pathStroke ?? "none"}
                strokeWidth={1}
                clipPath={`url(#${prefix}-clip)`}
              />
            )}
          </g>
        ))}
        {/* White tick marking one conflicted half-edge (left edge variant). */}
        <path id={`${prefix}-conflict`} d="M-128,-128 L-128,128" stroke="#ffffff" strokeWidth={26} strokeLinecap="round" opacity={0.92} />
      </defs>
    </svg>
  );
}
