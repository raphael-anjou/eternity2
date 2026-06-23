import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";
import { DfsDemo } from "@/components/learn/DfsDemo";
import { BreakIndexLab } from "@/components/research/BreakIndexLab";

// The solver catalogue: how the notable solvers actually search. Prose for now,
// with links into the live playground (Watch / Paths) so readers can see a real
// depth-first search run and feel how much the fill order matters. Per-solver
// deep-dive pages can grow under this folder later.

const T = {
  en: {
    backLabel: "← Build a solver",
    title: "Solver catalogue",
    intro:
      "How the solvers that hold the records actually search. They are all, at heart, depth-first backtrackers; what separates them is the order they try things and how they bend the rules near the end.",
    watchCta: "Watch a real solver run",
    pathsCta: "Race fill orders yourself",
    demoIntro:
      "Here is exactly that, in slow motion: a real backtracker on a 3×3, one decision at a time. Step through it — watch a piece go down, a dead end appear, and the search un-place and try again.",
    items: [
      {
        name: "Plain backtracking",
        body: "Fill the board cell by cell in some fixed order. At each cell, try every piece and rotation whose edges match what's already placed; if none fit, back up and try the previous cell differently. Correct but slow: the search tree is astronomically wide.",
      },
      {
        name: "Order matters: the fill path",
        body: "The order you visit cells changes the difficulty by orders of magnitude, because some orders force conflicts to surface early (good) and others defer them until a lot of work is wasted (bad). Border-first beats row-major by a wide margin on the same puzzle.",
      },
      {
        name: "Blackwood: schedule + break-index",
        body: "The solver behind the community's best boards adds two ideas to plain backtracking. A heuristic schedule pre-commits to placing a few high-frequency edge colors on a set timetable by depth, pruning branches that fall behind. And a break-index allowance lets a handful of fixed late positions carry one mismatch instead of demanding a perfect match, which is what makes a 469 (eleven mismatches) reachable at all.",
      },
      {
        name: "McGavin: raw throughput",
        body: "The 469 board came from running a heavily optimized version of that solver at hundreds of millions of placements per second, across many cores for weeks. No new idea over Blackwood's schedule; just an extremely fast engine and a lot of compute aimed at the right parameter set.",
      },
    ],
    runTitle: "See it run",
    runBody: "The playground runs a real depth-first solver in your browser. Watch it search live, or draw your own fill order and race it against the classics to feel how much the order matters.",
    deadEndsNote: "For approaches that don't work, see the dead ends.",
    deadEndsCta: "Dead ends",
  },
  fr: {
    backLabel: "← Construire un solveur",
    title: "Catalogue des solveurs",
    intro:
      "Comment les solveurs qui détiennent les records cherchent vraiment. Ce sont tous, au fond, des backtrackers en profondeur ; ce qui les distingue, c'est l'ordre dans lequel ils essaient les choses et la façon dont ils assouplissent les règles vers la fin.",
    watchCta: "Regarder un vrai solveur tourner",
    pathsCta: "Comparer les ordres vous-même",
    demoIntro:
      "Le voici précisément, au ralenti : un vrai backtracker sur un 3×3, une décision à la fois. Avancez pas à pas — voyez une pièce se poser, une impasse apparaître, et la recherche retirer la pièce et réessayer.",
    items: [
      {
        name: "Backtracking simple",
        body: "Remplir le plateau case par case dans un ordre fixe. À chaque case, essayer chaque pièce et rotation dont les bords s'accordent avec ce qui est déjà posé ; si rien ne convient, revenir en arrière. Correct mais lent : l'arbre de recherche est démesurément large.",
      },
      {
        name: "L'ordre compte : le chemin de remplissage",
        body: "L'ordre de visite des cases change la difficulté de plusieurs ordres de grandeur, car certains ordres font surgir les conflits tôt (bien) et d'autres les repoussent jusqu'à gâcher beaucoup de travail (mal). Le bord d'abord bat largement le balayage par lignes sur le même puzzle.",
      },
      {
        name: "Blackwood : calendrier + index de rupture",
        body: "Le solveur derrière les meilleurs plateaux de la communauté ajoute deux idées au backtracking. Un calendrier heuristique s'engage à placer quelques couleurs de bord fréquentes selon un échéancier par profondeur, élaguant les branches en retard. Et une tolérance d'index de rupture laisse quelques positions tardives fixes porter un défaut au lieu d'exiger un accord parfait, ce qui rend un 469 (onze défauts) atteignable.",
      },
      {
        name: "McGavin : débit brut",
        body: "Le plateau 469 est venu de l'exécution d'une version très optimisée de ce solveur à des centaines de millions de placements par seconde, sur de nombreux cœurs pendant des semaines. Aucune idée nouvelle par rapport au calendrier de Blackwood ; juste un moteur extrêmement rapide et beaucoup de calcul visant le bon jeu de paramètres.",
      },
    ],
    runTitle: "Le voir tourner",
    runBody: "L'aire de jeu exécute un vrai solveur en profondeur dans votre navigateur. Regardez-le chercher en direct, ou dessinez votre propre ordre de remplissage et comparez-le aux ordres classiques pour sentir à quel point l'ordre compte.",
    deadEndsNote: "Pour les approches qui ne marchent pas, voir les impasses.",
    deadEndsCta: "Impasses",
  },
};

export default function SolversHub() {
  const t = useT(T);
  return (
    <div className="space-y-10">
      <div>
        <LocalizedLink
          to="/research/build"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      <section className="space-y-4">
        {t.items.map((item, i) => (
          <div key={item.name} className="space-y-4">
            <article className="max-w-3xl rounded-lg border p-5">
              <h2 className="text-lg font-semibold tracking-tight">{item.name}</h2>
              <p className="mt-2 text-sm leading-relaxed text-muted-foreground">{item.body}</p>
            </article>
            {i === 0 && (
              <div className="space-y-2">
                <p className="max-w-3xl text-sm text-muted-foreground">{t.demoIntro}</p>
                <DfsDemo />
              </div>
            )}
            {i === 2 && <BreakIndexLab />}
          </div>
        ))}
      </section>

      <section className="max-w-3xl space-y-3 rounded-lg border bg-muted/30 p-6">
        <h2 className="text-xl font-semibold tracking-tight">{t.runTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.runBody}</p>
        <div className="flex flex-wrap gap-3 pt-1">
          <LocalizedLink
            to="/playground/watch"
            className="rounded-md bg-primary px-4 py-1.5 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90"
          >
            {t.watchCta}
          </LocalizedLink>
          <LocalizedLink
            to="/playground/paths"
            className="rounded-md border px-4 py-1.5 text-sm font-medium transition-colors hover:bg-muted"
          >
            {t.pathsCta}
          </LocalizedLink>
        </div>
      </section>

      <p className="max-w-3xl text-sm text-muted-foreground">
        {t.deadEndsNote}{" "}
        <LocalizedLink
          to="/research/build/dead-ends"
          className="font-medium underline hover:text-foreground"
        >
          {t.deadEndsCta}
        </LocalizedLink>
      </p>
    </div>
  );
}

export const meta = pageMeta("solvers");
