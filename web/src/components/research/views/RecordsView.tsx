import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { KNOWN_BOARDS } from "@/data/known-boards";
import { RECORDS, STRICT_RECORD_ROW, type RecordRow } from "@/data/records-timeline";
import { RECORD_BOARDS } from "@/data/record-boards";
import { FIVE_CLUE_CENSUS_2026 } from "@/data/community-boards-2026";
import { RecordTimeline } from "@/components/research/RecordTimeline";

// Records we have a real, bundled board for (e2.bucas.name viewer data) get a
// clickable preview into our own /viewer. Keyed by the record row's `board`.
// The strict five-clue 466 is not in the viewer's bundled set, so its edges are
// added here: a copy of `board_edges` in
// research/topics/record-boards/community/fiveclue-466-2c037e70f7e9.json, which
// `just research-record-boards-pieces` re-verifies from pieces and rotations.
const STRICT_466_PARAMS =
  "puzzle=FiveClue_466_2c037e70f7e9&board_w=16&board_h=16&board_edges=abdaafubafufadufaftdacnfaepcabteabgbabjbaeqbacqeadgcaendaeteaabedhdauqlhuiwqujsitrujnkvrpgnktnpggrinjkgrqtjkqqwtgqmqngwqtrkgbacrdofalnkowlmnslrluqrlvntqnvjnpnsvigpngjigjrijwmmrmupmwswukppscacpfqeakwhqmqhwrpnqrilptvvijshvsskspgmsigtgikkgmnrkpgnnwwvgppvwcadpeueahhruhunhnkoulwokvlrwhwqlksmwmtrstlqtkpllroppnjrovomjvmjodabmemfarmhmnlomouploujurijuqphimonprrmoqunrlplupjppttpjmqntjskqbacsfqfahmkqorhmphhrjqphjklqhvrkntovmsutnplslmtppjnmprsjphkrkprhcabpfvcakvwvhunvhwkupmiwlljmrqwloriqusgrlgostnqgnnsnspwnkrwpqpqrbaepchbawjhhnthjkvrtijnvjlijwmulijjmgwvjollwqiklstjiwvwtwlwvqovjeabobtfahjlthukjrhounlvhiwiluiowjvvivlwvloglkqmonqoqwinqwgiivvlgbacvfhcalsuhknhsouqnvusuisouoqwsvgtqwstggvgsmqlvosnqntksijpjttkmcadtckfauwokhptwqgmpsqugooiqwmsotrvmtrgrglorlkilnvokkokvpstokigmdadifjbaosujthusmwuhushwiukssrtuvwkrgsjwoiisigqiomnikrvmtrqrgtmrdaepbjfaumhjujvmuhsjhlghkvultkuvkntkjminiommqtjonlktirilqvprmtlgeadtfwdahtrwvphtsuvpgoluuvgouwovtgiwimhgmsomjgiskoggiqwophsslsnhdadsdcaarfachbafveablfaegfafoeafibaehcabocacidacgdadweadseaencaedaac";
const BOARD_PARAMS: Record<string, string> = {
  ...Object.fromEntries(KNOWN_BOARDS.map((b) => [b.id, b.params])),
  FiveClue_466_2c037e70f7e9: STRICT_466_PARAMS,
};

// Every board id that a headline record row already points at. The sibling
// table below shows the KNOWN_BOARDS that are NOT one of these: independent
// ties and near-misses (extra 469s, the second 470, the partial run) that carry
// a real, re-scorable board but never earned their own row in the timeline.
const RECORD_BOARD_IDS = new Set(RECORDS.map((r) => r.board).filter(Boolean));
const SIBLING_BOARDS = KNOWN_BOARDS.filter(
  (b) => b.score !== null && !RECORD_BOARD_IDS.has(b.id),
).sort((a, b) => (b.score ?? 0) - (a.score ?? 0));

// The project's own bundled boards, tagged with their scoring convention. Every
// one is scored by matched edges out of 480 with only the mandatory starter
// clue placed (the open regime), recomputed from the board's own edges by the
// record-boards pipeline. The experiment page each was produced on. Ordered by
// score so the best (PALIMPSEST, 463) leads.
const PROJECT_EXPERIMENT_PATH: Record<string, string> = {
  "palimpsest-463": "/research/lab/experiments/raphael-anjou/learning/palimpsest",
  "keyring-460": "/research/lab/experiments/raphael-anjou/learning/keyring",
  "prior-460": "/research/lab/experiments/raphael-anjou/learning/prior",
  "gauntlet-458": "/research/lab/experiments/raphael-anjou/pipelines/gauntlet",
};
const PROJECT_BEST = [...RECORD_BOARDS].sort((a, b) => b.score - a.score);

