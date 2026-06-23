import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";

// A synthesis page: the four structural walls (the "Why it's hard" door) on one
// axis, the invented algorithms (the "Lab notebook" door) on the other. Each
// method is mapped to the wall(s) it fundamentally attacks and the exact score
// where it saturated. Every cell is grounded in the research vault; the four
// walls themselves are corroborated by the published literature (the 17-colour
// phase transition: Ansótegui et al., "How Hard is a Commercial Puzzle"; the
// missing gradient / deep basins: the E2 local-search papers). The invention
// results are this project's own work, reproducible from source — they are not
// claimed to be externally verified, only the walls are.

type WallKey = "forced" | "peak" | "area" | "rigidity";

const WALL_KEYS: WallKey[] = ["forced", "peak", "area", "rigidity"];

// path (without lang prefix) for each wall's explainer page
const WALL_LINK: Record<WallKey, string> = {
  forced: "/research/why/no-forced-moves",
  peak: "/research/why/phase-transition",
  area: "/research/why/entropy-area-law",
  rigidity: "/research/why/rigidity-wall",
};

type Method = {
  key: string;
  to: string;
  score: number;
  family: "scratch" | "corpus" | "concentrate" | "anchor" | "exact" | "decode";
  /** walls this method primarily attacks (1-2). */
  walls: WallKey[];
  /** did it open a genuinely new basin/family? */
  newBasin: boolean;
};

// Order roughly by score so the table reads as a ladder up to the 469 ceiling.
const METHODS: Method[] = [
  { key: "palimpsest", to: "/research/lab/experiments/palimpsest", score: 463, family: "corpus", walls: ["rigidity"], newBasin: false },
  { key: "prior", to: "/research/lab/experiments/prior", score: 460, family: "scratch", walls: ["rigidity", "peak"], newBasin: false },
  { key: "keyring", to: "/research/lab/experiments/keyring", score: 460, family: "scratch", walls: ["rigidity"], newBasin: true },
  { key: "replay", to: "/research/lab/experiments/replay", score: 460, family: "decode", walls: ["rigidity"], newBasin: false },
  { key: "gauntlet", to: "/research/lab/experiments/gauntlet", score: 458, family: "scratch", walls: ["rigidity"], newBasin: true },
  { key: "cloister", to: "/research/lab/experiments/cloister", score: 453, family: "anchor", walls: ["area", "rigidity"], newBasin: false },
  { key: "midden", to: "/research/lab/experiments/midden", score: 452, family: "anchor", walls: ["rigidity", "area"], newBasin: false },
  { key: "ladder", to: "/research/lab/experiments/ladder", score: 451, family: "concentrate", walls: ["peak", "forced"], newBasin: true },
  { key: "lodestone", to: "/research/lab/experiments/lodestone", score: 451, family: "scratch", walls: ["forced"], newBasin: false },
  { key: "mosaic", to: "/research/lab/experiments/mosaic", score: 448, family: "exact", walls: ["forced", "peak"], newBasin: false },
  { key: "bandsaw", to: "/research/lab/experiments/bandsaw", score: 437, family: "exact", walls: ["forced", "peak"], newBasin: false },
  { key: "staged", to: "/research/lab/experiments/staged", score: 436, family: "scratch", walls: ["forced", "area"], newBasin: false },
];

