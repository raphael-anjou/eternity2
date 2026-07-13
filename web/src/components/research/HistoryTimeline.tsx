import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

// A scannable vertical timeline of the big steps in the Eternity II story, from
// the mailing list's founding in 2000 to the latest record in 2026. This is the
// front door to the history: each milestone is one line you can read at a
// glance, and the full prose lives in the two-part history pages (linked from
// each entry and from the page around this component). It is deliberately NOT
// the record-score chart (that lives on /research/records and plots the number
// over time); this is the narrative spine.
//
// Every event is sourced to the archive message where it happened, matching the
// two history pages. Kept in sync by hand with those pages: this is a curated
// shortlist of turning points, not every record.

type Kind = "launch" | "record" | "milestone" | "theory";

interface Milestone {
  year: string;
  /** Short scannable headline. */
  head: { en: string; fr: string };
  /** One line of context. */
  note: { en: string; fr: string };
  kind: Kind;
  /** Where to read the full story. */
  to: string;
  /** Optional primary source. */
  src?: string;
}

const MILESTONES: Milestone[] = [
  {
    year: "2000",
    kind: "milestone",
    head: { en: "The mailing list is founded", fr: "La liste de diffusion est fondée" },
    note: {
      en: "Brendan Owen starts the eternity_two group, more than six years before the puzzle exists.",
      fr: "Brendan Owen crée le groupe eternity_two, plus de six ans avant que le puzzle n'existe.",
    },
    to: "/research/community/hunt",
    src: "https://groups.io/g/eternity2/message/384",
  },
  {
    year: "2007",
    kind: "launch",
    head: { en: "Eternity II launches with a $2M prize", fr: "Eternity II sort, prime de 2 M$ à la clé" },
    note: {
      en: "TOMY releases the 256-piece puzzle in July. Within a fortnight the list has digitized it and called it unsolvable.",
      fr: "TOMY sort le puzzle de 256 pièces en juillet. En quinze jours, la liste l'a numérisé et déclaré insoluble.",
    },
    to: "/research/community/hunt",
    src: "https://en.wikipedia.org/wiki/Eternity_II_puzzle",
  },
  {
    year: "2007",
    kind: "theory",
    head: { en: "The hardness is shown to be by design", fr: "La difficulté se révèle voulue par conception" },
    note: {
      en: "Owen and Stertenbrink derive that ~17 interior and ~5 border colours sit exactly at the unique-solution boundary.",
      fr: "Owen et Stertenbrink établissent que ≈17 couleurs intérieures et ≈5 de bord placent le puzzle pile à la frontière d'unicité.",
    },
    to: "/research/why/phase-transition",
    src: "https://groups.io/g/eternity2/message/1947",
  },
  {
    year: "2008",
    kind: "record",
    head: { en: "Verhaard's 467 wins the $10,000 prize", fr: "Le 467 de Verhaard remporte les 10 000 $" },
    note: {
      en: "Louis Verhaard's eii solver reaches 467/480, entered under his wife's name. It holds the record for twelve years.",
      fr: "Le solveur eii de Louis Verhaard atteint 467/480, engagé sous le nom de son épouse. Le record tiendra douze ans.",
    },
    to: "/research/community/hunt",
    src: "https://www.shortestpath.se/eii/eii_details.html",
  },
  {
    year: "2010",
    kind: "milestone",
    head: { en: "The contest closes, prize unclaimed", fr: "Le concours se clôt, la prime non réclamée" },
    note: {
      en: "The competition ends at noon on 31 December. The $2,000,000 goes unpaid; the puzzle is still unsolved.",
      fr: "Le concours prend fin le 31 décembre à midi. Les 2 000 000 $ ne sont jamais versés ; le puzzle reste non résolu.",
    },
    to: "/research/community/hunt-part-2",
    src: "https://groups.io/g/eternity2/message/8477",
  },
  {
    year: "2017",
    kind: "theory",
    head: { en: "Complex theory is validated", fr: "La théorie complexe est validée" },
    note: {
      en: "McGavin solves Owen's hint-free 10×10 in ~180 core-years, landing inside the theory's predicted error bars.",
      fr: "McGavin résout le 10×10 sans indice d'Owen en ≈180 années-cœur, dans les marges d'erreur prévues par la théorie.",
    },
    to: "/research/why/complex-theory",
    src: "https://groups.io/g/eternity2/message/9686",
  },
  {
    year: "2019",
    kind: "milestone",
    head: { en: "The archive survives Yahoo's shutdown", fr: "L'archive survit à la fermeture de Yahoo" },
    note: {
      en: "Yahoo Groups is erased. Members move two decades of the mailing list to groups.io days before it vanishes.",
      fr: "Yahoo Groups est effacé. Des membres transfèrent vingt ans de liste vers groups.io quelques jours avant sa disparition.",
    },
    to: "/research/community/hunt-part-2",
    src: "https://groups.io/g/eternity2/message/2",
  },
  {
    year: "2020",
    kind: "record",
    head: { en: "Blackwood ends the twelve-year freeze", fr: "Blackwood met fin à douze ans de statu quo" },
    note: {
      en: "An outsider posts a 468 on Reddit, open-sources the solver, and the record climbs 468 → 469 within weeks.",
      fr: "Un inconnu publie un 468 sur Reddit, ouvre le code du solveur, et le record grimpe de 468 à 469 en quelques semaines.",
    },
    to: "/research/build/solvers/blackwood",
    src: "https://groups.io/g/eternity2/message/10045",
  },
  {
    year: "2021",
    kind: "record",
    head: { en: "The 470 record, still standing", fr: "Le record de 470, toujours debout" },
    note: {
      en: "Joshua Blackwood reaches 470/480 with a retuned schedule. No one has beaten it since; the last 10 edges hold.",
      fr: "Joshua Blackwood atteint 470/480 avec un ordonnancement réglé à neuf. Personne ne l'a battu depuis ; les 10 dernières arêtes tiennent.",
    },
    to: "/research/build/solvers/blackwood",
    src: "https://groups.io/g/eternity2/message/10117",
  },
  {
    year: "2024",
    kind: "record",
    head: { en: "Bucas ties the 470", fr: "Bucas égale le 470" },
    note: {
      en: "Jef Bucas reaches 470 again with restarted threads of Blackwood's solver, the first independent tie of the record.",
      fr: "Jef Bucas atteint de nouveau 470 avec des fils redémarrés du solveur de Blackwood : la première égalisation indépendante du record.",
    },
    to: "/research/records",
    src: "https://groups.io/g/eternity2/message/11401",
  },
  {
    year: "2026",
    kind: "record",
    head: { en: "Riotte's 464 moves the five-clue line", fr: "Le 464 de Riotte fait bouger la ligne à cinq indices" },
    note: {
      en: "The best board respecting all five clue placements rises from 460 to 464, the first movement on that line since 2023.",
      fr: "Le meilleur plateau respectant les cinq indices passe de 460 à 464, premier mouvement sur cette ligne depuis 2023.",
    },
    to: "/research/records",
    src: "https://groups.io/g/eternity2/message/11919",
  },
];

