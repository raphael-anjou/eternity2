// Compute-cost helpers for the hardware widget and the experiment results
// table. Kept out of the component file so both can import them without
// tripping react-refresh's "components only" rule.

import type { HardwareInfo } from "./types";

/** Parse the leading duration out of a wall-clock string like "10 × 60 s" or
 *  "10 h" into hours. Understands s/sec, m/min, h/hr, d/day (case-insensitive).
 *  Returns null when no duration is legible. */
export function wallClockHours(s: string | undefined): number | null {
  if (!s) return null;
  const m = /(\d+(?:\.\d+)?)\s*(s|sec|secs|seconds?|m|min|mins|minutes?|h|hr|hrs|hours?|d|days?)\b/i.exec(
    s,
  );
  if (!m || m[1] === undefined || m[2] === undefined) return null;
  const n = Number(m[1]);
  const unit = m[2].toLowerCase();
  if (unit.startsWith("s")) return n / 3600;
  if (unit.startsWith("m")) return n / 60;
  if (unit.startsWith("h")) return n;
  if (unit.startsWith("d")) return n * 24;
  return null;
}

/** Total compute a run cost: cores × wall-clock hours × runs. Uses the
 *  per-run budget from `wallClock` and multiplies by `runs` when both parse, so
 *  a "10 × 60 s" bench reports the summed cost of all ten runs. Null when the
 *  budget can't be read. */
export function coreHours(hw: HardwareInfo): number | null {
  const per = wallClockHours(hw.wallClock);
  if (per == null) return null;
  return hw.cores * per * (hw.runs ?? 1);
}

/** Format core-hours compactly: 0.017, 0.34, 12, 4.0k, 1.2M. */
export function formatCoreHours(v: number): string {
  if (v >= 1e6) return `${(v / 1e6).toFixed(1)}M`;
  if (v >= 1e3) return `${(v / 1e3).toFixed(1)}k`;
  if (v >= 100) return `${Math.round(v)}`;
  if (v >= 1) return v.toFixed(1);
  return v.toPrecision(2);
}
