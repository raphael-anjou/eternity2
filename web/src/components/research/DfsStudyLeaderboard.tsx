import { useMemo } from "react";
import {
  Bar,
  BarChart,
  Cell,
  LabelList,
  ReferenceLine,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useT, useLang } from "@/i18n";
import { useIsClient } from "@/lib/utils";
import data from "@/data/dfs-study.json";

// The DFS-study results, rendered from the committed run data
// (web/src/data/dfs-study.json, derived from the experiment's results.jsonl by
// research/experiments/dfs-study/scripts/make_site_json.py) so the page can
// never drift from what was measured.
//
// Three pieces:
//   1. the leaderboard — mean score per variant over ten corner-pinned
//      instances, coloured by FAMILY (identity, never rank), family also named
//      in the label so colour is never the only signal;
//   2. the raised-stats panel — max depth reached and median node throughput,
//      the axes this study foregrounds (node rate is never cross-compared);
//   3. the matrix — every variant as a delta over its parent (the "what stacks
//      on what" story), generated from the engine registry, not hand-kept.
//
// Community reference engines (mcgavin-c, blackwood-cs) carry no measured score
// here — they run their own instance, not our variants — so they appear only in
// the matrix, labelled as cited, never on the score chart.

type Variant = {
  name: string;
  display: string;
  family: string;
  kind: string;
  parent: string | null;
  delta: string;
  path: string;
  value: string;
  propagate: string;
  breaks: string;
  allows_breaks: boolean;
  note: string;
  n?: number;
  mean?: number | null;
  best?: number | null;
  worst?: number | null;
  median_nps?: number | null;
  nps_unit?: string;
  max_depth?: number | null;
  median_breaks?: number | null;
};

type CommunityEngine = {
  name: string;
  display: string;
  language: string;
  instance: string;
  max_depth: number | null;
  throughput: string;
  note: string;
  collapse?: string;
};

const D = data as {
  budget_s: number;
  seed: number;
  n_instances: number;
  max_score: number;
  community_5clue_record: number;
  variants: Variant[];
  community?: CommunityEngine[];
};

// One hue per family. Validated (dataviz skill, --pairs adjacent): worst
// adjacent CVD ΔE 8.1, normal-vision ΔE 26.5, both light and dark. Family is
// also always named in text, so colour is secondary, never the sole signal.
const FAMILY: Record<string, { fill: string; en: string; fr: string }> = {
  baseline: { fill: "#f59e0b", en: "baseline", fr: "référence" },
  path: { fill: "#3b82f6", en: "path order", fr: "ordre de parcours" },
  heuristic: { fill: "#10b981", en: "heuristic", fr: "heuristique" },
  break: { fill: "#ef4444", en: "breaks", fr: "cassures" },
  community: { fill: "#8b5cf6", en: "community", fr: "communauté" },
};
const familyFill = (f: string) => FAMILY[f]?.fill ?? "#94a3b8";

// The depth the strongest strict variants top out at (row-major 208). Marked on
// the depth chart so the "only breaks pass the wall" reading is immediate.
const STRICT_WALL = 208;

