import { pageMeta } from "@/seo";
import { InventionLayout, type InventionCopy } from "@/components/research/InventionLayout";

const copy: { en: InventionCopy; fr: InventionCopy } = {
  en: {
    name: "STAGED",
    tagline: "Build the whole board from scratch with no pre-set frame, in stages, letting the border emerge last from whatever pieces are left.",
    idea: "Almost every solver starts by locking down the border, because the border is the most constrained part and pinning it shrinks the search. STAGED refuses that crutch. It builds the board in stages from the top down with all 256 pieces free, and never commits to a border until the very end, when the border simply falls out of what remains. The question it answers: can you reach a strong board without ever anchoring to a frame?",
    how: (
      <>
        <p>
          The build runs in four stages. The first two fill the top half of the board with a fast
          plain search, banking the partial boards that survive. At the handoff, a cheap admissible
          estimate throws out any partial that clearly can't be finished well, so later stages only
          work on promising starts.
        </p>
        <p>
          The third stage grows the next band of rows from those survivors. The fourth is an exact
          finisher on the bottom rows that minimizes mismatches, and crucially it chooses the bottom
          border last, against whatever pieces are still unused. So the frame is not designed up
          front; it emerges as a consequence of everything above it.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          STAGED reaches 436 of 480 from scratch, with an emergent border and all five official clues
          respected, built end to end with no frame to lean on. That's well below the records, and
          that gap is the finding: it measures honestly how much the usual frame-first anchor is worth,
          and it showed the frame-free machinery works at full scale.
        </p>
        <p>
          Along the way it pinned down the anatomy of the very best boards: they are a perfect block of
          most of the board plus a thin band of mismatches concentrated in a few top rows. That shape
          is what later builders aim to reproduce on purpose.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Can a generator that deliberately spends its mismatches in the top rows, to keep the rest
          perfect, reach the 450s frame-free? How much of the 436-to-record gap is the missing frame
          anchor versus the harder endgame? And does a learned finish-quality estimate pick better
          survivors than the cheap admissible one?
        </p>
      </>
    ),
  },
  fr: {
    name: "STAGED",
    tagline: "Construire tout le plateau de zéro sans cadre préétabli, par étapes, en laissant la bordure émerger en dernier à partir des pièces restantes.",
    idea: "Presque tous les solveurs commencent par verrouiller la bordure, car c'est la partie la plus contrainte et la fixer réduit la recherche. STAGED refuse cette béquille. Il construit le plateau par étapes du haut vers le bas avec les 256 pièces libres, et ne s'engage sur une bordure qu'à la toute fin, où elle découle simplement de ce qui reste. La question qu'il tranche : peut-on atteindre un bon plateau sans jamais s'ancrer à un cadre ?",
    how: (
      <>
        <p>
          La construction se fait en quatre étapes. Les deux premières remplissent la moitié supérieure
          par une recherche simple et rapide, en mettant de côté les plateaux partiels qui survivent. Au
          passage de relais, une estimation admissible bon marché écarte tout partiel manifestement
          infinissable, pour que les étapes suivantes ne travaillent que sur des départs prometteurs.
        </p>
        <p>
          La troisième étape fait croître la bande de rangées suivante à partir de ces survivants. La
          quatrième est un finisseur exact sur les rangées du bas qui minimise les défauts, et surtout
          elle choisit la bordure du bas en dernier, contre les pièces encore inutilisées. Le cadre
          n'est donc pas conçu d'avance ; il émerge comme conséquence de tout ce qui est au-dessus.
        </p>
      </>
    ),
    result: (
      <>
        <p>
          STAGED atteint 436 sur 480 de zéro, avec une bordure émergente et les cinq indices officiels
          respectés, construit de bout en bout sans cadre d'appui. C'est bien en dessous des records, et
          cet écart est le résultat : il mesure honnêtement ce que vaut l'ancrage habituel par le cadre,
          et il a montré que la mécanique sans cadre fonctionne à pleine échelle.
        </p>
        <p>
          En chemin, il a cerné l'anatomie des tout meilleurs plateaux : un bloc parfait sur la majeure
          partie du plateau plus une fine bande de défauts concentrée dans quelques rangées du haut.
          C'est cette forme que les constructeurs ultérieurs cherchent à reproduire à dessein.
        </p>
      </>
    ),
    open: (
      <>
        <p>
          Un générateur qui dépense délibérément ses défauts dans les rangées du haut, pour garder le
          reste parfait, peut-il atteindre les 450 sans cadre ? Quelle part de l'écart 436-record tient à
          l'absence d'ancrage par le cadre plutôt qu'à une fin de partie plus dure ? Et une estimation
          apprise de la qualité de fin choisit-elle de meilleurs survivants que l'estimation admissible
          bon marché ?
        </p>
      </>
    ),
  },
};

export default function Staged() {
  return <InventionLayout copy={copy} score={436} reproducibility="stochastic" />;
}

export const meta = pageMeta("inv-staged");