const DOT: Record<Kind, string> = {
  launch: "bg-pink-500",
  record: "bg-emerald-500",
  milestone: "bg-sky-500",
  theory: "bg-amber-500",
};

const T = {
  en: {
    legend: [
      ["launch", "Launch"],
      ["record", "Record"],
      ["theory", "Theory"],
      ["milestone", "Milestone"],
    ] as [Kind, string][],
    read: "Read more",
    source: "source",
  },
  fr: {
    legend: [
      ["launch", "Sortie"],
      ["record", "Record"],
      ["theory", "Théorie"],
      ["milestone", "Étape"],
    ] as [Kind, string][],
    read: "En savoir plus",
    source: "source",
  },
};

export function HistoryTimeline() {
  const t = useT(T);
  const lang = useT({ en: { l: "en" as const }, fr: { l: "fr" as const } }).l;

  return (
    <div className="not-prose space-y-6">
      <div className="flex flex-wrap gap-x-4 gap-y-1.5 text-xs text-muted-foreground">
        {t.legend.map(([kind, label]) => (
          <span key={kind} className="flex items-center gap-1.5">
            <span className={`inline-block h-2 w-2 rounded-full ${DOT[kind]}`} aria-hidden />
            {label}
          </span>
        ))}
      </div>

      <ol className="relative space-y-6 border-l border-border pl-6">
        {MILESTONES.map((m, i) => (
          <li key={`${m.year}-${i}`} className="relative">
            {/* dot on the rail */}
            <span
              className={`absolute -left-[1.6875rem] top-1.5 h-3 w-3 rounded-full ring-4 ring-background ${DOT[m.kind]}`}
              aria-hidden
            />
            <div className="flex flex-wrap items-baseline gap-x-3 gap-y-1">
              <span className="text-sm font-bold tabular-nums text-muted-foreground">{m.year}</span>
              <h3 className="text-base font-semibold tracking-tight text-foreground">
                {m.head[lang]}
              </h3>
            </div>
            <p className="mt-1 max-w-[62ch] text-sm leading-relaxed text-muted-foreground">
              {m.note[lang]}
            </p>
            <div className="mt-1.5 flex flex-wrap items-center gap-x-4 text-xs">
              <LocalizedLink
                to={m.to}
                className="font-medium text-foreground underline underline-offset-2 hover:text-muted-foreground"
              >
                {t.read}
              </LocalizedLink>
              {m.src && (
                <a
                  href={m.src}
                  target="_blank"
                  rel="noreferrer"
                  className="text-muted-foreground underline underline-offset-2 hover:text-foreground"
                >
                  {t.source}
                </a>
              )}
            </div>
          </li>
        ))}
      </ol>
    </div>
  );
}
