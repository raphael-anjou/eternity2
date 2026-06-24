import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { ComplexFunnel } from "@/components/research/ComplexFunnel";

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Complex theory: counting the search before you run it",
    lede: "Brendan Owen's complex theory estimates how wide the search tree is at every depth — and how many solutions exist at all. Many in the community consider it the single most important thing to understand about Eternity II.",
    creditTitle: "Whose idea this is",
    credit:
      "Complex theory is due to Brendan Owen, one of the puzzle's vetters; Peter McGavin implemented it in C with arbitrary-precision arithmetic and posted the numbers. We're writing it up here because a community member (Dan Karlsson) rightly pointed out it was missing, and because it underpins almost every good decision you can make about a solver — especially the choice of search order.",
    ideaTitle: "The idea",
    p1: "Take a scan order and walk it one cell at a time. At each new cell, a random unused piece matches its already-placed neighbours with some probability — a product of per-edge colour-match chances. Multiply that by how many pieces are left and you get the expected number of ways to extend the board one cell further. Chain this down all 256 cells and you have a closed-form estimate of how wide the search tree is at every depth, and — at the last cell — how many full solutions the puzzle has.",
    p2: "It is an average, not a true count: it assumes the 22 edge colours are drawn independently, which they aren't (four edges are bolted to one rigid piece). But calibrated against small puzzles where the real count is known, it lands within about a factor of two. That is more than enough to see the shape.",
    numbersTitle: "The headline numbers",
    oneHint: "≈ 14,702",
    oneHintLabel: "expected solutions with one clue (the centre piece only)",
    fiveHint: "≈ 1",
    fiveHintLabel: "expected solutions with all five official clues",
    numbersNote:
      "With only the mandatory centre piece the puzzle has on the order of fifteen thousand solutions; add the four corner clues and the expected count drops to about 4×10⁻⁸ — overwhelmingly, exactly one. This is the formal reason the 5-clue puzzle has a single designed solution.",
    funnelTitle: "The funnel",
    funnelIntro:
      "Plot the expected width at each depth and three regimes appear — the shape the community calls the E2 funnel.",
    regimesTitle: "Three regimes",
    regimes: [
      ["Growth (depth 1–50)", "Solutions multiply geometrically from one to about 10²⁸. Every placement is essentially free; nothing constrains you yet."],
      ["Plateau (depth 50–200)", "The tree is at its widest — about 10⁴⁵ ways to extend — while the solution count barely moves. This is where backtrackers spend roughly 99% of their time, matching Joe's empirical finding that most time is spent below depth 150."],
      ["Collapse (depth 200–256)", "The width falls from 10⁴⁵ back to about 10⁴. The last ~60 pieces are tightly constrained: each one placed eliminates orders of magnitude of branches. The endgame is locally easy — the hard part is reaching it."],
    ] as [string, string][],
    soWhatTitle: "Why it changes how you search",
    soWhat:
      "If almost all the work is in the plateau, the goal isn't raw speed — it's getting across the plateau to the funnel entrance (around depth 200), after which the search chains down deterministically. And because complex theory scores a scan order before you run it, you can compare orders by the height of their plateau peak rather than by trial and error. That is the rigorous version of a rule this site states everywhere: the fill order is a first-class choice, and McGavin's bottom-left, left-to-right scan was picked because complex theory said it was good.",
    biggerTitle: "Bigger tiles",
    bigger:
      "The same idea works if you place 2×2 or 3×3 tiles instead of single pieces: a whole block of cells is committed at once, with its internal edges already matched. The search-path playground lets you do this for real — pick a block shape (1×1, 2×1, 2×2, 3×3, …) and stamp blocks onto the grid to build a block path. Race it and a dedicated macro-piece solver commits one whole valid sub-assembly per block instead of one piece at a time, so the search advances region by region. The plateau-peak estimate alongside still scores the cell order your blocks imply, predicting the cost before you run a single node.",
    caveatTitle: "What it can't see",
    caveat:
      "Complex theory is a first-moment estimate, so it is blind to one thing: whether the many counted partial boards are genuinely distinct. The entropy and area-law results show that distinctness collapses past ~80 cells — a second-order effect the independent-edge model cannot capture. So use complex theory to choose orders and read the tree's shape, never as a true count or a bound.",
    sourceLink: "Community thread: solution-count estimates (groups.io)",
    sourceUrl: "https://groups.io/g/eternity2/message/5209",
    relatedLink: "See why shrinking the search beats speeding it up",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "La théorie complexe : compter la recherche avant de la lancer",
    lede: "La théorie complexe de Brendan Owen estime la largeur de l'arbre de recherche à chaque profondeur — et combien de solutions existent en tout. Beaucoup dans la communauté la considèrent comme la chose la plus importante à comprendre sur Eternity II.",
    creditTitle: "À qui revient l'idée",
    credit:
      "La théorie complexe est due à Brendan Owen, l'un des vérificateurs du puzzle ; Peter McGavin l'a implémentée en C avec une arithmétique à précision arbitraire et en a publié les chiffres. Nous la rédigeons ici parce qu'un membre de la communauté (Dan Karlsson) a justement signalé son absence, et parce qu'elle sous-tend presque toute bonne décision sur un solveur — surtout le choix de l'ordre de parcours.",
    ideaTitle: "L'idée",
    p1: "Prenez un ordre de parcours et suivez-le case par case. À chaque nouvelle case, une pièce inutilisée au hasard s'accorde à ses voisines déjà posées avec une certaine probabilité — un produit de chances d'accord par bord. Multipliez par le nombre de pièces restantes et vous obtenez le nombre attendu de façons d'étendre le plateau d'une case. Enchaînez sur les 256 cases et vous avez une estimation en forme close de la largeur de l'arbre à chaque profondeur, et — à la dernière case — du nombre de solutions complètes.",
    p2: "C'est une moyenne, pas un compte exact : elle suppose que les 22 couleurs de bord sont tirées indépendamment, ce qui est faux (quatre bords sont fixés à une même pièce rigide). Mais calibrée sur de petits puzzles au compte connu, elle tombe à un facteur deux près. C'est largement assez pour voir la forme.",
    numbersTitle: "Les chiffres clés",
    oneHint: "≈ 14 702",
    oneHintLabel: "solutions attendues avec un seul indice (la pièce centrale)",
    fiveHint: "≈ 1",
    fiveHintLabel: "solutions attendues avec les cinq indices officiels",
    numbersNote:
      "Avec la seule pièce centrale obligatoire, le puzzle a de l'ordre de quinze mille solutions ; ajoutez les quatre indices de coin et le compte attendu chute à environ 4×10⁻⁸ — très majoritairement, exactement une. C'est la raison formelle pour laquelle le puzzle à 5 indices a une unique solution conçue.",
    funnelTitle: "L'entonnoir",
    funnelIntro:
      "Tracez la largeur attendue à chaque profondeur et trois régimes apparaissent — la forme que la communauté appelle l'entonnoir d'E2.",
    regimesTitle: "Trois régimes",
    regimes: [
      ["Croissance (profondeur 1–50)", "Les solutions se multiplient géométriquement de une à environ 10²⁸. Chaque pose est essentiellement libre ; rien ne vous contraint encore."],
      ["Plateau (profondeur 50–200)", "L'arbre est au plus large — environ 10⁴⁵ façons d'étendre — tandis que le nombre de solutions bouge à peine. C'est là que les backtrackers passent environ 99 % de leur temps, en accord avec le constat empirique de Joe sur le temps passé au-delà de la profondeur 150."],
      ["Effondrement (profondeur 200–256)", "La largeur retombe de 10⁴⁵ à environ 10⁴. Les ~60 dernières pièces sont très contraintes : chacune posée élimine des ordres de grandeur de branches. La fin de partie est localement facile — le dur est d'y arriver."],
    ] as [string, string][],
    soWhatTitle: "Pourquoi cela change la façon de chercher",
    soWhat:
      "Si presque tout le travail est dans le plateau, l'objectif n'est pas la vitesse brute — c'est de traverser le plateau jusqu'à l'entrée de l'entonnoir (vers la profondeur 200), après quoi la recherche s'enchaîne de façon déterministe. Et comme la théorie complexe note un ordre de parcours avant de l'exécuter, on peut comparer les ordres par la hauteur de leur pic de plateau plutôt qu'à tâtons. C'est la version rigoureuse d'une règle que ce site répète partout : l'ordre de remplissage est un choix de premier plan, et le balayage bas-gauche, gauche-à-droite de McGavin a été choisi parce que la théorie complexe le disait bon.",
    biggerTitle: "Des tuiles plus grandes",
    bigger:
      "La même idée tient si l'on pose des tuiles 2×2 ou 3×3 au lieu de pièces seules : tout un bloc de cases est validé d'un coup, ses arêtes internes déjà accordées. Le terrain de jeu des ordres de parcours permet de le faire pour de vrai : choisissez une forme de bloc (1×1, 2×1, 2×2, 3×3, …) et tamponnez des blocs sur la grille pour construire un parcours par blocs. Lancez la course et un solveur de macro-pièces dédié valide un sous-assemblage valide complet par bloc au lieu d'une pièce à la fois, si bien que la recherche progresse région par région. L'estimation du pic de plateau à côté évalue toujours l'ordre des cases impliqué par vos blocs, prédisant le coût avant de lancer le moindre nœud.",
    caveatTitle: "Ce qu'elle ne voit pas",
    caveat:
      "La théorie complexe est une estimation de premier moment, donc aveugle à une chose : si les nombreux plateaux partiels comptés sont vraiment distincts. Les résultats d'entropie et de loi d'aire montrent que la distinctivité s'effondre au-delà de ~80 cases — un effet de second ordre que le modèle à bords indépendants ne capte pas. Utilisez-la donc pour choisir les ordres et lire la forme de l'arbre, jamais comme un compte exact ni une borne.",
    sourceLink: "Fil communautaire : estimations du nombre de solutions (groups.io)",
    sourceUrl: "https://groups.io/g/eternity2/message/5209",
    relatedLink: "Voir pourquoi réduire la recherche bat l'accélérer",
  },
};

