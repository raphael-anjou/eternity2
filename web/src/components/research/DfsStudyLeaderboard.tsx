import { useMemo } from "react";
import { useT, useLang } from "@/i18n";
import { HorizontalScoreChart } from "@/components/research/HorizontalScoreChart";
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
// Community reference engines (mcgavin-c, blackwood-cs) cannot take the pinned
// grid, so they are measured on their own two grids in section 4: a fair
// unpinned grid (centre clue only), and the pinned collapse shown as a score.
// Every one of their scores is canonically rescored from the engine's own board.

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
  // A foreign community engine whose score/depth here is a pin-collapse, not a
  // real result: badged so the low bar reads as the finding, not weakness.
  collapsed?: boolean;
};

type CommunityEngine = {
  name: string;
  display: string;
  language: string;
  instance: string;
  score?: number | null;
  max_depth: number | null;
  throughput: string;
  note: string;
  collapse?: string;
  collapse_score?: number | null;
  collapse_depth?: number | null;
};

// A row of the unpinned "fair" grid: the community engines and our break-family
// baseline, all run on the single mandatory centre clue, one core, sixty
// seconds. Every score is canonically rescored from the engine's own board.
type EngineRecord = { score: number; label: string; cited: boolean };
type LongerRun = { score: number; note: string };

type UnpinnedRow = {
  name: string;
  display: string;
  family: string;
  kind: string;
  instance: string;
  n: number;
  score: number;
  best?: number | null;
  worst?: number | null;
  placed?: number | null;
  max_depth?: number | null;
  throughput: string;
  nps_unit: string;
  note: string;
  record?: EngineRecord | null;
  longer_run_here?: LongerRun | null;
};

const D = data as {
  budget_s: number;
  seed: number;
  n_instances: number;
  max_score: number;
  community_5clue_record: number;
  variants: Variant[];
  community?: CommunityEngine[];
  unpinned?: {
    hint: string;
    seeds: number[];
    budget_s: number;
    max_score: number;
    rows: UnpinnedRow[];
  };
};

// One hue per family. Validated (dataviz skill, --pairs adjacent): worst
// adjacent CVD ΔE 8.1, normal-vision ΔE 26.5, both light and dark. Family is
// also always named in text, so colour is secondary, never the sole signal.
// Families are ALGORITHMIC types, not provenance. "Community" is deliberately
// not here: whose code an engine is (ours, a reimplementation, a foreign
// binary) is provenance, carried by the row's `kind` and its labels, never by a
// colour. McGavin and Blackwood are break-DFS engines, so they wear the break
// colour like every other break variant; that they are third-party is said in
// words, not hue.
const FAMILY: Record<string, { fill: string; en: string; fr: string }> = {
  baseline: { fill: "#f59e0b", en: "baseline", fr: "référence" },
  path: { fill: "#3b82f6", en: "path order", fr: "ordre de parcours" },
  heuristic: { fill: "#10b981", en: "heuristic", fr: "heuristique" },
  break: { fill: "#ef4444", en: "breaks", fr: "cassures" },
};
const familyFill = (f: string) => FAMILY[f]?.fill ?? "#94a3b8";