const CEILING = 469;
const TARGET = 480;

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Which wall stops which method",
    lede: "The research section has two halves: the structural walls that make Eternity II hard, and the algorithms built to climb them. This page is the bridge. Each method here is lined up against the wall it actually attacks — and the score where that wall stopped it.",
    wallsTitle: "The four walls, in one line each",
    wallsIntro: "Every one of them is a version of the same statement — there is nothing local to prune on. They are corroborated by the published literature, not just our own measurements.",
    walls: {
      forced: ["No forced moves", "Every interior cell keeps 73–137 legal pieces, so the branching factor never collapses."],
      peak: ["The hardness peak", "≈17 interior colours put the puzzle at the phase transition: about one expected solution, the worst place to search."],
      area: ["The area law", "Genuinely-distinct partial boards collapse past ~80 cells, but no local score can see that global fact."],
      rigidity: ["Rigidity", "Records are locally frozen; the step to a better board is one giant, indivisible swap with no gradient to follow."],
    } as Record<WallKey, [string, string]>,
    matrixTitle: "The map",
    matrixIntro: "Columns are the walls; rows are the methods, best score first. A filled mark means the method is fundamentally working against that wall. The community ceiling on this puzzle is 469; the full solution is 480.",
    colMethod: "Method",
    colScore: "Best",
    colNew: "New basin",
    newYes: "new family",
    newNo: "—",
    readTitle: "How to read it",
    read: "Almost every method ends up against rigidity, the wall that says the great boards are isolated islands. The build-from-scratch and corpus methods (PRIOR, KEYRING, PALIMPSEST, GAUNTLET) try to reach a new island by steering construction with learned signal; they top out at 458–463 and a couple of them do reach genuinely new families, but none crosses to the ceiling. The concentrate and exact methods (LADDER, BANDSAW) instead attack the search itself — the high branching factor and the unsearchable peak — and pay for it at the endgame. The anchor methods (CLOISTER, MIDDEN) localise the damage but hit the area-law wall in the interior. No single wall is the whole story, and no method gets through all four.",
    saturationTitle: "Where each one stopped, and why",
    saturationIntro: "The ceiling is never arbitrary. For each method the vault records the exact reason the score stopped climbing — quoted here in one line.",
    saturation: {
      palimpsest: "Reached 463, the project best, by reading the corpus to find which shared choices are traps and steering a 15-basin sweep around them. Forcing the search to avoid the traps directly made boards worse — the value was in where to look, not a hard rule.",
      prior: "Plateaus at 460: the learned position prior gets a from-scratch build into the 460 class fast, but the corpus signal alone is not enough to leave it.",
      keyring: "Three learned signals voting (position, adjacency, 2×2 patch) reached 460 in a corner arrangement no board had cracked before — a new family — but the patch signal is marginal and the polish still caps at 460.",
      replay: "Replays the community's strict-460 boards exactly and reveals the move ordinary search misses: 4–5 cells that take two mismatches at once, unreachable for a search that allows at most one.",
      gauntlet: "Running the beam in nine scan directions opened a brand-new 458 family (scan order is a stronger diversity axis than the random seed) — but a second round topped out at 457 with no 461: the new family saturates like the others.",
      cloister: "As a standalone interior solver it confirms a real rim-compatibility bonus, but that bonus cannot be retrofitted after the fact — the same rigidity as the full board — so it settles in the low 450s.",
      midden: "Choosing where (not when) the board may break extends the perfect run from 153 to 167–174 cells, but the dispersed geometry still fails at the endgame: nothing absorbs the last damage.",
      ladder: "Floods cheap probes and promotes the deepest, reaching a 451 strict board with no record to copy — the first escape from the universal 444–450 band — but the supply of perfect openings runs out and the rungs all converge to one ceiling.",
      bandsaw: "Solves an endgame band to proven optimality, and in doing so measures the exactness wall: the search tree grows about twentyfold per extra allowed mismatch, on both sides, so meeting in the middle stops paying at full size.",
      staged: "Builds the whole board with no pre-set frame and an emergent border, reaching 436 — well below the records. That honest gap is the finding: it measures exactly how much the usual frame-first anchor is worth.",
      mosaic: "Composes exact 4×4 block solutions with soft seams, reaching 448 from scratch — but the shortfall lands almost entirely in the last three corner blocks, where the piece pool runs thin: the same piece-theft, now a single bright spot.",
      lodestone: "A scarce-demand prior used only as a tiebreaker lifts the from-scratch median by two (449→451) and tightens the variance, but any larger weight collapses it — scarcity is a real but weak signal, and it never touches the basin ceiling.",
    } as Record<string, string>,
    families: {
      scratch: "from scratch",
      corpus: "from the corpus",
      concentrate: "concentrate effort",
      anchor: "anchor & constrain",
      exact: "solve a piece exactly",
      decode: "decode & replay",
    } as Record<Method["family"], string>,
    closingTitle: "The shape of the gap",
    closing: "Read down the table and the lesson of the whole project is visible at a glance: the methods that move the score change the shape of the search — a scan order, a learned prior, a confined region — never its raw speed. And every one of them stops at a wall that is global, not local. The eleven edges from 469 to 480 are not a polishing problem; they are on the far side of all four walls at once.",
    sourceNote: "Each saturation line is distilled from the project's lab notebook (one volume per invention). The four walls are corroborated by the published literature: the 17-colour phase transition by Ansótegui, Béjar, Fernández & Mateu, “How Hard is a Commercial Puzzle: the Eternity II Challenge”; the missing gradient and deep basins by the Eternity II local-search literature.",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Quel mur arrête quelle méthode",
    lede: "La section recherche a deux moitiés : les murs structurels qui rendent Eternity II difficile, et les algorithmes conçus pour les gravir. Cette page est le pont. Chaque méthode est alignée face au mur qu'elle attaque réellement — et au score où ce mur l'a arrêtée.",
    wallsTitle: "Les quatre murs, en une ligne chacun",
    wallsIntro: "Chacun est une version de la même affirmation — il n'y a rien de local à élaguer. Ils sont corroborés par la littérature publiée, pas seulement par nos mesures.",
    walls: {
      forced: ["Aucun coup forcé", "Chaque case intérieure garde 73–137 pièces légales, donc le facteur de branchement ne s'effondre jamais."],
      peak: ["Le pic de difficulté", "≈17 couleurs intérieures placent le puzzle à la transition de phase : environ une solution attendue, le pire endroit où chercher."],
      area: ["La loi d'aire", "Les plateaux partiels vraiment distincts s'effondrent au-delà de ~80 cellules, mais aucun score local ne voit ce fait global."],
      rigidity: ["La rigidité", "Les records sont figés localement ; le pas vers un meilleur plateau est un seul échange géant, indivisible, sans gradient à suivre."],
    } as Record<WallKey, [string, string]>,
    matrixTitle: "La carte",
    matrixIntro: "Les colonnes sont les murs ; les lignes, les méthodes, meilleur score d'abord. Une marque pleine signifie que la méthode travaille fondamentalement contre ce mur. Le plafond communautaire sur ce puzzle est 469 ; la solution complète est 480.",
    colMethod: "Méthode",
    colScore: "Best",
    colNew: "Nouveau bassin",
    newYes: "nouvelle famille",
    newNo: "—",
    readTitle: "Comment la lire",
    read: "Presque toutes les méthodes finissent face à la rigidité, le mur qui dit que les grands plateaux sont des îles isolées. Les méthodes de construction et de corpus (PRIOR, KEYRING, PALIMPSEST, GAUNTLET) tentent d'atteindre une nouvelle île en orientant la construction par un signal appris ; elles plafonnent à 458–463, et quelques-unes atteignent de vraies nouvelles familles, mais aucune ne franchit le plafond. Les méthodes de concentration et exactes (LADDER, BANDSAW) attaquent plutôt la recherche elle-même — le fort facteur de branchement et le pic inexplorable — et le paient en fin de partie. Les méthodes d'ancrage (CLOISTER, MIDDEN) localisent les défauts mais butent sur la loi d'aire à l'intérieur. Aucun mur seul n'est toute l'histoire, et aucune méthode ne passe les quatre.",
    saturationTitle: "Où chacune s'est arrêtée, et pourquoi",
    saturationIntro: "Le plafond n'est jamais arbitraire. Pour chaque méthode, le carnet consigne la raison exacte de l'arrêt — citée ici en une ligne.",
    saturation: {
      palimpsest: "Atteint 463, le meilleur du projet, en lisant le corpus pour repérer quels choix partagés sont des pièges et en orientant un balayage de 15 bassins autour d'eux. Forcer la recherche à éviter directement les pièges dégradait les plateaux — la valeur était dans où chercher, pas dans une règle dure.",
      prior: "Plafonne à 460 : le prior de position appris amène vite une construction de zéro dans la classe 460, mais le signal du corpus seul ne suffit pas à en sortir.",
      keyring: "Trois signaux appris qui votent (position, adjacence, motif 2×2) ont atteint 460 dans un arrangement de coins qu'aucun plateau n'avait percé — une nouvelle famille — mais le signal de motif est marginal et le polissage plafonne encore à 460.",
      replay: "Rejoue exactement les plateaux 460 stricts de la communauté et révèle le coup que la recherche ordinaire manque : 4–5 cases qui prennent deux défauts à la fois, hors de portée d'une recherche qui n'en autorise qu'un.",
      gauntlet: "Lancer le faisceau dans neuf directions de balayage a ouvert une toute nouvelle famille à 458 (l'ordre de balayage est un axe de diversité plus fort que la graine aléatoire) — mais un second tour a plafonné à 457 sans 461 : la nouvelle famille sature comme les autres.",
      cloister: "Comme solveur d'intérieur autonome, il confirme un vrai bonus de compatibilité au cadre, mais ce bonus ne peut être rajouté après coup — la même rigidité que le plateau complet — donc il se fixe dans les bas 450.",
      midden: "Choisir où (et non quand) le plateau peut casser prolonge la suite parfaite de 153 à 167–174 cellules, mais la géométrie dispersée échoue encore en fin de partie : rien n'absorbe le dernier défaut.",
      ladder: "Inonde de sondes bon marché et promeut les plus profondes, atteignant un plateau 451 strict sans record à copier — la première sortie de la bande universelle 444–450 — mais le stock d'ouvertures parfaites s'épuise et les barreaux convergent tous vers un seul plafond.",
      bandsaw: "Résout une bande de fin de partie à l'optimum prouvé, et mesure ce faisant le mur d'exactitude : l'arbre de recherche croît d'environ vingt fois par défaut autorisé supplémentaire, des deux côtés, donc se rejoindre au milieu cesse de payer à pleine taille.",
      staged: "Construit tout le plateau sans cadre préétabli et avec une bordure émergente, atteignant 436 — bien sous les records. Cet écart honnête est le résultat : il mesure exactement ce que vaut l'ancrage habituel par le cadre.",
      mosaic: "Compose des solutions exactes de blocs 4×4 avec jointures souples, atteignant 448 de zéro — mais le déficit tombe presque entièrement dans les trois derniers blocs du coin, où le stock de pièces s'amincit : le même vol de pièce, désormais un seul point vif.",
      lodestone: "Un prior de demande rare utilisé seulement comme départage relève la médiane de zéro de deux (449→451) et resserre la variance, mais tout poids plus grand l'effondre — la rareté est un signal réel mais faible, et il ne touche jamais au plafond de bassin.",
    } as Record<string, string>,
    families: {
      scratch: "de zéro",
      corpus: "du corpus",
      concentrate: "concentrer l'effort",
      anchor: "ancrer & contraindre",
      exact: "résoudre exactement",
      decode: "décoder & rejouer",
    } as Record<Method["family"], string>,
    closingTitle: "La forme de l'écart",
    closing: "Parcourez la table vers le bas et la leçon de tout le projet saute aux yeux : les méthodes qui font bouger le score changent la forme de la recherche — un ordre de balayage, un prior appris, une région confinée — jamais sa vitesse brute. Et chacune s'arrête à un mur global, pas local. Les onze bords de 469 à 480 ne sont pas un problème de polissage ; ils sont de l'autre côté des quatre murs à la fois.",
    sourceNote: "Chaque ligne de saturation est distillée du carnet de laboratoire du projet (un volume par invention). Les quatre murs sont corroborés par la littérature publiée : la transition de phase à 17 couleurs par Ansótegui, Béjar, Fernández et Mateu, « How Hard is a Commercial Puzzle: the Eternity II Challenge » ; le gradient manquant et les bassins profonds par la littérature de recherche locale sur Eternity II.",
  },
};