const T = {
  en: {
    boardTitle: "The leaderboard — mean score by variant",
    boardIntro:
      "Mean matched-edge score over ten corner-pinned variants of the official puzzle, single core, 60 s per run. Colour marks the family; the family is named on every bar, so colour is never the only signal. Community reference engines run their own instance and are not scored here.",
    ceiling: "5-clue record 464",
    of: "/ 480",
    depthTitle: "How far each search reached, and how fast",
    depthIntro:
      "The deepest placement each variant reached, out of 256. The dashed line marks the strict backtracking wall near 208; only the break family passes it, reaching depth 243 to 245. The table below adds median throughput, in search-nodes per second, which is never compared across families because a propagating node is not a naive node.",
    depthAxis: "max depth reached (of 256)",
    npsAxis: "median throughput",
    depthReached: "reached depth",
    strictWall: "strict wall ≈ 208",
    matrixTitle: "What stacks on what",
    matrixIntro:
      "Every variant is a depth-first backtracker declared as one change over its parent. This table is generated from the engine registry, so it always matches the code that ran.",
    colVariant: "variant",
    colFamily: "family",
    colDelta: "the one change it adds over its parent",
    colPath: "path",
    colBreaks: "breaks",
    colScore: "mean",
    colDepth: "depth",
    cited: "cited, not run here",
    busy: "Drawing…",
    strict: "strict",
    commTitle: "The community's record engines, run here",
    commIntro:
      "McGavin's C and Blackwood's C# both build and run on the same machine — but neither can take the corner-pinned variants: each is built around one clue configuration and collapses when it changes. So they are measured on their native instance, as reference points, not on the leaderboard above. This is exactly why the from-scratch break variants (which handle a pin as a pre-placed cell) are the runnable stand-ins.",
    commEngine: "engine",
    commInstance: "instance run",
    commDepth: "depth reached",
    commThroughput: "throughput",
    collapseLabel: "The corner-pin collapse",
  },
  fr: {
    boardTitle: "Le classement — score moyen par variante",
    boardIntro:
      "Score moyen (arêtes appariées) sur dix variantes à coins fixés du puzzle officiel, un cœur, 60 s par run. La couleur marque la famille, nommée sur chaque barre : la couleur n'est jamais le seul signal. Les moteurs de référence de la communauté tournent sur leur propre instance et ne sont pas notés ici.",
    ceiling: "record 5 indices 464",
    of: "/ 480",
    depthTitle: "Jusqu'où chaque recherche est allée, et à quelle vitesse",
    depthIntro:
      "La profondeur maximale atteinte par chaque variante, sur 256. La ligne pointillée marque le mur du backtracking strict, vers 208 ; seule la famille des cassures le franchit, atteignant 243 à 245. Le tableau ci-dessous ajoute le débit médian, en nœuds de recherche par seconde, jamais comparé entre familles car un nœud avec propagation n'est pas un nœud naïf.",
    depthAxis: "profondeur max atteinte (sur 256)",
    npsAxis: "débit médian",
    depthReached: "profondeur atteinte",
    strictWall: "mur strict ≈ 208",
    matrixTitle: "Ce qui s'empile sur quoi",
    matrixIntro:
      "Chaque variante est un backtracking en profondeur déclaré comme un seul changement par rapport à son parent. Ce tableau est généré depuis le registre du moteur : il correspond toujours au code exécuté.",
    colVariant: "variante",
    colFamily: "famille",
    colDelta: "le changement qu'elle ajoute à son parent",
    colPath: "parcours",
    colBreaks: "cassures",
    colScore: "moy.",
    colDepth: "prof.",
    cited: "cité, non exécuté ici",
    busy: "Tracé…",
    strict: "strict",
    commTitle: "Les moteurs record de la communauté, exécutés ici",
    commIntro:
      "Le C de McGavin et le C# de Blackwood se compilent et tournent sur la même machine — mais aucun ne peut prendre les variantes à coins fixés : chacun est bâti autour d'une configuration d'indices précise et s'effondre dès qu'elle change. Ils sont donc mesurés sur leur instance native, comme points de repère, hors du classement ci-dessus. C'est précisément pourquoi les variantes à cassures écrites de zéro (qui traitent un indice comme une case déjà posée) sont les substituts exécutables.",
    commEngine: "moteur",
    commInstance: "instance exécutée",
    commDepth: "profondeur atteinte",
    commThroughput: "débit",
    collapseLabel: "L'effondrement dû aux coins fixés",
  },
};

function npsNum(nps: number | null | undefined): string {
  if (nps == null) return "—";
  return nps >= 1e6 ? `${(nps / 1e6).toFixed(1)}M` : `${Math.round(nps / 1e3)}K`;
}