// Distilled from our research vault's community history note, which is itself
// distilled from ~11,500 groups.io messages (2007–2026) + the Discord archive
// + 124 decoded community boards. The flag distinguishes boards on the
// official pieces under the contest's own rules (only the starter piece was
// mandatory: TOMY's entry form listed piece numbers only, see groups.io
// #11046) from boards on different or mixed piece sets. Several headline
// "480" boards are NOT the official puzzle. Note: none of the record boards
// from 468 up respect the four optional clue placements; boards that do are
// tracked as "strict 5-clue" (best known: 466, board FiveClue_466_2c037e70f7e9
// of the Eternity@Home library, re-verified from pieces and rotations in
// research/topics/record-boards; before it the fleet's 465 of August 2026,
// Riotte's 464 of July 2026 and Gauthier's 460 of 2023).
const T = {
  en: {
    bestTitle: "The state of the art",
    best:
      "The community ceiling on the official puzzle is 470 of 480 matched edges (Joshua Blackwood, 2021, tied once since by Jef Bucas, December 2024). The contest's own rules pinned only the starter piece (the entry form listed piece numbers, not rotations), and every record board from 468 up is in that starter-only regime, including the 469s long quoted as the ceiling; the 470s are the same puzzle, not an easier variant. Boards that also respect the four optional clue placements are tracked separately. That line moved fast in 2026: Benjamin Riotte's 464 in July ended more than three years of Bruno Gauthier's 460, the shared five-clue fleet reached 465 in August, and the best known is now 466 (14 broken edges), a board in the Eternity@Home public board library that the library shows as found by \"Jef\" on 19 September 2026, re-verified on this site from the official pieces with all five clues at their official cells and rotations. The library listed 14 boards at 466 on 13 November 2026; one of them was checked here. The full solution (480) has never been found; the 10-edge gap on the open record has stood since 2021.",
    bestProject:
      "This project's own boards reach 460 to 463 of 480 on the same puzzle: a few edges short of the community ceiling, in the same starter-only regime, and every one recomputed from its own edges and openable in the viewer.",
    bestProjectLink: "See the project's boards plotted against the community ceiling",
    openProblemsLink: "What still stands between 470 and 480: the open problems",
    standsTitle: "Where this project stands",
    standsIntro:
      "The best board on record, the best strict five-clue board, and this project's own boards, side by side. Every score is matched edges out of 480; the regime tag says which clues were pinned, since a starter-only score and a five-clue score are not the same target.",
    standsCols: { who: "Board", score: "Score", regime: "Regime", link: "Open" },
    regimeOpen: "starter-only",
    regimeStrict: "five clues",
    standsCommunityOpen: "Community best (open)",
    standsCommunityStrict: "Community best (five-clue)",
    standsProjectBest: "This project's best",
    standsProjectOther: "This project",
    standsViewCommunity: "Community boards",
    standsViewExperiment: "Experiment",
    timelineTitle: "Record timeline",
    cols: { date: "Date", score: "Score", author: "Author", puzzle: "Puzzle", method: "Method", source: "Source", preview: "Re-score" },
    canonical: "official pieces",
    variant: "other set",
    view: "View board",
    verifyNote:
      "“View board” opens the board in the viewer, which recomputes the matched-edge score from the board's own edges. The number is not taken on trust: every listed score can be checked edge by edge.",
    siblingTitle: "Sibling boards",
    siblingIntro:
      "Independent ties and near-misses that carry a real, re-scorable board but never took a timeline row of their own: the extra 469s from the Blackwood-solver runs, the second 470, and a deep partial. Each opens in the viewer.",
    siblingCols: { board: "Board", score: "Score", open: "Open" },
    censusStripTitle: "Distinct five-clue boards near the top, July 2026 census",
    censusStripNote:
      "The author's own count of distinct boards at each score. Many at the lower scores are close variants within one basin, so the totals overstate the number of structurally independent solutions.",
    censusStripBoards: "boards",
    timelineNote:
      "Mailing-list sources link the exact announcement message in the eternity2 groups.io archive; reading them requires a free groups.io account. Entries without a link were only reported in places with no public archive (e.g. Discord).",
    falsePositiveTitle: "Why some “480” boards don't count",
    falsePositive:
      "The real puzzle is the board TOMY sold: 256 specific pieces, a 16×16 frame, 22 colors, the starter piece pinned at I8 (the four other clues were optional aids under the contest rules). Several boards posted as 480/480 use different piece sets: Brendan Owen's smaller Clue-1 / Clue-2 designs, the unframed TopCoder variant, or boards mixing pieces from multiple expansion sets. They have solutions; those solutions do not transfer to the official puzzle. The official 480 remains unfound.",
    methodsTitle: "The key algorithms",
    methodsDeadEnds: "The approaches Blackwood and others reported did not help (SAT solvers, GPUs, pre-solved caches) are catalogued in full on the",
    methodsDeadEndsLink: "dead ends page",
    methods: [
      [
        "The 17+5 colour design (2007)",
        "Brendan Owen and Günter Stertenbrink's 2007 first-moment estimate puts a 16×16 with about 17 interior colours near one expected solution. That matches Eternity II's 17 interior and 5 frame-only colours, close to the hardness band that SAT/CSP studies find for random edge-matching ensembles. It is an ensemble average, not a count of this puzzle: for the real piece set the community estimates about 14,702 solutions with the centre clue, and effectively one with all five.",
      ],
      [
        "Verhaard set-annealing (2008)",
        "Louis Verhaard's 467 annealed on the composition of piece sets between regions, not just on positions: an early metaheuristic that held the record for over a decade.",
      ],
      [
        "Blackwood heuristic-schedule + break-index (2020)",
        "The dominant modern algorithm behind every 469/470 board. A scan-row backtracker driven by a piecewise-linear color-exhaustion schedule plus a small set of allowed 'break' depths, run for tens of billions of iterations before restart. Blackwood reported that SAT solvers, GPUs and pre-solved 2×2 caches did not help.",
      ],
      [
        "McGavin's engine (2020–2026)",
        "His own C backtracker, measured in 2026 at ~295 million tile placements per second on one core on an easy board (~105M on a hard one), roughly 4× a typical community engine there, via PGO, aggressive compiler flags, auto-generated per-cell unrolled code and a bottom-left, left-to-right scan. His 2020 469 was found with Blackwood's solver on about 200 cores; the speed figure is a separate engineering result. On a hard board a portable-Rust reimplementation on this site ties it (his C stays ~2.3× faster on easy boards). More throughput buys more attempts in a record hunt, but neither engine makes an exhaustive search of the full board feasible.",
      ],
      [
        "Joe's prune-back policy (2026)",
        "A recent, concrete idea: almost all backtracking time is spent very deep, so if a search spends too long below a depth threshold without progress, prune back to that depth and restart. Measured 17–49% fewer iterations depending on clue count.",
      ],
    ],
    castTitle: "Cast of contributors",
    castNote: "A non-exhaustive who's-who of the people behind the records and the infrastructure.",
    cast: [
      ["Peter McGavin", "The 469 (2020, Blackwood's solver on ~200 cores); his own auto-generated C engines and the complex-theory implementations; the community's throughput and theory anchor."],
      ["Joshua Blackwood", "The 469/470-class algorithm (heuristic schedule + break-index); the modern standard."],
      ["Jef Bucas", "libblackwood (a C port of Blackwood's solver) and bucas.name, the board viewer + URL format the community shares boards in."],
      ["Louis Verhaard", "The 467 record (2008) and set-composition swap-annealing."],
      ["Brendan Owen & Günter Stertenbrink", "The 2007 17+5 hardness estimate: about 17 interior colours puts a 16×16 near one expected solution."],
      ["Marijn Heule", "The canonical SAT encoding (2008) and ongoing SAT experiments."],
      ["Al Hopfer", "The NS-1 deficit invariant (2022): the border↔interior multiset-equality condition; Carlos Fernandez showed 14×14 quadrants then solve in minutes."],
      ["Ramon van der Werf", "A public solver blog and Wang-tile analysis of the puzzle."],
    ],
    linkBlackwoodSrc: "Blackwood's solver (original C#)",
    linkBlackwood: "Bucas's libblackwood (C port)",
    linkBlog: "Ramon van der Werf's solver blog",
    linkVerhaard: "Louis Verhaard's algorithm details",
    sourceNote:
      "Sourced from our research notebook's community history, itself distilled from ~11,500 mailing-list messages and the community Discord.",
    referencesTitle: "References",
  },
  fr: {
    bestTitle: "L'état de l'art",
    best:
      "Le plafond communautaire sur le puzzle officiel est de 470 bords appariés sur 480 (Joshua Blackwood, 2021, égalé une fois depuis par Jef Bucas, décembre 2024). Le règlement du concours n'épinglait que la pièce de départ (le formulaire ne listait que des numéros de pièces, pas des rotations), et tous les plateaux records à partir de 468 relèvent de ce régime « pièce de départ seule », y compris les 469 longtemps cités comme plafond ; les 470 sont le même puzzle, pas une variante plus facile. Les plateaux qui respectent aussi les quatre indices facultatifs sont suivis à part. Cette ligne a vite bougé en 2026 : le 464 de Benjamin Riotte, en juillet, a mis fin à plus de trois ans de règne du 460 de Bruno Gauthier ; la flotte partagée à cinq indices a atteint 465 en août ; et le meilleur connu est désormais 466 (14 arêtes cassées), un plateau de la bibliothèque publique d'Eternity@Home, qui l'attribue à « Jef » à la date du 19 septembre 2026, revérifié sur ce site à partir des pièces officielles, les cinq indices à leurs cases et rotations officielles. La bibliothèque recensait 14 plateaux à 466 le 13 novembre 2026 ; un seul a été vérifié ici. La solution complète (480) reste introuvée ; l'écart de 10 arêtes sur le record ouvert tient depuis 2021.",
    bestProject:
      "Les plateaux propres à ce projet atteignent 460 à 463 sur 480 sur le même puzzle : à quelques arêtes du plafond communautaire, dans le même régime « pièce de départ seule », et chacun recalculé depuis ses propres arêtes et ouvrable dans le visualiseur.",
    bestProjectLink: "Voir les plateaux du projet tracés face au plafond communautaire",
    openProblemsLink: "Ce qui sépare encore 470 de 480 : les problèmes ouverts",
    standsTitle: "Où en est ce projet",
    standsIntro:
      "Le meilleur plateau connu, le meilleur plateau strict à cinq indices, et les plateaux propres à ce projet, côte à côte. Chaque score est en arêtes appariées sur 480 ; l'étiquette de régime indique quels indices étaient épinglés, car un score « pièce de départ seule » et un score à cinq indices ne visent pas la même cible.",
    standsCols: { who: "Plateau", score: "Score", regime: "Régime", link: "Ouvrir" },
    regimeOpen: "pièce de départ",
    regimeStrict: "cinq indices",
    standsCommunityOpen: "Meilleur communautaire (ouvert)",
    standsCommunityStrict: "Meilleur communautaire (cinq indices)",
    standsProjectBest: "Meilleur du projet",
    standsProjectOther: "Ce projet",
    standsViewCommunity: "Plateaux communautaires",
    standsViewExperiment: "Expérience",
    timelineTitle: "Chronologie des records",
    cols: { date: "Date", score: "Score", author: "Auteur", puzzle: "Puzzle", method: "Méthode", source: "Source", preview: "Re-noter" },
    canonical: "pièces officielles",
    variant: "autre jeu",
    view: "Voir le plateau",
    verifyNote:
      "« Voir le plateau » ouvre le plateau dans le visualiseur, qui recalcule le score d'arêtes appariées depuis les arêtes du plateau lui-même. Le nombre n'est pas pris sur parole : chaque score listé se vérifie arête par arête.",
    siblingTitle: "Plateaux frères",
    siblingIntro:
      "Des égalités indépendantes et des quasi-records qui portent un vrai plateau re-notable mais n'ont jamais pris de ligne propre dans la chronologie : les 469 supplémentaires issus des runs du solveur de Blackwood, le second 470, et un partiel profond. Chacun s'ouvre dans le visualiseur.",
    siblingCols: { board: "Plateau", score: "Score", open: "Ouvrir" },
    censusStripTitle: "Plateaux distincts à cinq indices, en haut de l'échelle (recensement de juillet 2026)",
    censusStripNote:
      "Le décompte de l'auteur des plateaux distincts à chaque score. Beaucoup, aux scores plus bas, sont des variantes proches d'un même bassin ; les totaux surestiment donc le nombre de solutions structurellement indépendantes.",
    censusStripBoards: "plateaux",
    timelineNote:
      "Les sources « groups.io » pointent vers le message d'annonce exact dans les archives de la liste eternity2 ; leur lecture demande un compte groups.io gratuit. Les entrées sans lien n'ont été rapportées que dans des espaces sans archive publique (Discord, par exemple).",
    falsePositiveTitle: "Pourquoi certains plateaux « 480 » ne comptent pas",
    falsePositive:
      "Le vrai puzzle est le plateau vendu par TOMY : 256 pièces précises, un cadre 16×16, 22 couleurs, la pièce de départ épinglée en I8 (les quatre autres indices n'étaient que des aides facultatives selon le règlement). Plusieurs plateaux publiés en 480/480 utilisent d'autres jeux de pièces : les designs Clue-1 / Clue-2 plus petits de Brendan Owen, la variante non encadrée de TopCoder, ou des plateaux mélangeant des pièces de plusieurs jeux d'extension. Ils ont des solutions ; celles-ci ne se transposent pas au puzzle officiel. Le 480 officiel reste introuvé.",
    methodsTitle: "Les algorithmes clés",
    methodsDeadEnds: "Les approches que Blackwood et d'autres ont signalées comme sans effet (solveurs SAT, GPU, caches pré-résolus) sont recensées en détail sur la",
    methodsDeadEndsLink: "page des impasses",
    methods: [
      [
        "Le choix des 17+5 couleurs (2007)",
        "L'estimation au premier moment de Brendan Owen et Günter Stertenbrink, en 2007, place un 16×16 à environ 17 couleurs intérieures près d'une solution attendue. Cela correspond aux 17 couleurs intérieures et 5 couleurs de cadre d'Eternity II, à proximité de la zone de difficulté que les études SAT/CSP observent sur des ensembles aléatoires de puzzles d'appariement. C'est une moyenne sur l'ensemble, pas un décompte de ce puzzle : pour le vrai jeu de pièces, la communauté estime environ 14 702 solutions avec l'indice central, et pratiquement une seule avec les cinq.",
      ],
      [
        "Recuit sur jeux de Verhaard (2008)",
        "Le 467 de Louis Verhaard recuit sur la composition des jeux de pièces entre régions, pas seulement sur les positions : une métaheuristique précoce qui a tenu le record plus d'une décennie.",
      ],
      [
        "Schedule heuristique + break-index de Blackwood (2020)",
        "L'algorithme moderne dominant derrière chaque plateau 469/470. Un backtracker en balayage de lignes piloté par un objectif d'épuisement des couleurs linéaire par morceaux plus un petit ensemble de profondeurs de « rupture » autorisées, lancé sur des dizaines de milliards d'itérations avant redémarrage. Blackwood a rapporté que les solveurs SAT, les GPU et les caches de 2×2 pré-résolus n'aidaient pas.",
      ],
      [
        "Le moteur de McGavin (2020–2026)",
        "Son propre backtracker C, mesuré en 2026 à ≈295 millions de placements de tuiles par seconde sur un cœur sur un plateau facile (~105 M sur un plateau difficile), soit environ 4× un moteur communautaire typique dans ce cas, grâce à la PGO, à des options de compilation agressives, à du code déroulé par case auto-généré et à un balayage partant du coin bas-gauche, de gauche à droite. Son 469 de 2020 a été trouvé avec le solveur de Blackwood sur environ 200 cœurs ; le chiffre de vitesse est un résultat d'ingénierie distinct. Sur un plateau difficile, une réimplémentation en Rust portable sur ce site l'égale (son C reste ~2,3× plus rapide sur les plateaux faciles). Plus de débit, c'est plus de tentatives dans une chasse au record, mais aucun des deux moteurs ne rend faisable une recherche exhaustive du plateau complet.",
      ],
      [
        "La politique de prune-back de Joe (2026)",
        "Une idée récente et concrète : presque tout le temps de retour sur trace se passe très en profondeur ; donc si une recherche passe trop de temps sous un seuil de profondeur sans progrès, on élague jusqu'à ce seuil et on redémarre. Mesuré : 17 à 49 % d'itérations en moins selon le nombre d'indices.",
      ],
    ],
    castTitle: "Les contributeurs",
    castNote: "Un who's-who non exhaustif des personnes derrière les records et l'infrastructure.",
    cast: [
      ["Peter McGavin", "Le 469 (2020, solveur de Blackwood sur ~200 cœurs) ; ses moteurs C auto-générés et ses implémentations de la théorie complexe ; l'ancre débit et théorie de la communauté."],
      ["Joshua Blackwood", "L'algorithme de classe 469/470 (schedule heuristique + break-index) ; le standard moderne."],
      ["Jef Bucas", "libblackwood (un port C du solveur de Blackwood) et bucas.name, le visualiseur et le format d'URL avec lequel la communauté partage les plateaux."],
      ["Louis Verhaard", "Le record 467 (2008) et le recuit par composition de jeux."],
      ["Brendan Owen & Günter Stertenbrink", "L'estimation de difficulté 17+5 de 2007 : environ 17 couleurs intérieures placent un 16×16 près d'une solution attendue."],
      ["Marijn Heule", "L'encodage SAT canonique (2008) et des expériences SAT continues."],
      ["Al Hopfer", "L'invariant de déficit NS-1 (2022) : la condition d'égalité des multisets bord↔intérieur ; Carlos Fernandez a montré que les quadrants 14×14 se résolvent alors en minutes."],
      ["Ramon van der Werf", "Un blog public de solveur et une analyse en tuiles de Wang du puzzle."],
    ],
    linkBlackwoodSrc: "Le solveur de Blackwood (C# original)",
    linkBlackwood: "libblackwood de Bucas (portage C)",
    linkBlog: "Le blog de solveur de Ramon van der Werf",
    linkVerhaard: "Les détails de l'algorithme de Louis Verhaard",
    sourceNote:
      "Tiré de l'historique communautaire de notre carnet de recherche, lui-même distillé de ≈11 500 messages de la liste de diffusion et du Discord de la communauté.",
    referencesTitle: "Références",
  },
  es: {
    bestTitle: "El estado del arte",
    best:
      "El techo de la comunidad sobre el puzzle oficial es de 470 aristas coincidentes de 480 (Joshua Blackwood, 2021, igualado una sola vez desde entonces por Jef Bucas, diciembre de 2024). El reglamento del concurso solo fijaba la pieza de partida (el formulario listaba números de pieza, no rotaciones), y todos los tableros récord a partir de 468 pertenecen a ese régimen de «solo pieza de partida», incluidos los 469 citados durante mucho tiempo como techo; los 470 son el mismo puzzle, no una variante más fácil. Los tableros que además respetan las cuatro pistas opcionales se registran aparte. Esa línea avanzó deprisa en 2026: el 464 de Benjamin Riotte, en julio, puso fin a más de tres años del 460 de Bruno Gauthier; la flota compartida de cinco pistas llegó a 465 en agosto; y el mejor conocido es ahora 466 (14 aristas rotas), un tablero de la biblioteca pública de Eternity@Home, que lo atribuye a «Jef» con fecha del 19 de septiembre de 2026, verificado de nuevo en este sitio a partir de las piezas oficiales, con las cinco pistas en sus casillas y rotaciones oficiales. La biblioteca listaba 14 tableros con 466 el 13 de noviembre de 2026; aquí solo se comprobó uno. La solución completa (480) nunca se ha encontrado; la diferencia de 10 aristas en el récord abierto se mantiene desde 2021.",
    bestProject:
      "Los tableros propios de este proyecto alcanzan de 460 a 463 de 480 en el mismo puzzle: a pocas aristas del techo de la comunidad, en el mismo régimen de «solo pieza de partida», y cada uno recalculado a partir de sus propias aristas y abrible en el visor.",
    bestProjectLink: "Ver los tableros del proyecto trazados frente al techo de la comunidad",
    openProblemsLink: "Lo que aún separa 470 de 480: los problemas abiertos",
    standsTitle: "Dónde se sitúa este proyecto",
    standsIntro:
      "El mejor tablero conocido, el mejor tablero estricto de cinco pistas y los tableros propios de este proyecto, uno al lado del otro. Cada puntuación es en aristas coincidentes sobre 480; la etiqueta de régimen indica qué pistas se fijaron, porque una puntuación de «solo pieza de partida» y una de cinco pistas no apuntan al mismo objetivo.",
    standsCols: { who: "Tablero", score: "Puntuación", regime: "Régimen", link: "Abrir" },
    regimeOpen: "pieza de partida",
    regimeStrict: "cinco pistas",
    standsCommunityOpen: "Mejor de la comunidad (abierto)",
    standsCommunityStrict: "Mejor de la comunidad (cinco pistas)",
    standsProjectBest: "Lo mejor del proyecto",
    standsProjectOther: "Este proyecto",
    standsViewCommunity: "Tableros de la comunidad",
    standsViewExperiment: "Experimento",
    timelineTitle: "Cronología de récords",
    cols: { date: "Fecha", score: "Puntuación", author: "Autor", puzzle: "Puzzle", method: "Método", source: "Fuente", preview: "Recalcular" },
    canonical: "piezas oficiales",
    variant: "otro conjunto",
    view: "Ver tablero",
    verifyNote:
      "«Ver tablero» abre el tablero en el visor, que recalcula la puntuación de aristas coincidentes a partir de las propias aristas del tablero. El número no se toma por fe: cada puntuación listada se puede comprobar arista por arista.",
    siblingTitle: "Tableros hermanos",
    siblingIntro:
      "Empates independientes y casi-récords que llevan un tablero real y recalculable pero que nunca ocuparon una fila propia en la cronología: los 469 adicionales de las ejecuciones del solucionador de Blackwood, el segundo 470 y un parcial profundo. Cada uno se abre en el visor.",
    siblingCols: { board: "Tablero", score: "Puntuación", open: "Abrir" },
    censusStripTitle: "Tableros distintos de cinco pistas en la cima (recuento de julio de 2026)",
    censusStripNote:
      "El recuento del propio autor de tableros distintos en cada puntuación. Muchos, en las puntuaciones más bajas, son variantes cercanas dentro de una misma cuenca, de modo que los totales sobrestiman el número de soluciones estructuralmente independientes.",
    censusStripBoards: "tableros",
    timelineNote:
      "Las fuentes de la lista de correo enlazan al mensaje exacto de anuncio en el archivo de eternity2 en groups.io; leerlas requiere una cuenta gratuita de groups.io. Las entradas sin enlace solo se comunicaron en espacios sin archivo público (por ejemplo, Discord).",
    falsePositiveTitle: "Por qué algunos tableros «480» no cuentan",
    falsePositive:
      "El puzzle real es el tablero que vendió TOMY: 256 piezas concretas, un marco de 16×16, 22 colores, la pieza de partida fijada en I8 (las otras cuatro pistas eran ayudas opcionales según el reglamento). Varios tableros publicados como 480/480 usan otros conjuntos de piezas: los diseños Clue-1 / Clue-2 más pequeños de Brendan Owen, la variante sin marco de TopCoder, o tableros que mezclan piezas de varios conjuntos de expansión. Tienen solución, pero esa solución no se traslada al puzzle oficial. El 480 oficial sigue sin encontrarse.",
    methodsTitle: "Los algoritmos clave",
    methodsDeadEnds: "Los enfoques que Blackwood y otros reportaron como inútiles (solucionadores SAT, GPU, cachés preresueltas) están catalogados en detalle en la",
    methodsDeadEndsLink: "página de callejones sin salida",
    methods: [
      [
        "El diseño de 17+5 colores (2007)",
        "La estimación de primer momento de Brendan Owen y Günter Stertenbrink, de 2007, sitúa un 16×16 con unos 17 colores interiores cerca de una solución esperada. Eso coincide con los 17 colores interiores y 5 de marco de Eternity II, cerca de la franja de dificultad que los estudios SAT/CSP encuentran en conjuntos aleatorios de puzzles de encaje de bordes. Es un promedio sobre el conjunto, no un recuento de este puzzle: para el juego real de piezas, la comunidad estima unas 14.702 soluciones con la pista central, y en la práctica una sola con las cinco.",
      ],
      [
        "Recocido sobre conjuntos de Verhaard (2008)",
        "El 467 de Louis Verhaard aplicaba recocido a la composición de los conjuntos de piezas entre regiones, no solo a las posiciones: una metaheurística temprana que mantuvo el récord durante más de una década.",
      ],
      [
        "Programación heurística + índice de ruptura de Blackwood (2020)",
        "El algoritmo moderno dominante detrás de cada tablero 469/470. Un backtracker por barrido de filas guiado por un objetivo de agotamiento de colores lineal por tramos más un pequeño conjunto de profundidades de «ruptura» permitidas, ejecutado durante decenas de miles de millones de iteraciones antes de reiniciar. Blackwood informó de que los solucionadores SAT, las GPU y las cachés de 2×2 preresueltas no ayudaban.",
      ],
      [
        "El motor de McGavin (2020–2026)",
        "Su propio backtracker en C, medido en 2026 a ≈295 millones de colocaciones de piezas por segundo en un solo núcleo sobre un tablero fácil (~105 M sobre uno difícil), unas 4× más que un motor comunitario típico en ese caso, gracias a PGO, opciones de compilación agresivas, código desenrollado por celda generado automáticamente y un recorrido que empieza abajo a la izquierda y avanza de izquierda a derecha. Su 469 de 2020 se encontró con el solucionador de Blackwood en unos 200 núcleos; la cifra de velocidad es un resultado de ingeniería aparte. En un tablero difícil, una reimplementación en Rust portable en este sitio lo iguala (su C sigue siendo ~2,3× más rápido en los tableros fáciles). Más rendimiento significa más intentos en la caza de récords, pero ninguno de los dos motores vuelve factible una búsqueda exhaustiva del tablero completo.",
      ],
      [
        "La política de poda-atrás de Joe (2026)",
        "Una idea reciente y concreta: casi todo el tiempo de backtracking se pasa a mucha profundidad, así que si una búsqueda tarda demasiado por debajo de un umbral de profundidad sin avanzar, se poda hasta ese umbral y se reinicia. Se midió un 17–49 % menos de iteraciones según el número de pistas.",
      ],
    ],
    castTitle: "Elenco de colaboradores",
    castNote: "Un quién es quién no exhaustivo de las personas detrás de los récords y la infraestructura.",
    cast: [
      ["Peter McGavin", "El 469 (2020, con el solucionador de Blackwood sobre ~200 núcleos); sus propios motores en C generados automáticamente y las implementaciones de la teoría compleja; el ancla de rendimiento y teoría de la comunidad."],
      ["Joshua Blackwood", "El algoritmo de clase 469/470 (programación heurística + índice de ruptura); el estándar moderno."],
      ["Jef Bucas", "libblackwood (un port en C del solucionador de Blackwood) y bucas.name, el visor de tableros y el formato de URL con el que la comunidad comparte tableros."],
      ["Louis Verhaard", "El récord 467 (2008) y el recocido por intercambio de composición de conjuntos."],
      ["Brendan Owen & Günter Stertenbrink", "La estimación de dificultad 17+5 de 2007: unos 17 colores interiores sitúan un 16×16 cerca de una solución esperada."],
      ["Marijn Heule", "La codificación SAT canónica (2008) y experimentos SAT en curso."],
      ["Al Hopfer", "El invariante de déficit NS-1 (2022): la condición de igualdad de multiconjuntos borde↔interior; Carlos Fernandez demostró que los cuadrantes 14×14 se resuelven entonces en minutos."],
      ["Ramon van der Werf", "Un blog público sobre solucionadores y un análisis del puzzle con teselas de Wang."],
    ],
    linkBlackwoodSrc: "El solucionador de Blackwood (C# original)",
    linkBlackwood: "libblackwood de Bucas (port en C)",
    linkBlog: "El blog de solucionadores de Ramon van der Werf",
    linkVerhaard: "Los detalles del algoritmo de Louis Verhaard",
    sourceNote:
      "Extraído del historial comunitario de nuestro cuaderno de investigación, a su vez destilado de ≈11 500 mensajes de la lista de correo y del Discord de la comunidad.",
    referencesTitle: "Referencias",
  },
};

