import type { ReactNode, Ref } from "react";
import { cn } from "@/lib/utils";

// The shared shell for the research "lab" widgets (the interactive concept
// demos). Every one of them was repeating the same card + title/intro block
// (and often a trailing note), which meant a styling tweak had to be made in a
// dozen files. This owns that chrome; each lab supplies only its title, intro,
// its body as children, and an optional note.
//
// The lab keeps its own useRunWhileVisible() (it reads `visible` in its own
// animation effect); it just hands the returned `ref` to `Lab` so the card root
// is what the visibility observer watches. `title`/`intro`/`note` are ReactNode
// so a lab can pass already-translated strings (the usual `t.title`) or richer
// markup.

export function Lab({
  ref,
  title,
  intro,
  note,
  className,
  children,
}: {
  ref?: Ref<HTMLDivElement>;
  title: ReactNode;
  intro?: ReactNode;
  note?: ReactNode;
  className?: string;
  children: ReactNode;
}) {
  return (
    <div ref={ref} className={cn("space-y-4 rounded-lg border bg-card p-4", className)}>
      <div className="space-y-1">
        <h3 className="text-sm font-semibold">{title}</h3>
        {intro != null && (
          <p className="text-xs leading-relaxed text-muted-foreground">{intro}</p>
        )}
      </div>
      {children}
      {note != null && <p className="text-xs text-muted-foreground">{note}</p>}
    </div>
  );
}