function ScoreBar({ score }: { score: number }) {
  // Visualise the score relative to the 469 ceiling and 480 target on a scale
  // that starts at 430 so the spread between methods is legible.
  const lo = 430;
  const pct = ((score - lo) / (TARGET - lo)) * 100;
  const ceilPct = ((CEILING - lo) / (TARGET - lo)) * 100;
  return (
    <div className="relative h-2 w-full overflow-hidden rounded-full bg-muted">
      <div className="h-full bg-primary/70" style={{ width: `${pct}%` }} />
      <div
        className="absolute top-0 h-full w-px bg-amber-500"
        style={{ left: `${ceilPct}%` }}
        title={`community ceiling ${CEILING}`}
      />
    </div>
  );
}

export default function WallsAndMethods() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div className="mx-auto max-w-3xl text-center">
        <LocalizedLink to="/research/why" className="text-sm text-muted-foreground hover:text-foreground">
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-3 text-4xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-4 text-xl text-muted-foreground">{t.lede}</p>
      </div>

      {/* The four walls */}
      <section className="mx-auto max-w-3xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.wallsTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.wallsIntro}</p>
        <div className="grid gap-3 sm:grid-cols-2">
          {WALL_KEYS.map((w) => (
            <LocalizedLink
              key={w}
              to={WALL_LINK[w]}
              className="group block rounded-lg border p-4 transition-shadow hover:shadow-md"
            >
              <div className="text-sm font-semibold tracking-tight group-hover:underline">{t.walls[w][0]}</div>
              <p className="mt-1 text-xs leading-relaxed text-muted-foreground">{t.walls[w][1]}</p>
            </LocalizedLink>
          ))}
        </div>
      </section>

      {/* The matrix */}
      <section className="space-y-4">
        <div className="mx-auto max-w-3xl">
          <h2 className="text-2xl font-semibold tracking-tight">{t.matrixTitle}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.matrixIntro}</p>
        </div>
        <div className="mx-auto max-w-3xl overflow-x-auto">
          <table className="w-full border-collapse text-sm">
            <thead>
              <tr className="border-b text-xs text-muted-foreground">
                <th className="py-2 pr-3 text-left font-medium">{t.colMethod}</th>
                {WALL_KEYS.map((w) => (
                  <th key={w} className="px-2 py-2 text-center font-medium">
                    <LocalizedLink to={WALL_LINK[w]} className="hover:text-foreground hover:underline">
                      {t.walls[w][0]}
                    </LocalizedLink>
                  </th>
                ))}
                <th className="hidden px-2 py-2 text-left font-medium sm:table-cell">{t.colScore}</th>
                <th className="px-2 py-2 text-left font-medium">{t.colNew}</th>
              </tr>
            </thead>
            <tbody>
              {METHODS.map((m) => (
                <tr key={m.key} className="border-b last:border-0">
                  <td className="py-2.5 pr-3">
                    <LocalizedLink to={m.to} className="font-medium tracking-tight hover:underline">
                      {m.key.toUpperCase()}
                    </LocalizedLink>
                    <span className="ml-2 text-xs text-muted-foreground">{m.score}/480</span>
                    <div className="text-[11px] text-muted-foreground">{t.families[m.family]}</div>
                  </td>
                  {WALL_KEYS.map((w) => (
                    <td key={w} className="px-2 text-center">
                      {m.walls.includes(w) ? (
                        <span
                          className="inline-block h-2.5 w-2.5 rounded-full bg-primary"
                          aria-label={`${m.key} attacks ${w}`}
                        />
                      ) : (
                        <span className="inline-block h-2.5 w-2.5 rounded-full bg-muted" aria-hidden />
                      )}
                    </td>
                  ))}
                  <td className="hidden w-32 px-2 sm:table-cell">
                    <ScoreBar score={m.score} />
                  </td>
                  <td className="px-2">
                    {m.newBasin ? (
                      <span className="rounded-full bg-emerald-500/15 px-2 py-0.5 text-[10px] font-medium tracking-wide text-emerald-600 uppercase dark:text-emerald-400">
                        {t.newYes}
                      </span>
                    ) : (
                      <span className="text-xs text-muted-foreground">{t.newNo}</span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <div className="mx-auto max-w-3xl space-y-2">
          <h3 className="text-base font-semibold tracking-tight">{t.readTitle}</h3>
          <p className="text-sm leading-relaxed text-muted-foreground">{t.read}</p>
        </div>
      </section>

      {/* Per-method saturation */}
      <section className="mx-auto max-w-3xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.saturationTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.saturationIntro}</p>
        <dl className="space-y-3">
          {METHODS.map((m) => (
            <div key={m.key} className="rounded-lg border p-4">
              <dt className="flex items-baseline justify-between gap-2">
                <LocalizedLink to={m.to} className="font-semibold tracking-tight hover:underline">
                  {m.key.toUpperCase()}
                </LocalizedLink>
                <span className="shrink-0 text-xs text-muted-foreground">
                  <span className="font-bold text-foreground tabular-nums">{m.score}</span> / 480
                </span>
              </dt>
              <dd className="mt-1 text-sm leading-relaxed text-muted-foreground">{t.saturation[m.key]}</dd>
            </div>
          ))}
        </dl>
      </section>

      {/* Closing */}
      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.closingTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.closing}</p>
      </section>

      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.sourceNote}</p>

      <RelatedRail path="/research/why/walls-and-methods" />
    </div>
  );
}

export const meta = pageMeta("walls-and-methods");