const BADGE: Record<RecordRow["canonical"], string> = {
  canonical: "bg-emerald-100 text-emerald-900 dark:bg-emerald-950 dark:text-emerald-200",
  variant: "bg-muted text-muted-foreground",
};

// Numbered references for the contestable record claims. The community records
// (468/469/470) are not in any peer-reviewed or encyclopedic source (they live
// in the mailing list and on the board viewer), so we point at the primary
// community sources directly and flag what mainstream sources do and don't say.
// Exact community sources. The 468/469/470/460/464/465 boards are not in any
// peer-reviewed or encyclopedic source (they were announced on the mailing
// list), so we link the specific groups.io messages, using the archive's
// sequential message numbers (msg_num; author, date and subject all verified
// against our full message export). Reading a message needs a free groups.io
// login: anonymous access to the archive is login-walled.
const GROUPS_IO = "https://groups.io/g/eternity2";
const REFS: { href: string; label: string }[] = [
  {
    href: `${GROUPS_IO}/message/10033`,
    label:
      "468: Jef Bucas, “New best solution found (12 breaks)”, eternity2@groups.io, 2020-08-31 (Blackwood's board, relayed to the group).",
  },
  {
    href: `${GROUPS_IO}/message/10045`,
    label:
      "469: Peter McGavin, “EternityII Solver” thread, eternity2@groups.io, 2020-09-09: “New record score of 469! Only 11 breaks!”, the ceiling until the 470.",
  },
  {
    href: `${GROUPS_IO}/message/10117`,
    label:
      "470: Joshua Blackwood, “470 Found”, eternity2@groups.io, 2021-03-30, same starter-only regime as the 468/469 (board-level verification: groups.io #10554); the standing record.",
  },
  {
    href: `${GROUPS_IO}/message/11074`,
    label:
      "460: Bruno Gauthier, “Highest points (of 480) with using all 5 (!) hints?” thread, eternity2@groups.io, 2023-03-09: the best board respecting all five clues for over three years (Eternity II Editor).",
  },
  {
    href: `${GROUPS_IO}/message/11919`,
    label:
      "464: Benjamin Riotte, “Record of Eternity2 with 5 hints ?” thread, eternity2@groups.io, 2026-07-06: the strict-five-clue record from July to August 2026 (16 broken edges), all five clues at their official cells, found with his own modified-Blackwood solver; Igor Pejic reached the same 463–464 range independently in the thread.",
  },
  {
    href: `${GROUPS_IO}/topic/120997660`,
    label:
      "465: Benjamin Riotte, “Join the five-clue Eternity II effort” topic, eternity2@groups.io, 2026-08-30: the strict-five-clue record from August to September 2026 (15 broken edges), found by the community's distributed fleet; 11 distinct 465 boards at the announcement and many more since, credited across several contributors' machines.",
  },
  {
    href: "https://stats.eternityathome.org/library/board/2c037e70f7e93518a48733c7aacd096226b7f23728efabd6289088b91c0945cd",
    label:
      "466: board FiveClue_466_2c037e70f7e9, Eternity@Home public board library, shown there as discovered by “Jef” on 2026-09-19 (retrieved 2026-11-13): the highest verified strict-five-clue board as of 2026-11-13 (14 broken edges). Re-verified from official pieces and rotations in research/topics/record-boards (just research-record-boards-pieces).",
  },
  {
    href: "https://e2.bucas.name",
    label:
      "e2.bucas.name (Jef Bucas): the board viewer and shared URL format; every board above can be loaded and re-scored there.",
  },
  {
    href: "https://en.wikipedia.org/wiki/Eternity_II_puzzle",
    label:
      "Wikipedia, “Eternity II puzzle”: documents only the encyclopedic facts: the $2M prize, the deadline of noon 31 Dec 2010, and Verhaard's 467 ($10,000). The 468+ records remain community-reported, not yet in a source Wikipedia accepts.",
  },
];

