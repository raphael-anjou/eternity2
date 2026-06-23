import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Plain statements about general approaches that don't crack E2, so people don't
// spend weeks re-discovering the same walls. "evidence" is the firmness of the
// claim: "proven" (theorem or exact computation) or "measured/known"
// (empirical or a structural fact of the set).
type Firmness = "proven" | "measured";
type DeadEnd = { key: string; firmness: Firmness; link?: string };

const DEAD_ENDS: DeadEnd[] = [
  { key: "symmetry", firmness: "proven", link: "/research/why/rare-color-geography" },
  { key: "moreCompute", firmness: "measured", link: "/research/why/prune-vs-speed" },
  { key: "surveyProp", firmness: "proven" },
  { key: "belief", firmness: "measured", link: "/research/why/no-forced-moves" },
  { key: "tensor", firmness: "proven", link: "/research/why/entropy-area-law" },
  { key: "lpBound", firmness: "measured" },
  { key: "learned", firmness: "measured", link: "/research/why/rigidity-wall" },
  { key: "enumerate", firmness: "measured", link: "/research/why/rigidity-wall" },
];

const T = {
  en: {
    backLabel: "← Build a solver",
    title: "Dead ends",
    lede: "Approaches we tried that look promising and don't move the needle on Eternity II. None of these are bad ideas in general; they just don't crack this puzzle. We're writing down what we found so you can spend your time elsewhere.",
    firmnessProven: "proven",
    firmnessMeasured: "measured",
    seeWhy: "Why this is a wall →",
    ends: {
      symmetry: {
        name: "Symmetry breaking",
        what: "Assume the board has rotational or mirror symmetries and fix some pieces to shrink the search.",
        why: "The official set was built with no rotationally-symmetric pieces and no duplicates, and the single center clue pins orientation. There's no global symmetry to break, so fixing corners just makes an arbitrary, not a free, choice.",
      },
      moreCompute: {
        name: "Just throw more compute at it",
        what: "Run the same SAT or search solver on a bigger machine, a GPU, an FPGA, or quantum hardware.",
        why: "The wall isn't clock speed, it's how well the problem is encoded and how the search space is shaped. GPU SAT solvers give a small constant speedup at best; quantum annealers show no advantage at the scale this would need. The same algorithm, faster, hits the same wall a little sooner.",
      },
      surveyProp: {
        name: "Survey propagation",
        what: "The message-passing method that cracked huge random SAT problems by finding solution clusters.",
        why: "It assumes the problem looks locally like a tree. Eternity II is a grid with a short cycle in every 2x2, which breaks that assumption. In practice the messages flatten out instead of sharpening, the opposite of what makes it work on random SAT. There are no published successes for it on grid puzzles.",
      },
      belief: {
        name: "Belief-propagation move ordering",
        what: "Run belief propagation over the colors and use its per-cell hints to decide which piece to try first.",
        why: "The hints come out almost uniform, so they barely rank the candidates. When we raced it head-to-head, picking the next move at random did as well or better, because a fixed informed order tends to repeat the same mistakes.",
      },
      tensor: {
        name: "Tensor-network counting",
        what: "Treat the board as a tensor network and contract it to count or score boundary-consistent colorings.",
        why: "It can only enforce that touching edges match, not that every piece is used exactly once. That blind spot is enormous: it counts on the order of 10^90 boundary-consistent colorings against the puzzle's roughly one real solution. Local message passing simply can't see the global one-piece-per-cell rule.",
      },
      lpBound: {
        name: "Relaxation bounds",
        what: "Solve the linear-programming relaxation to get a tight ceiling on how many edges a board can match.",
        why: "The relaxation lets pieces be fractional and spread across cells, which fakes matches that no real board can have. The result is a ceiling around 478 while the best real boards sit near 458, a gap too large to certify anything. The binding constraint is global piece-uniqueness, which the relaxation throws away.",
      },
      learned: {
        name: "Learn a heuristic on small boards",
        what: "Train a neural network on small puzzles, then transfer it to the full 16x16 to guide the search.",
        why: "We trained a model that nailed small boards and watched it collapse on the real one. It learns from candidate moves filtered one way and is then asked about moves filtered very differently, and the full puzzle's colors never appeared in training. The skill doesn't carry across the size and color gap.",
      },
      enumerate: {
        name: "Enumerate local clusters first",
        what: "List every valid 3x3 or 4x4 cluster, then stitch the clusters together into a full board.",
        why: "The counts blow up before they help. When we tried it, valid clusters around a single region already ran into the tens of millions, and combining four corners reaches the order of 10^12 piece-disjoint tuples. You run out of time and disk long before the constraints prune anything.",
      },
    },
    closingTitle: "The honest summary",
    closing: "The gap from the best known board to a full solution doesn't look like a missing optimization. The good boards are locally frozen and globally constrained in ways that local fixes, faster hardware, and standard relaxations don't touch. Reaching the end seems to need an idea of a different kind, not more of the same.",
    seeAlso: "See why a faster computer doesn't help",
  },
  fr: {
    backLabel: "← Construire un solveur",
    title: "Impasses",
    lede: "Des approches que nous avons essayées, prometteuses sur le papier, qui ne font pas avancer Eternity II. Aucune n'est mauvaise en général ; elles ne percent simplement pas ce puzzle. Nous notons ce que nous avons trouvé pour que vous passiez votre temps ailleurs.",
    firmnessProven: "prouvé",
    firmnessMeasured: "mesuré",
    seeWhy: "Pourquoi c'est un mur →",
    ends: {
      symmetry: {
        name: "Casser les symétries",
        what: "Supposer que le plateau a des symétries de rotation ou de miroir et fixer des pièces pour réduire la recherche.",
        why: "Le jeu officiel a été conçu sans aucune pièce invariante par rotation ni aucun doublon, et l'unique indice central fige l'orientation. Il n'y a aucune symétrie globale à casser : fixer des coins relève d'un choix arbitraire, pas d'un gain.",
      },
      moreCompute: {
        name: "Juste plus de puissance de calcul",
        what: "Lancer le même solveur SAT ou de recherche sur une plus grosse machine, un GPU, un FPGA ou du matériel quantique.",
        why: "Le mur n'est pas la vitesse d'horloge, mais la qualité de l'encodage et la forme de l'espace de recherche. Les solveurs SAT sur GPU n'offrent au mieux qu'un petit gain constant ; les recuits quantiques n'apportent aucun avantage à l'échelle requise. Le même algorithme, plus rapide, atteint le même mur un peu plus tôt.",
      },
      surveyProp: {
        name: "Survey propagation",
        what: "La méthode de passage de messages qui a percé d'énormes problèmes SAT aléatoires en trouvant des grappes de solutions.",
        why: "Elle suppose que le problème ressemble localement à un arbre. Eternity II est une grille avec un cycle court dans chaque 2x2, ce qui brise cette hypothèse. En pratique les messages s'aplatissent au lieu de se renforcer, l'inverse de ce qui marche sur le SAT aléatoire. Aucun succès publié sur des puzzles en grille.",
      },
      belief: {
        name: "Ordonnancement par belief propagation",
        what: "Faire tourner la belief propagation sur les couleurs et utiliser ses indices par case pour choisir quelle pièce essayer d'abord.",
        why: "Les indices ressortent presque uniformes et ne classent guère les candidats. Quand nous l'avons mis en compétition directe, choisir le coup suivant au hasard faisait aussi bien ou mieux, car un ordre fixe tend à répéter les mêmes erreurs.",
      },
      tensor: {
        name: "Comptage par réseaux de tenseurs",
        what: "Traiter le plateau comme un réseau de tenseurs et le contracter pour compter ou évaluer les coloriages cohérents au bord.",
        why: "Cela ne peut imposer que l'accord des bords qui se touchent, pas l'usage unique de chaque pièce. Cet angle mort est énorme : on compte de l'ordre de 10^90 coloriages cohérents au bord contre l'unique vraie solution du puzzle. Le passage de messages local ne voit tout simplement pas la règle globale une-pièce-par-case.",
      },
      lpBound: {
        name: "Bornes de relaxation",
        what: "Résoudre la relaxation linéaire pour obtenir un plafond serré du nombre de bords qu'un plateau peut apparier.",
        why: "La relaxation autorise des pièces fractionnaires réparties sur plusieurs cases, ce qui simule des accords qu'aucun vrai plateau ne peut avoir. Résultat : un plafond autour de 478 alors que les meilleurs vrais plateaux sont près de 458, un écart trop grand pour certifier quoi que ce soit. La contrainte qui compte, l'unicité des pièces, est justement jetée.",
      },
      learned: {
        name: "Apprendre une heuristique sur de petits plateaux",
        what: "Entraîner un réseau de neurones sur de petits puzzles, puis le transférer au 16x16 complet pour guider la recherche.",
        why: "Nous avons entraîné un modèle excellent sur les petits plateaux et l'avons vu s'effondrer sur le vrai. Il apprend sur des coups filtrés d'une façon, puis on l'interroge sur des coups filtrés tout autrement, et les couleurs du vrai puzzle n'ont jamais été vues à l'entraînement. La compétence ne franchit pas l'écart de taille et de couleurs.",
      },
      enumerate: {
        name: "Énumérer d'abord les grappes locales",
        what: "Lister toutes les grappes 3x3 ou 4x4 valides, puis les assembler en un plateau complet.",
        why: "Les comptages explosent avant d'aider. Quand nous l'avons tenté, les grappes valides autour d'une seule région se chiffraient déjà en dizaines de millions, et combiner quatre coins atteint l'ordre de 10^12 quadruplets à pièces distinctes. On manque de temps et de disque bien avant que les contraintes n'élaguent quoi que ce soit.",
      },
    },
    closingTitle: "Le résumé honnête",
    closing: "L'écart entre le meilleur plateau connu et une solution complète ne ressemble pas à une optimisation manquante. Les bons plateaux sont figés localement et contraints globalement d'une manière que les retouches locales, le matériel plus rapide et les relaxations standard n'atteignent pas. Arriver au bout semble exiger une idée d'un autre genre, pas davantage de la même.",
    seeAlso: "Voir pourquoi un ordinateur plus rapide n'aide pas",
  },
};

