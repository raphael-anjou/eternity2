// Build a self-contained SVG string for a single piece, suitable for download
// (no external <defs>/<use> references — every motif used by the piece is
// inlined). Mirrors MotifDefs (triangle + clip) and PieceSvg (four rotated
// edges + border), so the output looks identical to what's shown on the page.

import { motifFor, directionRotation } from "@/lib/motifs";
import type { Edges } from "@/lib/bucas";
import { zipSync, strToU8 } from "fflate";

const TRIANGLE = "M0,0 L-128,128 L-128,-128 Z";

/** Inline <g> definition for one motif, referenced as #e2m-<color>. */
function motifDef(color: number): string {
  const m = motifFor(color);
  const bg = `<path d="${TRIANGLE}" fill="${m.bg}"/>`;
  const deco = m.path
    ? `<path d="${m.path}" fill="${m.pathFill ?? "none"}" stroke="${m.pathStroke ?? "none"}" stroke-width="1" clip-path="url(#e2m-clip)"/>`
    : "";
  return `<g id="e2m-${color}">${bg}${deco}</g>`;
}

/** A complete, standalone SVG document for the given piece edges. */
export function pieceToSvg(edges: Edges): string {
  // Only inline the motif defs the piece actually uses (plus the clip path).
  const used = [...new Set(edges)].sort((a, b) => a - b);
  const defs = used.map(motifDef).join("");
  const uses = edges
    .map((c, dir) => `<use href="#e2m-${c}" transform="rotate(${directionRotation(dir)})"/>`)
    .join("");
  return [
    `<svg xmlns="http://www.w3.org/2000/svg" viewBox="-130 -130 260 260" width="256" height="256">`,
    `<defs><clipPath id="e2m-clip"><path d="${TRIANGLE}"/></clipPath>${defs}</defs>`,
    uses,
    `<rect x="-128" y="-128" width="256" height="256" fill="none" stroke="#000" stroke-width="4"/>`,
    `</svg>`,
  ].join("");
}

/** Trigger a browser download of a Blob under the given filename. */
function downloadBlob(blob: Blob, filename: string): void {
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}

/**
 * Zip every piece as piece-001.svg … piece-256.svg (1-based, zero-padded to
 * keep file order) and download the archive.
 */
export function downloadPiecesZip(pieces: Edges[], filename = "eternity2-pieces.svg.zip"): void {
  const files: Record<string, Uint8Array> = {};
  pieces.forEach((edges, i) => {
    const n = String(i + 1).padStart(3, "0");
    files[`piece-${n}.svg`] = strToU8(pieceToSvg(edges));
  });
  const zipped = zipSync(files, { level: 6 });
  // Copy into a fresh ArrayBuffer-backed view so the Blob owns its bytes.
  downloadBlob(new Blob([zipped.slice()], { type: "application/zip" }), filename);
}
