import { pageMeta } from "@/seo";
import { useState } from "react";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";

// The "Start here" front door. The site holds a great deal of content across
// several sections; the hard part is not the content, it is routing a visitor
// to the part pitched at them, and surfacing adjacent things they would never
// have searched for. This page asks one question ("who are you here as?") and
// hands each answer a short curated trail through pages that already exist.

interface Step {
  to: string;
  title: string;
  why: string;
}
interface Profile {
  key: string;
  emoji: string;
  who: string;
  tagline: string;
  trail: Step[];
  // A single "you might not have thought to look at this" pointer.
  surprise: Step;
}

const T = {
  en: {
    title: "Where would you like to start?",
    intro:
      "This site has a lot in it: a playground, a board viewer, and a deep research wiki. Tell us who you are here as, and we will hand you a short path through the parts pitched at you. You can always wander off it.",
    bigQuestions: "The two questions almost everyone starts with:",
    q1: { to: "/puzzle", title: "What is Eternity II?", why: "The puzzle, the prize, and the whole story." },
    q2: { to: "/status", title: "Has it been solved?", why: "The straight answer, and the best score anyone has reached." },
    pick: "Or pick the profile that fits you best. Nothing is locked; every path is just a set of links.",
    trailLabel: "Your path",
    surpriseLabel: "You might not have thought to look at",
    profiles: [
      {
        key: "curious",
        emoji: "🤔",
        who: "Just curious",
        tagline: "You heard about the $2M puzzle nobody solved and want the story.",
        trail: [
          { to: "/puzzle", title: "What is Eternity II?", why: "The whole story, from launch to the unclaimed prize." },
          { to: "/status", title: "Has it been solved?", why: "The straight answer, and the best anyone has ever done." },
          { to: "/is-it-a-scam", title: "Is it a scam?", why: "Why it is unsolved but almost certainly not impossible." },
          { to: "/playground/solve", title: "Try a small one", why: "Feel the puzzle in your hands, sized for a human." },
        ],
        surprise: {
          to: "/research/why/rare-color-geography",
          title: "The rare colours hide on the frame",
          why: "A small, strange fact about how the puzzle is built.",
        },
      },
      {
        key: "teacher",
        emoji: "🧒",
        who: "A kid, or a teacher",
        tagline: "You want to show combinatorial explosion, hands-on, no code.",
        trail: [
          { to: "/playground/solve", title: "Play a level", why: "Start at 3×3 and climb; the last level is the real 16×16 nobody wins." },
          { to: "/playground/print", title: "Print and cut", why: "Make real paper puzzles for the table." },
          { to: "/algorithms", title: "Why simple looks hard", why: "Backtracking and exponential growth, explained from scratch." },
          { to: "/playground/watch", title: "Watch the machine think", why: "A real search placing and undoing pieces, live." },
        ],
        surprise: {
          to: "/status",
          title: "Nobody has ever finished the real one",
          why: "The record has stood since 2021. Ten edges from the end.",
        },
      },
      {
        key: "hobbyist",
        emoji: "🧩",
        who: "A solver, tinkering",
        tagline: "You want the record, the pieces, and which method to try.",
        trail: [
          { to: "/status", title: "The current record", why: "470/480, who holds it, and what counts as the real puzzle." },
          { to: "/research/build/tooling", title: "Pieces, clues and formats", why: "The e2pieces.txt and clue conventions, and why they drift." },
          { to: "/convert", title: "Convert a board", why: "Turn a bucas link into edge and piece lists, and back." },
          { to: "/research/build/concepts/approaches-map", title: "Every known approach", why: "The map of methods, what each reached, where each walls out." },
        ],
        surprise: {
          to: "/research/why/rigidity-wall",
          title: "Why everyone stalls at the same score",
          why: "The wall that stops fast and clever solvers alike.",
        },
      },
      {
        key: "student",
        emoji: "🎓",
        who: "A CS student",
        tagline: "You have an assignment, or you want the complexity angle.",
        trail: [
          { to: "/research/why/how-hard-is-this-instance", title: "Is this instance NP-complete?", why: "The category error, and how to actually encode it (SAT, DLX, ILP)." },
          { to: "/algorithms", title: "Backtracking, measured", why: "Real timings from the site's own engine as size grows." },
          { to: "/research/build/concepts", title: "The technique shelf", why: "Each method with what it costs and what it bought on this puzzle." },
          { to: "/research/build/run-it-yourself", title: "Run a solver yourself", why: "Get your hands on a working solver, not just the theory." },
        ],
        surprise: {
          to: "/research/why/complex-theory",
          title: "Counting the search before you run it",
          why: "Estimating the tree size without touching a solver.",
        },
      },
      {
        key: "researcher",
        emoji: "🔬",
        who: "A researcher",
        tagline: "You want the frontier: what is known, and what is open.",
        trail: [
          { to: "/research/build/concepts/approaches-map", title: "The survey nobody wrote", why: "All approaches in one map, with where each hits its ceiling." },
          { to: "/research/why", title: "Why it is hard", why: "The structural results: walls, phase transition, forbidden patterns." },
          { to: "/research/lab/findings", title: "Open findings", why: "The lab notebook: what has been measured, what is still loose." },
          { to: "/research/papers", title: "The literature", why: "The academic reading list, ranked by use if you are writing a solver." },
        ],
        surprise: {
          to: "/research/why/sigma-cycles",
          title: "Why basin-hopping is impossible",
          why: "A structural reason a whole class of local search cannot work.",
        },
      },
    ] as Profile[],
  },
  fr: {
    title: "Par où voulez-vous commencer ?",
    intro:
      "Ce site contient beaucoup de choses : une aire de jeu, un visualiseur de plateaux et un wiki de recherche fourni. Dites-nous à quel titre vous êtes là, et on vous tend un court parcours dans les parties faites pour vous. Libre à vous d'en sortir.",
    bigQuestions: "Les deux questions que presque tout le monde se pose d'abord :",
    q1: { to: "/puzzle", title: "C'est quoi, Eternity II ?", why: "Le puzzle, le prix, et toute l'histoire." },
    q2: { to: "/status", title: "A-t-il été résolu ?", why: "La réponse franche, et le meilleur score jamais atteint." },
    pick: "Ou choisissez le profil qui vous ressemble le plus. Rien n'est verrouillé ; chaque parcours n'est qu'une série de liens.",
    trailLabel: "Votre parcours",
    surpriseLabel: "Vous n'auriez peut-être pas pensé à regarder",
    profiles: [
      {
        key: "curious",
        emoji: "🤔",
        who: "Simple curiosité",
        tagline: "Vous avez entendu parler du puzzle à 2 M$ que personne n'a résolu.",
        trail: [
          { to: "/puzzle", title: "C'est quoi, Eternity II ?", why: "Toute l'histoire, du lancement au prix jamais réclamé." },
          { to: "/status", title: "A-t-il été résolu ?", why: "La réponse franche, et le meilleur score jamais atteint." },
          { to: "/is-it-a-scam", title: "Est-ce une arnaque ?", why: "Pourquoi il reste non résolu sans être pour autant impossible." },
          { to: "/playground/solve", title: "Essayez-en un petit", why: "Sentez le puzzle sous vos doigts, à taille humaine." },
        ],
        surprise: {
          to: "/research/why/rare-color-geography",
          title: "Les couleurs rares se cachent sur le cadre",
          why: "Un petit fait étrange sur la façon dont le puzzle est construit.",
        },
      },
      {
        key: "teacher",
        emoji: "🧒",
        who: "Un enfant, ou un enseignant",
        tagline: "Vous voulez montrer l'explosion combinatoire, sans code.",
        trail: [
          { to: "/playground/solve", title: "Jouez un niveau", why: "Commencez en 3×3 et grimpez ; le dernier niveau est le vrai 16×16 que personne ne gagne." },
          { to: "/playground/print", title: "Imprimez et découpez", why: "Fabriquez de vrais puzzles en papier à poser sur la table." },
          { to: "/algorithms", title: "Pourquoi simple rime avec difficile", why: "Le retour arrière et la croissance exponentielle, depuis le début." },
          { to: "/playground/watch", title: "Regardez la machine réfléchir", why: "Une vraie recherche qui pose et retire des pièces, en direct." },
        ],
        surprise: {
          to: "/status",
          title: "Personne n'a jamais bouclé le vrai",
          why: "Le record tient depuis 2021. À dix côtés de la fin.",
        },
      },
      {
        key: "hobbyist",
        emoji: "🧩",
        who: "Un joueur qui bricole",
        tagline: "Vous voulez le record, les pièces, et quelle méthode tenter.",
        trail: [
          { to: "/status", title: "Le record actuel", why: "470/480, qui le détient, et ce qui compte comme le vrai puzzle." },
          { to: "/research/build/tooling", title: "Pièces, indices et formats", why: "Les conventions e2pieces.txt et indices, et pourquoi elles dérivent." },
          { to: "/convert", title: "Convertir un plateau", why: "D'un lien bucas vers des listes de côtés et de pièces, et l'inverse." },
          { to: "/research/build/concepts/approaches-map", title: "Toutes les approches connues", why: "La carte des méthodes, ce que chacune a atteint, où chacune bute." },
        ],
        surprise: {
          to: "/research/why/rigidity-wall",
          title: "Pourquoi tout le monde cale au même score",
          why: "Le mur qui arrête aussi bien les solveurs rapides que malins.",
        },
      },
      {
        key: "student",
        emoji: "🎓",
        who: "Un étudiant en info",
        tagline: "Vous avez un projet, ou vous voulez l'angle complexité.",
        trail: [
          { to: "/research/why/how-hard-is-this-instance", title: "Cette instance est-elle NP-complète ?", why: "L'erreur de raisonnement, et comment vraiment l'encoder (SAT, DLX, PLNE)." },
          { to: "/algorithms", title: "Le retour arrière, mesuré", why: "De vrais temps du moteur du site à mesure que la taille grandit." },
          { to: "/research/build/concepts", title: "L'étagère des techniques", why: "Chaque méthode, son coût, et ce qu'elle a rapporté sur ce puzzle." },
          { to: "/research/build/run-it-yourself", title: "Lancez un solveur vous-même", why: "Mettez la main sur un solveur qui tourne, pas juste la théorie." },
        ],
        surprise: {
          to: "/research/why/complex-theory",
          title: "Compter la recherche avant de la lancer",
          why: "Estimer la taille de l'arbre sans toucher à un solveur.",
        },
      },
      {
        key: "researcher",
        emoji: "🔬",
        who: "Un chercheur",
        tagline: "Vous voulez la frontière : ce qui est su, et ce qui reste ouvert.",
        trail: [
          { to: "/research/build/concepts/approaches-map", title: "La synthèse que personne n'a écrite", why: "Toutes les approches sur une carte, avec le plafond de chacune." },
          { to: "/research/why", title: "Pourquoi c'est difficile", why: "Les résultats de structure : murs, transition de phase, motifs interdits." },
          { to: "/research/lab/findings", title: "Découvertes ouvertes", why: "Le carnet du labo : ce qui est mesuré, ce qui reste flou." },
          { to: "/research/papers", title: "La littérature", why: "La liste de lecture, classée par utilité si vous écrivez un solveur." },
        ],
        surprise: {
          to: "/research/why/sigma-cycles",
          title: "Pourquoi le saut de bassin est impossible",
          why: "Une raison de structure qui condamne toute une classe de recherche locale.",
        },
      },
    ] as Profile[],
  },
};