export default function DeadEnds() {
  const t = useT(T);
  return (
    <div className="space-y-10">
      <div>
        <LocalizedLink
          to="/research/build"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-3 max-w-3xl text-lg text-muted-foreground">{t.lede}</p>
      </div>

      <section className="space-y-4">
        {DEAD_ENDS.map((d) => {
          const copy = t.ends[d.key as keyof typeof t.ends];
          return (
            <article key={d.key} className="max-w-3xl rounded-lg border p-5">
              <div className="flex items-start justify-between gap-3">
                <h2 className="text-lg font-semibold tracking-tight">{copy.name}</h2>
                <span
                  className={
                    "shrink-0 rounded-full px-2 py-0.5 text-[10px] font-medium tracking-wide uppercase " +
                    (d.firmness === "proven"
                      ? "bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300"
                      : "bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300")
                  }
                >
                  {d.firmness === "proven" ? t.firmnessProven : t.firmnessMeasured}
                </span>
              </div>
              <p className="mt-2 text-sm font-medium text-foreground/80">{copy.what}</p>
              <p className="mt-1.5 text-sm leading-relaxed text-muted-foreground">{copy.why}</p>
              {d.link && (
                <LocalizedLink
                  to={d.link}
                  className="mt-2 inline-block text-xs font-medium underline hover:text-foreground"
                >
                  {t.seeWhy}
                </LocalizedLink>
              )}
            </article>
          );
        })}
      </section>

      <section className="max-w-3xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.closingTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.closing}</p>
        <LocalizedLink
          to="/research/why/prune-vs-speed"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.seeAlso}
        </LocalizedLink>
      </section>
    </div>
  );
}

export const meta = pageMeta("dead-ends");