const T = {
  en: {
    boardTitle: "The leaderboard — mean score by variant",
    boardIntro:
      "Mean matched-edge score over ten corner-pinned variants of the official puzzle, single core, 60 s per run. Colour marks the family; the family is named on every bar, so colour is never the only signal. McGavin's C and Blackwood's C# appear here too, at the score their fixed scan path reaches before a corner pin dead-ends it, badged where they stall; the two-grid section below shows them running properly once the pins are gone.",
    ceiling: "5-clue record 464",
    of: "/ 480",
    depthTitle: "How far each search reached, and how fast",
    depthIntro:
      "The deepest placement each variant reached, out of 256. Strict backtrackers top out in the low 200s (the fastest, NAIVE-CODEGEN, reaches 216); the break family reaches materially deeper, 243 to 245, because it can push past a locally unmatchable edge instead of backtracking out of it. The table below adds median throughput, in search-nodes per second, which is never compared across families because a propagating node is not a naive node.",
    depthAxis: "max depth reached (of 256)",
    npsAxis: "median throughput",
    depthReached: "reached depth",
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
    stallsOnPins: "stalls on pins",
    busy: "Drawing…",
    strict: "strict",
    commTitle: "The community's record engines, run here",
    commIntro:
      "McGavin's C and Blackwood's C# both build and run on the same machine. Neither can take the corner-pinned grid above: each is built around one clue configuration, so an arbitrary corner pin its scan never reaches early dead-ends it at once. The two panels below show both sides of that. First, the pins the study uses collapse them. Second, on a fair grid with only the one mandatory centre clue, they run as designed.",
    commEngine: "engine",
    commInstance: "instance run",
    commDepth: "depth reached",
    commThroughput: "throughput",
    collapseLabel: "The corner-pin collapse",
    unpinnedTitle: "Same budget, same core: 60 s from scratch",
    unpinnedIntro:
      "The same engines on the official pieces with only the mandatory centre clue pinned, so nothing dead-ends on an arbitrary corner. Every score is canonically rescored from the engine's own board, single core, 60 seconds, cold start. This is a controlled-budget comparison, identical conditions for all four, not a contest of best strength: it shows how far each gets in one core-minute, not each engine's ceiling. That one pinned clue still bites: Blackwood scores 214 here, well below the 454 it reaches on its solver page, where its search places every piece freely instead of committing the centre clue up front (a legal board still, since only the centre clue binds, but an easier search). The faint marker on the two foreign engines is their best documented score, which needs long multi-core runs a 60-second budget cannot reach. What the grid does show is that all four run properly once the pins that break a fixed scan path are gone, which the pinned grid denied the two foreign engines.",
    unpinnedCaveat:
      "Throughput is labelled per engine (McGavin counts tiles, the others search-nodes) and is never compared across engines, because the units are not the same work. McGavin is built with its author's own ARM flags (native tuning and link-time optimisation).",
    recordMarkerNote:
      "The dashed line on each foreign engine marks its best documented score (Blackwood ~470, McGavin 469): the official puzzle has never been solved, so no engine has a genuine 480. These records need long multi-core runs a 60-second single-core budget cannot reach; Blackwood's own longer run on this same machine already rescored to 454.",
    collapsePanelTitle: "Pinned: the collapse, as a score",
    collapsePanelIntro:
      "The same two foreign engines on the study's pinned configuration. The bar is the canonical score their board reaches before the fixed scan path strands them; the faint bar behind is what the same engine reaches unpinned. The gap is the collapse.",
    unpinnedGrid: "centre clue only",
    pinnedGrid: "corner-pinned",
  },
  fr: {
    boardTitle: "Le classement — score moyen par variante",
    boardIntro:
      "Score moyen (arêtes appariées) sur dix variantes à coins fixés du puzzle officiel, un cœur, 60 s par run. La couleur marque la famille, nommée sur chaque barre : la couleur n'est jamais le seul signal. Le C de McGavin et le C# de Blackwood y figurent aussi, au score que leur parcours figé atteint avant qu'un coin fixé ne le bloque, étiquetés là où ils calent ; la section à deux grilles plus bas les montre tourner correctement une fois les coins ôtés.",
    ceiling: "record 5 indices 464",
    of: "/ 480",
    depthTitle: "Jusqu'où chaque recherche est allée, et à quelle vitesse",
    depthIntro:
      "La profondeur maximale atteinte par chaque variante, sur 256. Les backtrackers stricts plafonnent autour de 200 (le plus rapide, NAIVE-CODEGEN, atteint 216) ; la famille des cassures va nettement plus loin, 243 à 245, parce qu'elle peut franchir une arête localement inappariable au lieu de rebrousser chemin. Le tableau ci-dessous ajoute le débit médian, en nœuds de recherche par seconde, jamais comparé entre familles car un nœud avec propagation n'est pas un nœud naïf.",
    depthAxis: "profondeur max atteinte (sur 256)",
    npsAxis: "débit médian",
    depthReached: "profondeur atteinte",
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
    stallsOnPins: "bloqué par les coins",
    busy: "Tracé…",
    strict: "strict",
    commTitle: "Les moteurs record de la communauté, exécutés ici",
    commIntro:
      "Le C de McGavin et le C# de Blackwood se compilent et tournent sur la même machine. Aucun ne peut prendre la grille à coins fixés ci-dessus : chacun est bâti autour d'une configuration d'indices précise, si bien qu'un coin fixé que son parcours n'atteint jamais tôt le bloque aussitôt. Les deux panneaux ci-dessous en montrent les deux faces. D'abord, les coins fixés de l'étude les effondrent. Ensuite, sur une grille équitable réduite au seul indice central obligatoire, ils tournent comme prévu.",
    commEngine: "moteur",
    commInstance: "instance exécutée",
    commDepth: "profondeur atteinte",
    commThroughput: "débit",
    collapseLabel: "L'effondrement dû aux coins fixés",
    unpinnedTitle: "Même budget, même cœur : 60 s à froid",
    unpinnedIntro:
      "Les mêmes moteurs sur les pièces officielles avec le seul indice central obligatoire, si bien que rien ne se bloque sur un coin arbitraire. Chaque score est recalculé canoniquement depuis le plateau du moteur, un cœur, 60 secondes, à froid. C'est une comparaison à budget contrôlé, conditions identiques pour les quatre, non un concours de force maximale : elle montre jusqu'où chacun va en une minute sur un cœur, pas le plafond de chaque moteur. Cet unique indice fixé pèse quand même : Blackwood n'obtient ici que 214, bien en deçà des 454 de sa page solveur, où sa recherche place chaque pièce librement au lieu d'ancrer l'indice central d'emblée (un plateau légal malgré tout, puisque seul l'indice central est contraignant, mais une recherche plus facile). Le repère pâle sur les deux moteurs étrangers est leur meilleur score documenté, qui exige de longs runs multi-cœurs qu'un budget d'une minute ne peut atteindre. Ce que la grille montre : les quatre tournent correctement une fois ôtés les coins fixés qui brisent un parcours figé, ce que la grille fixée refusait aux deux moteurs étrangers.",
    unpinnedCaveat:
      "Le débit est étiqueté par moteur (McGavin compte des tuiles, les autres des nœuds de recherche) et n'est jamais comparé entre moteurs, car les unités ne mesurent pas le même travail. McGavin est compilé avec les propres options ARM de son auteur (réglage natif et optimisation à l'édition de liens).",
    recordMarkerNote:
      "La ligne pointillée sur chaque moteur étranger marque son meilleur score documenté (Blackwood ~470, McGavin 469) : le puzzle officiel n'a jamais été résolu, aucun moteur n'atteint donc un véritable 480. Ces records exigent de longs runs multi-cœurs qu'un budget d'une minute sur un cœur ne peut atteindre ; le propre run plus long de Blackwood sur cette même machine atteignait déjà 454.",
    collapsePanelTitle: "Fixé : l'effondrement, en score",
    collapsePanelIntro:
      "Les deux mêmes moteurs étrangers sur la configuration fixée de l'étude. La barre est le score canonique que leur plateau atteint avant que le parcours figé ne les bloque ; la barre pâle derrière est ce que le même moteur atteint sans coins fixés. L'écart est l'effondrement.",
    unpinnedGrid: "indice central seul",
    pinnedGrid: "coins fixés",
  },
};