export default function Start() {
  const t = useT(T);
  const [open, setOpen] = useState<string | null>(null);
  const active = t.profiles.find((p) => p.key === open) ?? null;

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-2xl text-muted-foreground">{t.intro}</p>
      </div>

      <div>
        <p className="mb-3 text-sm font-medium">{t.bigQuestions}</p>
        <div className="grid gap-4 sm:grid-cols-2">
          {[t.q1, t.q2].map((q) => (
            <Link key={q.to} to={q.to} className="group">
              <Card className="h-full transition-shadow group-hover:shadow-lg">
                <CardHeader>
                  <CardTitle className="text-lg underline-offset-2 group-hover:underline">
                    {q.title}
                  </CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">{q.why}</CardContent>
              </Card>
            </Link>
          ))}
        </div>
      </div>

      <p className="text-sm text-muted-foreground">{t.pick}</p>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {t.profiles.map((p) => {
          const isOpen = p.key === open;
          return (
            <button
              key={p.key}
              type="button"
              onClick={() => setOpen(isOpen ? null : p.key)}
              aria-pressed={isOpen}
              className="text-left"
            >
              <Card
                className={
                  "h-full transition-shadow hover:shadow-lg " +
                  (isOpen ? "ring-2 ring-primary" : "")
                }
              >
                <CardHeader>
                  <div className="text-4xl" aria-hidden="true">
                    {p.emoji}
                  </div>
                  <CardTitle>{p.who}</CardTitle>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">{p.tagline}</CardContent>
              </Card>
            </button>
          );
        })}
      </div>

      {active && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <span aria-hidden="true">{active.emoji}</span>
              {t.trailLabel}
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-1">
            <ol className="space-y-3">
              {active.trail.map((s, i) => (
                <li key={s.to} className="flex gap-3">
                  <span className="mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-muted font-mono text-xs">
                    {i + 1}
                  </span>
                  <div>
                    <Link to={s.to} className="font-medium underline underline-offset-2">
                      {s.title}
                    </Link>
                    <p className="text-sm text-muted-foreground">{s.why}</p>
                  </div>
                </li>
              ))}
            </ol>

            <div className="mt-5 rounded-lg border border-dashed p-4">
              <p className="text-xs font-medium uppercase tracking-wide text-muted-foreground">
                {t.surpriseLabel}
              </p>
              <Link
                to={active.surprise.to}
                className="mt-1 block font-medium underline underline-offset-2"
              >
                {active.surprise.title}
              </Link>
              <p className="text-sm text-muted-foreground">{active.surprise.why}</p>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

export const meta = pageMeta("start");
