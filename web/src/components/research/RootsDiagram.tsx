// The research homepage's primary wayfinder: a rooted "tree of paths". One
// trunk (the puzzle) splits into the nine ways people attack it — the theme
// registry (topics.json), minus the meta "records" theme, which is not a method
// and lives in the band below. Each root is a link to its topic hub, which
// already aggregates every why/build/lab page tagged with that theme, so the
// diagram is a live index, not a static picture. SVG on desktop; the same nodes
// reflow to a plain responsive grid on small screens (and are the a11y content).

import { useLang } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import { LocalizedLink } from "@/components/LocalizedLink";
import { researchTopics, topicUrl } from "@/lib/research/manifest";
import { topicMembers } from "@/lib/research/nav";

// Short node label + one-line hook per theme slug. The registry descriptions
// are full paragraphs; these are the tree-node versions. Colour is the accent
// dot, echoing the kind-dot palette used across the wiki.
const ROOT: Record<string, { label: string; hook: string; color: string }> = {
  structure: {
    label: "Why it resists",
    hook: "The design tuned to be unsolvable, and the walls that prove it.",
    color: "#a78bfa", // violet
  },
  "search-space": {
    label: "Reduce the search",
    hook: "Throw away hopeless states before the search wastes time on them.",
    color: "#38bdf8", // sky
  },
  backtracking: {
    label: "Backtracking",
    hook: "DFS done seriously: fill orders, restarts, break indices.",
    color: "#22d3ee", // cyan
  },
  speed: {
    label: "Go faster",
    hook: "Bit tricks and cache — and why raw speed alone won't crack it.",
    color: "#fbbf24", // amber
  },
  construction: {
    label: "Build boards up",
    hook: "Beam search, priors, staged assembly — compose, don't dig.",
    color: "#34d399", // emerald
  },
  "local-search": {
    label: "Local search",
    hook: "Improve a board you already have; the rigidity walls that stop you.",
    color: "#4ade80", // green
  },
  "exact-methods": {
    label: "Exact methods",
    hook: "Solvers with proofs: SAT, MIP, exact-cover, meet-in-the-middle.",
    color: "#60a5fa", // blue
  },
  learning: {
    label: "Learned guidance",
    hook: "Belief propagation, learned heuristics, corpus priors.",
    color: "#f472b6", // pink
  },
  hardware: {
    label: "GPU & hardware",
    hook: "GPU, FPGA, distributed sweeps — silicon thrown at the wall.",
    color: "#fb923c", // orange
  },
};

// The meta "records" theme is not a solving path; it belongs in the band below
// the tree with People and History, so the tree stays "the nine ways to attack".
const EXCLUDE = new Set(["records"]);

interface Root {
  slug: string;
  label: string;
  hook: string;
  color: string;
  count: number;
}

function useRoots(): Root[] {
  const { lang } = useLang();
  const out: Root[] = [];
  for (const t of researchTopics(lang)) {
    const def = ROOT[t.slug];
    if (EXCLUDE.has(t.slug) || !def) continue;
    out.push({ slug: t.slug, ...def, count: topicMembers(lang, t.slug).length });
  }
  return out;
}

/** The SVG tree: a trunk that forks into curved roots ending at each theme. */
function Tree({ roots }: { roots: Root[] }) {
  const n = roots.length;
  const W = 920;
  // Inset the columns so the outermost node labels never touch the edge.
  const PAD = 54;
  const COL = (W - 2 * PAD) / (n - 1);
  const colX = (i: number) => PAD + COL * i;
  const trunkX = W / 2;
  const trunkTop = 34; // leaves headroom above the crown for its label
  const forkY = 96; // where the trunk stops and roots fan out
  const nodeY = 196;
  const H = 250;

  return (
    <svg
      viewBox={`0 0 ${W} ${H}`}
      className="w-full"
      role="img"
      aria-label="A tree of the nine ways to attack Eternity II, each a link to its topic hub."
    >
      {/* trunk */}
      <line
        x1={trunkX}
        y1={trunkTop}
        x2={trunkX}
        y2={forkY}
        stroke="currentColor"
        strokeWidth={6}
        strokeLinecap="round"
        className="text-muted-foreground/50"
      />
      {/* the puzzle, at the crown */}
      <g>
        <circle cx={trunkX} cy={trunkTop} r={9} className="fill-foreground" />
        <text
          x={trunkX}
          y={trunkTop - 14}
          textAnchor="middle"
          className="fill-foreground font-semibold"
          style={{ fontSize: 15 }}
        >
          Eternity II
        </text>
      </g>

      {/* one curved root per theme, drawn back-to-front so nodes sit on top */}
      {roots.map((r, i) => {
        const x = colX(i);
        // A smooth cubic from the fork down to the node column.
        const d = `M ${trunkX} ${forkY} C ${trunkX} ${forkY + 44}, ${x} ${nodeY - 60}, ${x} ${nodeY - 12}`;
        return (
          <path
            key={r.slug}
            d={d}
            fill="none"
            stroke={r.color}
            strokeWidth={2.5}
            strokeOpacity={0.55}
            strokeLinecap="round"
          />
        );
      })}

      {/* nodes: a linked circle + label + count under each root */}
      {roots.map((r, i) => {
        const x = colX(i);
        return (
          <a key={r.slug} href={topicUrl(r.slug)} className="group">
            <title>{r.hook}</title>
            <circle
              cx={x}
              cy={nodeY - 12}
              r={7}
              fill={r.color}
              className="transition-[r] group-hover:[r:9px]"
            />
            <text
              x={x}
              y={nodeY + 14}
              textAnchor="middle"
              className="fill-foreground font-medium group-hover:underline"
              style={{ fontSize: 12.5 }}
            >
              {r.label}
            </text>
            <text
              x={x}
              y={nodeY + 32}
              textAnchor="middle"
              className="fill-muted-foreground"
              style={{ fontSize: 10.5 }}
            >
              {r.count} {r.count === 1 ? "page" : "pages"}
            </text>
          </a>
        );
      })}
    </svg>
  );
}

/** The reflow / accessible version: a responsive grid of the same links. */
function Grid({ roots }: { roots: Root[] }) {
  return (
    <ul className="not-prose grid list-none grid-cols-1 gap-2 p-0 sm:grid-cols-2 lg:grid-cols-3">
      {roots.map((r) => (
        <li key={r.slug}>
          <LocalizedLink
            to={topicUrl(r.slug)}
            className="group flex gap-3 rounded-lg border p-3 no-underline transition-shadow hover:shadow-sm"
          >
            <span
              className="mt-1 h-3 w-3 shrink-0 rounded-full"
              style={{ backgroundColor: r.color }}
              aria-hidden
            />
            <span className="min-w-0">
              <span className="block text-sm font-semibold group-hover:underline">
                {r.label}{" "}
                <span className="font-normal text-muted-foreground">· {r.count}</span>
              </span>
              <span className="block text-xs leading-snug text-muted-foreground">{r.hook}</span>
            </span>
          </LocalizedLink>
        </li>
      ))}
    </ul>
  );
}

export function RootsDiagram() {
  const roots = useRoots();
  const isClient = useIsClient();

  return (
    <div className="not-prose my-8">
      {/* The SVG tree is a desktop luxury; the grid is the source of truth and
          the only thing shown on small screens or before hydration. */}
      {isClient && (
        <div className="hidden md:block">
          <Tree roots={roots} />
        </div>
      )}
      <div className={isClient ? "md:hidden" : ""}>
        <Grid roots={roots} />
      </div>
    </div>
  );
}