export default function ComplexTheory() {
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

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.creditTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.credit}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.ideaTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.p1}</p>
        <p className="text-base leading-relaxed text-muted-foreground">{t.p2}</p>
      </section>

      <section className="mx-auto grid max-w-2xl gap-4 sm:grid-cols-2">
        <div className="rounded-lg border p-5 text-center">
          <div className="text-3xl font-bold tabular-nums">{t.oneHint}</div>
          <div className="mt-1 text-xs text-muted-foreground">{t.oneHintLabel}</div>
        </div>
        <div className="rounded-lg border p-5 text-center">
          <div className="text-3xl font-bold tabular-nums">{t.fiveHint}</div>
          <div className="mt-1 text-xs text-muted-foreground">{t.fiveHintLabel}</div>
        </div>
        <p className="text-sm leading-relaxed text-muted-foreground sm:col-span-2">{t.numbersNote}</p>
      </section>

      <section className="space-y-4">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-2xl font-semibold tracking-tight">{t.funnelTitle}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.funnelIntro}</p>
        </div>
        <ComplexFunnel />
      </section>

      <section className="mx-auto max-w-2xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.regimesTitle}</h2>
        <div className="space-y-2">
          {t.regimes.map(([name, body]) => (
            <div key={name} className="rounded-lg border p-4">
              <div className="text-sm font-semibold">{name}</div>
              <p className="mt-1 text-sm leading-relaxed text-muted-foreground">{body}</p>
            </div>
          ))}
        </div>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.soWhatTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.soWhat}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.biggerTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.bigger}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.caveatTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.caveat}</p>
      </section>

      <div className="mx-auto max-w-2xl space-y-3 text-center">
        <a
          href={t.sourceUrl}
          target="_blank"
          rel="noreferrer"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.sourceLink}
        </a>
      </div>

      <RelatedRail path="/research/why/complex-theory" />
    </div>
  );
}

export const meta = pageMeta("complex-theory");
