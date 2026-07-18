import { useT, useLang, pick, type Dict } from "@/i18n";
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
  head: Dict<string>;
  /** One line of context. */
  note: Dict<string>;
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
    head: { en: "The mailing list is founded", fr: "La liste de diffusion est fondée", es: "Se funda la lista de correo" },
    note: {
      en: "Brendan Owen starts the eternity_two group, more than six years before the puzzle exists.",
      fr: "Brendan Owen crée le groupe eternity_two, plus de six ans avant que le puzzle n'existe.",
      es: "Brendan Owen crea el grupo eternity_two, más de seis años antes de que exista el puzzle.",
    },
    to: "/research/community/hunt",
    src: "https://groups.io/g/eternity2/message/384",
  },
  {
    year: "2007",
    kind: "launch",
    head: { en: "Eternity II launches with a $2M prize", fr: "Eternity II sort, prime de 2 M$ à la clé", es: "Eternity II sale con un premio de 2 M$" },
    note: {
      en: "TOMY releases the 256-piece puzzle in July. Within a fortnight the list has digitized it and called it unsolvable.",
      fr: "TOMY sort le puzzle de 256 pièces en juillet. En quinze jours, la liste l'a numérisé et déclaré insoluble.",
      es: "TOMY lanza el puzzle de 256 piezas en julio. En dos semanas, la lista lo ha digitalizado y lo declara irresoluble.",
    },
    to: "/research/community/hunt",
    src: "https://en.wikipedia.org/wiki/Eternity_II_puzzle",
  },
  {
    year: "2007",
    kind: "theory",
    head: { en: "The hardness is shown to be by design", fr: "La difficulté se révèle voulue par conception", es: "Se demuestra que la dificultad es deliberada" },
    note: {
      en: "Owen and Stertenbrink derive that ~17 interior and ~5 border colours sit exactly at the unique-solution boundary.",
      fr: "Owen et Stertenbrink établissent que ≈17 couleurs intérieures et ≈5 de bord placent le puzzle pile à la frontière d'unicité.",
      es: "Owen y Stertenbrink deducen que ≈17 colores interiores y ≈5 de borde sitúan el puzzle justo en la frontera de solución única.",
    },
    to: "/research/why/phase-transition",
    src: "https://groups.io/g/eternity2/message/1947",
  },
  {
    year: "2008",
    kind: "record",
    head: { en: "Verhaard's 467 wins the $10,000 prize", fr: "Le 467 de Verhaard remporte les 10 000 $", es: "El 467 de Verhaard gana el premio de 10 000 $" },
    note: {
      en: "Louis Verhaard's eii solver reaches 467/480, entered under his wife's name. It holds the record for twelve years.",
      fr: "Le solveur eii de Louis Verhaard atteint 467/480, engagé sous le nom de son épouse. Le record tiendra douze ans.",
      es: "El solucionador eii de Louis Verhaard alcanza 467/480, inscrito a nombre de su esposa. Mantiene el récord durante doce años.",
    },
    to: "/research/community/hunt",
    src: "https://www.shortestpath.se/eii/eii_details.html",
  },
  {
    year: "2010",
    kind: "milestone",
    head: { en: "The contest closes, prize unclaimed", fr: "Le concours se clôt, la prime non réclamée", es: "El concurso cierra, con el premio sin reclamar" },
    note: {
      en: "The competition ends at noon on 31 December. The $2,000,000 goes unpaid; the puzzle is still unsolved.",
      fr: "Le concours prend fin le 31 décembre à midi. Les 2 000 000 $ ne sont jamais versés ; le puzzle reste non résolu.",
      es: "La competición termina al mediodía del 31 de diciembre. Los 2 000 000 $ nunca se pagan; el puzzle sigue sin resolverse.",
    },
    to: "/research/community/hunt-part-2",
    src: "https://groups.io/g/eternity2/message/8477",
  },
  {
    year: "2017",
    kind: "theory",
    head: { en: "Complex theory is validated", fr: "La théorie complexe est validée", es: "La teoría compleja queda validada" },
    note: {
      en: "McGavin solves Owen's hint-free 10×10 in ~180 core-years, landing inside the theory's predicted error bars.",
      fr: "McGavin résout le 10×10 sans indice d'Owen en ≈180 années-cœur, dans les marges d'erreur prévues par la théorie.",
      es: "McGavin resuelve el 10×10 sin pistas de Owen en ≈180 años-núcleo, dentro de los márgenes de error previstos por la teoría.",
    },
    to: "/research/why/complex-theory",
    src: "https://groups.io/g/eternity2/message/9686",
  },
  {
    year: "2019",
    kind: "milestone",
    head: { en: "The archive survives Yahoo's shutdown", fr: "L'archive survit à la fermeture de Yahoo", es: "El archivo sobrevive al cierre de Yahoo" },
    note: {
      en: "Yahoo Groups is erased. Members move two decades of the mailing list to groups.io days before it vanishes.",
      fr: "Yahoo Groups est effacé. Des membres transfèrent vingt ans de liste vers groups.io quelques jours avant sa disparition.",
      es: "Yahoo Groups se borra. Varios miembros trasladan dos décadas de la lista de correo a groups.io pocos días antes de que desaparezca.",
    },
    to: "/research/community/hunt-part-2",
    src: "https://groups.io/g/eternity2/message/2",
  },
  {
    year: "2020",
    kind: "record",
    head: { en: "Blackwood ends the twelve-year freeze", fr: "Blackwood met fin à douze ans de statu quo", es: "Blackwood rompe doce años de estancamiento" },
    note: {
      en: "An outsider posts a 468 on Reddit, open-sources the solver, and the record climbs 468 → 469 within weeks.",
      fr: "Un inconnu publie un 468 sur Reddit, ouvre le code du solveur, et le record grimpe de 468 à 469 en quelques semaines.",
      es: "Un desconocido publica un 468 en Reddit, libera el solucionador como código abierto, y el récord sube de 468 a 469 en pocas semanas.",
    },
    to: "/research/lab/experiments/joshua-blackwood/solver",
    src: "https://groups.io/g/eternity2/message/10045",
  },
  {
    year: "2021",
    kind: "record",
    head: { en: "The 470 record, still standing", fr: "Le record de 470, toujours debout", es: "El récord de 470, aún imbatido" },
    note: {
      en: "Joshua Blackwood reaches 470/480 with a retuned schedule. No one has beaten it since; the last 10 edges hold.",
      fr: "Joshua Blackwood atteint 470/480 avec un ordonnancement réglé à neuf. Personne ne l'a battu depuis ; les 10 dernières arêtes tiennent.",
      es: "Joshua Blackwood alcanza 470/480 con un cronograma reajustado. Nadie lo ha superado desde entonces; las últimas 10 aristas resisten.",
    },
    to: "/research/lab/experiments/joshua-blackwood/solver",
    src: "https://groups.io/g/eternity2/message/10117",
  },
  {
    year: "2024",
    kind: "record",
    head: { en: "Bucas ties the 470", fr: "Bucas égale le 470", es: "Bucas iguala el 470" },
    note: {
      en: "Jef Bucas reaches 470 again with restarted threads of Blackwood's solver, the first independent tie of the record.",
      fr: "Jef Bucas atteint de nouveau 470 avec des fils redémarrés du solveur de Blackwood : la première égalisation indépendante du record.",
      es: "Jef Bucas vuelve a alcanzar 470 con hilos reiniciados del solucionador de Blackwood: la primera igualación independiente del récord.",
    },
    to: "/research/records",
    src: "https://groups.io/g/eternity2/message/11401",
  },
  {
    year: "2026",
    kind: "record",
    head: { en: "Riotte's 464 moves the five-clue line", fr: "Le 464 de Riotte fait bouger la ligne à cinq indices", es: "El 464 de Riotte mueve la línea de las cinco pistas" },
    note: {
      en: "The best board respecting all five clue placements rises from 460 to 464, the first movement on that line since 2023.",
      fr: "Le meilleur plateau respectant les cinq indices passe de 460 à 464, premier mouvement sur cette ligne depuis 2023.",
      es: "El mejor tablero que respeta la colocación de las cinco pistas sube de 460 a 464, el primer movimiento en esa línea desde 2023.",
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
  es: {
    legend: [
      ["launch", "Lanzamiento"],
      ["record", "Récord"],
      ["theory", "Teoría"],
      ["milestone", "Hito"],
    ] as [Kind, string][],
    read: "Leer más",
    source: "fuente",
  },
};

export function HistoryTimeline() {
  const t = useT(T);
  const { lang } = useLang();

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
                {pick(m.head, lang)}
              </h3>
            </div>
            <p className="mt-1 max-w-[62ch] text-sm leading-relaxed text-muted-foreground">
              {pick(m.note, lang)}
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
