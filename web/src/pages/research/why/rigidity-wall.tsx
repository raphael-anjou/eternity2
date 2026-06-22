import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RigidityHalo } from "@/components/research/RigidityHalo";

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "The rigidity wall",
    lede: "Every record board we have is frozen in place. You cannot nudge your way from a great board to a perfect one, and we can prove it.",
    p1: "Here is the intuition almost everyone starts with: get close, then tidy up the last few mismatches. Swap a couple of pieces, rotate a few, and surely the score creeps up to 480.",
    p2: "It doesn't. On every top board we have tested, the last mismatches are locked. Move anything nearby and the score stays the same or drops. The good boards aren't almost-solutions waiting for a polish; they're isolated points with nowhere better to go next door.",
    howTitle: "How we know",
    how1: "We didn't just try a lot of swaps and give up. We asked an integer-programming solver an exact question: take a region of the board, free every piece inside it, and find the single best way to fill it back in. The solver searches that whole local space exactly, not by sampling.",
    how2: "The answer keeps coming back the same: the best arrangement is the one already there. We proved it for region after region, across three different record boards, and out to a radius of four cells, which is a block of dozens of cells at once. Every time, no improvement exists.",
    vizTitle: "Watch the region grow",
    impactTitle: "Why it matters",
    impact1: "This reframes the whole gap to 480. The distance from the best known board to a solution is not a pile of small fixes waiting to be found. If it were, this kind of local search would have found them. The barrier is that the good boards sit at the bottom of their own little valleys, and the valley walls are exact, not approximate.",
    impact2: "It also tells you what won't work. Polishing, hill-climbing, and most local-repair heuristics are trying to walk uphill from a frozen point. There is no uphill. Reaching a solution needs a move that rearranges a large region all at once, or a different starting point entirely, not a better polish.",
    relatedTitle: "The next piece of the picture",
    related: "If you can't improve a board locally, maybe you can hop from one good board to another. That fails too, for a related reason.",
    relatedLink: "See why basin-hopping is impossible",
    note: "The proofs use integer programming on regions of each board; they run for minutes per region on a solver, so they aren't reproduced live here. The boards themselves are bundled and checkable in the viewer.",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "Le mur de rigidité",
    lede: "Chaque plateau record que nous avons est figé. On ne peut pas, de proche en proche, passer d'un excellent plateau à un parfait, et nous pouvons le prouver.",
    p1: "Voici l'intuition par laquelle presque tout le monde commence : s'approcher, puis corriger les derniers défauts. Échanger deux pièces, en tourner quelques-unes, et sûrement le score grimpe jusqu'à 480.",
    p2: "Ce n'est pas le cas. Sur chaque plateau de tête que nous avons testé, les derniers défauts sont verrouillés. Bougez quoi que ce soit autour et le score reste identique ou baisse. Les bons plateaux ne sont pas des quasi-solutions attendant un polissage ; ce sont des points isolés sans meilleure case voisine.",
    howTitle: "Comment on le sait",
    how1: "Nous n'avons pas seulement essayé beaucoup d'échanges avant d'abandonner. Nous avons posé une question exacte à un solveur de programmation en nombres entiers : prendre une région du plateau, libérer chaque pièce à l'intérieur, et trouver la meilleure façon de la remplir. Le solveur explore tout cet espace local exactement, pas par échantillonnage.",
    how2: "La réponse revient toujours la même : le meilleur agencement est celui déjà en place. Nous l'avons prouvé région après région, sur trois plateaux records différents, et jusqu'à un rayon de quatre cellules, soit un bloc de dizaines de cellules d'un coup. À chaque fois, aucune amélioration n'existe.",
    vizTitle: "Regardez la région grandir",
    impactTitle: "Pourquoi c'est important",
    impact1: "Cela recadre tout l'écart vers 480. La distance entre le meilleur plateau connu et une solution n'est pas un tas de petites corrections à trouver. Si c'en était un, ce type de recherche locale les aurait trouvées. L'obstacle, c'est que les bons plateaux sont au fond de leurs propres petites vallées, et les parois de ces vallées sont exactes, pas approximatives.",
    impact2: "Cela indique aussi ce qui ne marchera pas. Le polissage, la montée de colline et la plupart des heuristiques de réparation locale essaient de monter depuis un point figé. Il n'y a pas de montée. Atteindre une solution exige un mouvement qui réarrange une grande région d'un seul coup, ou un tout autre point de départ, pas un meilleur polissage.",
    relatedTitle: "La pièce suivante du tableau",
    related: "Si l'on ne peut pas améliorer un plateau localement, peut-être peut-on sauter d'un bon plateau à un autre. Cela échoue aussi, pour une raison voisine.",
    relatedLink: "Voir pourquoi sauter de bassin en bassin est impossible",
    note: "Les preuves utilisent la programmation en nombres entiers sur des régions de chaque plateau ; elles tournent plusieurs minutes par région sur un solveur, donc ne sont pas reproduites en direct ici. Les plateaux eux-mêmes sont fournis et vérifiables dans le visualiseur.",
  },
};

export default function RigidityWall() {
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
        <RigidityHalo />
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.howTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.how1}</p>
        <p className="text-base leading-relaxed text-muted-foreground">{t.how2}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.impactTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.impact1}</p>
        <p className="text-base leading-relaxed text-muted-foreground">{t.impact2}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-3 rounded-lg border bg-muted/30 p-6 text-center">
        <h2 className="text-lg font-semibold tracking-tight">{t.relatedTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.related}</p>
        <LocalizedLink
          to="/research/why/sigma-cycles"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.relatedLink}
        </LocalizedLink>
      </section>

      <p className="mx-auto max-w-2xl text-center text-xs text-muted-foreground">{t.note}</p>
    </div>
  );
}

export const meta = pageMeta("rigidity-wall");
