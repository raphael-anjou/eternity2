import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { SigmaCycleDiagram } from "@/components/research/SigmaCycleDiagram";

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Why basin-hopping is impossible",
    lede: "If you can't improve a great board by polishing it, maybe you can jump to a different great board. You can't, and the reason is beautiful.",
    p1: "Two top boards look totally different, but they're related: you can turn one into the other by picking up a set of pieces and shifting each one into the spot the next was using, all the way around a loop. Mathematicians call that loop a cycle. Do the whole loop and you arrive at the other board.",
    p2: "Here's the catch. Between the best boards, that loop is enormous and there's only one of it. Going from a 458 board to the community's 469 means one single interlocking cycle of 154 cells, spanning the entire interior of the board.",
    vizTitle: "One loop, all or nothing",
    howTitle: "How we know",
    how1: "We computed the exact cycles between three different record boards and then tried every partial move: apply just some of the loop and see what the score does. The result is blunt. Every partial application makes the score worse, with drops ranging from a few points to over a hundred. There is no subset of the loop that helps, not one.",
    how2: "That's what makes it a wall. To move from one good board toward a better one you'd have to commit to shifting up to 154 cells at once, with no improving step along the way to guide you there. Every search that works in steps is blind to a move like that.",
    numbersTitle: "The cycles, measured",
    n1: "459 to 458: 14 cycles, largest 85 cells.",
    n2: "458 to McGavin 469: one giant cycle of 80 cells, spanning rows 1 to 14.",
    n3: "459 to McGavin 469: one giant cycle of 154 cells.",
    n4: "In every case, every smaller piece of the cycle scores worse. The property is universal across the basins we tested.",
    impactTitle: "Why it matters",
    impact: "Together with the rigidity wall, this closes off the two obvious escape routes at once. You can't climb out of a good board locally, and you can't hop to a neighbour either, because the nearest better board is one indivisible 150-cell move away. The 469 record has stood since 2020 for exactly this reason: the moves that would beat it are too large for any step-by-step search to find.",
    note: "Cycles are computed exactly from pairs of record boards by composing their piece permutations; the subset tests enumerate sub-cycles and rescore. The animation above is a schematic of the mechanism, not the 154-cell cycle itself.",
    backToWall: "← Back to the rigidity wall",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Pourquoi sauter de bassin en bassin est impossible",
    lede: "Si l'on ne peut pas améliorer un bon plateau en le polissant, peut-être peut-on sauter vers un autre bon plateau. C'est impossible, et la raison est élégante.",
    p1: "Deux plateaux de tête semblent totalement différents, mais ils sont liés : on peut transformer l'un en l'autre en prenant un ensemble de pièces et en décalant chacune vers la place qu'occupait la suivante, tout le long d'une boucle. Les mathématiciens appellent cette boucle un cycle. Faites toute la boucle et vous arrivez à l'autre plateau.",
    p2: "Voici le piège. Entre les meilleurs plateaux, cette boucle est énorme et il n'y en a qu'une. Passer d'un plateau 458 au 469 de la communauté demande un seul cycle imbriqué de 154 cellules, couvrant tout l'intérieur du plateau.",
    vizTitle: "Une boucle, tout ou rien",
    howTitle: "Comment on le sait",
    how1: "Nous avons calculé les cycles exacts entre trois plateaux records différents, puis essayé chaque mouvement partiel : appliquer seulement une partie de la boucle et observer le score. Le résultat est net. Chaque application partielle dégrade le score, avec des baisses allant de quelques points à plus de cent. Aucun sous-ensemble de la boucle n'aide, pas un seul.",
    how2: "C'est ce qui en fait un mur. Pour passer d'un bon plateau vers un meilleur, il faudrait s'engager à décaler jusqu'à 154 cellules d'un coup, sans aucune étape améliorante pour vous y guider. Toute recherche qui procède par étapes est aveugle à un tel mouvement.",
    numbersTitle: "Les cycles, mesurés",
    n1: "459 vers 458 : 14 cycles, le plus grand de 85 cellules.",
    n2: "458 vers McGavin 469 : un cycle géant de 80 cellules, couvrant les rangées 1 à 14.",
    n3: "459 vers McGavin 469 : un cycle géant de 154 cellules.",
    n4: "Dans tous les cas, chaque morceau plus petit du cycle fait moins bien. La propriété est universelle sur les bassins testés.",
    impactTitle: "Pourquoi c'est important",
    impact: "Avec le mur de rigidité, cela ferme d'un coup les deux échappatoires évidentes. On ne peut pas sortir localement d'un bon plateau, ni sauter vers un voisin, car le plateau meilleur le plus proche est à un seul mouvement indivisible de 150 cellules. Le record de 469 tient depuis 2020 exactement pour cette raison : les mouvements qui le battraient sont trop grands pour qu'une recherche pas à pas les trouve.",
    note: "Les cycles sont calculés exactement à partir de paires de plateaux records en composant leurs permutations de pièces ; les tests de sous-ensembles énumèrent les sous-cycles et recalculent le score. L'animation ci-dessus est un schéma du mécanisme, pas le cycle de 154 cellules lui-même.",
    backToWall: "← Retour au mur de rigidité",
  },
};

export default function SigmaCycles() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div className="mx-auto max-w-3xl text-center">
        <LocalizedLink
          to="/research/why"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-3 text-4xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-4 text-xl text-muted-foreground">{t.lede}</p>
      </div>

      <section className="mx-auto max-w-2xl space-y-4 text-base leading-relaxed text-muted-foreground">
        <p>{t.p1}</p>
        <p className="text-foreground">{t.p2}</p>
      </section>

      <section className="space-y-6">
        <h2 className="text-center text-2xl font-semibold tracking-tight">{t.vizTitle}</h2>
        <SigmaCycleDiagram />
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.howTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.how1}</p>
        <p className="text-base leading-relaxed text-muted-foreground">{t.how2}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border p-6">
        <h2 className="text-xl font-semibold tracking-tight">{t.numbersTitle}</h2>
        <ul className="space-y-1.5 text-sm text-muted-foreground">
          <li>{t.n1}</li>
          <li>{t.n2}</li>
          <li>{t.n3}</li>
          <li className="pt-1 font-medium text-foreground">{t.n4}</li>
        </ul>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.impactTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.impact}</p>
      </section>

      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.note}</p>

      <div className="mx-auto max-w-2xl text-center">
        <LocalizedLink
          to="/research/why/rigidity-wall"
          className="text-sm font-medium underline hover:text-foreground"
        >
          {t.backToWall}
        </LocalizedLink>
      </div>

      <RelatedRail path="/research/why/sigma-cycles" />
    </div>
  );
}

export const meta = pageMeta("sigma-cycles");
