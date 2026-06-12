import { Link } from "react-router-dom";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";

const T = {
  en: {
    title: "Playground",
    intro:
      "Four ways to get your hands on the puzzle: no installs, no account, everything runs right here in your browser.",
    modes: [
      {
        to: "/playground/solve",
        emoji: "🧩",
        title: "Solve it yourself",
        text: "A real edge-matching puzzle, sized for humans (3×3 to 5×5). Beat the clock, then find out how many times a computer could have solved it while you played.",
      },
      {
        to: "/playground/watch",
        emoji: "🤖",
        title: "Watch the machine think",
        text: "A real backtracking search, live in your browser. See it place pieces, get stuck and undo. From 1 step per second up to millions.",
      },
      {
        to: "/playground/paths",
        emoji: "🗺️",
        title: "Invent a search path",
        text: "Draw the order the solver explores the board (spiral? zigzag? chaos?) and race your idea against the classic strategies.",
      },
      {
        to: "/playground/print",
        emoji: "🖨️",
        title: "Print and cut",
        text: "Make real puzzles for the table: pick sizes and colors, print double-sided with piece numbers on the back, cut along the lines.",
      },
    ],
  },
  fr: {
    title: "Aire de jeu",
    intro:
      "Quatre façons de jouer avec le puzzle : rien à installer, pas de compte, tout tourne directement dans votre navigateur.",
    modes: [
      {
        to: "/playground/solve",
        emoji: "🧩",
        title: "Résolvez-le vous-même",
        text: "Un vrai puzzle où les côtés doivent correspondre, à taille humaine (de 3×3 à 5×5). Battez le chrono, puis découvrez combien de fois un ordinateur aurait pu le résoudre pendant que vous jouiez.",
      },
      {
        to: "/playground/watch",
        emoji: "🤖",
        title: "Regardez la machine réfléchir",
        text: "Une vraie recherche par retour en arrière (backtracking), en direct dans votre navigateur. Regardez-la placer des pièces, se bloquer et revenir sur ses pas. De 1 étape par seconde à plusieurs millions.",
      },
      {
        to: "/playground/paths",
        emoji: "🗺️",
        title: "Inventez un chemin de parcours",
        text: "Dessinez l'ordre dans lequel le solveur explore le plateau (en spirale ? en zigzag ? au hasard ?) puis faites la course contre les stratégies classiques.",
      },
      {
        to: "/playground/print",
        emoji: "🖨️",
        title: "Imprimer et découper",
        text: "Fabriquez de vrais puzzles pour la table : choisissez tailles et couleurs, imprimez en recto verso avec les numéros au dos, découpez le long des lignes.",
      },
    ],
  },
};

export default function Hub() {
  const t = useT(T);
  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-1 max-w-2xl text-muted-foreground">{t.intro}</p>
      </div>
      <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-4">
        {t.modes.map((m) => (
          <Link key={m.to} to={m.to} className="group">
            <Card className="h-full transition-shadow group-hover:shadow-lg">
              <CardHeader>
                <div className="text-4xl">{m.emoji}</div>
                <CardTitle>{m.title}</CardTitle>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{m.text}</CardContent>
            </Card>
          </Link>
        ))}
      </div>
    </div>
  );
}
