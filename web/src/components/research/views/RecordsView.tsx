import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { KNOWN_BOARDS } from "@/data/known-boards";
import { RECORDS, type RecordRow } from "@/data/records-timeline";
import { RecordTimeline } from "@/components/research/RecordTimeline";

// Records we have a real, bundled board for (e2.bucas.name viewer data) get a
// clickable preview into our own /viewer. Keyed by the record row's `board`.
const BOARD_PARAMS: Record<string, string> = Object.fromEntries(
  KNOWN_BOARDS.map((b) => [b.id, b.params]),
);

// Distilled from our research vault's community history note, which is itself
// distilled from ~11,500 groups.io messages (2007–2026) + the Discord archive
// + 124 decoded community boards. The flag distinguishes boards on the
// official pieces under the contest's own rules (only the starter piece was
// mandatory — TOMY's entry form listed piece numbers only, see groups.io
// #11046) from boards on different or mixed piece sets. Several headline
// "480" boards are NOT the official puzzle. Note: none of the record boards
// from 468 up respect the four optional clue placements; boards that do are
// tracked as "strict 5-clue" in the method text (best known: 464, Benjamin
// Riotte 2026, beating Gauthier's 460 of 2023). Verified at board level
// against the archive (digest 0014) and the groups.io thread (topic 120056886).
const T = {
  en: {
    bestTitle: "The state of the art",
    best:
      "The community ceiling on the official puzzle is 470 of 480 matched edges — Joshua Blackwood, 2021, tied once since (Jef Bucas, December 2024). The contest's own rules pinned only the starter piece (the entry form listed piece numbers, not rotations), and every record board from 468 up is in that starter-only regime — including the 469s long quoted as the ceiling; the 470s are the same puzzle, not an easier variant. Boards that also respect the four optional clue placements are tracked separately: the best known is 464 (Benjamin Riotte, July 2026), which finally beat Bruno Gauthier's 460 after more than three years. The full solution (480) has never been found; the 10-edge gap on the open record has stood since 2021.",
    timelineTitle: "Record timeline",
    cols: { date: "Date", score: "Score", author: "Author", puzzle: "Puzzle", method: "Method", source: "Source", preview: "Preview" },
    canonical: "official pieces",
    variant: "other set",
    view: "View board",
    timelineNote:
      "Mailing-list sources link the exact announcement message in the eternity2 groups.io archive; reading them requires a free groups.io account. Entries without a link were only reported in places with no public archive (e.g. Discord).",
    falsePositiveTitle: "Why some “480” boards don't count",
    falsePositive:
      "The real puzzle is the board TOMY sold: 256 specific pieces, a 16×16 frame, 22 colors, the starter piece pinned at I8 (the four other clues were optional aids under the contest rules). Several boards posted as 480/480 use different piece sets — Brendan Owen's smaller Clue-1 / Clue-2 designs, the unframed TopCoder variant, or boards mixing pieces from multiple expansion sets. They have solutions; those solutions do not transfer to the official puzzle. The official 480 remains unfound.",
    methodsTitle: "The key algorithms",
    methods: [
      [
        "Selby–Riordan generator (2007)",
        "The puzzle was generated to have about one expected solution. Brendan Owen and Günter Stertenbrink derived in 2007 that ~17 interior colors and ~5 border colors put it exactly at the unique-solution boundary — later confirmed as the SAT/CSP phase-transition peak. The hardness is by design.",
      ],
      [
        "Verhaard set-annealing (2008)",
        "Louis Verhaard's 467 annealed on the composition of piece sets between regions, not just on positions — an early metaheuristic that held the record for over a decade.",
      ],
      [
        "Blackwood heuristic-schedule + break-index (2020)",
        "The dominant modern algorithm behind every 469/470 board. A scan-row backtracker driven by a piecewise-linear color-exhaustion schedule plus a small set of allowed 'break' depths, run for tens of billions of iterations before restart. Blackwood reported that SAT solvers, GPUs and pre-solved 2×2 caches did not help.",
      ],
      [
        "McGavin's engine (2020–2026)",
        "A C backtracker hitting ~295 million tile placements per second on one core — roughly 4× a typical community engine — via PGO, aggressive compiler flags and auto-generated per-cell unrolled code. Raw speed plus the right scan order (bottom-left start, left-to-right rows) is half the story of the 469.",
      ],
      [
        "Joe's prune-back policy (2026)",
        "A recent, concrete idea: almost all backtracking time is spent very deep, so if a search spends too long below a depth threshold without progress, prune back to that depth and restart. Measured 17–49% fewer iterations depending on clue count.",
      ],
    ],
    castTitle: "Cast of contributors",
    castNote: "A non-exhaustive who's-who of the people behind the records and the infrastructure.",
    cast: [
      ["Peter McGavin", "The 469 (2020, Blackwood's solver on ~200 cores); his own auto-generated C engines and the complex-theory implementations — the community's throughput and theory anchor."],
      ["Joshua Blackwood", "The 469/470-class algorithm (heuristic schedule + break-index); the modern standard."],
      ["Jef Bucas", "libblackwood (a C port of Blackwood's solver) and bucas.name — the board viewer + URL format the community shares boards in."],
      ["Louis Verhaard", "The 467 record (2008) and set-composition swap-annealing."],
      ["Brendan Owen & Günter Stertenbrink", "The 2007 hardness derivation: why 17/5 colors sit at the unique-solution boundary."],
      ["Marijn Heule", "The canonical SAT encoding (2008) and ongoing SAT experiments."],
      ["Al Hopfer", "The NS-1 deficit invariant (2022) — the border↔interior multiset-equality condition; Carlos Fernandez showed 14×14 quadrants then solve in minutes."],
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
      "Le plafond communautaire sur le puzzle officiel est de 470 bords appariés sur 480 — Joshua Blackwood, 2021, égalé une fois depuis (Jef Bucas, décembre 2024). Le règlement du concours n'épinglait que la pièce de départ (le formulaire ne listait que des numéros de pièces, pas des rotations), et tous les plateaux records à partir de 468 relèvent de ce régime « pièce de départ seule » — y compris les 469 longtemps cités comme plafond ; les 470 sont le même puzzle, pas une variante plus facile. Les plateaux qui respectent aussi les quatre indices facultatifs sont suivis à part : le meilleur connu est 464 (Benjamin Riotte, juillet 2026), qui a enfin battu le 460 de Bruno Gauthier après plus de trois ans. La solution complète (480) reste introuvée ; l'écart de 10 arêtes sur le record ouvert tient depuis 2021.",
    timelineTitle: "Chronologie des records",
    cols: { date: "Date", score: "Score", author: "Auteur", puzzle: "Puzzle", method: "Méthode", source: "Source", preview: "Aperçu" },
    canonical: "pièces officielles",
    variant: "autre jeu",
    view: "Voir le plateau",
    timelineNote:
      "Les sources « groups.io » pointent vers le message d'annonce exact dans les archives de la liste eternity2 ; leur lecture demande un compte groups.io gratuit. Les entrées sans lien n'ont été rapportées que dans des espaces sans archive publique (Discord, par exemple).",
    falsePositiveTitle: "Pourquoi certains plateaux « 480 » ne comptent pas",
    falsePositive:
      "Le vrai puzzle est le plateau vendu par TOMY : 256 pièces précises, un cadre 16×16, 22 couleurs, la pièce de départ épinglée en I8 (les quatre autres indices n'étaient que des aides facultatives selon le règlement). Plusieurs plateaux publiés en 480/480 utilisent d'autres jeux de pièces — les designs Clue-1 / Clue-2 plus petits de Brendan Owen, la variante non encadrée de TopCoder, ou des plateaux mélangeant des pièces de plusieurs jeux d'extension. Ils ont des solutions ; celles-ci ne se transposent pas au puzzle officiel. Le 480 officiel reste introuvé.",
    methodsTitle: "Les algorithmes clés",
    methods: [
      [
        "Générateur Selby–Riordan (2007)",
        "Le puzzle a été généré pour n'avoir qu'environ une solution attendue. Brendan Owen et Günter Stertenbrink ont dérivé en 2007 que ≈17 couleurs intérieures et ≈5 de bord le placent pile à la frontière d'unicité — confirmé plus tard comme le pic de transition de phase SAT/CSP. La difficulté est voulue.",
      ],
      [
        "Recuit sur jeux de Verhaard (2008)",
        "Le 467 de Louis Verhaard recuit sur la composition des jeux de pièces entre régions, pas seulement sur les positions — une métaheuristique précoce qui a tenu le record plus d'une décennie.",
      ],
      [
        "Schedule heuristique + break-index de Blackwood (2020)",
        "L'algorithme moderne dominant derrière chaque plateau 469/470. Un backtracker en balayage de lignes piloté par un objectif d'épuisement des couleurs linéaire par morceaux plus un petit ensemble de profondeurs de « rupture » autorisées, lancé sur des dizaines de milliards d'itérations avant redémarrage. Blackwood a rapporté que les solveurs SAT, les GPU et les caches de 2×2 pré-résolus n'aidaient pas.",
      ],
      [
        "Le moteur de McGavin (2020–2026)",
        "Un backtracker C atteignant ≈295 millions de placements de tuiles par seconde sur un cœur — environ 4× un moteur communautaire typique — via PGO, options de compilation agressives et code déroulé par case auto-généré. La vitesse brute plus le bon ordre de balayage (départ bas-gauche, lignes de gauche à droite) est la moitié de l'histoire du 469.",
      ],
      [
        "La politique de prune-back de Joe (2026)",
        "Une idée récente et concrète : presque tout le temps de retour sur trace se passe très en profondeur ; donc si une recherche passe trop de temps sous un seuil de profondeur sans progrès, on élague jusqu'à ce seuil et on redémarre. Mesuré : 17 à 49 % d'itérations en moins selon le nombre d'indices.",
      ],
    ],
    castTitle: "Les contributeurs",
    castNote: "Un who's-who non exhaustif des personnes derrière les records et l'infrastructure.",
    cast: [
      ["Peter McGavin", "Le 469 (2020, solveur de Blackwood sur ~200 cœurs) ; ses moteurs C auto-générés et ses implémentations de la théorie complexe — l'ancre débit et théorie de la communauté."],
      ["Joshua Blackwood", "L'algorithme de classe 469/470 (schedule heuristique + break-index) ; le standard moderne."],
      ["Jef Bucas", "libblackwood (un port C du solveur de Blackwood) et bucas.name — le visualiseur et le format d'URL avec lequel la communauté partage les plateaux."],
      ["Louis Verhaard", "Le record 467 (2008) et le recuit par composition de jeux."],
      ["Brendan Owen & Günter Stertenbrink", "La dérivation de difficulté de 2007 : pourquoi 17/5 couleurs sont à la frontière d'unicité."],
      ["Marijn Heule", "L'encodage SAT canonique (2008) et des expériences SAT continues."],
      ["Al Hopfer", "L'invariant de déficit NS-1 (2022) — la condition d'égalité des multisets bord↔intérieur ; Carlos Fernandez a montré que les quadrants 14×14 se résolvent alors en minutes."],
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
      "El techo de la comunidad sobre el puzzle oficial es de 470 aristas coincidentes de 480 — Joshua Blackwood, 2021, igualado una sola vez desde entonces (Jef Bucas, diciembre de 2024). El reglamento del concurso solo fijaba la pieza de partida (el formulario listaba números de pieza, no rotaciones), y todos los tableros récord a partir de 468 pertenecen a ese régimen de «solo pieza de partida» —incluidos los 469 citados durante mucho tiempo como techo; los 470 son el mismo puzzle, no una variante más fácil. Los tableros que además respetan las cuatro pistas opcionales se registran aparte: el mejor conocido es 464 (Benjamin Riotte, julio de 2026), que por fin batió el 460 de Bruno Gauthier tras más de tres años. La solución completa (480) nunca se ha encontrado; la diferencia de 10 aristas en el récord abierto se mantiene desde 2021.",
    timelineTitle: "Cronología de récords",
    cols: { date: "Fecha", score: "Puntuación", author: "Autor", puzzle: "Puzzle", method: "Método", source: "Fuente", preview: "Vista previa" },
    canonical: "piezas oficiales",
    variant: "otro conjunto",
    view: "Ver tablero",
    timelineNote:
      "Las fuentes de la lista de correo enlazan al mensaje exacto de anuncio en el archivo de eternity2 en groups.io; leerlas requiere una cuenta gratuita de groups.io. Las entradas sin enlace solo se comunicaron en espacios sin archivo público (por ejemplo, Discord).",
    falsePositiveTitle: "Por qué algunos tableros «480» no cuentan",
    falsePositive:
      "El puzzle real es el tablero que vendió TOMY: 256 piezas concretas, un marco de 16×16, 22 colores, la pieza de partida fijada en I8 (las otras cuatro pistas eran ayudas opcionales según el reglamento). Varios tableros publicados como 480/480 usan otros conjuntos de piezas —los diseños Clue-1 / Clue-2 más pequeños de Brendan Owen, la variante sin marco de TopCoder, o tableros que mezclan piezas de varios conjuntos de expansión—. Tienen solución, pero esa solución no se traslada al puzzle oficial. El 480 oficial sigue sin encontrarse.",
    methodsTitle: "Los algoritmos clave",
    methods: [
      [
        "Generador de Selby–Riordan (2007)",
        "El puzzle se generó para tener alrededor de una única solución esperada. Brendan Owen y Günter Stertenbrink dedujeron en 2007 que ≈17 colores interiores y ≈5 de borde lo sitúan justo en la frontera de solución única —confirmado después como el pico de transición de fase SAT/CSP—. La dificultad es intencionada.",
      ],
      [
        "Recocido sobre conjuntos de Verhaard (2008)",
        "El 467 de Louis Verhaard aplicaba recocido a la composición de los conjuntos de piezas entre regiones, no solo a las posiciones —una metaheurística temprana que mantuvo el récord durante más de una década.",
      ],
      [
        "Programación heurística + índice de ruptura de Blackwood (2020)",
        "El algoritmo moderno dominante detrás de cada tablero 469/470. Un backtracker por barrido de filas guiado por un objetivo de agotamiento de colores lineal por tramos más un pequeño conjunto de profundidades de «ruptura» permitidas, ejecutado durante decenas de miles de millones de iteraciones antes de reiniciar. Blackwood informó de que los solucionadores SAT, las GPU y las cachés de 2×2 preresueltas no ayudaban.",
      ],
      [
        "El motor de McGavin (2020–2026)",
        "Un backtracker en C que alcanza ≈295 millones de colocaciones de piezas por segundo en un solo núcleo —unas 4× más que un motor comunitario típico— mediante PGO, opciones de compilación agresivas y código desenrollado por celda generado automáticamente. La velocidad bruta más el orden de recorrido adecuado (inicio abajo a la izquierda, filas de izquierda a derecha) es la mitad de la historia del 469.",
      ],
      [
        "La política de poda-atrás de Joe (2026)",
        "Una idea reciente y concreta: casi todo el tiempo de backtracking se pasa a mucha profundidad, así que si una búsqueda tarda demasiado por debajo de un umbral de profundidad sin avanzar, se poda hasta ese umbral y se reinicia. Se midió un 17–49 % menos de iteraciones según el número de pistas.",
      ],
    ],
    castTitle: "Elenco de colaboradores",
    castNote: "Un quién es quién no exhaustivo de las personas detrás de los récords y la infraestructura.",
    cast: [
      ["Peter McGavin", "El 469 (2020, con el solucionador de Blackwood sobre ~200 núcleos); sus propios motores en C generados automáticamente y las implementaciones de la teoría compleja —el ancla de rendimiento y teoría de la comunidad."],
      ["Joshua Blackwood", "El algoritmo de clase 469/470 (programación heurística + índice de ruptura); el estándar moderno."],
      ["Jef Bucas", "libblackwood (un port en C del solucionador de Blackwood) y bucas.name —el visor de tableros y el formato de URL con el que la comunidad comparte tableros."],
      ["Louis Verhaard", "El récord 467 (2008) y el recocido por intercambio de composición de conjuntos."],
      ["Brendan Owen & Günter Stertenbrink", "La deducción de la dificultad de 2007: por qué 17/5 colores se sitúan en la frontera de solución única."],
      ["Marijn Heule", "La codificación SAT canónica (2008) y experimentos SAT en curso."],
      ["Al Hopfer", "El invariante de déficit NS-1 (2022) —la condición de igualdad de multiconjuntos borde↔interior; Carlos Fernandez demostró que los cuadrantes 14×14 se resuelven entonces en minutos."],
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
// (468/469/470) are not in any peer-reviewed or encyclopedic source — they live
// in the mailing list and on the board viewer — so we point at the primary
// community sources directly and flag what mainstream sources do and don't say.
// Exact community sources. The 468/469/470/460/464 boards are not in any
// peer-reviewed or encyclopedic source — they were announced on the mailing
// list — so we link the specific groups.io messages, using the archive's
// sequential message numbers (msg_num; author, date and subject all verified
// against our full message export). Reading a message needs a free groups.io
// login — anonymous access to the archive is login-walled.
const GROUPS_IO = "https://groups.io/g/eternity2";
const REFS: { href: string; label: string }[] = [
  {
    href: `${GROUPS_IO}/message/10033`,
    label:
      "468 — Jef Bucas, “New best solution found (12 breaks)”, eternity2@groups.io, 2020-08-31 (Blackwood's board, relayed to the group).",
  },
  {
    href: `${GROUPS_IO}/message/10045`,
    label:
      "469 — Peter McGavin, “EternityII Solver” thread, eternity2@groups.io, 2020-09-09: “New record score of 469! Only 11 breaks!” — the ceiling until the 470.",
  },
  {
    href: `${GROUPS_IO}/message/10117`,
    label:
      "470 — Joshua Blackwood, “470 Found”, eternity2@groups.io, 2021-03-30 — same starter-only regime as the 468/469 (board-level verification: groups.io #10554); the standing record.",
  },
  {
    href: `${GROUPS_IO}/message/11074`,
    label:
      "460 — Bruno Gauthier, “Highest points (of 480) with using all 5 (!) hints?” thread, eternity2@groups.io, 2023-03-09: the best board respecting all five clues for over three years (Eternity II Editor).",
  },
  {
    href: `${GROUPS_IO}/message/11919`,
    label:
      "464 — Benjamin Riotte, “Record of Eternity2 with 5 hints ?” thread, eternity2@groups.io, 2026-07-06: the new strict-five-clue record (16 broken edges), all five clues at their official cells, found with his own modified-Blackwood solver; Igor Pejic reached the same 463–464 range independently in the thread.",
  },
  {
    href: "https://e2.bucas.name",
    label:
      "e2.bucas.name (Jef Bucas) — the board viewer and shared URL format; every board above can be loaded and re-scored there.",
  },
  {
    href: "https://en.wikipedia.org/wiki/Eternity_II_puzzle",
    label:
      "Wikipedia, “Eternity II puzzle” — documents only the encyclopedic facts: the $2M prize, the deadline of noon 31 Dec 2010, and Verhaard's 467 ($10,000). The 468+ records remain community-reported, not yet in a source Wikipedia accepts.",
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

export function RecordsView() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <section className="max-w-3xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.bestTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">
          {t.best}
          <Cite n={2} />
          <Cite n={6} />
        </p>
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
                    <span className="text-sm text-muted-foreground">—</span>
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
                    <span className="text-sm text-muted-foreground">—</span>
                  )}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        <p className="max-w-3xl text-xs text-muted-foreground">{t.timelineNote}</p>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.falsePositiveTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">
          {t.falsePositive}
          <Cite n={2} />
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
