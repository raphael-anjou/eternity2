import { pageMeta } from "@/seo";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { BoardSvg } from "@/components/board/BoardSvg";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { HERO_BOARD_CELLS } from "@/data/hero-board";

// The community fleet the CTA below points at: a live distributed search on the
// strict five-clue board, run by Benjamin Riotte (holder of the five-clue
// record). The site itself hosts nothing of it — these are the project's own
// endpoints — so they live here as plain constants, next to the internal page
// that documents the effort (/research/community/five-clue-fleet).
const FLEET_PAGE = "/research/community/five-clue-fleet";
const FLEET_DOWNLOAD = "https://eternity-control-plane-prod.eternity-cp.workers.dev/download";
const FLEET_DASHBOARD = "https://eternity-control-plane-prod.eternity-cp.workers.dev/dashboard";

// The hero shows a full, solved-looking 16×16 board. It used to be a live
// 8×8 solver looping forever, but that ran a requestAnimationFrame + engine
// step on every frame — main-thread work that hurt interaction responsiveness
// (INP) on the landing page for no real benefit. It's now a single static SVG
// of a pre-generated board (src/data/hero-board.ts): zero runtime cost, no
// engine, no repaint. The board is randomly generated, not the official puzzle
// — the caption says so.
function HeroBoard() {
  return <BoardSvg width={16} height={16} cells={HERO_BOARD_CELLS} className="w-full" />;
}

