import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// Door 1 of the research section: the science of *why* Eternity II resists every
// approach. It carries the "engineered to resist cleverness" framing (the
// design story) and indexes the deeper structural results. The individual
// finding articles are published one by one through the research/topics pipeline;
// until each lands here it is shown as an upcoming card (no dead links).
type Topic = { key: string; ready: boolean; to?: string };

const TOPICS: Topic[] = [
  { key: "phaseTransition", ready: true, to: "/research/why/phase-transition" },
  { key: "rigidity", ready: false },
  { key: "entropy", ready: false },
  { key: "forbidden", ready: true, to: "/research/why/forbidden-patterns" },
];

const T = {
  en: {
    title: "Why it's hard",
    intro:
      "Eternity II is not accidentally difficult. This is the science of why no search, however clever, has reached the end — the puzzle's design, and the structural walls that show up once you start measuring.",
    backLabel: "← Research",
    engineeredTitle: "Engineered to resist cleverness",
    engineeredBody:
      "Eternity I fell in 2000 because Alex Selby and Oliver Riordan discovered the puzzle had vastly more solutions than its designer believed, and aimed their search at the most \"solution-dense\" regions. For Eternity II, the publisher hired the winners: Selby and Riordan helped design and stress-test the new puzzle so that no such statistical shortcut survives. The visible fingerprints of that vetting: a single designed solution baked into balanced color counts, no rotationally-symmetric pieces, no duplicate pieces, rare motifs confined to the frame, and piece-count/color-count parameters sitting at the empirical hardness peak (later confirmed by Ansótegui et al.). The puzzle isn't accidentally hard. It was tuned to be.",
    deeperTitle: "The structural walls",
    deeperIntro:
      "Beyond the design story, the puzzle has measurable structure that explains the gap between the best known board (469/480) and a full solution. Each of these is published with the exact computation behind it.",
    soon: "In preparation",
    topics: {
      phaseTransition: {
        title: "The phase-transition argument",
        body: "Why ≈17 interior colors and 5 border colors put the puzzle exactly on the SAT/CSP hardness peak — roughly one expected solution, the worst possible place to search.",
      },
      rigidity: {
        title: "The rigidity wall",
        body: "Every known top board is locally frozen: no small rearrangement improves it. You cannot nudge your way from a record board to a solution — the good boards are isolated.",
      },
      entropy: {
        title: "Entropy and the area law",
        body: "How the count of genuinely-distinct partial boards collapses past roughly eighty cells — a universal wall that no local scoring signal can see through.",
      },
      forbidden: {
        title: "Forbidden patterns",
        body: "Almost every small patch of colors you could draw is illegal under the piece set; a real solution contains none of the forbidden ones. A sharp combinatorial fingerprint of validity.",
      },
    },
  },
  fr: {
    title: "Pourquoi c'est dur",
    intro:
      "Eternity II n'est pas difficile par hasard. Voici la science de cette difficulté : la conception du puzzle, et les murs structurels qui apparaissent dès qu'on se met à mesurer — et qui expliquent pourquoi aucune recherche, si ingénieuse soit-elle, n'est arrivée au bout.",
    backLabel: "← Recherche",
    engineeredTitle: "Conçu pour déjouer l'ingéniosité",
    engineeredBody:
      "Eternity I est tombé en 2000 parce qu'Alex Selby et Oliver Riordan ont compris que le puzzle admettait infiniment plus de solutions que ne le croyait son créateur, et qu'ils ont orienté leur recherche vers les régions les plus « riches en solutions ». Pour Eternity II, l'éditeur a recruté les gagnants : Selby et Riordan ont participé à la conception du nouveau puzzle et l'ont mis à l'épreuve pour qu'aucun raccourci statistique de ce type ne tienne. Les traces visibles de ce travail de blindage : une solution unique pensée dès le départ et noyée dans des répartitions de couleurs équilibrées, aucune pièce invariante par rotation, aucune pièce en double, des motifs rares confinés au cadre, et un nombre de pièces et de couleurs calé pile sur le pic de difficulté observé (ce qu'Ansótegui et al. confirmeront plus tard). Ce puzzle n'est pas difficile par hasard : il a été calibré pour l'être.",
    deeperTitle: "Les murs structurels",
    deeperIntro:
      "Au-delà de la conception, le puzzle présente une structure mesurable qui explique l'écart entre le meilleur plateau connu (469/480) et une solution complète. Chacun de ces résultats est publié avec le calcul exact qui le sous-tend.",
    soon: "En préparation",
    topics: {
      phaseTransition: {
        title: "L'argument de la transition de phase",
        body: "Pourquoi ≈17 couleurs intérieures et 5 couleurs de bord placent le puzzle pile sur le pic de difficulté SAT/CSP — environ une solution attendue, le pire endroit où chercher.",
      },
      rigidity: {
        title: "Le mur de rigidité",
        body: "Tout plateau record connu est figé localement : aucun petit réarrangement ne l'améliore. On ne peut pas, de proche en proche, passer d'un plateau record à une solution — les bons plateaux sont isolés.",
      },
      entropy: {
        title: "Entropie et loi d'aire",
        body: "Comment le nombre de plateaux partiels réellement distincts s'effondre au-delà d'environ quatre-vingts cellules — un mur universel qu'aucun signal de score local ne peut traverser.",
      },
      forbidden: {
        title: "Motifs interdits",
        body: "Presque tout petit carré de couleurs que l'on pourrait dessiner est illégal au regard du jeu de pièces ; une vraie solution n'en contient aucun. Une signature combinatoire nette de la validité.",
      },
    },
  },
};

export default function Why() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div>
        <LocalizedLink
          to="/research"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.engineeredTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.engineeredBody}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.deeperTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.deeperIntro}</p>
        <div className="grid gap-3 md:grid-cols-2">
          {TOPICS.map((topic) => {
            const copy = t.topics[topic.key as keyof typeof t.topics];
            const card = (
              <Card className="h-full transition-shadow group-hover:shadow-md">
                <CardHeader className="pb-2">
                  <div className="flex items-start justify-between gap-2">
                    <CardTitle className="text-base group-hover:underline">{copy.title}</CardTitle>
                    {!topic.ready && (
                      <span className="shrink-0 rounded-full bg-muted px-2 py-0.5 text-[10px] font-medium tracking-wide text-muted-foreground uppercase">
                        {t.soon}
                      </span>
                    )}
                  </div>
                </CardHeader>
                <CardContent className="text-sm text-muted-foreground">{copy.body}</CardContent>
              </Card>
            );
            return topic.ready && topic.to ? (
              <LocalizedLink key={topic.key} to={topic.to} className="group block">
                {card}
              </LocalizedLink>
            ) : (
              <div key={topic.key} className="group block opacity-75">
                {card}
              </div>
            );
          })}
        </div>
      </section>
    </div>
  );
}

export const meta = pageMeta("why");
