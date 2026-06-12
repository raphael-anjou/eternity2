export function formatInt(n: number): string {
  return Math.round(n).toLocaleString("en-US");
}

/** Compact 12.3M / 1.2B style. */
export function formatCompact(n: number): string {
  if (n < 10_000) return formatInt(n);
  return Intl.NumberFormat("en-US", { notation: "compact", maximumFractionDigits: 1 }).format(n);
}

export function formatSeconds(totalSeconds: number): string {
  const m = Math.floor(totalSeconds / 60);
  const s = Math.floor(totalSeconds % 60);
  return `${m}:${String(s).padStart(2, "0")}`;
}