const T = {
  en: {
    heroTitleTop: "Eternity II:",
    heroTitleAccent: "the puzzle that beat everyone.",
    heroLede: (
      <>
        Eternity II looks like a children's jigsaw: 256 square pieces, match the colors. Launched
        in 2007 with a <strong>$2 million</strong> prize, it has{" "}
        <Link className="underline underline-offset-4 hover:text-foreground" to="/status">
          never been solved
        </Link>
        : not by people, not by supercomputers, and{" "}
        <Link className="underline underline-offset-4 hover:text-foreground" to="/is-it-a-scam">
          it is not a scam
        </Link>
        . This site lets you <em>feel</em> why, right in your browser.
      </>
    ),
    ctaStart: "Start here",
    ctaPlayground: "Play Eternity II online",
    ctaPuzzle: "What is Eternity II?",
    heroCaption:
      "A full 16×16 board, every edge matched and the border grey — this is what a solved Eternity II looks like. (It's a randomly generated board, not the official puzzle.)",
    ctaBadge: "Live right now",
    ctaTitle: "Lend a few CPU threads to the search",
    ctaText:
      "A community fleet is attacking the strict five-clue board right now, and it is short of machines. It has already pushed the five-clue line past 464, the best score this wiki has on record, and its dashboard shows what the fleet is doing this minute. The worker runs on Windows and Linux, uses only the threads you give it, and stops whenever you want.",
    ctaJoin: "Join the search",
    ctaDownload: "Download the worker",
    ctaFine: (
      <>
        Run by Benjamin Riotte, who holds the five-clue record. Follow the fleet on the{" "}
        <a
          className="underline underline-offset-4 hover:text-foreground"
          href={FLEET_DASHBOARD}
          target="_blank"
          rel="noreferrer"
        >
          live dashboard
        </a>
        . No account, no reward: just the puzzle.
      </>
    ),
    cards: [
      {
        to: "/puzzle",
        title: "The Puzzle",
        text: "256 pieces, 22 motifs, a $2 million prize nobody ever claimed. The full story and anatomy of Eternity II.",
      },
      {
        to: "/playground",
        title: "Playground",
        text: "Solve a small one yourself, watch a real solver run at a million steps per second, or design its search path.",
      },
      {
        to: "/algorithms",
        title: "Algorithms",
        text: "Backtracking, binary tricks and exponential walls: how computer scientists actually attack the puzzle, explained from scratch.",
      },
      {
        to: "/viewer",
        title: "Board Viewer",
        text: "Display and share any board. Fully compatible with e2.bucas.name links, including the famous 467/469/470 record boards.",
      },
      {
        to: "/research",
        title: "Research",
        text: "Papers, community records, solver internals, and an open lab notebook of attempts at the full 480.",
      },
    ],
    classroomTitle: "For classrooms",
    classroomText: (
      <>
        Built for{" "}
        <a className="underline" href="https://terra-numerica.org" target="_blank" rel="noreferrer">
          Terra Numerica
        </a>
        : a hands-on way to discover combinatorial explosion, algorithms and why "simple-looking"
        can mean "impossibly hard". No prerequisites needed.
      </>
    ),
  },
  fr: {
    heroTitleTop: "Eternity II :",
    heroTitleAccent: "le puzzle qui a battu tout le monde.",
    heroLede: (
      <>
        Eternity II a tout d'un puzzle pour enfants : 256 pièces carrées, et des couleurs à faire
        coïncider. Lancé en 2007, doté d'un prix de{" "}
        <strong>2 millions de dollars</strong>, il n'a pourtant{" "}
        <Link className="underline underline-offset-4 hover:text-foreground" to="/status">
          jamais été résolu
        </Link>{" "}
        : ni par l'homme, ni par les superordinateurs, et{" "}
        <Link className="underline underline-offset-4 hover:text-foreground" to="/is-it-a-scam">
          ce n'est pas une arnaque
        </Link>
        . Ce site vous fait <em>toucher du doigt</em> pourquoi, sans rien installer.
      </>
    ),
    ctaStart: "Par où commencer",
    ctaPlayground: "Jouer à Eternity II en ligne",
    ctaPuzzle: "C'est quoi, Eternity II ?",
    heroCaption:
      "Un plateau 16×16 complet, tous les côtés appariés et le bord gris : voilà à quoi ressemble un Eternity II résolu. (C'est un plateau généré au hasard, pas le puzzle officiel.)",
    ctaBadge: "En cours, maintenant",
    ctaTitle: "Prêtez quelques fils d'exécution à la recherche",
    ctaText:
      "Une flotte communautaire attaque en ce moment le plateau strict à cinq indices, et elle manque de machines. Elle a déjà poussé la ligne des cinq indices au-delà de 464, le meilleur score enregistré sur ce wiki, et son tableau de bord montre ce que fait la flotte à la minute près. Le client tourne sous Windows et Linux, n'utilise que les fils que vous lui donnez, et s'arrête quand vous voulez.",
    ctaJoin: "Rejoindre la recherche",
    ctaDownload: "Télécharger le client",
    ctaFine: (
      <>
        Menée par Benjamin Riotte, détenteur du record à cinq indices. Suivez la flotte sur le{" "}
        <a
          className="underline underline-offset-4 hover:text-foreground"
          href={FLEET_DASHBOARD}
          target="_blank"
          rel="noreferrer"
        >
          tableau de bord
        </a>
        . Sans compte ni récompense : juste le puzzle.
      </>
    ),
    cards: [
      {
        to: "/puzzle",
        title: "Le Puzzle",
        text: "256 pièces, 22 motifs et un prix de 2 millions de dollars jamais réclamé. Toute l'histoire et l'anatomie d'Eternity II.",
      },
      {
        to: "/playground",
        title: "Aire de jeu",
        text: "Résolvez un petit puzzle à la main, regardez un vrai solveur filer à un million d'étapes par seconde, ou tracez vous-même son parcours.",
      },
      {
        to: "/algorithms",
        title: "Algorithmes",
        text: "Retour en arrière (backtracking), astuces binaires et murs exponentiels : comment les informaticiens s'y prennent réellement face au puzzle. Tout est expliqué depuis le début.",
      },
      {
        to: "/viewer",
        title: "Visualiseur",
        text: "Affichez et partagez n'importe quel plateau. Pleinement compatible avec les liens e2.bucas.name, jusqu'aux célèbres plateaux records de 467, 469 et 470 côtés appariés.",
      },
      {
        to: "/research",
        title: "Recherche",
        text: "Articles scientifiques, records de la communauté, dessous techniques du solveur, et un carnet de laboratoire ouvert sur la quête des 480 côtés appariés.",
      },
    ],
    classroomTitle: "Pour les classes",
    classroomText: (
      <>
        Pensé pour{" "}
        <a className="underline" href="https://terra-numerica.org" target="_blank" rel="noreferrer">
          Terra Numerica
        </a>
        : une manière concrète d'aborder l'explosion combinatoire et les algorithmes, et de
        comprendre pourquoi un jeu « tout simple » peut se révéler « impossiblement difficile ».
        Aucun prérequis.
      </>
    ),
  },
  es: {
    heroTitleTop: "Eternity II:",
    heroTitleAccent: "el puzzle que venció a todos.",
    heroLede: (
      <>
        Eternity II parece un rompecabezas infantil: 256 piezas cuadradas y colores que hay que
        hacer coincidir. Lanzado en 2007 con un premio de <strong>2 millones de dólares</strong>,{" "}
        <Link className="underline underline-offset-4 hover:text-foreground" to="/status">
          nunca se ha resuelto
        </Link>
        : ni por personas, ni por superordenadores, y{" "}
        <Link className="underline underline-offset-4 hover:text-foreground" to="/is-it-a-scam">
          no es una estafa
        </Link>
        . Este sitio te permite <em>sentir</em> por qué, directamente en tu navegador.
      </>
    ),
    ctaStart: "Empieza aquí",
    ctaPlayground: "Jugar a Eternity II en línea",
    ctaPuzzle: "¿Qué es Eternity II?",
    heroCaption:
      "Un tablero 16×16 completo, con todas las aristas coincidentes y el borde gris: así se ve un Eternity II resuelto. (Es un tablero generado al azar, no el puzzle oficial.)",
    ctaBadge: "En marcha ahora",
    ctaTitle: "Presta unos hilos de CPU a la búsqueda",
    ctaText:
      "Una flota comunitaria está atacando ahora mismo el tablero estricto de cinco pistas, y le faltan máquinas. Ya ha llevado la línea de cinco pistas más allá de 464, la mejor puntuación registrada en este wiki, y su panel muestra lo que hace la flota al minuto. El cliente funciona en Windows y Linux, usa solo los hilos que le cedas y se para cuando quieras.",
    ctaJoin: "Unirse a la búsqueda",
    ctaDownload: "Descargar el cliente",
    ctaFine: (
      <>
        Dirigida por Benjamin Riotte, que tiene el récord de cinco pistas. Sigue la flota en el{" "}
        <a
          className="underline underline-offset-4 hover:text-foreground"
          href={FLEET_DASHBOARD}
          target="_blank"
          rel="noreferrer"
        >
          panel en vivo
        </a>
        . Sin cuenta ni recompensa: solo el puzzle.
      </>
    ),
    cards: [
      {
        to: "/puzzle",
        title: "El Puzzle",
        text: "256 piezas, 22 motivos y un premio de 2 millones de dólares que nadie reclamó jamás. Toda la historia y la anatomía de Eternity II.",
      },
      {
        to: "/playground",
        title: "Zona de juego",
        text: "Resuelve uno pequeño tú mismo, observa un solucionador real avanzar a un millón de pasos por segundo o diseña su ruta de búsqueda.",
      },
      {
        to: "/algorithms",
        title: "Algoritmos",
        text: "Backtracking, trucos binarios y muros exponenciales: cómo atacan realmente el puzzle los informáticos, explicado desde cero.",
      },
      {
        to: "/viewer",
        title: "Visor de tableros",
        text: "Muestra y comparte cualquier tablero. Totalmente compatible con los enlaces de e2.bucas.name, incluidos los famosos tableros récord de 467/469/470.",
      },
      {
        to: "/research",
        title: "Investigación",
        text: "Artículos científicos, récords de la comunidad, las entrañas de los solucionadores y un cuaderno de laboratorio abierto sobre los intentos de alcanzar las 480 completas.",
      },
    ],
    classroomTitle: "Para las aulas",
    classroomText: (
      <>
        Pensado para{" "}
        <a className="underline" href="https://terra-numerica.org" target="_blank" rel="noreferrer">
          Terra Numerica
        </a>
        : una forma práctica de descubrir la explosión combinatoria, los algoritmos y por qué algo
        «de apariencia sencilla» puede resultar «imposiblemente difícil». Sin conocimientos previos.
      </>
    ),
  },
};

