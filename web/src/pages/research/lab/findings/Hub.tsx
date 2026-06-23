import { pageMeta } from "@/seo";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// The structural findings, as an index. Each one is written up in full (with its
// reproduce command, data, and where relevant the math) under "Why it's hard";
// this page collects them as standing results, grouped by what kind of claim they
// are, so a researcher can see the whole picture at once.
type Finding = { key: string; to: string; kind: "design" | "wall" | "proof" };

const FINDINGS: Finding[] = [
  { key: "phase", to: "/research/why/phase-transition", kind: "design" },
  { key: "rareColor", to: "/research/why/rare-color-geography", kind: "design" },
  { key: "forbidden", to: "/research/why/forbidden-patterns", kind: "wall" },
  { key: "noForced", to: "/research/why/no-forced-moves", kind: "wall" },
  { key: "entropy", to: "/research/why/entropy-area-law", kind: "wall" },
  { key: "rigidity", to: "/research/why/rigidity-wall", kind: "proof" },
  { key: "sigma", to: "/research/why/sigma-cycles", kind: "proof" },
  { key: "archipelago", to: "/research/why/basin-archipelago", kind: "proof" },
  { key: "mismatch", to: "/research/why/mismatch-geometry", kind: "proof" },
];

const T = {
  en: {
    backLabel: "← The lab notebook",
    title: "Findings",
    intro:
      "The standing structural results about the puzzle, each written up with the computation behind it. They fall into three kinds: signatures of the design, walls that no local method gets past, and proofs of why the records are stuck. Every one is reproducible.",
    kinds: {
      design: "Design signatures",
      wall: "Structural walls",
      proof: "Rigidity proofs",
    },
    reproNote: "Each finding links to its full write-up, with the exact computation and a reproduce command.",
    synthesisCta: "See how each wall maps to the methods that hit it →",
    findings: {
      phase: {
        title: "Tuned to the hardness peak",
        body: "17 interior colors and 5 frame colors put the puzzle on the difficulty peak by design.",
      },
      rareColor: {
        title: "The rare colors live on the frame",
        body: "Five colors appear only on the border ring, each on exactly 24 edges, balanced on purpose.",
      },
      forbidden: {
        title: "Forbidden patterns",
        body: "99.72% of 2x2 placements are impossible; a real solution contains none.",
      },
      noForced: {
        title: "No forced moves",
        body: "Yet no piece is ever pinned: every interior piece has 73 to 137 neighbours.",
      },
      entropy: {
        title: "Entropy and the area law",
        body: "The matching grammar is rich; the use-each-piece-once rule collapses it past ~80 cells.",
      },
      rigidity: {
        title: "The rigidity wall",
        body: "Integer programming proves every record board is locally frozen, out to radius 4.",
      },
      sigma: {
        title: "Why basin-hopping is impossible",
        body: "Two great boards differ by one 154-cell cycle; every smaller piece of it scores worse.",
      },
      archipelago: {
        title: "The archipelago of record boards",
        body: "The strong boards form 47 isolated islands; McGavin's 469 sits alone, 247 cells from anything.",
      },
      mismatch: {
        title: "Where the mismatches live",
        body: "A record board packs all its errors into one five-row band; scan direction decides which end.",
      },
    },
  },
  fr: {
    backLabel: "← Le carnet de laboratoire",
    title: "Résultats",
    intro:
      "Les résultats structurels établis sur le puzzle, chacun rédigé avec le calcul qui le sous-tend. Ils se répartissent en trois types : des signatures de la conception, des murs qu'aucune méthode locale ne franchit, et des preuves expliquant pourquoi les records sont bloqués. Chacun est reproductible.",
    kinds: {
      design: "Signatures de conception",
      wall: "Murs structurels",
      proof: "Preuves de rigidité",
    },
    reproNote: "Chaque résultat renvoie à sa rédaction complète, avec le calcul exact et une commande de reproduction.",
    synthesisCta: "Voir comment chaque mur correspond aux méthodes qui s'y heurtent →",
    findings: {
      phase: {
        title: "Calé sur le pic de difficulté",
        body: "17 couleurs intérieures et 5 de cadre placent le puzzle sur le pic de difficulté, à dessein.",
      },
      rareColor: {
        title: "Les couleurs rares vivent sur le cadre",
        body: "Cinq couleurs n'apparaissent que sur le cadre, chacune sur exactement 24 bords, équilibrées exprès.",
      },
      forbidden: {
        title: "Motifs interdits",
        body: "99,72 % des placements 2x2 sont impossibles ; une vraie solution n'en contient aucun.",
      },
      noForced: {
        title: "Aucun coup forcé",
        body: "Pourtant aucune pièce n'est jamais coincée : chaque pièce intérieure a 73 à 137 voisines.",
      },
      entropy: {
        title: "Entropie et loi d'aire",
        body: "La grammaire d'accord est riche ; la règle « chaque pièce une fois » l'effondre au-delà de ~80 cellules.",
      },
      rigidity: {
        title: "Le mur de rigidité",
        body: "La programmation en nombres entiers prouve que chaque plateau record est figé localement, jusqu'au rayon 4.",
      },
      sigma: {
        title: "Pourquoi sauter de bassin en bassin est impossible",
        body: "Deux bons plateaux diffèrent par un cycle de 154 cellules ; chaque morceau plus petit fait moins bien.",
      },
      archipelago: {
        title: "L'archipel des plateaux records",
        body: "Les bons plateaux forment 47 îles isolées ; le 469 de McGavin est seul, à 247 cellules de tout.",
      },
      mismatch: {
        title: "Où vivent les défauts",
        body: "Un plateau record entasse ses erreurs dans une bande de cinq rangées ; le sens de balayage décide laquelle.",
      },
    },
  },
};

const KIND_ORDER: ("design" | "wall" | "proof")[] = ["design", "wall", "proof"];

export default function FindingsHub() {
  const t = useT(T);
  return (
    <div className="space-y-10">
      <div>
        <LocalizedLink
          to="/research/lab"
          className="text-sm text-muted-foreground hover:text-foreground"
        >
          {t.backLabel}
        </LocalizedLink>
        <h1 className="mt-2 text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.intro}</p>
      </div>

      {KIND_ORDER.map((kind) => (
        <section key={kind} className="space-y-3">
          <h2 className="text-xl font-semibold tracking-tight">{t.kinds[kind]}</h2>
          <div className="grid gap-3 md:grid-cols-3">
            {FINDINGS.filter((f) => f.kind === kind).map((f) => {
              const copy = t.findings[f.key as keyof typeof t.findings];
              return (
                <LocalizedLink key={f.key} to={f.to} className="group block">
                  <Card className="h-full transition-shadow group-hover:shadow-md">
                    <CardHeader className="pb-2">
                      <CardTitle className="text-base group-hover:underline">{copy.title}</CardTitle>
                    </CardHeader>
                    <CardContent className="text-sm text-muted-foreground">{copy.body}</CardContent>
                  </Card>
                </LocalizedLink>
              );
            })}
          </div>
        </section>
      ))}

      <p className="max-w-3xl text-sm text-muted-foreground">{t.reproNote}</p>

      <LocalizedLink
        to="/research/why/walls-and-methods"
        className="inline-block text-sm font-medium underline underline-offset-2 hover:text-foreground"
      >
        {t.synthesisCta}
      </LocalizedLink>
    </div>
  );
}

export const meta = pageMeta("findings");