// A superscript [n] linking down to the References list.
function Cite({ n }: { n: number }) {
  return (
    <sup>
      <a href="#refs" className="px-0.5 text-xs underline underline-offset-2 hover:text-foreground">
        [{n}]
      </a>
    </sup>
  );
}

// The community anchor rows for the "where this project stands" table, pulled
// from the timeline so they never drift from it: the standing open record and
// the standing strict five-clue record (flagged on its row with strictRecord).
const COMMUNITY_OPEN = RECORDS.find((r) => r.board === "Joshua_Blackwood_470");
const COMMUNITY_STRICT = STRICT_RECORD_ROW;

export function RecordsView() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <section className="max-w-3xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.bestTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">
          {t.best}
          <Cite n={3} />
          <Cite n={5} />
          <Cite n={6} />
          <Cite n={7} />
        </p>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.bestProject}</p>
        <div className="flex flex-col gap-1.5 text-sm">
          <LocalizedLink
            to="/research/lab/experiments/raphael-anjou"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.bestProjectLink}
          </LocalizedLink>
          <LocalizedLink
            to="/research/open-problems"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.openProblemsLink}
          </LocalizedLink>
        </div>
      </section>

      <section className="space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.standsTitle}</h2>
        <p className="max-w-3xl text-sm leading-relaxed text-muted-foreground">{t.standsIntro}</p>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t.standsCols.who}</TableHead>
              <TableHead className="text-right">{t.standsCols.score}</TableHead>
              <TableHead>{t.standsCols.regime}</TableHead>
              <TableHead>{t.standsCols.link}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {COMMUNITY_OPEN && (
              <TableRow>
                <TableCell>
                  <span className="font-medium">{t.standsCommunityOpen}</span>{" "}
                  <span className="text-muted-foreground">{COMMUNITY_OPEN.author}</span>
                </TableCell>
                <TableCell className="text-right font-semibold tabular-nums">{COMMUNITY_OPEN.score}</TableCell>
                <TableCell>
                  <span className={`rounded px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${BADGE.canonical}`}>
                    {t.regimeOpen}
                  </span>
                </TableCell>
                <TableCell className="whitespace-nowrap">
                  <LocalizedLink to="/research/community/boards" className="text-sm underline underline-offset-2 hover:text-foreground">
                    {t.standsViewCommunity}
                  </LocalizedLink>
                </TableCell>
              </TableRow>
            )}
            {COMMUNITY_STRICT && (
              <TableRow>
                <TableCell>
                  <span className="font-medium">{t.standsCommunityStrict}</span>{" "}
                  <span className="text-muted-foreground">{COMMUNITY_STRICT.author}</span>
                </TableCell>
                <TableCell className="text-right font-semibold tabular-nums">{COMMUNITY_STRICT.score}</TableCell>
                <TableCell>
                  <span className={`rounded px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${BADGE.canonical}`}>
                    {t.regimeStrict}
                  </span>
                </TableCell>
                <TableCell className="whitespace-nowrap">
                  {COMMUNITY_STRICT.board && BOARD_PARAMS[COMMUNITY_STRICT.board] ? (
                    <LocalizedLink
                      to={`/viewer?${BOARD_PARAMS[COMMUNITY_STRICT.board]}`}
                      className="text-sm underline underline-offset-2 hover:text-foreground"
                    >
                      {t.view}
                    </LocalizedLink>
                  ) : (
                    <LocalizedLink to="/research/community/boards" className="text-sm underline underline-offset-2 hover:text-foreground">
                      {t.standsViewCommunity}
                    </LocalizedLink>
                  )}
                </TableCell>
              </TableRow>
            )}
            {PROJECT_BEST.map((b, i) => {
              const experimentPath = PROJECT_EXPERIMENT_PATH[b.id];
              return (
                <TableRow key={b.id}>
                  <TableCell>
                    <span className="font-medium">
                      {i === 0 ? t.standsProjectBest : t.standsProjectOther}
                    </span>{" "}
                    <span className="text-muted-foreground">{b.invention}</span>
                  </TableCell>
                  <TableCell className="text-right font-semibold tabular-nums">{b.score}</TableCell>
                  <TableCell>
                    <span className={`rounded px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${BADGE.canonical}`}>
                      {t.regimeOpen}
                    </span>
                  </TableCell>
                  <TableCell className="whitespace-nowrap">
                    {experimentPath ? (
                      <LocalizedLink
                        to={experimentPath}
                        className="text-sm underline underline-offset-2 hover:text-foreground"
                      >
                        {t.standsViewExperiment}
                      </LocalizedLink>
                    ) : (
                      <span className="text-sm text-muted-foreground">{b.invention}</span>
                    )}
                  </TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </section>

      <section className="space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.timelineTitle}</h2>
        <RecordTimeline />
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t.cols.date}</TableHead>
              <TableHead className="text-right">{t.cols.score}</TableHead>
              <TableHead>{t.cols.author}</TableHead>
              <TableHead>{t.cols.puzzle}</TableHead>
              <TableHead>{t.cols.method}</TableHead>
              <TableHead>{t.cols.source}</TableHead>
              <TableHead>{t.cols.preview}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {RECORDS.map((r, i) => (
              <TableRow key={`${r.date}-${i}`}>
                <TableCell className="whitespace-nowrap text-muted-foreground">{r.date}</TableCell>
                <TableCell className="text-right font-semibold tabular-nums">{r.score}</TableCell>
                <TableCell className="whitespace-nowrap">{r.author}</TableCell>
                <TableCell>
                  <span
                    className={`rounded px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${BADGE[r.canonical]}`}
                  >
                    {r.canonical === "canonical" ? t.canonical : t.variant}
                  </span>
                </TableCell>
                <TableCell className="text-sm text-muted-foreground">{r.method}</TableCell>
                <TableCell className="whitespace-nowrap">
                  {r.source ? (
                    <a
                      href={r.source.href}
                      target="_blank"
                      rel="noreferrer"
                      className="text-xs underline underline-offset-2 hover:text-foreground"
                    >
                      {r.source.label}
                    </a>
                  ) : (
                    <span className="text-sm text-muted-foreground">–</span>
                  )}
                </TableCell>
                <TableCell className="whitespace-nowrap">
                  {r.board && BOARD_PARAMS[r.board] ? (
                    <LocalizedLink
                      to={`/viewer?${BOARD_PARAMS[r.board]}`}
                      className="text-sm underline underline-offset-2 hover:text-foreground"
                    >
                      {t.view}
                    </LocalizedLink>
                  ) : (
                    <span className="text-sm text-muted-foreground">–</span>
                  )}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        <p className="max-w-3xl text-xs text-muted-foreground">{t.verifyNote}</p>
        <p className="max-w-3xl text-xs text-muted-foreground">{t.timelineNote}</p>

        <div className="max-w-2xl space-y-2 pt-2">
          <p className="text-sm font-medium">{t.censusStripTitle}</p>
          <div className="flex flex-wrap gap-2">
            {FIVE_CLUE_CENSUS_2026.map((c) => (
              <span
                key={c.score}
                className="rounded-md border bg-muted/30 px-2.5 py-1 text-xs tabular-nums"
              >
                <span className="font-semibold text-foreground">{c.found.toLocaleString()}</span>{" "}
                {t.censusStripBoards} @ {c.score}
              </span>
            ))}
          </div>
          <p className="max-w-xl text-xs text-muted-foreground">{t.censusStripNote}</p>
        </div>

        <Accordion className="max-w-3xl pt-2">
          <AccordionItem value="siblings">
            <AccordionTrigger>{t.siblingTitle}</AccordionTrigger>
            <AccordionContent>
              <p className="mb-3 max-w-2xl text-sm text-muted-foreground">{t.siblingIntro}</p>
              <table className="w-full max-w-xl text-sm">
                <thead>
                  <tr className="border-b text-left text-xs text-muted-foreground">
                    <th className="py-1 pr-4 font-medium">{t.siblingCols.board}</th>
                    <th className="py-1 pr-4 text-right font-medium">{t.siblingCols.score}</th>
                    <th className="py-1 font-medium">{t.siblingCols.open}</th>
                  </tr>
                </thead>
                <tbody>
                  {SIBLING_BOARDS.map((b) => (
                    <tr key={b.id} className="border-b last:border-0">
                      <td className="py-1 pr-4">{b.label}</td>
                      <td className="py-1 pr-4 text-right font-semibold tabular-nums">{b.score}</td>
                      <td className="py-1 whitespace-nowrap">
                        <LocalizedLink
                          to={`/viewer?${b.params}`}
                          className="underline underline-offset-2 hover:text-foreground"
                        >
                          {t.view}
                        </LocalizedLink>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.falsePositiveTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">
          {t.falsePositive}
          <Cite n={9} />
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.methodsTitle}</h2>
        <dl className="max-w-3xl space-y-3 text-sm">
          {t.methods.map(([term, desc]) => (
            <div key={term}>
              <dt className="font-medium">{term}</dt>
              <dd className="mt-0.5 text-muted-foreground">{desc}</dd>
            </div>
          ))}
        </dl>
        <p className="max-w-3xl text-sm text-muted-foreground">
          {t.methodsDeadEnds}{" "}
          <LocalizedLink to="/research/build/dead-ends" className="underline underline-offset-2 hover:text-foreground">
            {t.methodsDeadEndsLink}
          </LocalizedLink>
          .
        </p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.castTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.castNote}</p>
        <dl className="max-w-3xl space-y-2 text-sm">
          {t.cast.map(([name, desc]) => (
            <div key={name} className="grid grid-cols-1 gap-1 sm:grid-cols-[14rem_1fr] sm:gap-3">
              <dt className="font-medium">{name}</dt>
              <dd className="text-muted-foreground">{desc}</dd>
            </div>
          ))}
        </dl>
        <div className="flex flex-wrap gap-x-6 gap-y-1 pt-1 text-sm">
          <a
            href="https://github.com/jblackwood345/EternityII_Solver"
            target="_blank"
            rel="noreferrer"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.linkBlackwoodSrc}
          </a>
          <a
            href="https://github.com/jfbucas/libblackwood"
            target="_blank"
            rel="noreferrer"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.linkBlackwood}
          </a>
          <a
            href="https://ramonvdw.github.io/e2solver23/"
            target="_blank"
            rel="noreferrer"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.linkBlog}
          </a>
          <a
            href="https://www.shortestpath.se/eii/eii_details.html"
            target="_blank"
            rel="noreferrer"
            className="underline underline-offset-2 hover:text-foreground"
          >
            {t.linkVerhaard}
          </a>
        </div>
      </section>

      <section id="refs" className="max-w-3xl space-y-2 scroll-mt-20">
        <h2 className="text-xl font-semibold tracking-tight">{t.referencesTitle}</h2>
        <ol className="space-y-1.5 text-xs text-muted-foreground">
          {REFS.map((r, i) => (
            <li key={r.href} className="flex gap-2">
              <span className="tabular-nums">[{i + 1}]</span>
              <a
                href={r.href}
                target="_blank"
                rel="noreferrer"
                className="underline underline-offset-2 hover:text-foreground"
              >
                {r.label}
              </a>
            </li>
          ))}
        </ol>
      </section>

      <p className="max-w-3xl text-xs text-muted-foreground">{t.sourceNote}</p>
    </div>
  );
}
