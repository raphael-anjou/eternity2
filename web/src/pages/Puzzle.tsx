import { pageMeta } from "@/seo";
import { useMemo, useState } from "react";
import type { ReactNode } from "react";
import { Download } from "lucide-react";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { PieceSvg } from "@/components/board/PieceSvg";
import { MotifSwatch } from "@/components/board/MotifSwatch";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { useEngine } from "@/engine/useEngine";
import { getOfficialPuzzle } from "@/engine";
import { colorToLetter } from "@/lib/motifs";
import { downloadPiecesZip } from "@/lib/pieceSvg";
import { KNOWN_BOARDS } from "@/data/known-boards";
import { parseParams, toOurParams } from "@/lib/bucas";
import { rotateEdges } from "@/lib/types";
import { useT } from "@/i18n";

// A sample piece with its four triangles outlined: pieces are made of 4
// triangular motifs (up, right, down, left), not 4 squares.
function PieceAnatomy({ edges }: { edges: [number, number, number, number] }) {
  return (
    <svg viewBox="-130 -130 260 260" width={120} height={120}>
      {edges.map((c, dir) => (
        <use key={dir} href={`#e2m-${c}`} transform={`rotate(${[90, 180, -90, 0][dir]})`} />
      ))}
      <path d="M-128,-128 L128,128 M-128,128 L128,-128" stroke="#fff" strokeWidth={4} strokeDasharray="14 10" opacity={0.9} />
      <rect x={-128} y={-128} width={256} height={256} fill="none" stroke="#000" strokeWidth={5} />
      {[
        [0, -76, "U"],
        [88, 8, "R"],
        [0, 100, "D"],
        [-88, 8, "L"],
      ].map(([x, y, l]) => (
        <text key={l} x={x as number} y={y as number} textAnchor="middle" fontSize={44} fontWeight={700} fill="#fff" stroke="#000" strokeWidth={4} paintOrder="stroke">
          {l}
        </text>
      ))}
    </svg>
  );
}

