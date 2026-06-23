import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { BasinArchipelagoDiagram } from "@/components/research/BasinArchipelagoDiagram";

// Why page: the record boards are an archipelago of isolated islands, not a
// connected landscape. Numbers from the vault basin analyses (vol-65
// basin-component-landscape: 47 components, McGavin singleton, min Hamming 247;
// vol-118 two-cluster; vol-111 skeleton: 3 invariant cells, 24 fully-variable in
// rows 13-15). Extends sigma-cycles (why you can't hop) with the global picture.

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "The archipelago of record boards",
    lede: "Picture every near-record board as a point, and join two of them whenever they are close — differing in fewer than a hundred of their 256 cells. You might expect a connected landscape you could wander across. Instead you get scattered islands, and the best boards sit entirely alone.",
    p1: "Take the 135 distinct boards the project has found at 455 or above and build that graph. It falls apart into 47 separate components. Most are singletons — one board, no near neighbours at all. A few form small families of two to six boards. The single largest family has 22 members, all stuck around 458.",
    p2: "The most important island is a single point. McGavin's 469, the best board anyone has, is a singleton: the nearest other strong board differs from it in 247 of 256 cells. It is not the peak of a hill you could climb — it is a speck of land with open ocean on every side.",
    clusterTitle: "Two kinds of distance",
    cluster: "Look closer at one small family and a sharp split appears. Within a family, boards differ by only 34 to 44 cells, and you can convert one into another by rotating a few small interlocking groups of pieces — the small cycles. Between families, boards differ by 251 to 253 cells, and the conversion is one single board-spanning swap of pieces that all depend on each other. That giant swap is exactly the move you cannot make in steps: this is the basin-hopping wall, seen from above.",
    skeletonTitle: "Where the islands actually differ",
    skeleton: "Line up several boards from different islands cell by cell and almost everything agrees. Only three cells are truly fixed across all of them — two corners and one border position, pinned by the frame. Most cells have just two possible occupants, a quiet swap-pair. The genuine freedom — the cells that take a different piece on every island — is just 24 cells, and all of them are in the bottom three rows. The boards are nearly identical bodies of work that disagree only about how to finish the last rows, the same place the mismatches pile up.",
    diagramTitle: "The map",
    meaningTitle: "Why it matters",
    meaning: "An archipelago is the worst possible shape for a local search. Every technique that improves a board by small moves can only ever explore the island it started on, and the islands top out below 469. Reaching a higher board isn't a matter of climbing — it means crossing open water to an island you can't see, with no trail of improving steps to follow. That is the same conclusion the rigidity wall and the sigma-cycles reach, here drawn as a map: the record boards are not a range of foothills below one summit, they are separate peaks with nothing between them.",
    sourceNote: "The diagram is schematic; the numbers are the measured ones — 47 components and McGavin's 247-cell isolation from the project's 455+ corpus, the 34–44 vs 251–253 intra- versus inter-family distances, and the 3 invariant / 24 fully-variable cells from the basin-skeleton analysis. All reproduce from the bundled boards.",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "L'archipel des plateaux records",
    lede: "Imaginez chaque plateau quasi record comme un point, et reliez-en deux dès qu'ils sont proches — différant par moins de cent de leurs 256 cellules. On s'attendrait à un paysage connexe que l'on pourrait parcourir. On obtient au contraire des îles éparses, et les meilleurs plateaux sont entièrement seuls.",
    p1: "Prenez les 135 plateaux distincts que le projet a trouvés à 455 ou plus et construisez ce graphe. Il se disloque en 47 composantes séparées. La plupart sont des singletons — un plateau, aucun voisin proche. Quelques-uns forment de petites familles de deux à six plateaux. La plus grande famille compte 22 membres, tous bloqués autour de 458.",
    p2: "L'île la plus importante est un point unique. Le 469 de McGavin, le meilleur plateau qui soit, est un singleton : le plateau fort le plus proche en diffère par 247 cellules sur 256. Ce n'est pas le sommet d'une colline que l'on pourrait gravir — c'est un îlot avec l'océan ouvert de tous les côtés.",
    clusterTitle: "Deux sortes de distance",
    cluster: "Regardez de plus près une petite famille et une coupure nette apparaît. Au sein d'une famille, les plateaux ne diffèrent que de 34 à 44 cellules, et l'on convertit l'un en l'autre en tournant quelques petits groupes de pièces imbriqués — les petits cycles. Entre familles, les plateaux diffèrent de 251 à 253 cellules, et la conversion est un unique échange à l'échelle du plateau, de pièces qui dépendent toutes les unes des autres. Ce gigantesque échange est précisément le coup qu'on ne peut pas faire par étapes : c'est le mur du saut de bassin, vu d'en haut.",
    skeletonTitle: "Où les îles diffèrent réellement",
    skeleton: "Alignez cellule par cellule plusieurs plateaux d'îles différentes et presque tout concorde. Seules trois cellules sont vraiment fixes pour toutes — deux coins et une position de bord, épinglés par le cadre. La plupart des cellules n'ont que deux occupants possibles, une discrète paire d'échange. La vraie liberté — les cellules qui prennent une pièce différente sur chaque île — n'est que de 24 cellules, toutes dans les trois rangées du bas. Les plateaux sont des œuvres presque identiques qui ne divergent que sur la façon de finir les dernières rangées, là même où s'entassent les défauts.",
    diagramTitle: "La carte",
    meaningTitle: "Pourquoi c'est important",
    meaning: "Un archipel est la pire forme possible pour une recherche locale. Toute technique qui améliore un plateau par petits coups ne peut explorer que l'île d'où elle est partie, et les îles plafonnent sous 469. Atteindre un plateau plus haut n'est pas une affaire d'escalade — il faut traverser l'eau libre vers une île qu'on ne voit pas, sans piste de pas améliorants à suivre. C'est la même conclusion que le mur de rigidité et les σ-cycles, dessinée ici en carte : les plateaux records ne sont pas une chaîne de contreforts sous un seul sommet, ce sont des pics séparés sans rien entre eux.",
    sourceNote: "Le schéma est illustratif ; les nombres sont mesurés — 47 composantes et l'isolement de 247 cellules de McGavin dans le corpus 455+ du projet, les distances intra- contre inter-familles de 34–44 contre 251–253, et les 3 cellules invariantes / 24 pleinement variables de l'analyse de squelette de bassin. Tout se reproduit depuis les plateaux fournis.",
  },
};

export default function BasinArchipelago() {
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

      <section className="mx-auto max-w-2xl space-y-4 text-base leading-relaxed text-muted-foreground">
        <p>{t.p1}</p>
        <p className="text-foreground">{t.p2}</p>
      </section>

      <section className="space-y-3">
        <h2 className="mx-auto max-w-2xl text-2xl font-semibold tracking-tight">{t.diagramTitle}</h2>
        <BasinArchipelagoDiagram />
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.clusterTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.cluster}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.skeletonTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.skeleton}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.meaningTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.meaning}</p>
      </section>

      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.sourceNote}</p>

      <RelatedRail path="/research/why/basin-archipelago" />
    </div>
  );
}

export const meta = pageMeta("basin-archipelago");