export function DfsStudyLeaderboard() {
  const t = useT(T);
  const { lang } = useLang();
  const isClient = useIsClient();

  // Scored variants, best-first, for the leaderboard and stat panels.
  const scored = useMemo(
    () =>
      D.variants
        .filter((v) => v.mean != null)
        .slice()
        .sort((a, b) => (b.mean ?? 0) - (a.mean ?? 0)),
    [],
  );

  // Same variants, ordered by how deep the search reached, for the depth panel.
  const byDepth = useMemo(
    () =>
      D.variants
        .filter((v) => v.max_depth != null)
        .slice()
        .sort((a, b) => (b.max_depth ?? 0) - (a.max_depth ?? 0)),
    [],
  );

  const familyLabel = (f: string) => FAMILY[f]?.[lang] ?? f;

  return (
    <div className="not-prose space-y-10">
      {/* 1. Leaderboard */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.boardTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.boardIntro}</p>
        <div className="mt-4 h-[520px] w-full">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%">
              <BarChart
                layout="vertical"
                data={scored}
                margin={{ top: 8, right: 56, bottom: 8, left: 8 }}
              >
                <XAxis
                  type="number"
                  domain={[0, 480]}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <YAxis
                  type="category"
                  dataKey="display"
                  width={150}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <Tooltip
                  cursor={{ fill: "currentColor", opacity: 0.06 }}
                  content={({ active, payload }) => {
                    if (!active || !payload?.length) return null;
                    const v = payload[0]?.payload as Variant | undefined;
                    if (!v) return null;
                    return (
                      <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                        <div className="font-semibold">{v.display}</div>
                        <div className="mt-1 text-muted-foreground">
                          {familyLabel(v.family)} · {v.breaks}
                        </div>
                        <div className="mt-1">
                          mean {v.mean} · best {v.best} · worst {v.worst} {t.of}
                        </div>
                        <div className="text-muted-foreground">
                          depth {v.max_depth} · {npsNum(v.median_nps)} {v.nps_unit}
                        </div>
                      </div>
                    );
                  }}
                />
                <ReferenceLine
                  x={D.community_5clue_record}
                  stroke="currentColor"
                  strokeDasharray="4 4"
                  className="text-muted-foreground"
                  label={{
                    value: t.ceiling,
                    position: "top",
                    fontSize: 10,
                    fill: "currentColor",
                  }}
                />
                <Bar dataKey="mean" radius={[0, 4, 4, 0]} isAnimationActive={false}>
                  {scored.map((v) => (
                    <Cell key={v.name} fill={familyFill(v.family)} />
                  ))}
                  <LabelList
                    dataKey="mean"
                    position="right"
                    fontSize={11}
                    className="fill-foreground"
                  />
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
              {t.busy}
            </div>
          )}
        </div>
        <FamilyLegend lang={lang} />
      </section>

      {/* 2. Raised-stats panel: depth + throughput */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.depthTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.depthIntro}</p>

        {/* Depth reached vs the 256-cell ceiling, coloured by family, with the
            strict-backtracking wall marked. Breaks are the only family past it. */}
        <div className="mt-4 h-[520px] w-full">
          {isClient ? (
            <ResponsiveContainer width="100%" height="100%">
              <BarChart
                layout="vertical"
                data={byDepth}
                margin={{ top: 16, right: 40, bottom: 8, left: 8 }}
              >
                <XAxis
                  type="number"
                  domain={[0, 256]}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <YAxis
                  type="category"
                  dataKey="display"
                  width={150}
                  tick={{ fontSize: 11, fill: "currentColor" }}
                  className="text-muted-foreground"
                />
                <Tooltip
                  cursor={{ fill: "currentColor", opacity: 0.06 }}
                  content={({ active, payload }) => {
                    if (!active || !payload?.length) return null;
                    const v = payload[0]?.payload as Variant | undefined;
                    if (!v) return null;
                    return (
                      <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                        <div className="font-semibold">{v.display}</div>
                        <div className="mt-1">
                          {t.depthReached} {v.max_depth} / 256
                        </div>
                        <div className="text-muted-foreground">
                          {familyLabel(v.family)} · mean {v.mean}
                        </div>
                      </div>
                    );
                  }}
                />
                <ReferenceLine
                  x={STRICT_WALL}
                  stroke="currentColor"
                  strokeDasharray="4 4"
                  className="text-muted-foreground"
                  label={{
                    value: t.strictWall,
                    position: "top",
                    fontSize: 10,
                    fill: "currentColor",
                  }}
                />
                <Bar dataKey="max_depth" radius={[0, 4, 4, 0]} isAnimationActive={false}>
                  {byDepth.map((v) => (
                    <Cell key={v.name} fill={familyFill(v.family)} />
                  ))}
                  <LabelList
                    dataKey="max_depth"
                    position="right"
                    fontSize={11}
                    className="fill-foreground"
                  />
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex h-full items-center justify-center text-sm text-muted-foreground">
              {t.busy}
            </div>
          )}
        </div>
        <FamilyLegend lang={lang} />

        <div className="mt-4 overflow-x-auto">
          <table className="w-full min-w-[520px] border-collapse text-sm">
            <thead>
              <tr className="border-b text-left text-xs text-muted-foreground">
                <th className="py-2 pr-4 font-medium">{t.colVariant}</th>
                <th className="py-2 pr-4 font-medium">{t.colFamily}</th>
                <th className="py-2 pr-4 text-right font-medium">{t.depthAxis}</th>
                <th className="py-2 text-right font-medium">{t.npsAxis}</th>
              </tr>
            </thead>
            <tbody>
              {scored.map((v) => (
                <tr key={v.name} className="border-b last:border-0">
                  <td className="py-1.5 pr-4 font-medium">{v.display}</td>
                  <td className="py-1.5 pr-4">
                    <FamilyTag family={v.family} label={familyLabel(v.family)} />
                  </td>
                  <td className="py-1.5 pr-4 text-right tabular-nums">{v.max_depth ?? "—"}</td>
                  <td className="py-1.5 text-right tabular-nums text-muted-foreground">
                    {npsNum(v.median_nps)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>

      {/* 3. The what-stacks-on-what matrix */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.matrixTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.matrixIntro}</p>
        <div className="mt-4 overflow-x-auto">
          <table className="w-full min-w-[680px] border-collapse text-sm">
            <thead>
              <tr className="border-b text-left text-xs text-muted-foreground">
                <th className="py-2 pr-4 font-medium">{t.colVariant}</th>
                <th className="py-2 pr-4 font-medium">{t.colFamily}</th>
                <th className="py-2 pr-4 font-medium">{t.colDelta}</th>
                <th className="py-2 pr-4 font-medium">{t.colBreaks}</th>
                <th className="py-2 pr-2 text-right font-medium">{t.colScore}</th>
                <th className="py-2 text-right font-medium">{t.colDepth}</th>
              </tr>
            </thead>
            <tbody>
              {D.variants.map((v) => (
                <tr key={v.name} className="border-b align-top last:border-0">
                  <td className="py-2 pr-4 font-medium">{v.display}</td>
                  <td className="py-2 pr-4">
                    <FamilyTag family={v.family} label={familyLabel(v.family)} />
                  </td>
                  <td className="py-2 pr-4 text-muted-foreground">{v.delta}</td>
                  <td className="py-2 pr-4 tabular-nums">
                    {v.allows_breaks ? v.breaks : t.strict}
                  </td>
                  <td className="py-2 pr-2 text-right tabular-nums">
                    {v.mean != null ? v.mean : <span className="text-muted-foreground">{t.cited}</span>}
                  </td>
                  <td className="py-2 text-right tabular-nums text-muted-foreground">
                    {v.max_depth ?? "—"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>

      {/* 4. Community reference engines, measured on their native instances */}
      {D.community && D.community.length > 0 && (
        <section>
          <h3 className="text-base font-semibold tracking-tight">{t.commTitle}</h3>
          <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.commIntro}</p>
          <div className="mt-4 overflow-x-auto">
            <table className="w-full min-w-[560px] border-collapse text-sm">
              <thead>
                <tr className="border-b text-left text-xs text-muted-foreground">
                  <th className="py-2 pr-4 font-medium">{t.commEngine}</th>
                  <th className="py-2 pr-4 font-medium">{t.commInstance}</th>
                  <th className="py-2 pr-4 text-right font-medium">{t.commDepth}</th>
                  <th className="py-2 text-right font-medium">{t.commThroughput}</th>
                </tr>
              </thead>
              <tbody>
                {D.community.map((e) => (
                  <tr key={e.name} className="border-b align-top last:border-0">
                    <td className="py-2 pr-4">
                      <span className="font-medium">{e.display}</span>
                      <span className="ml-1.5 text-xs text-muted-foreground">({e.language})</span>
                    </td>
                    <td className="py-2 pr-4 text-muted-foreground">{e.instance}</td>
                    <td className="py-2 pr-4 text-right tabular-nums">{e.max_depth ?? "—"}</td>
                    <td className="py-2 text-right tabular-nums text-muted-foreground">
                      {e.throughput}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          {D.community.map(
            (e) =>
              e.collapse && (
                <p key={`${e.name}-collapse`} className="mt-4 text-sm leading-relaxed">
                  <span className="font-semibold">{t.collapseLabel}. </span>
                  <span className="text-muted-foreground">{e.collapse}</span>
                </p>
              ),
          )}
          {D.community.map((e) => (
            <p key={`${e.name}-note`} className="mt-2 text-sm leading-relaxed text-muted-foreground">
              <span className="font-medium text-foreground">{e.display}: </span>
              {e.note}
            </p>
          ))}
        </section>
      )}
    </div>
  );
}

function FamilyTag({ family, label }: { family: string; label: string }) {
  return (
    <span className="inline-flex items-center gap-1.5">
      <span
        className="inline-block h-2.5 w-2.5 shrink-0 rounded-sm"
        style={{ backgroundColor: familyFill(family) }}
        aria-hidden
      />
      <span className="text-muted-foreground">{label}</span>
    </span>
  );
}

function FamilyLegend({ lang }: { lang: "en" | "fr" }) {
  return (
    <div className="mt-3 flex flex-wrap gap-x-4 gap-y-1.5 text-xs text-muted-foreground">
      {Object.entries(FAMILY).map(([key, f]) => (
        <span key={key} className="inline-flex items-center gap-1.5">
          <span
            className="inline-block h-2.5 w-2.5 rounded-sm"
            style={{ backgroundColor: f.fill }}
            aria-hidden
          />
          {f[lang]}
        </span>
      ))}
    </div>
  );
}
