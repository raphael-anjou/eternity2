import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { MismatchGeometryLab } from "@/components/research/MismatchGeometryLab";

// Why page: where the mismatches live. Every strong board's handful of errors
// is not scattered — it is packed into one band of five rows, and which band is
// decided by the direction the solver filled the board. Verified live from the
// real record boards in the lab below; the underlying finding is in the vault
// (mcgavin-469-mismatch-geometry, local459-mismatch-geometry, vol-14 scan-order).

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "Where the mismatches live",
    lede: "A near-perfect board doesn't scatter its few errors evenly. It packs them into one band of five rows and leaves all the rest flawless. Which band is decided by the direction the search filled the board — and you can see the mirror on the real record boards.",
    p1: "Score a record board edge by edge and a striking thing shows up: the mismatches are not spread around. McGavin's 469 has all eleven of its unmatched edges in the top five rows; rows 5 through 15 are locally perfect. The board is, in effect, a flawless 11-row slab with all the damage swept up against one edge.",
    p2: "Now score this project's best from-scratch boards the same way. The damage is again in a five-row band — but at the bottom. KEYRING's and GAUNTLET's mismatches sit in rows 11 to 15, with rows 0 through 10 perfect. It is the same picture flipped top to bottom.",
    mirrorTitle: "Why it flips: scan order",
    mirror: "The flip is not a coincidence; it is the fingerprint of how each board was built. A search that fills the board from the bottom up spends its perfect placements early, low down, and is forced to absorb every accumulated conflict in the last rows it reaches — the top. A search that fills top-down does the exact opposite and piles the damage at the bottom. The mismatches always end up crammed against the edge the solver finished at. Same puzzle, same kind of board, opposite construction order.",
    labTitle: "See it on the real boards",
    labIntro: "Pick a board. The shaded band is where its mismatches actually fall, computed live with the engine's own scoring rule — community boards at the top, this project's at the bottom.",
    meaningTitle: "What it tells us",
    meaning: "This is the visible form of two deeper facts. First, the great boards really are almost-complete: the gap to 480 is concentrated, not diffuse, which is why integer programming finds them locally frozen everywhere except that one band (the rigidity wall). Second, it says the endgame is the whole game — whichever rows you fill last are where the puzzle makes you pay, so the order you search in decides where the difficulty lands. That is the same lesson the playground's path-racing makes you feel, here written into the structure of every record board.",
    sourceNote: "The per-row counts and the highlighted band are computed in your browser from the real board edges (the same scoring rule the engine uses), not hand-placed. The underlying finding and the scan-order explanation are recorded in the project lab notebook.",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Où vivent les défauts",
    lede: "Un plateau quasi parfait ne disperse pas ses rares erreurs uniformément. Il les entasse dans une bande de cinq rangées et laisse tout le reste impeccable. Quelle bande ? Cela dépend du sens dans lequel la recherche a rempli le plateau — et le miroir se voit sur les vrais plateaux records.",
    p1: "Notez un plateau record bord par bord et une chose frappante apparaît : les défauts ne sont pas répartis. Le 469 de McGavin a ses onze bords non appariés dans les cinq rangées du haut ; les rangées 5 à 15 sont localement parfaites. Le plateau est, en somme, une dalle impeccable de 11 rangées avec tous les dégâts repoussés contre un bord.",
    p2: "Notez maintenant de la même façon les meilleurs plateaux construits de zéro de ce projet. Les dégâts sont de nouveau dans une bande de cinq rangées — mais en bas. Les défauts de KEYRING et GAUNTLET sont dans les rangées 11 à 15, les rangées 0 à 10 parfaites. C'est la même image retournée de haut en bas.",
    mirrorTitle: "Pourquoi ça bascule : l'ordre de balayage",
    mirror: "Le basculement n'est pas un hasard ; c'est l'empreinte de la façon dont chaque plateau a été bâti. Une recherche qui remplit le plateau de bas en haut dépense tôt ses placements parfaits, en bas, et doit absorber tous les conflits accumulés dans les dernières rangées qu'elle atteint — le haut. Une recherche qui remplit de haut en bas fait exactement l'inverse et entasse les dégâts en bas. Les défauts finissent toujours collés contre le bord où le solveur a terminé. Même puzzle, même genre de plateau, ordre de construction opposé.",
    labTitle: "Voyez-le sur les vrais plateaux",
    labIntro: "Choisissez un plateau. La bande ombrée est l'endroit où tombent réellement ses défauts, calculée en direct avec la règle de score du moteur — plateaux de la communauté en haut, ceux de ce projet en bas.",
    meaningTitle: "Ce que cela nous dit",
    meaning: "C'est la forme visible de deux faits plus profonds. D'abord, les grands plateaux sont vraiment presque complets : l'écart vers 480 est concentré, pas diffus, et c'est pourquoi la programmation en nombres entiers les trouve figés localement partout sauf dans cette bande (le mur de rigidité). Ensuite, cela dit que la fin de partie est toute la partie : les rangées que vous remplissez en dernier sont là où le puzzle vous fait payer, donc l'ordre de recherche décide où tombe la difficulté. C'est la leçon même que la course de parcours de l'aire de jeu fait ressentir, ici inscrite dans la structure de chaque plateau record.",
    sourceNote: "Les comptages par rangée et la bande surlignée sont calculés dans votre navigateur à partir des vrais bords du plateau (la règle de score utilisée par le moteur), et non placés à la main. Le résultat sous-jacent et l'explication par l'ordre de balayage sont consignés dans le carnet de laboratoire du projet.",
  },
};

export default function MismatchGeometry() {
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
        <p>{t.p2}</p>
      </section>

      <section className="space-y-4">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-2xl font-semibold tracking-tight">{t.labTitle}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.labIntro}</p>
        </div>
        <div className="mx-auto max-w-3xl">
          <MismatchGeometryLab />
        </div>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.mirrorTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.mirror}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.meaningTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.meaning}</p>
      </section>

      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.sourceNote}</p>

      <RelatedRail path="/research/why/mismatch-geometry" />
    </div>
  );
}

export const meta = pageMeta("mismatch-geometry");
