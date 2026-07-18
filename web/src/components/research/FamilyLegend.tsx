import { pick, type Dict, type Lang } from "@/i18n";
import { cn } from "@/lib/utils";

// Shared legend primitives for the study leaderboards and the research diagrams.
// Every chart/diagram that maps a colour to a label was rebuilding the same
// "small rounded square + text" chip inline; these give it one home.
//
// `Swatch` is domain-free (it takes a raw CSS colour), so a diagram passing a
// hex or a leaderboard passing a family fill both use it. `FamilyLegend` is the
// leaderboard convenience built on top: hand it the family map and the active
// language and it renders the full legend row.

/** A colour chip + label, laid out inline. `color` is any CSS colour string. */
export function Swatch({
  color,
  label,
  className,
}: {
  color: string;
  label: React.ReactNode;
  className?: string;
}) {
  return (
    <span className={cn("inline-flex items-center gap-1.5", className)}>
      <span
        className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm"
        style={{ backgroundColor: color }}
        aria-hidden
      />
      <span className="text-muted-foreground">{label}</span>
    </span>
  );
}

/** The family map every leaderboard shares: a fill colour + a translated name. */
export type FamilyMap = Record<string, { fill: string } & Dict<string>>;

/** A single family chip (colour resolved from the map by key). */
export function FamilyTag({ color, label }: { color: string; label: string }) {
  return <Swatch color={color} label={label} />;
}

/** The full family legend row for a leaderboard. */
export function FamilyLegend({ families, lang }: { families: FamilyMap; lang: Lang }) {
  return (
    <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1.5 text-xs text-muted-foreground">
      {Object.entries(families).map(([key, f]) => (
        <Swatch key={key} color={f.fill} label={pick(f, lang)} />
      ))}
    </div>
  );
}