export default function Home() {
  const t = useT(T);
  return (
    <div className="space-y-14">
      <section className="grid items-center gap-10 pt-6 md:grid-cols-2">
        <div className="space-y-5">
          <h1 className="text-4xl font-extrabold leading-tight tracking-tight md:text-5xl">
            {t.heroTitleTop}
            <br />
            <span className="bg-gradient-to-r from-pink-500 via-amber-500 to-sky-500 bg-clip-text text-transparent">
              {t.heroTitleAccent}
            </span>
          </h1>
          <p className="max-w-prose text-lg text-muted-foreground">{t.heroLede}</p>
          <div className="flex flex-wrap gap-3">
            <Button size="lg" render={<Link to="/start" />}>
              {t.ctaStart}
            </Button>
            <Button size="lg" variant="outline" render={<Link to="/playground" />}>
              {t.ctaPlayground}
            </Button>
            <Button size="lg" variant="outline" render={<Link to="/puzzle" />}>
              {t.ctaPuzzle}
            </Button>
          </div>
          <p className="text-sm text-muted-foreground">{t.heroCaption}</p>
        </div>
        <HeroBoard />
      </section>

      <section className="rounded-2xl border border-amber-500/40 bg-amber-500/5 p-6 md:p-8">
        <div className="flex flex-col gap-6 md:flex-row md:items-center md:justify-between">
          <div className="space-y-3">
            <Badge variant="outline" className="border-amber-500/50 text-amber-700 dark:text-amber-400">
              {t.ctaBadge}
            </Badge>
            <h2 className="text-2xl font-bold tracking-tight md:text-3xl">{t.ctaTitle}</h2>
            <p className="max-w-prose text-sm leading-relaxed text-muted-foreground">{t.ctaText}</p>
          </div>
          <div className="flex shrink-0 flex-wrap gap-3">
            <Button size="lg" render={<Link to={FLEET_PAGE} />}>
              {t.ctaJoin}
            </Button>
            <Button
              size="lg"
              variant="outline"
              render={<a href={FLEET_DOWNLOAD} target="_blank" rel="noreferrer" />}
            >
              {t.ctaDownload}
            </Button>
          </div>
        </div>
        <p className="mt-5 text-xs text-muted-foreground">{t.ctaFine}</p>
      </section>

      <section className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {t.cards.map((c) => (
          <Link key={c.to} to={c.to} className="group">
            <Card className="h-full transition-shadow group-hover:shadow-lg">
              <CardHeader>
                <CardTitle>{c.title}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{c.text}</CardContent>
            </Card>
          </Link>
        ))}
        <Card className="h-full border-dashed">
          <CardHeader>
            <CardTitle>{t.classroomTitle}</CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-muted-foreground">{t.classroomText}</CardContent>
        </Card>
      </section>
    </div>
  );
}

export const meta = pageMeta("home");
