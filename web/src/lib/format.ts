export function formatInt(n: number): string {
  return Math.round(n).toLocaleString("en-US");
}

/** Compact 12.3M / 1.2B style. */
export function formatCompact(n: number): string {
  if (n < 10_000) return formatInt(n);
  return Intl.NumberFormat("en-US", { notation: "compact", maximumFractionDigits: 1 }).format(n);
}

/**
 * Uppercase-K/M compaction for the leaderboard tables and tooltips (throughput,
 * iteration counts): `1.2M`, `440K`, `—` for null/undefined. `exactBelow` is the
 * threshold under which a value is shown as a plain rounded integer instead of
 * being compacted to `K`: the default 0 always compacts (the nps style, e.g.
 * `500` -> `1K`), while `1000` keeps sub-thousand values exact (the
 * iteration-count style, e.g. `500` -> `500`).
 */
export function formatKM(n: number | null | undefined, exactBelow = 0): string {
  if (n == null) return "—";
  if (n >= 1e6) return `${(n / 1e6).toFixed(1)}M`;
  if (n < exactBelow) return `${Math.round(n)}`;
  return `${Math.round(n / 1e3)}K`;
}

export function formatSeconds(totalSeconds: number): string {
  const m = Math.floor(totalSeconds / 60);
  const s = Math.floor(totalSeconds % 60);
  return `${m}:${String(s).padStart(2, "0")}`;
}

const SUPERSCRIPTS: Record<string, string> = {
  "0": "⁰", "1": "¹", "2": "²", "3": "³", "4": "⁴",
  "5": "⁵", "6": "⁶", "7": "⁷", "8": "⁸", "9": "⁹", "-": "⁻",
};

/** Render an integer as Unicode superscript digits (e.g. 45 → "⁴⁵"). */
export function superscript(n: number): string {
  return String(n)
    .split("")
    .map((c) => SUPERSCRIPTS[c] ?? c)
    .join("");
}

/** A large number as a compact power-of-ten string (e.g. 3.4e37 → "3.4×10³⁷").
 *  Below `plainBelow` (default 1000) it falls back to a grouped integer. */
export function formatScientific(n: number, plainBelow = 1000): string {
  if (!Number.isFinite(n)) return String(n);
  if (Math.abs(n) < plainBelow) return formatInt(n);
  const exp = Math.floor(Math.log10(Math.abs(n)));
  const mant = n / Math.pow(10, exp);
  return `${mant.toFixed(1)}×10${superscript(exp)}`;
}