function npsNum(nps: number | null | undefined): string {
  if (nps == null) return "—";
  return nps >= 1e6 ? `${(nps / 1e6).toFixed(1)}M` : `${Math.round(nps / 1e3)}K`;
}

export function DfsStudyLeaderboard() {
  const t = useT(T);
  const { lang } = useLang();

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

  // The unpinned rows, best-first, each carrying `recordScore` (its documented
  // record, faded behind the measured bar) so the chart data and the coloured
  // cells stay in one order.
  const unpinnedSorted = useMemo(
    () =>
      (D.unpinned?.rows ?? [])
        .map((r) => ({ ...r, recordScore: r.record?.score ?? null }))
        .sort((a, b) => b.score - a.score),
    [],
  );

  const familyLabel = (f: string) => FAMILY[f]?.[lang] ?? f;

  return (
    <div className="not-prose space-y-10">
      {/* 1. Leaderboard */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.boardTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.boardIntro}</p>
        <div className="mt-4">
          <HorizontalScoreChart
            rows={scored}
            valueKey="mean"
            domainMax={480}
            colorOf={(v) => familyFill(v.family)}
            barLabel={(v) => (v.collapsed ? `${v.mean} · ${t.stallsOnPins}` : `${v.mean}`)}
            referenceLines={[{ x: D.community_5clue_record, label: t.ceiling }]}
            busyLabel={t.busy}
            tooltip={(v) => (
              <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                <div className="font-semibold">{v.display}</div>
                <div className="mt-1 text-muted-foreground">
                  {familyLabel(v.family)} · {v.breaks}
                </div>
                <div className="mt-1">
                  {v.collapsed ? (
                    <>
                      {v.mean} {t.of} · {t.stallsOnPins}
                    </>
                  ) : (
                    <>
                      mean {v.mean} · best {v.best} · worst {v.worst} {t.of}
                    </>
                  )}
                </div>
                <div className="text-muted-foreground">
                  depth {v.max_depth} · {npsNum(v.median_nps)} {v.nps_unit}
                </div>
              </div>
            )}
          />
        </div>
        <FamilyLegend lang={lang} />
      </section>

      {/* 2. Raised-stats panel: depth + throughput */}
      <section>
        <h3 className="text-base font-semibold tracking-tight">{t.depthTitle}</h3>
        <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.depthIntro}</p>

        {/* Depth reached vs the 256-cell ceiling, coloured by family. The break
            family reaches materially deeper than any strict variant. */}
        <div className="mt-4">
          <HorizontalScoreChart
            rows={byDepth}
            valueKey="max_depth"
            domainMax={256}
            colorOf={(v) => familyFill(v.family)}
            barLabel={(v) => (v.collapsed ? `${v.max_depth} · ${t.stallsOnPins}` : `${v.max_depth}`)}
            busyLabel={t.busy}
            tooltip={(v) => (
              <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                <div className="font-semibold">{v.display}</div>
                <div className="mt-1">
                  {t.depthReached} {v.max_depth} / 256
                </div>
                <div className="text-muted-foreground">
                  {familyLabel(v.family)} · mean {v.mean}
                </div>
              </div>
            )}
          />
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

      {/* 4. Community engines: the two-grid story. First the fair unpinned
          leaderboard, then the pinned collapse as a score. */}
      {(D.unpinned || (D.community && D.community.length > 0)) && (
        <section className="space-y-8">
          <div>
            <h3 className="text-base font-semibold tracking-tight">{t.commTitle}</h3>
            <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.commIntro}</p>
          </div>

          {/* 4a. The unpinned "fair grid" leaderboard. */}
          {D.unpinned && D.unpinned.rows.length > 0 && (
            <div>
              <h4 className="text-sm font-semibold tracking-tight">{t.unpinnedTitle}</h4>
              <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.unpinnedIntro}</p>
              <div className="mt-4">
                <HorizontalScoreChart
                  rows={unpinnedSorted}
                  valueKey="score"
                  domainMax={480}
                  height={320}
                  colorOf={(r) => familyFill(r.family)}
                  busyLabel={t.busy}
                  referenceLines={[
                    { x: D.community_5clue_record, label: t.ceiling },
                    ...unpinnedSorted
                      .filter((r) => r.recordScore != null)
                      .map((r, i) => ({
                        x: r.recordScore as number,
                        color: familyFill(r.family),
                        opacity: 0.7,
                        label: `${r.display.replace(/\s*\(.*\)/, "")} ${r.recordScore}`,
                        labelPosition:
                          i % 2 === 0
                            ? ("insideTopRight" as const)
                            : ("insideBottomRight" as const),
                        angle: -90,
                      })),
                  ]}
                  tooltip={(r) => (
                    <div className="rounded-md border bg-popover px-3 py-2 text-xs shadow-md">
                      <div className="font-semibold">{r.display}</div>
                      <div className="mt-1">
                        {r.score} {t.of}
                      </div>
                      <div className="text-muted-foreground">
                        {r.max_depth != null ? `depth ${r.max_depth} · ` : ""}
                        {r.throughput}
                      </div>
                      {r.record && (
                        <div className="mt-1 text-muted-foreground">
                          {r.record.label}: {r.record.score}
                        </div>
                      )}
                      {r.longer_run_here && (
                        <div className="text-muted-foreground">
                          {r.longer_run_here.score} {r.longer_run_here.note}
                        </div>
                      )}
                    </div>
                  )}
                />
              </div>
              <p className="mt-2 text-xs leading-relaxed text-muted-foreground">
                {t.recordMarkerNote} {t.unpinnedCaveat}
              </p>
              {/* Per-row throughput and note, with unit labels kept explicit. */}
              <div className="mt-4 overflow-x-auto">
                <table className="w-full min-w-[560px] border-collapse text-sm">
                  <thead>
                    <tr className="border-b text-left text-xs text-muted-foreground">
                      <th className="py-2 pr-4 font-medium">{t.commEngine}</th>
                      <th className="py-2 pr-2 text-right font-medium">{t.colScore}</th>
                      <th className="py-2 pr-4 text-right font-medium">{t.commDepth}</th>
                      <th className="py-2 text-right font-medium">{t.commThroughput}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {[...D.unpinned.rows]
                      .sort((a, b) => b.score - a.score)
                      .map((r) => (
                        <tr key={r.name} className="border-b align-top last:border-0">
                          <td className="py-2 pr-4">
                            <FamilyTag family={r.family} label={r.display} />
                          </td>
                          <td className="py-2 pr-2 text-right tabular-nums">{r.score}</td>
                          <td className="py-2 pr-4 text-right tabular-nums text-muted-foreground">
                            {r.max_depth ?? r.placed ?? "—"}
                          </td>
                          <td className="py-2 text-right tabular-nums text-muted-foreground">
                            {r.throughput}
                          </td>
                        </tr>
                      ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

          {/* 4b. The pinned collapse, as a score: each foreign engine's pinned
              score with its unpinned score faded behind for contrast. */}
          {D.community && D.community.some((e) => e.collapse_score != null) && (
            <div>
              <h4 className="text-sm font-semibold tracking-tight">{t.collapsePanelTitle}</h4>
              <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.collapsePanelIntro}</p>
              <div className="mt-4 space-y-3">
                {D.community
                  .filter((e) => e.collapse_score != null)
                  .map((e) => (
                    <CollapseBar
                      key={e.name}
                      display={e.display}
                      language={e.language}
                      unpinned={e.score ?? null}
                      pinned={e.collapse_score ?? null}
                      pinnedDepth={e.collapse_depth ?? null}
                      max={D.max_score}
                      unpinnedLabel={t.unpinnedGrid}
                      pinnedLabel={t.pinnedGrid}
                    />
                  ))}
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
            </div>
          )}
        </section>
      )}
    </div>
  );
}

// One engine's pinned-vs-unpinned score as a horizontal pair: the faded bar is
// the unpinned (fair-grid) score, the solid bar the pinned score it collapses
// to. The visual gap is the collapse. Plain divs, not recharts, so the two
// scores read as one before/after unit per engine.
function CollapseBar({
  display,
  language,
  unpinned,
  pinned,
  pinnedDepth,
  max,
  unpinnedLabel,
  pinnedLabel,
}: {
  display: string;
  language: string;
  unpinned: number | null;
  pinned: number | null;
  pinnedDepth: number | null;
  max: number;
  unpinnedLabel: string;
  pinnedLabel: string;
}) {
  // McGavin and Blackwood are break-DFS engines, so their bars wear the break
  // colour, like every other break variant on the page.
  const fill = familyFill("break");
  const pct = (v: number | null) => (v == null ? 0 : Math.max(1.5, (v / max) * 100));
  return (
    <div>
      <div className="flex items-baseline justify-between text-sm">
        <span className="font-medium">
          {display}
          <span className="ml-1.5 text-xs text-muted-foreground">({language})</span>
        </span>
        <span className="tabular-nums text-muted-foreground">
          {pinnedLabel} {pinned} · {unpinnedLabel} {unpinned}
        </span>
      </div>
      {/* The track carries the faded unpinned bar; the solid pinned bar sits on
          top from the same left origin, so the collapse is the visible gap. */}
      <div className="relative mt-1.5 h-5 w-full overflow-hidden rounded-sm bg-muted/40">
        <div
          className="absolute inset-y-0 left-0 rounded-sm opacity-25"
          style={{ width: `${pct(unpinned)}%`, backgroundColor: fill }}
          aria-hidden
        />
        <div
          className="absolute inset-y-0 left-0 rounded-sm"
          style={{ width: `${pct(pinned)}%`, backgroundColor: fill }}
          aria-hidden
        />
        {pinnedDepth != null && (
          <span className="absolute inset-y-0 left-2 flex items-center text-xs font-medium text-foreground">
            depth {pinnedDepth}
          </span>
        )}
      </div>
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
