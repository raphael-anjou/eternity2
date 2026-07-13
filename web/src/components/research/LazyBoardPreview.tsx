// A board preview that mounts its heavy 16x16 SVG only when it scrolls near the
// viewport — and never at build time. The notable-boards page shows dozens of
// full boards at once; a 256-cell BoardSvg is ~1,300 SVG nodes, so prerendering
// them all inlined a ~6 MB HTML file with tens of thousands of nodes, which is
// slow on desktop and crashes phones. So: the server (prerender) and the first
// client paint render a light placeholder; each board mounts only once it
// scrolls near, via an IntersectionObserver.
//
// It also highlights the non-matching edges (the same conflict marks the board
// viewer shows), computed once from the decoded board.

import { memo, useEffect, useMemo, useRef, useState } from "react";
import { BoardSvg } from "@/components/board/BoardSvg";
import { decodeBucas, conflictEdges } from "@/lib/bucas";
import { useIsClient } from "@/lib/utils";

/** Reserve the board's square footprint so nothing shifts when it mounts. */
function Placeholder() {
  return (
    <div
      className="w-full animate-pulse rounded-md bg-muted"
      style={{ aspectRatio: "1 / 1" }}
      aria-hidden
    />
  );
}

export interface LazyBoardPreviewProps {
  /** Bucas viewer query string (e.g. "puzzle_size=16&board_edges=..."). */
  params: string;
  /** Mark the non-matching edges, as the board viewer does. Default true. */
  showConflicts?: boolean;
  className?: string;
}

export const LazyBoardPreview = memo(function LazyBoardPreview({
  params,
  showConflicts = true,
  className,
}: LazyBoardPreviewProps) {
  const ref = useRef<HTMLDivElement | null>(null);
  const isClient = useIsClient();
  // No IntersectionObserver (very old browser): treat as always near. The
  // server-render gate (isClient) still keeps it out of the prerendered HTML.
  const [near, setNear] = useState(() => typeof IntersectionObserver === "undefined");

  useEffect(() => {
    const el = ref.current;
    if (!el || near || typeof IntersectionObserver === "undefined") return;
    const io = new IntersectionObserver(
      (entries) => {
        if (entries.some((e) => e.isIntersecting)) {
          setNear(true);
          io.disconnect();
        }
      },
      // Start rendering a little before the board enters view.
      { rootMargin: "400px 0px" },
    );
    io.observe(el);
    return () => io.disconnect();
  }, [near]);

  // Render the board only on the client (never in the prerendered HTML) and
  // only once it has scrolled near.
  const visible = isClient && near;

  // Decode + score only once the board is about to show.
  const board = useMemo(() => {
    if (!visible) return null;
    try {
      const decoded = decodeBucas(params);
      const conflicts = showConflicts ? conflictEdges(decoded) : undefined;
      return { decoded, conflicts };
    } catch {
      return null;
    }
  }, [visible, params, showConflicts]);

  return (
    <div ref={ref} className={className}>
      {board ? (
        <BoardSvg
          width={board.decoded.width}
          height={board.decoded.height}
          cells={board.decoded.cells}
          conflicts={board.conflicts}
        />
      ) : (
        <Placeholder />
      )}
    </div>
  );
});
