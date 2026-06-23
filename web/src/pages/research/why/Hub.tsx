import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Door 1 of the research section: the science of *why* Eternity II resists every
// approach. It carries the "engineered to resist cleverness" framing (the
// design story) and indexes the deeper structural results. The individual
// finding articles are published one by one through the research/topics pipeline;
// until each lands here it is shown as an upcoming card (no dead links).
type Topic = { key: string; ready: boolean; to?: string };

const TOPICS: Topic[] = [
  { key: "pruneVsSpeed", ready: true, to: "/research/why/prune-vs-speed" },
  { key: "wallsAndMethods", ready: true, to: "/research/why/walls-and-methods" },
  { key: "complexTheory", ready: true, to: "/research/why/complex-theory" },
  { key: "phaseTransition", ready: true, to: "/research/why/phase-transition" },
  { key: "rigidity", ready: true, to: "/research/why/rigidity-wall" },
  { key: "sigmaCycles", ready: true, to: "/research/why/sigma-cycles" },
  { key: "mismatchGeometry", ready: true, to: "/research/why/mismatch-geometry" },
  { key: "forbidden", ready: true, to: "/research/why/forbidden-patterns" },
  { key: "noForced", ready: true, to: "/research/why/no-forced-moves" },
  { key: "pieceTheft", ready: true, to: "/research/why/piece-theft" },
  { key: "rareColor", ready: true, to: "/research/why/rare-color-geography" },
  { key: "entropy", ready: true, to: "/research/why/entropy-area-law" },
];

