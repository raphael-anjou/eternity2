import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { RelatedRail } from "@/components/research/RelatedRail";
import { Ns1Lab } from "@/components/research/Ns1Lab";

// "The border balance" — Brendan Owen / Hopfer's NS-1 deficit invariant, made
// live. Verified against vault concept `ns1-deficit` (the multiset-equality
// invariant, vol-10..12) and memory `project_e2_ns1_deficit_invariant`.

const T = {
  en: {
    backLabel: "← Why it's hard",
    title: "The border balance",
    lede: "A solved board hides a simple bookkeeping law: every colour the border hands to the interior, the interior hands straight back. Break it and you know instantly the board is wrong — but obeying it guarantees nothing.",
    p1: "Look only at the seam between the outer ring of border pieces and the first ring of interior pieces. Each edge across that seam shows one colour — counted once from the border side, once from the interior side. In any complete solution the two tallies are identical: the multiset of colours the border presents inward exactly equals the multiset the interior presents outward.",
    p2: "Call the imbalance the deficit, Δ — half the total mismatch between the two tallies. A finished, correct board has Δ = 0. The four known full solutions, across four different piece sets, all satisfy it exactly. So Δ > 0 is a certificate that a board can never be completed: a real, cheap necessary condition.",
    invariantTitle: "The law, in one line",
    invariantBody:
      "Across the border↔interior seam: { colours the border shows inward } = { colours the interior shows outward }. Equivalently, Δ = ½ ∑_c |A[c] − B[c]| = 0.",
    tryTitle: "See it on a real board",
    tryIntro:
      "A solved 8×8 starts in balance (Δ = 0). Lift a border piece and watch which colours fall out of balance, and the deficit climb. Then try the swap button — and watch the catch.",
    catchTitle: "The catch — and why it matters",
    catch:
      "Swapping two border pieces leaves Δ at 0. The swap moves colours around the seam without changing either tally, so the invariant is blind to it. Worse, on a near-solution most of the remaining errors aren't on the border seam at all — they're interior-to-interior, where NS-1 never looks: on 469-class boards over 85% of the unmatched edges are invisible to it. That is the whole texture of Eternity II in miniature. A check this clean still only ever says 'definitely broken', never 'definitely fine'. The puzzle resists every cheap certificate of progress.",
    useTitle: "Is it useful, then?",
    use: "Yes — as a late-search pruner. Once a solver has closed the border ring, enforcing Δ = 0 rejects 10–28% of deep dead-ends for the price of one pass over the 56 seam edges. It is necessary-but-loose: it throws away bad states cheaply, and leaves the hard part — the interior — untouched.",
    sourceLabel: "Origin: Hopfer's 2022 community post on the multiset-equality condition",
    sourceUrl: "https://groups.io/g/eternity2",
  },
  fr: {
    backLabel: "← Pourquoi c'est dur",
    title: "L'équilibre du bord",
    lede: "Un plateau résolu cache une loi comptable simple : chaque couleur que le bord tend à l'intérieur, l'intérieur la rend aussitôt. Brisez-la et vous savez sur-le-champ que le plateau est faux — mais la respecter ne garantit rien.",
    p1: "Ne regardez que la couture entre l'anneau extérieur de pièces de bord et le premier anneau de pièces intérieures. Chaque arête traversant cette couture montre une couleur — comptée une fois côté bord, une fois côté intérieur. Dans toute solution complète, les deux décomptes sont identiques : le multiset de couleurs que le bord présente vers l'intérieur égale exactement celui que l'intérieur présente vers l'extérieur.",
    p2: "Appelons déficit Δ le déséquilibre — la moitié de l'écart total entre les deux décomptes. Un plateau correct et terminé a Δ = 0. Les quatre solutions complètes connues, sur quatre jeux de pièces différents, le vérifient exactement. Donc Δ > 0 certifie qu'un plateau ne pourra jamais être complété : une vraie condition nécessaire, peu coûteuse.",
    invariantTitle: "La loi, en une ligne",
    invariantBody:
      "À travers la couture bord↔intérieur : { couleurs que le bord montre vers l'intérieur } = { couleurs que l'intérieur montre vers l'extérieur }. De façon équivalente, Δ = ½ ∑_c |A[c] − B[c]| = 0.",
    tryTitle: "Voyez-le sur un vrai plateau",
    tryIntro:
      "Un 8×8 résolu démarre équilibré (Δ = 0). Retirez une pièce de bord et observez quelles couleurs se déséquilibrent, et le déficit grimper. Essayez ensuite le bouton d'échange — et observez le piège.",
    catchTitle: "Le piège — et pourquoi il compte",
    catch:
      "Échanger deux pièces de bord laisse Δ à 0. L'échange déplace des couleurs le long de la couture sans changer aucun décompte, donc l'invariant ne le voit pas. Pire, sur une quasi-solution la plupart des erreurs restantes ne sont pas du tout sur la couture du bord — elles sont intérieur-à-intérieur, là où NS-1 ne regarde jamais : sur les plateaux de classe 469, plus de 85 % des arêtes non appariées lui sont invisibles. C'est toute la texture d'Eternity II en miniature. Un test si propre ne dit jamais que « certainement cassé », jamais « certainement bon ». Le puzzle résiste à tout certificat de progrès facile.",
    useTitle: "Est-ce utile, alors ?",
    use: "Oui — comme élagueur de fin de recherche. Une fois l'anneau de bord refermé, imposer Δ = 0 rejette 10 à 28 % des impasses profondes pour le prix d'un passage sur les 56 arêtes de couture. C'est nécessaire-mais-lâche : cela jette les mauvais états à bas coût, et laisse intacte la partie dure — l'intérieur.",
    sourceLabel: "Origine : le post communautaire de Hopfer (2022) sur la condition d'égalité de multisets",
    sourceUrl: "https://groups.io/g/eternity2",
  },
};

export default function BorderBalance() {
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
        <p className="text-foreground">{t.p1}</p>
        <p>{t.p2}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-2 rounded-lg border bg-muted/30 p-5">
        <h2 className="text-lg font-semibold tracking-tight">{t.invariantTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.invariantBody}</p>
      </section>

      <section className="space-y-4">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-2xl font-semibold tracking-tight">{t.tryTitle}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{t.tryIntro}</p>
        </div>
        <div className="mx-auto max-w-3xl">
          <Ns1Lab />
        </div>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.catchTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.catch}</p>
      </section>

      <section className="mx-auto max-w-2xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.useTitle}</h2>
        <p className="text-base leading-relaxed text-muted-foreground">{t.use}</p>
        <a
          href={t.sourceUrl}
          target="_blank"
          rel="noreferrer"
          className="inline-block text-sm font-medium underline hover:text-foreground"
        >
          {t.sourceLabel}
        </a>
      </section>

      <RelatedRail path="/research/why/border-balance" />
    </div>
  );
}

export const meta = pageMeta("border-balance");
