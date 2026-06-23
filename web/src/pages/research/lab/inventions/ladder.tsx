import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { LadderDiagram } from "@/components/research/LadderDiagram";
import { LadderLiveLab } from "@/components/research/LadderLiveLab";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "LADDER",
    tagline: "Throw hundreds of cheap short searches at the board, keep only the deepest starts, and promote the survivors through longer and longer rounds.",
    idea: "Most of a long search is wasted on starts that were doomed early. LADDER spends almost nothing to find out which beginnings are worth pursuing. It runs a flood of very short probes, keeps the few that got deepest, and only then pays for longer runs, on those alone. It's tournament selection for search starts.",
    how: (
      <>
        <p>
          Round one is hundreds of five-second probes from different random seeds, each trying to lay
          down a long run of perfectly-matched cells. Keep the deepest prefixes, and throw out ones
          that are near-duplicates of each other so the survivors stay diverse.
        </p>
        <p>
          Promote those to a longer round with tighter quality gates, then promote the best of those
          to a full-length run. Each rung spends more time on fewer, better candidates, the way
          successive-halving tournaments allocate effort to the contenders that keep winning.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          LADDER produced a 451 board obeying all five official clues, with no guidance from any known
          record, the first time the project escaped the band of 444 to 450 that unguided search kept
          landing in. The finishes are determined by their prefix: once a strong enough opening is
          banked, the rest follows. A longer run later reached 452.
        </p>
        <p>
          It also mapped the limits: the supply of perfect openings runs out, and beyond a point the
          rungs all converge to the same ceiling. So LADDER is a good way to find the best start, not a
          way past the structural walls that stop every method near the top.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          What's the real ceiling of prefix-first selection given more compute on the early rungs?
          Could the diversity rule be smarter about which near-duplicates to keep? And does combining
          the deepest prefixes from different families beat promoting within one?
        </p>
      </>
    ),
  },
  fr: {
    name: "LADDER",
    tagline: "Lancer des centaines de recherches courtes et bon marché sur le plateau, ne garder que les départs les plus profonds, et promouvoir les survivants à travers des tours de plus en plus longs.",
    idea: "L'essentiel d'une longue recherche est gâché sur des départs condamnés tôt. LADDER ne dépense presque rien pour découvrir quels débuts valent la peine. Il lance une nuée de sondes très courtes, garde les quelques-unes allées le plus profond, et ne paie qu'ensuite des exécutions plus longues, sur celles-là seules. C'est une sélection par tournoi pour les départs de recherche.",
    how: (
      <>
        <p>
          Le premier tour, ce sont des centaines de sondes de cinq secondes depuis des graines
          aléatoires différentes, chacune tentant de poser une longue suite de cellules parfaitement
          accordées. On garde les préfixes les plus profonds, et on écarte les quasi-doublons pour que
          les survivants restent diversifiés.
        </p>
        <p>
          On les promeut vers un tour plus long avec des seuils de qualité plus stricts, puis on promeut
          les meilleurs vers une exécution complète. Chaque échelon consacre plus de temps à moins de
          candidats, meilleurs, à la manière des tournois à demi-élimination qui allouent l'effort aux
          concurrents qui continuent de gagner.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          LADDER a produit un plateau 451 respectant les cinq indices officiels, sans aucune aide d'un
          record connu, la première fois que le projet s'échappait de la bande 444-450 où la recherche
          non guidée retombait. Les fins sont déterminées par leur préfixe : une fois une ouverture
          assez forte mise en banque, le reste suit. Une exécution plus longue a ensuite atteint 452.
        </p>
        <p>
          Il a aussi cartographié les limites : la réserve d'ouvertures parfaites s'épuise, et au-delà
          d'un point les échelons convergent tous vers le même plafond. LADDER est donc un bon moyen de
          trouver le meilleur départ, pas un moyen de franchir les murs structurels qui arrêtent toute
          méthode près du sommet.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Quel est le vrai plafond de la sélection par préfixe avec plus de calcul sur les premiers
          échelons ? La règle de diversité pourrait-elle mieux choisir quels quasi-doublons garder ? Et
          combiner les préfixes les plus profonds de familles différentes bat-il la promotion au sein
          d'une seule ?
        </p>
      </>
    ),
  },
};

export default function Ladder() {
  return (
    <InventionLayout
      copy={copy}
      score={451}
      reproducibility="seeded"
      visual={
        <div className="space-y-6">
          <LadderDiagram />
          <LadderLiveLab />
        </div>
      }
    />
  );
}

export const meta = pageMeta("inv-ladder");
