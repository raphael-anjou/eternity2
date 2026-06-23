import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";

// LODESTONE (vol-208): a scarce-demand prior for the from-scratch beam. High
// boards activate a soft, score-correlated set of scarce (north,west) demands;
// LODESTONE turns that into a per-piece tiebreaker so the beam commits the right
// scarce pieces early. At a tiny weight it gives a consistent median +2 and
// tightens the variance; at any larger weight it hurts. A real but modest
// construction-quality lever, not a record-mover. Figures from the vault concept
// lodestone-demand-prior.

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "LODESTONE",
    tagline: "A faint compass for a from-scratch search: nudge it to commit the rare pieces early, where they're needed. It doesn't raise the ceiling — it makes the search reliably reach the top of its own range.",
    idea: "The strong boards quietly agree on something: as the score rises, they increasingly satisfy a particular set of scarce demands — places where only one or two pieces in the whole set can serve a cell's north-and-west colours. LODESTONE asks whether telling a from-scratch search about those demands helps it commit the right rare pieces before they're stolen.",
    how: (
      <>
        <p>
          From the corpus of strong boards, LODESTONE builds a prior: for each piece, how often it
          ends up serving one of those scarce north-west demands in a good board, weighted up sharply
          for the rarest (a piece that is the only possible server gets the biggest boost). The beam
          search then ranks its candidate placements by the usual matched-edge score plus a small
          multiple of this prior — so among otherwise-equal moves it prefers to place the pieces the
          good boards learned to spend early.
        </p>
        <p>
          The crucial detail is the size of that multiple. The prior has to be a pure tiebreaker, not
          part of the objective: it breaks ties between equally-matching moves and nothing more.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          At a tiny weight the prior gives a small, consistent gain: across five seeds the median
          from-scratch score rises from 449 to 451, and — more usefully — the spread tightens, from a
          446–451 scatter to a reliable 450–451. It makes the constructor land at the top of its range
          instead of sometimes stumbling.
        </p>
        <p>
          Turn the weight up even slightly and it collapses — 422, then 380 — because chasing the
          corpus's scarce demands then trades directly against matching the edge in front of you. That
          failure is itself the finding: scarcity is a real signal for <em>which</em> piece to prefer,
          but a weak one, and only safe as a tiebreaker. LODESTONE is honest about its size: it
          improves construction quality and consistency by a couple of edges, and does not touch the
          basin ceiling that stops every method near the top.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Could the prior be made position-aware without becoming part of the objective — strong only
          in the regions where scarce demands actually concentrate? Does combining it with KEYRING's
          three signals add a fourth useful vote, or just more noise? And is the consistency gain worth
          more than the median gain — a tighter search is easier to build a ladder on?
        </p>
      </>
    ),
  },
  fr: {
    name: "LODESTONE",
    tagline: "Une faible boussole pour une recherche partie de zéro : l'inciter à engager tôt les pièces rares, là où on en a besoin. Elle ne relève pas le plafond — elle fait que la recherche atteint de façon fiable le haut de sa propre fourchette.",
    idea: "Les bons plateaux s'accordent discrètement sur une chose : à mesure que le score monte, ils satisfont de plus en plus un certain ensemble de demandes rares — des endroits où une ou deux pièces seulement dans tout le jeu peuvent servir les couleurs nord-et-ouest d'une case. LODESTONE se demande si renseigner une recherche partie de zéro sur ces demandes l'aide à engager les bonnes pièces rares avant qu'elles soient volées.",
    how: (
      <>
        <p>
          À partir du corpus des bons plateaux, LODESTONE construit un prior : pour chaque pièce,
          combien de fois elle finit par servir l'une de ces demandes nord-ouest rares dans un bon
          plateau, fortement pondéré pour les plus rares (une pièce qui est le seul serveur possible
          reçoit le plus grand bonus). La recherche en faisceau classe alors ses placements candidats
          par le score habituel de bords appariés plus un petit multiple de ce prior — de sorte qu'à
          coups par ailleurs égaux, elle préfère poser les pièces que les bons plateaux ont appris à
          dépenser tôt.
        </p>
        <p>
          Le détail crucial est la taille de ce multiple. Le prior doit être un pur départage, pas une
          partie de l'objectif : il départage des coups qui apparient autant, et rien de plus.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          À un poids minuscule, le prior donne un gain petit et constant : sur cinq graines, le score
          médian de zéro passe de 449 à 451 et — plus utile — la dispersion se resserre, d'un éparpillement
          446–451 à un fiable 450–451. Il fait atterrir le constructeur en haut de sa fourchette au lieu
          de trébucher parfois.
        </p>
        <p>
          Montez le poids ne serait-ce qu'un peu et tout s'effondre — 422, puis 380 — car courir après
          les demandes rares du corpus se paie alors directement contre l'appariement du bord devant
          soi. Cet échec est lui-même le résultat : la rareté est un vrai signal pour savoir <em>quelle</em>{" "}
          pièce préférer, mais un signal faible, sûr seulement comme départage. LODESTONE est honnête
          sur sa taille : il améliore la qualité et la régularité de construction de quelques bords, et
          ne touche pas au plafond de bassin qui arrête chaque méthode près du sommet.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Le prior pourrait-il devenir sensible à la position sans entrer dans l'objectif — fort
          seulement là où les demandes rares se concentrent vraiment ? Le combiner aux trois signaux de
          KEYRING ajoute-t-il un quatrième vote utile, ou seulement du bruit ? Et le gain de régularité
          vaut-il plus que le gain médian — une recherche plus resserrée est plus facile à transformer
          en échelle ?
        </p>
      </>
    ),
  },
};

export default function Lodestone() {
  return <InventionLayout copy={copy} score={451} reproducibility="seeded" />;
}

export const meta = pageMeta("exp-lodestone");