const T = {
  en: {
    title: "The Puzzle",
    intro: (
      <>
        Eternity II is an <strong>edge-matching puzzle</strong>: 256 square pieces on a 16×16
        board. Wherever two pieces touch, their half-patterns must form the same motif; the
        board rim must be grey. That's the whole rule. It fits on a coffee table, and it has
        resisted every human and machine attempt since 2007.
      </>
    ),
    historyTitle: "A two-million-dollar grudge match",
    timeline: [
      {
        year: "1999",
        title: "Eternity I launches",
        text: "Christopher Monckton releases a 209-piece tiling puzzle with a £1,000,000 prize, convinced it will stand for years.",
      },
      {
        year: "2000",
        title: "…and falls in 18 months",
        text: "Cambridge mathematicians Alex Selby and Oliver Riordan claim the million. They didn't out-search everyone; they out-thought the puzzle, proving it had far more solutions than intended and steering their computers to the easy ones.",
      },
      {
        year: "2005–2007",
        title: "Designing the revenge",
        text: "For the sequel, Monckton brings in the very people who beat him. Selby and Riordan computer-test candidate designs so the new puzzle resists every trick that killed Eternity I: a designed solution, balanced color counts, and no exploitable statistical structure.",
      },
      {
        year: "July 2007",
        title: "Eternity II launches worldwide",
        text: "256 pieces, 22 edge motifs, published by Tomy. Prize: $2,000,000 for the first complete solution. Four companion 'clue puzzles' each revealed a hint placement.",
      },
      {
        year: "2008",
        title: "Best partial: 467 / 480",
        text: "At the first prize scrutiny, Louis Verhaard's team from Lund, Sweden holds the best partial solution, 467 matched edges out of 480, and wins the $10,000 consolation prize.",
      },
      {
        year: "Dec 31, 2010",
        title: "The prize expires unclaimed",
        text: "Nobody ever submits a complete solution. The $2,000,000 is never paid out. The puzzle quietly goes out of print, and becomes a legend.",
      },
      {
        year: "2010s–today",
        title: "The community keeps climbing",
        text: "Hobbyists with modern hardware push the record to 468, then 469 (Joshua Blackwood & Peter McGavin, 2020), then 470 (Blackwood, 2021) on the official puzzle. That 470 has stood ever since — tied twice, never beaten: ten edges still separate humanity from 480.",
      },
    ],
    anatomyTitle: "Anatomy of the piece set",
    cornerCard: (
      <>
        <strong className="text-foreground">corner pieces</strong>: two grey edges. Each fits a
        board corner in exactly one orientation.
      </>
    ),
    borderCard: (
      <>
        <strong className="text-foreground">border pieces</strong>: one grey edge, forced to sit
        on the rim with the grey facing out.
      </>
    ),
    interiorCard: (
      <>
        <strong className="text-foreground">interior pieces</strong>: four colored edges, four
        possible rotations each. This is where the explosion lives.
      </>
    ),
    edgePairs: (
      <>
        A full board has <strong>480 interior edge-pairs</strong> to match (2·16·16 − 16 − 16).
        "Score 467" means 467 of those 480 pairs match. No two pieces of the set are identical,
        and none is rotationally symmetric: the designers made sure every piece is genuinely
        distinct.
      </>
    ),
    motifsTitle: "The 22 motifs (and one grey)",
    pieceAnatomyText: (
      <>
        A piece is <strong>four triangular motifs</strong> (up, right, down, left) meeting at
        the center (here: the famous center-clue piece, n°139). A motif is therefore always a{" "}
        <em>half</em>-pattern: the full design only appears when two pieces meet and their
        triangles complete each other. That's what "matching an edge" means.
      </>
    ),
    motifDistribution: (
      <>
        Every edge carries one of 22 motifs, plus grey for the outside rim. The distribution is
        no accident: count them and you find a designed signature: five <strong>rare motifs</strong>{" "}
        appear only 24 times each and only on the frame (border-to-border joints); five appear
        48 times; the remaining twelve appear 50 times. Every count is even, because every edge
        of the hidden solution is a matched pair. In community notation each motif has a letter
        (the URL format of e2.bucas.name).
      </>
    ),
    rareFrameBadge: "rare · frame",
    borderBadge: "the border",
    loadingPieceSet: "Loading piece set…",
    allPiecesTitle: "All 256 pieces",
    allPiecesText: (
      <>
        The full official set, by piece number (1–256), shown in their stored orientation
        (rotation 0°). You can also lay them out on a board in the{" "}
        <Link className="underline" to="/viewer">Board Viewer</Link>.
      </>
    ),
    showPieceSet: "Show the full piece set",
    downloadPieces: "Download all as SVG (.zip)",
    downloadingPieces: "Zipping…",
    cluesTitle: "The five official clues",
    cluesText1: (
      <>
        The rules sheet pins one piece, <strong>piece 139 at square I8</strong>, and Tomy sold
        four smaller "clue puzzles" (two in 2007, two in 2008) that each revealed
        another placement. A board that respects all five is called <strong>strict-canonical</strong>; most
        record boards only respect the mandatory center clue, which is why the best board with all
        five clues (460, Bruno Gauthier) trails the open canonical record (470).
      </>
    ),
    cluesText2: (
      <>
        The pieces below are shown <strong>as placed on the board</strong>, already turned to
        their clue rotation (printed under each piece). At 0° a piece sits exactly as in the
        official piece list.
      </>
    ),
    pieceLabel: (n: number) => `piece ${n}`,
    cellAndRot: (cell: string, deg: number) => `${cell} · rot ${deg}°`,
    mandatoryBadge: "mandatory",
    recordsTitle: "Best known boards",
    colScore: "Score",
    colWho: "Who",
    colNotes: "Notes",
    records: [
      {
        score: "470 / 480",
        who: "Joshua Blackwood, 2021",
        notes: "the canonical record — tied in 2024 and 2025, never beaten" as ReactNode,
        viewId: "Joshua_Blackwood_470" as string | null,
      },
      {
        score: "469 / 480",
        who: "Blackwood & McGavin, 2020",
        notes: "the previous record — long quoted as the ceiling" as ReactNode,
        viewId: "JBlackwood+PMcGavin_469" as string | null,
      },
      {
        score: "468 / 480",
        who: "Joshua Blackwood",
        notes: "" as ReactNode,
        viewId: "Joshua_Blackwood_468" as string | null,
      },
      {
        score: "467 / 480",
        who: "Louis Verhaard's team, 2008",
        notes: "won the $10,000 partial prize" as ReactNode,
        viewId: "Louis_Verhaard_467" as string | null,
      },
      {
        score: "460 / 480",
        who: "Bruno Gauthier, 2023",
        notes: (
          <>
            best <em>strict-canonical</em> board (all 5 clues respected)
          </>
        ) as ReactNode,
        viewId: null as string | null,
      },
    ],
    viewLabel: "view",
    complexityTitle: "Why brute force is hopeless",
    bigNumber: (
      <>
        ~10<sup>560</sup>
      </>
    ),
    bigNumberText: (
      <>
        ways to arrange the pieces, even after exploiting that corners and borders are forced
        into their zones (4! · 56! · 196! · 4<sup>196</sup>). Writing the number out takes 560
        digits.
      </>
    ),
    universeAges: (
      <>
        10<sup>450+</sup> ages of the universe
      </>
    ),
    universeText: (
      <>
        Give every atom in the observable universe (10<sup>80</sup>) a billion boards per second
        since the Big Bang, and you cover about 10<sup>106</sup> boards. The gap isn't "needs a
        bigger computer". It's a different kind of impossible: solvers must be <em>smart</em>,
        not fast. See{" "}
        <Link className="underline" to="/algorithms">
          Algorithms
        </Link>
        .
      </>
    ),
    npComplete: (
      <>
        Formally, edge-matching puzzles are <strong>NP-complete</strong> (Demaine & Demaine,
        2007); they belong to the family of problems for which no efficient general algorithm is
        known. Eternity II is widely considered one of the hardest instances ever put in a box.
        The known clever methods that broke Eternity I were specifically engineered out: see{" "}
        <Link className="underline" to="/research">Research</Link>.
      </>
    ),
  },
  fr: {
    title: "Le Puzzle",
    intro: (
      <>
        Eternity II est un <strong>puzzle d'appariement de côtés</strong> : 256 pièces carrées à
        ranger sur un plateau de 16×16. La consigne tient en une phrase. Partout où deux pièces se
        touchent, leurs demi-dessins doivent composer le même motif, et tout le pourtour du
        plateau doit rester gris. Voilà toute la règle. Le jeu tient sur une table basse, et
        pourtant il tient tête à tous les cerveaux humains comme à toutes les machines depuis 2007.
      </>
    ),
    historyTitle: "Une revanche à deux millions de dollars",
    timeline: [
      {
        year: "1999",
        title: "Eternity I voit le jour",
        text: "Christopher Monckton met sur le marché un puzzle de pavage à 209 pièces, assorti d'un prix de 1 000 000 £. Il est persuadé que personne ne le résoudra avant des années.",
      },
      {
        year: "2000",
        title: "…et tombe en dix-huit mois",
        text: "Deux mathématiciens de Cambridge, Alex Selby et Oliver Riordan, raflent le million. Leur secret n'est pas d'avoir cherché plus vite que les autres, mais d'avoir déjoué le puzzle lui-même : ils démontrent qu'il cache bien plus de solutions que prévu, puis aiguillent leurs ordinateurs droit vers les plus accessibles.",
      },
      {
        year: "2005–2007",
        title: "L'art de préparer sa revanche",
        text: "Pour la suite, Monckton va chercher ceux-là mêmes qui l'ont humilié. Selby et Riordan passent au crible, ordinateur à l'appui, des dizaines de maquettes, jusqu'à obtenir un puzzle imperméable à toutes les astuces qui avaient eu raison d'Eternity I : une solution dessinée à l'avance, des motifs distribués en quantités soigneusement équilibrées, et plus la moindre régularité statistique à exploiter.",
      },
      {
        year: "Juillet 2007",
        title: "Eternity II débarque dans le monde entier",
        text: "256 pièces, 22 motifs de côté, le tout édité par Tomy. À la clé : 2 millions de dollars pour la première solution complète. Quatre petits « puzzles indices », vendus séparément, dévoilaient chacun la place d'une pièce.",
      },
      {
        year: "2008",
        title: "Meilleur score partiel : 467 / 480",
        text: "Lors du premier examen officiel des candidatures, c'est l'équipe de Louis Verhaard, à Lund en Suède, qui tient la meilleure solution partielle : 467 côtés appariés sur 480. De quoi décrocher le prix de consolation de 10 000 dollars.",
      },
      {
        year: "31 décembre 2010",
        title: "Le prix expire, sans vainqueur",
        text: "Personne n'aura jamais rendu de solution complète. Les 2 millions de dollars resteront dans les caisses. Le puzzle s'efface discrètement des rayons et entre dans la légende.",
      },
      {
        year: "Des années 2010 à aujourd'hui",
        title: "La communauté grignote, score après score",
        text: "Armés de machines modernes, des passionnés font grimper le record à 468, puis 469 (Joshua Blackwood et Peter McGavin, en 2020), puis 470 (Blackwood, en 2021) sur le puzzle officiel. Ce 470 tient depuis — égalé deux fois, jamais battu : il manque encore dix côtés pour atteindre le mythique 480.",
      },
    ],
    anatomyTitle: "Anatomie du jeu de pièces",
    cornerCard: (
      <>
        <strong className="text-foreground">pièces de coin</strong> : deux côtés gris. Chacune ne
        trouve sa place que dans un coin du plateau, et dans une seule orientation.
      </>
    ),
    borderCard: (
      <>
        <strong className="text-foreground">pièces de bord</strong> : un seul côté gris, qui les
        condamne au pourtour, le gris tourné vers l'extérieur.
      </>
    ),
    interiorCard: (
      <>
        <strong className="text-foreground">pièces intérieures</strong> : quatre côtés colorés,
        et quatre rotations possibles chacune. C'est là que l'explosion combinatoire se déchaîne.
      </>
    ),
    edgePairs: (
      <>
        Un plateau complet, ce sont <strong>480 paires de côtés intérieurs</strong> à faire
        coïncider (2·16·16 − 16 − 16). Un « score de 467 » veut donc dire que 467 de ces 480
        paires tombent juste. Détail crucial : dans tout le jeu, pas deux pièces ne sont
        identiques, et aucune n'est symétrique par rotation. Les concepteurs ont tout fait pour
        que chacune soit absolument unique.
      </>
    ),
    motifsTitle: "Les 22 motifs (et un gris)",
    pieceAnatomyText: (
      <>
        Une pièce, ce sont <strong>quatre motifs triangulaires</strong> (haut, droite, bas,
        gauche) qui se retrouvent en son centre (ici, la fameuse pièce de l'indice central, la
        n°139). Chaque motif n'est donc jamais qu'un <em>demi</em>-dessin : le dessin entier ne
        se révèle que lorsque deux pièces se rejoignent et que leurs triangles s'emboîtent. Voilà
        ce que veut dire « apparier un côté ».
      </>
    ),
    motifDistribution: (
      <>
        Chaque côté arbore l'un des 22 motifs, auxquels s'ajoute le gris du pourtour. Et leur
        répartition ne doit rien au hasard : faites le compte, et une signature délibérée se
        dessine. Cinq <strong>motifs rares</strong> ne reviennent que 24 fois chacun, et
        uniquement sur le cadre, là où deux pièces de bord se touchent ; cinq autres reviennent 48
        fois ; les douze derniers, 50 fois. Tous ces totaux sont pairs, et pour cause : dans la
        solution cachée, chaque côté est forcément moitié d'une paire appariée. Côté communauté,
        chaque motif se voit attribuer une lettre (c'est le format d'URL de e2.bucas.name).
      </>
    ),
    rareFrameBadge: "rare · cadre",
    borderBadge: "le bord",
    loadingPieceSet: "Chargement du jeu de pièces…",
    allPiecesTitle: "Les 256 pièces",
    allPiecesText: (
      <>
        Voici le jeu officiel au complet, rangé par numéro de pièce (1–256) et présenté dans son
        orientation de référence (rotation 0°). Envie de les poser vous-même sur un plateau ?
        Direction le{" "}
        <Link className="underline" to="/viewer">Visualiseur</Link>.
      </>
    ),
    showPieceSet: "Afficher le jeu de pièces complet",
    downloadPieces: "Tout télécharger en SVG (.zip)",
    downloadingPieces: "Compression…",
    cluesTitle: "Les cinq indices officiels",
    cluesText1: (
      <>
        La règle du jeu fige d'office une pièce, <strong>la pièce 139 sur la case I8</strong>.
        Tomy a par ailleurs commercialisé quatre petits « puzzles indices » (deux en 2007,
        deux en 2008), qui dévoilaient chacun un placement supplémentaire. Un plateau qui
        respecte les cinq est dit <strong>strict-canonique</strong>. Comme la plupart des plateaux
        records se contentent du seul indice central obligatoire, le meilleur plateau respectant
        les cinq indices (460, Bruno Gauthier) reste en deçà du record canonique libre (470).
      </>
    ),
    cluesText2: (
      <>
        Les pièces ci-dessous apparaissent <strong>telles qu'elles se posent sur le plateau</strong>,
        déjà pivotées selon la rotation de leur indice (notée sous chaque pièce). À 0°, une pièce
        est orientée exactement comme dans la liste officielle.
      </>
    ),
    pieceLabel: (n: number) => `pièce ${n}`,
    cellAndRot: (cell: string, deg: number) => `${cell} · rotation ${deg}°`,
    mandatoryBadge: "obligatoire",
    recordsTitle: "Les meilleurs plateaux connus",
    colScore: "Score",
    colWho: "Qui",
    colNotes: "Notes",
    records: [
      {
        score: "470 / 480",
        who: "Joshua Blackwood, 2021",
        notes: "le record canonique — égalé en 2024 et 2025, jamais battu" as ReactNode,
        viewId: "Joshua_Blackwood_470" as string | null,
      },
      {
        score: "469 / 480",
        who: "Blackwood & McGavin, 2020",
        notes: "le record précédent — longtemps cité comme le plafond" as ReactNode,
        viewId: "JBlackwood+PMcGavin_469" as string | null,
      },
      {
        score: "468 / 480",
        who: "Joshua Blackwood",
        notes: "" as ReactNode,
        viewId: "Joshua_Blackwood_468" as string | null,
      },
      {
        score: "467 / 480",
        who: "L'équipe de Louis Verhaard, 2008",
        notes: "prix de 10 000 $ pour la meilleure solution partielle" as ReactNode,
        viewId: "Louis_Verhaard_467" as string | null,
      },
      {
        score: "460 / 480",
        who: "Bruno Gauthier, 2023",
        notes: (
          <>
            meilleur plateau <em>strict-canonique</em> (les 5 indices respectés)
          </>
        ) as ReactNode,
        viewId: null as string | null,
      },
    ],
    viewLabel: "voir",
    complexityTitle: "Pourquoi la force brute est vouée à l'échec",
    bigNumber: (
      <>
        ~10<sup>560</sup>
      </>
    ),
    bigNumberText: (
      <>
        façons d'agencer les pièces, et ce alors même qu'on tire déjà parti du fait que coins et
        bords sont cantonnés à leurs zones (4! · 56! · 196! · 4<sup>196</sup>). Pour écrire ce
        nombre en toutes lettres, il faut aligner 560 chiffres.
      </>
    ),
    universeAges: (
      <>
        10<sup>450+</sup> âges de l'univers
      </>
    ),
    universeText: (
      <>
        Confiez à chacun des atomes de l'univers observable (10<sup>80</sup>) la tâche d'essayer
        un milliard de plateaux par seconde depuis le Big Bang : vous n'en explorerez qu'environ
        10<sup>106</sup>. Le problème n'est donc pas qu'« il faudrait une plus grosse machine ».
        On a affaire à un impossible d'un tout autre genre : pour avancer, un solveur doit être{" "}
        <em>malin</em>, pas rapide. Voir{" "}
        <Link className="underline" to="/algorithms">
          Algorithmes
        </Link>
        .
      </>
    ),
    npComplete: (
      <>
        En termes mathématiques, les puzzles d'appariement de côtés sont{" "}
        <strong>NP-complets</strong> (Demaine et Demaine, 2007) : ils font partie de ces problèmes
        pour lesquels on ne connaît aucun algorithme général efficace. Beaucoup voient en Eternity
        II l'une des instances les plus coriaces jamais glissées dans une boîte de jeu. Quant aux
        astuces qui avaient eu raison d'Eternity I, ses concepteurs les ont méthodiquement
        désamorcées : voir <Link className="underline" to="/research">Recherche</Link>.
      </>
    ),
  },
};