const T = {
  en: {
    title: "Why it's hard",
    intro:
      "Eternity II is not accidentally difficult. This is the science of why no search, however clever, has reached the end — the puzzle's design, and the structural walls that show up once you start measuring.",
    backLabel: "← Research",
    engineeredTitle: "Engineered to resist cleverness",
    engineeredBody:
      "Eternity I fell in 2000 because Alex Selby and Oliver Riordan discovered the puzzle had vastly more solutions than its designer believed, and aimed their search at the most \"solution-dense\" regions. For Eternity II, the publisher hired the winners: Selby and Riordan helped design and stress-test the new puzzle so that no such statistical shortcut survives. The visible fingerprints of that vetting: a single designed solution baked into balanced color counts, no rotationally-symmetric pieces, no duplicate pieces, rare motifs confined to the frame, and piece-count/color-count parameters sitting at the empirical hardness peak (later confirmed by Ansótegui et al.). The puzzle isn't accidentally hard. It was tuned to be.",
    deeperTitle: "The structural walls",
    deeperIntro:
      "Beyond the design story, the puzzle has measurable structure that explains the gap between the best known board (469/480) and a full solution. Each of these is published with the exact computation behind it.",
    soon: "In preparation",
    topics: {
      pruneVsSpeed: {
        title: "Why a faster computer doesn't help",
        body: "The master idea: shrinking the search space beats searching it faster, exponentially. Every wall below is, underneath, a way the puzzle stops you shrinking it. Start here.",
      },
      wallsAndMethods: {
        title: "Which wall stops which method",
        body: "The bridge to the lab notebook: every invented algorithm lined up against the wall it actually attacks, and the exact score where that wall stopped it. The map of the whole project on one page.",
      },
      complexTheory: {
        title: "Complex theory — counting the search",
        body: "Brendan Owen's method to estimate the tree's width at every depth: ~14,702 solutions with one clue, ~1 with five, and the funnel where backtrackers burn 99% of their time. The community's key idea.",
      },
      phaseTransition: {
        title: "The phase-transition argument",
        body: "Why ≈17 interior colors and 5 border colors put the puzzle exactly on the SAT/CSP hardness peak — roughly one expected solution, the worst possible place to search.",
      },
      rigidity: {
        title: "The rigidity wall",
        body: "Every known top board is locally frozen: integer programming proves no small rearrangement improves it. You cannot nudge your way from a record board to a solution.",
      },
      sigmaCycles: {
        title: "Why basin-hopping is impossible",
        body: "Two great boards differ by one giant interlocking swap of up to 154 cells. Every smaller piece of it scores worse, so you can't step from one to the other.",
      },
      mismatchGeometry: {
        title: "Where the mismatches live",
        body: "A record board packs all its few errors into one band of five rows and leaves the rest flawless — and which band is decided by the direction the search filled the board. Seen live on the real boards.",
      },
      entropy: {
        title: "Entropy and the area law",
        body: "How the count of genuinely-distinct partial boards collapses past roughly eighty cells — a universal wall that no local scoring signal can see through.",
      },
      forbidden: {
        title: "Forbidden patterns",
        body: "Almost every small patch of colors you could draw is illegal under the piece set; a real solution contains none of the forbidden ones. A sharp combinatorial fingerprint of validity.",
      },
      noForced: {
        title: "No forced moves",
        body: "And yet no single piece is ever pinned: every interior piece has 73 to 137 possible neighbours. Lots of local freedom, almost no global consistency.",
      },
      pieceTheft: {
        title: "Piece theft, where solvers die",
        body: "A cell's needed colors can be served by only ~3 pieces, 47 by just one. Spend that one elsewhere and a future cell dies while the box still looks full.",
      },
      rareColor: {
        title: "The rare colors live on the frame",
        body: "Five of the 22 colors appear only on the border ring, each on exactly 24 edges, never inside. Design you can see, balanced on purpose.",
      },
    },
  },
  fr: {
    title: "Pourquoi c'est dur",
    intro:
      "Eternity II n'est pas difficile par hasard. Voici la science de cette difficulté : la conception du puzzle, et les murs structurels qui apparaissent dès qu'on se met à mesurer — et qui expliquent pourquoi aucune recherche, si ingénieuse soit-elle, n'est arrivée au bout.",
    backLabel: "← Recherche",
    engineeredTitle: "Conçu pour déjouer l'ingéniosité",
    engineeredBody:
      "Eternity I est tombé en 2000 parce qu'Alex Selby et Oliver Riordan ont compris que le puzzle admettait infiniment plus de solutions que ne le croyait son créateur, et qu'ils ont orienté leur recherche vers les régions les plus « riches en solutions ». Pour Eternity II, l'éditeur a recruté les gagnants : Selby et Riordan ont participé à la conception du nouveau puzzle et l'ont mis à l'épreuve pour qu'aucun raccourci statistique de ce type ne tienne. Les traces visibles de ce travail de blindage : une solution unique pensée dès le départ et noyée dans des répartitions de couleurs équilibrées, aucune pièce invariante par rotation, aucune pièce en double, des motifs rares confinés au cadre, et un nombre de pièces et de couleurs calé pile sur le pic de difficulté observé (ce qu'Ansótegui et al. confirmeront plus tard). Ce puzzle n'est pas difficile par hasard : il a été calibré pour l'être.",
    deeperTitle: "Les murs structurels",
    deeperIntro:
      "Au-delà de la conception, le puzzle présente une structure mesurable qui explique l'écart entre le meilleur plateau connu (469/480) et une solution complète. Chacun de ces résultats est publié avec le calcul exact qui le sous-tend.",
    soon: "En préparation",
    topics: {
      pruneVsSpeed: {
        title: "Pourquoi un ordinateur plus rapide n'aide pas",
        body: "L'idée maîtresse : réduire l'espace de recherche bat le fait de l'explorer plus vite, exponentiellement. Chaque mur ci-dessous est, au fond, une façon dont le puzzle vous empêche de le réduire. Commencez ici.",
      },
      wallsAndMethods: {
        title: "Quel mur arrête quelle méthode",
        body: "Le pont vers le carnet de laboratoire : chaque algorithme inventé aligné face au mur qu'il attaque réellement, et le score exact où ce mur l'a arrêté. La carte de tout le projet sur une page.",
      },
      complexTheory: {
        title: "Théorie complexe — compter la recherche",
        body: "La méthode de Brendan Owen pour estimer la largeur de l'arbre à chaque profondeur : ~14 702 solutions avec un indice, ~1 avec cinq, et l'entonnoir où les backtrackers brûlent 99 % de leur temps. L'idée clé de la communauté.",
      },
      phaseTransition: {
        title: "L'argument de la transition de phase",
        body: "Pourquoi ≈17 couleurs intérieures et 5 couleurs de bord placent le puzzle pile sur le pic de difficulté SAT/CSP — environ une solution attendue, le pire endroit où chercher.",
      },
      rigidity: {
        title: "Le mur de rigidité",
        body: "Tout plateau record connu est figé localement : la programmation en nombres entiers prouve qu'aucun petit réarrangement ne l'améliore. On ne peut pas, de proche en proche, atteindre une solution.",
      },
      sigmaCycles: {
        title: "Pourquoi sauter de bassin en bassin est impossible",
        body: "Deux bons plateaux diffèrent par un seul échange imbriqué géant, jusqu'à 154 cellules. Chaque morceau plus petit fait moins bien, donc on ne peut pas passer de l'un à l'autre par étapes.",
      },
      mismatchGeometry: {
        title: "Où vivent les défauts",
        body: "Un plateau record entasse ses rares erreurs dans une bande de cinq rangées et laisse le reste impeccable — et quelle bande dépend du sens dans lequel la recherche a rempli le plateau. À voir en direct sur les vrais plateaux.",
      },
      entropy: {
        title: "Entropie et loi d'aire",
        body: "Comment le nombre de plateaux partiels réellement distincts s'effondre au-delà d'environ quatre-vingts cellules — un mur universel qu'aucun signal de score local ne peut traverser.",
      },
      forbidden: {
        title: "Motifs interdits",
        body: "Presque tout petit carré de couleurs que l'on pourrait dessiner est illégal au regard du jeu de pièces ; une vraie solution n'en contient aucun. Une signature combinatoire nette de la validité.",
      },
      noForced: {
        title: "Aucun coup forcé",
        body: "Et pourtant aucune pièce n'est jamais coincée : chaque pièce intérieure a entre 73 et 137 voisines possibles. Beaucoup de liberté locale, presque aucune cohérence globale.",
      },
      pieceTheft: {
        title: "Le vol de pièce, là où les solveurs meurent",
        body: "Les couleurs requises d'une cellule ne peuvent être servies que par ~3 pièces, 47 par une seule. Dépensez-la ailleurs et une cellule à venir meurt alors que la boîte semble pleine.",
      },
      rareColor: {
        title: "Les couleurs rares vivent sur le cadre",
        body: "Cinq des 22 couleurs n'apparaissent que sur le cadre, chacune sur exactement 24 bords, jamais à l'intérieur. Une conception visible, équilibrée à dessein.",
      },
    },
  },
};

export default function Why() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div>
        <LocalizedLink
          to="/research"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.engineeredTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.engineeredBody}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.deeperTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.deeperIntro}</p>
        <div className="grid gap-3 md:grid-cols-2">
          {TOPICS.map((topic) => {
            const copy = t.topics[topic.key as keyof typeof t.topics];
            const card = (
              <Card className="h-full transition-shadow group-hover:shadow-md">
                <CardHeader className="pb-2">
                  <div className="flex items-start justify-between gap-2">
                    <CardTitle className="text-base group-hover:underline">{copy.title}</CardTitle>
                    {!topic.ready && (
                      <span className="shrink-0 rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
                        {t.soon}
                      </span>
                    )}
                  </div>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">{copy.body}</CardContent>
              </Card>
            );
            return topic.ready && topic.to ? (
              <LocalizedLink key={topic.key} to={topic.to} className="group block">
                {card}
              </LocalizedLink>
            ) : (
              <div key={topic.key} className="group block opacity-75">
                {card}
              </div>
            );
          })}
        </div>
      </section>
    </div>
  );
}

export const meta = pageMeta("why");
