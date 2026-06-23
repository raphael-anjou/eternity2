import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";
import { PalimpsestDiagram } from "@/components/research/PalimpsestDiagram";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "PALIMPSEST",
    tagline: "Read every strong board to find the habits that quietly hold a board back, then break them. This produced the project's best board: 463 of 480.",
    idea: "When many independent searches all reach a high but not perfect board, they tend to agree on a lot of placements. Some of that agreement is genuine structure, and some of it is a shared bad habit: a local choice that looks good and keeps every search stuck just short of the top. The idea is to read the whole corpus of strong boards, separate the helpful agreement from the trap, and attack the traps.",
    how: (
      <>
        <p>
          Take every board anyone has found that scores reasonably well, and for each pair of
          neighbouring positions count how often a given pair of pieces sits there, weighted by how
          good the board was. Two patterns fall out. Some adjacencies show up again and again in the
          very best boards: those are safe, real structure. Others show up almost everywhere but
          never in the top boards: those are the traps, the choices that feel right and cap the score.
        </p>
        <p>
          The traps cluster in particular regions of the board rather than spreading evenly. Knowing
          where they are turns the corpus into a map: which placements to trust, and which to pull
          apart and rebuild. Grouping boards by how their corners are arranged then points the search
          at the most promising family to attack.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          Used as a map to steer the search, this reached 463 of 480 matched edges, the best board
          this project has produced. For comparison, the community's best on this puzzle is 469, and
          a complete solution is 480.
        </p>
        <p>
          One honest caveat: trying to use the trap list directly, by forcing the search to avoid
          trapped placements, did not work on its own and tended to make boards worse. The value was
          in reading the corpus to choose where to focus, not in hard-coding its conclusions into the
          search.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Why does rebuilding the trapped regions tend to land back on the same known top board
          rather than a genuinely new one? Would a separate map per corner family reveal structure
          that the combined map hides? And could a gentle penalty for trapped placements help where a
          hard ban hurt?
        </p>
      </>
    ),
  },
  fr: {
    name: "PALIMPSEST",
    tagline: "Lire tous les bons plateaux pour repérer les habitudes qui freinent discrètement un plateau, puis les casser. A produit le meilleur plateau du projet : 463 sur 480.",
    idea: "Quand de nombreuses recherches indépendantes atteignent toutes un plateau élevé mais imparfait, elles s'accordent souvent sur beaucoup de placements. Une partie de cet accord est une vraie structure, l'autre une mauvaise habitude partagée : un choix local qui paraît bon et maintient chaque recherche bloquée juste sous le sommet. L'idée est de lire tout le corpus des bons plateaux, de séparer l'accord utile du piège, et d'attaquer les pièges.",
    how: (
      <>
        <p>
          Prenez chaque plateau trouvé qui obtient un score correct, et pour chaque paire de
          positions voisines comptez combien de fois une paire de pièces donnée s'y trouve, pondérée
          par la qualité du plateau. Deux motifs ressortent. Certaines adjacences reviennent sans
          cesse dans les tout meilleurs plateaux : ce sont une structure sûre et réelle. D'autres
          apparaissent presque partout mais jamais dans les meilleurs : ce sont les pièges, les choix
          qui semblent justes et plafonnent le score.
        </p>
        <p>
          Les pièges se regroupent dans certaines régions du plateau plutôt que de se répartir
          uniformément. Savoir où ils sont transforme le corpus en carte : quels placements croire,
          et lesquels défaire et reconstruire. Regrouper les plateaux selon la disposition de leurs
          coins oriente ensuite la recherche vers la famille la plus prometteuse à attaquer.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          Utilisé comme carte pour orienter la recherche, cela a atteint 463 bords appariés sur 480,
          le meilleur plateau de ce projet. À titre de comparaison, le meilleur de la communauté sur
          ce puzzle est 469, et une solution complète vaut 480.
        </p>
        <p>
          Une réserve honnête : essayer d'utiliser directement la liste des pièges, en forçant la
          recherche à éviter les placements piégés, n'a pas marché en soi et tendait à dégrader les
          plateaux. La valeur était dans la lecture du corpus pour choisir où se concentrer, pas dans
          le codage en dur de ses conclusions dans la recherche.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Pourquoi reconstruire les régions piégées tend-il à retomber sur le même plateau sommet
          connu plutôt que sur un vraiment nouveau ? Une carte distincte par famille de coins
          révélerait-elle une structure que la carte combinée masque ? Et une pénalité douce pour les
          placements piégés aiderait-elle là où une interdiction stricte a nui ?
        </p>
      </>
    ),
  },
};

export default function Palimpsest() {
  return (
    <InventionLayout
      copy={copy}
      score={463}
      boardId="v129-palimpsest-463"
      reproducibility="seeded"
      visual={<PalimpsestDiagram />}
    />
  );
}

export const meta = pageMeta("exp-palimpsest");