export default function PuzzlePage() {
  const engineReady = useEngine();
  const t = useT(T);
  const [zipping, setZipping] = useState(false);

  const puzzle = useMemo(() => (engineReady ? getOfficialPuzzle() : null), [engineReady]);

  const handleDownloadPieces = () => {
    if (!puzzle) return;
    setZipping(true);
    // Defer so the button's disabled/label state paints before the (brief,
    // synchronous) zip of 256 SVGs blocks the main thread.
    setTimeout(() => {
      try {
        downloadPiecesZip(puzzle.pieces);
      } finally {
        setZipping(false);
      }
    }, 0);
  };

  const colorStats = useMemo(() => {
    if (!puzzle) return null;
    const counts = new Array(23).fill(0);
    for (const e of puzzle.pieces) for (const c of e) counts[c]++;
    return counts as number[];
  }, [puzzle]);

  const classCounts = useMemo(() => {
    if (!puzzle) return null;
    let corners = 0,
      edges = 0,
      interior = 0;
    for (const e of puzzle.pieces) {
      const greys = e.filter((c) => c === 0).length;
      if (greys === 2) corners++;
      else if (greys === 1) edges++;
      else interior++;
    }
    return { corners, edges, interior };
  }, [puzzle]);

  return (
    <div className="space-y-12">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      {/* History */}
      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.historyTitle}</h2>
        <div className="space-y-0">
          {t.timeline.map((ev, i) => (
            <div key={i} className="relative flex gap-4 pb-6">
              <div className="flex flex-col items-center">
                <div className="z-10 mt-1 h-3 w-3 rounded-full bg-primary" />
                {i < t.timeline.length - 1 && <div className="w-px flex-1 bg-border" />}
              </div>
              <div className="-mt-0.5">
                <div className="text-sm font-semibold text-primary">{ev.year}</div>
                <div className="font-medium">{ev.title}</div>
                <p className="max-w-2xl text-sm text-muted-foreground">{ev.text}</p>
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* Anatomy */}
      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.anatomyTitle}</h2>
        <div className="grid gap-4 sm:grid-cols-3">
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-4xl">{classCounts?.corners ?? 4}</CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">{t.cornerCard}</CardContent>
          </Card>
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-4xl">{classCounts?.edges ?? 56}</CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">{t.borderCard}</CardContent>
          </Card>
          <Card>
            <CardHeader className="pb-2">
              <CardTitle className="text-4xl">{classCounts?.interior ?? 196}</CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">{t.interiorCard}</CardContent>
          </Card>
        </div>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.edgePairs}</p>
      </section>

      {/* Colors */}
      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.motifsTitle}</h2>
        <div className="flex max-w-3xl flex-wrap items-center gap-6">
          {puzzle?.pieces[138] && <PieceAnatomy edges={puzzle.pieces[138]} />}
          <p className="flex-1 basis-72 text-sm text-muted-foreground">{t.pieceAnatomyText}</p>
        </div>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.motifDistribution}</p>
        {colorStats ? (
          <div className="grid grid-cols-2 gap-2 sm:grid-cols-4 lg:grid-cols-6">
            {colorStats.map((count, color) => (
                <div key={color} className="flex items-center gap-2 rounded-md border p-2">
                  <MotifSwatch color={color} width={60} />
                  <div className="text-xs">
                    <div className="font-mono font-bold">'{colorToLetter(color)}'</div>
                    <div className="text-muted-foreground">×{count}</div>
                    {color === 0 && <Badge variant="secondary" className="mt-0.5">{t.borderBadge}</Badge>}
                    {count === 24 && <Badge variant="secondary" className="mt-0.5">{t.rareFrameBadge}</Badge>}
                  </div>
                </div>
            ))}
          </div>
        ) : (
          <p className="text-sm text-muted-foreground">{t.loadingPieceSet}</p>
        )}
      </section>

      {/* All pieces */}
      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.allPiecesTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.allPiecesText}</p>
        {puzzle && (
          <Button
            variant="outline"
            size="sm"
            onClick={handleDownloadPieces}
            disabled={zipping}
            className="gap-2"
          >
            <Download className="h-4 w-4" />
            {zipping ? t.downloadingPieces : t.downloadPieces}
          </Button>
        )}
        {puzzle && (
          <Accordion>
            <AccordionItem value="pieces">
              <AccordionTrigger>{t.showPieceSet}</AccordionTrigger>
              <AccordionContent>
                <div className="grid grid-cols-4 gap-2 sm:grid-cols-8 lg:grid-cols-12">
                  {puzzle.pieces.map((edges, i) => (
                    <div key={i} className="flex flex-col items-center">
                      <PieceSvg edges={edges} size={52} />
                      <span className="mt-0.5 text-[10px] text-muted-foreground">{i + 1}</span>
                    </div>
                  ))}
                </div>
              </AccordionContent>
            </AccordionItem>
          </Accordion>
        )}
      </section>

      {/* Hints */}
      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.cluesTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.cluesText1}</p>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.cluesText2}</p>
        {puzzle && (
          <div className="flex flex-wrap gap-3">
            {puzzle.hints.map((h) => {
              const piece = puzzle.pieces[h.piece];
              if (!piece) return null;
              const x = h.pos % 16;
              const y = Math.floor(h.pos / 16);
              return (
                <Card key={h.pos} className="w-36">
                  <CardContent className="flex flex-col items-center gap-2 pt-4">
                    <PieceSvg edges={rotateEdges(piece, h.rot)} size={84} />
                    <div className="text-center text-xs text-muted-foreground">
                      <div className="font-semibold text-foreground">{t.pieceLabel(h.piece + 1)}</div>
                      {t.cellAndRot(`${String.fromCharCode(65 + y)}${x + 1}`, h.rot * 90)}
                      {h.pos === 135 && (
                        <Badge className="mt-1" variant="secondary">
                          {t.mandatoryBadge}
                        </Badge>
                      )}
                    </div>
                  </CardContent>
                </Card>
              );
            })}
          </div>
        )}
      </section>

      {/* Records */}
      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.recordsTitle}</h2>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t.colScore}</TableHead>
              <TableHead>{t.colWho}</TableHead>
              <TableHead>{t.colNotes}</TableHead>
              <TableHead></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {t.records.map((row) => (
              <TableRow key={row.score}>
                <TableCell className="font-mono font-bold">{row.score}</TableCell>
                <TableCell>{row.who}</TableCell>
                <TableCell className="text-muted-foreground">{row.notes}</TableCell>
                <TableCell>{row.viewId && <ViewLink id={row.viewId} label={t.viewLabel} />}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </section>

      {/* Complexity */}
      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.complexityTitle}</h2>
        <div className="grid gap-4 md:grid-cols-2">
          <Card>
            <CardContent className="space-y-2 pt-6 text-sm">
              <p className="text-3xl font-bold">{t.bigNumber}</p>
              <p className="text-muted-foreground">{t.bigNumberText}</p>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="space-y-2 pt-6 text-sm">
              <p className="text-3xl font-bold">{t.universeAges}</p>
              <p className="text-muted-foreground">{t.universeText}</p>
            </CardContent>
          </Card>
        </div>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.npComplete}</p>
      </section>
    </div>
  );
}

function ViewLink({ id, label }: { id: string; label: string }) {
  const kb = KNOWN_BOARDS.find((b) => b.id === id);
  if (!kb) return null;
  const query = new URLSearchParams(toOurParams(parseParams(kb.params))).toString();
  return (
    <Link className="text-sm underline" to={`/viewer?${query}`}>
      {label}
    </Link>
  );
}

export const meta = pageMeta("puzzle");
