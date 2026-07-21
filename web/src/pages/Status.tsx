import { pageMeta } from "@/seo";
import { PageFaq } from "@/components/PageFaq";
import { useMemo } from "react";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { BoardSvg } from "@/components/board/BoardSvg";
import { KNOWN_BOARDS } from "@/data/known-boards";
import { decodeBucas, parseParams, toOurParams } from "@/lib/bucas";
import { useT } from "@/i18n";

// The record board we lead with: Blackwood's 470, the standing open record on
// the official puzzle. Decoded once for a static preview; the viewer link
// re-encodes it into our own URL format so it opens re-scorable.
const RECORD_ID = "Joshua_Blackwood_470";

const T = {
  en: {
    dateStamp: "As of July 2026",
    title: "Has Eternity II been solved?",
    answer: (
      <>
        <strong>No.</strong> The complete solution has never been found, by anyone, human or
        machine. The best board anyone has ever built matches{" "}
        <strong>470 of the 480 internal edges</strong>. Ten edges still stand between the community
        and a finished puzzle, and that gap has not moved since 2021.
      </>
    ),
    recordLabel: "Current record",
    recordScore: "470 / 480",
    recordWho: "Joshua Blackwood, 2021",
    recordNote: (
      <>
        Tied once since, by Jef Bucas in December 2024, never beaten. This is the open record on the
        official puzzle: the 256 pieces Tomy sold, with only the mandatory center clue placed.
      </>
    ),
    viewBoard: "Open this board in the viewer",
    boardCaption:
      "Blackwood's 470-edge board. Ten of the 480 internal edges do not match; every other edge does.",
    factsTitle: "The facts, in brief",
    facts: [
      {
        term: "The prize expired unclaimed.",
        desc: "Tomy offered $2 million for the first complete solution. The competition ran from 28 July 2007 to noon on 31 December 2010, and closed with no winner.",
      },
      {
        term: "480 is the target.",
        desc: "A finished board has 2×15×16 = 480 internal edges that must match, side to side. The frame's grey outer edges are not counted. A score of 470 means 470 of those 480 edges match.",
      },
      {
        term: "The record has stood since 2021.",
        desc: "The climb went 467 (2008), then 468, 469, and 470 (2020–2021). The 470 has been equalled but never passed. The last ten edges are the hardest part of the puzzle.",
      },
      {
        term: "Boards using the five optional clues are tracked apart.",
        desc: "Tomy sold four clue puzzles that each fixed one more piece. The best board respecting all five clue placements is 464 (Benjamin Riotte, July 2026), which beat Bruno Gauthier's 460.",
      },
    ],
    notCountTitle: "About those “480 solved” boards online",
    notCount: (
      <>
        Several boards posted as 480/480 are real solutions, but not to this puzzle. They use
        different piece sets: Brendan Owen's smaller Clue-1 and Clue-2 designs, the unframed
        TopCoder variant, or boards that mix pieces from several expansion sets. Those solutions do
        not transfer to the official Eternity II. The official 480 remains unfound.
      </>
    ),
    whyTitle: "Why the last ten edges are so hard",
    why: (
      <>
        Eternity II was engineered to resist search. There are on the order of 10<sup>560</sup> ways
        to arrange the pieces, and the puzzle was generated to have close to a single solution, right
        at the point where the problem is hardest. Getting to 470 is already the work of
        purpose-built solvers running for tens of billions of steps. Closing the final gap is a
        different kind of problem: it needs a better idea, not just a faster machine.
      </>
    ),
    ctaRecords: "See the full record timeline",
    ctaOpenProblems: "The open problems that remain",
    ctaApproaches: "The map of every known approach",
    ctaPuzzle: "What is Eternity II?",
    ctaAlgorithms: "How solvers attack it",
  },
  fr: {
    dateStamp: "À jour en juillet 2026",
    title: "Eternity II a-t-il été résolu ?",
    answer: (
      <>
        <strong>Non.</strong> La solution complète n'a jamais été trouvée, ni par un humain, ni par
        une machine. Le meilleur plateau jamais assemblé fait coïncider{" "}
        <strong>470 des 480 arêtes intérieures</strong>. Il reste dix arêtes entre la communauté et
        un puzzle terminé, et cet écart n'a pas bougé depuis 2021.
      </>
    ),
    recordLabel: "Record actuel",
    recordScore: "470 / 480",
    recordWho: "Joshua Blackwood, 2021",
    recordNote: (
      <>
        Égalé une seule fois depuis, par Jef Bucas en décembre 2024, jamais battu. C'est le record
        ouvert sur le puzzle officiel : les 256 pièces vendues par Tomy, avec le seul indice central
        obligatoire posé.
      </>
    ),
    viewBoard: "Ouvrir ce plateau dans le visualiseur",
    boardCaption:
      "Le plateau à 470 arêtes de Blackwood. Dix des 480 jonctions intérieures ne coïncident pas ; toutes les autres oui.",
    factsTitle: "L'essentiel en bref",
    facts: [
      {
        term: "Le prix a expiré sans vainqueur.",
        desc: "Tomy offrait 2 000 000 $ à la première solution complète. Le concours a couru du 28 juillet 2007 à midi le 31 décembre 2010, et s'est refermé sans gagnant.",
      },
      {
        term: "L'objectif, c'est 480.",
        desc: "Un plateau terminé compte 2×15×16 = 480 arêtes intérieures à faire coïncider, côté contre côté. Le pourtour gris du cadre ne compte pas. Un score de 470 veut dire que 470 de ces 480 jonctions tombent juste.",
      },
      {
        term: "Le record tient depuis 2021.",
        desc: "La progression est passée par 467 (2008), puis 468, 469 et 470 (2020–2021). Le 470 a été égalé, jamais dépassé. Les dix dernières arêtes sont la partie la plus coriace du puzzle.",
      },
      {
        term: "Les plateaux qui respectent les cinq indices sont suivis à part.",
        desc: "Tomy a vendu quatre puzzles indices, fixant chacun une pièce de plus. Le meilleur plateau respectant les cinq indices est 464 (Benjamin Riotte, juillet 2026), qui a battu le 460 de Bruno Gauthier.",
      },
    ],
    notCountTitle: "À propos de ces plateaux « 480 résolus » que l'on trouve en ligne",
    notCount: (
      <>
        Plusieurs plateaux publiés en 480/480 sont de vraies solutions, mais pas à ce puzzle. Ils
        utilisent d'autres jeux de pièces : les designs Clue-1 et Clue-2 plus petits de Brendan Owen,
        la variante non encadrée de TopCoder, ou des plateaux mélangeant des pièces de plusieurs jeux
        d'extension. Ces solutions ne se transposent pas à l'Eternity II officiel. Le 480 officiel
        reste introuvé.
      </>
    ),
    whyTitle: "Pourquoi les dix dernières arêtes sont si dures",
    why: (
      <>
        Eternity II a été conçu pour résister à la recherche. Il existe de l'ordre de 10
        <sup>560</sup> façons d'agencer les pièces, et le puzzle a été généré pour n'avoir qu'à peu
        près une seule solution, pile là où le problème est le plus dur. Atteindre 470 est déjà
        l'affaire de solveurs taillés sur mesure, lancés sur des dizaines de milliards d'étapes.
        Combler le dernier écart relève d'un autre problème : il y faut une meilleure idée, pas
        seulement une machine plus rapide.
      </>
    ),
    ctaRecords: "Voir la chronologie complète des records",
    ctaOpenProblems: "Les problèmes ouverts qui restent",
    ctaApproaches: "La carte de toutes les approches connues",
    ctaPuzzle: "C'est quoi, Eternity II ?",
    ctaAlgorithms: "Comment les solveurs s'y attaquent",
  },
  es: {
    dateStamp: "Actualizado en julio de 2026",
    title: "¿Se ha resuelto Eternity II?",
    answer: (
      <>
        <strong>No.</strong> Nadie ha encontrado nunca la solución completa, ni una persona ni una
        máquina. El mejor tablero jamás construido hace coincidir{" "}
        <strong>470 de las 480 aristas interiores</strong>. Todavía quedan diez aristas entre la
        comunidad y un puzzle terminado, y esa diferencia no se ha movido desde 2021.
      </>
    ),
    recordLabel: "Récord actual",
    recordScore: "470 / 480",
    recordWho: "Joshua Blackwood, 2021",
    recordNote: (
      <>
        Igualado una sola vez desde entonces, por Jef Bucas en diciembre de 2024, nunca superado. Es
        el récord abierto sobre el puzzle oficial: las 256 piezas que vendió Tomy, con solo la pista
        central obligatoria colocada.
      </>
    ),
    viewBoard: "Abrir este tablero en el visor",
    boardCaption:
      "El tablero de 470 aristas de Blackwood. Diez de las 480 aristas interiores no coinciden; todas las demás sí.",
    factsTitle: "Lo esencial, en breve",
    facts: [
      {
        term: "El premio expiró sin reclamar.",
        desc: "Tomy ofreció 2 000 000 $ por la primera solución completa. El concurso estuvo abierto desde el 28 de julio de 2007 hasta el mediodía del 31 de diciembre de 2010, y se cerró sin ganador.",
      },
      {
        term: "El objetivo es 480.",
        desc: "Un tablero terminado tiene 2×15×16 = 480 aristas interiores que deben coincidir, lado con lado. El borde gris exterior del marco no cuenta. Una puntuación de 470 significa que 470 de esas 480 aristas coinciden.",
      },
      {
        term: "El récord se mantiene desde 2021.",
        desc: "El ascenso pasó por 467 (2008), luego 468, 469 y 470 (2020–2021). El 470 se ha igualado, pero nunca superado. Las últimas diez aristas son la parte más difícil del puzzle.",
      },
      {
        term: "Los tableros que usan las cinco pistas opcionales se contabilizan aparte.",
        desc: "Tomy vendió cuatro puzzles de pista que fijaban cada uno una pieza más. El mejor tablero que respeta las cinco colocaciones de pista es 464 (Benjamin Riotte, julio de 2026), que superó el 460 de Bruno Gauthier.",
      },
    ],
    notCountTitle: "Sobre esos tableros «480 resueltos» que circulan por internet",
    notCount: (
      <>
        Varios tableros publicados como 480/480 son soluciones reales, pero no de este puzzle. Usan
        conjuntos de piezas distintos: los diseños Clue-1 y Clue-2 más pequeños de Brendan Owen, la
        variante sin marco de TopCoder, o tableros que mezclan piezas de varios conjuntos de
        expansión. Esas soluciones no se trasladan al Eternity II oficial. El 480 oficial sigue sin
        encontrarse.
      </>
    ),
    whyTitle: "Por qué las últimas diez aristas son tan difíciles",
    why: (
      <>
        Eternity II se diseñó para resistir a la búsqueda. Hay del orden de 10<sup>560</sup> formas
        de disponer las piezas, y el puzzle se generó para tener casi una única solución, justo en el
        punto donde el problema es más difícil. Llegar a 470 es ya obra de solucionadores hechos a
        medida, ejecutados durante decenas de miles de millones de pasos. Cerrar la última diferencia
        es un problema de otra índole: hace falta una idea mejor, no solo una máquina más rápida.
      </>
    ),
    ctaRecords: "Ver la cronología completa de los récords",
    ctaOpenProblems: "Los problemas abiertos que quedan",
    ctaApproaches: "El mapa de todos los enfoques conocidos",
    ctaPuzzle: "¿Qué es Eternity II?",
    ctaAlgorithms: "Cómo lo atacan los solucionadores",
  },
};

export default function StatusPage() {
  const t = useT(T);

  const record = useMemo(() => KNOWN_BOARDS.find((b) => b.id === RECORD_ID) ?? null, []);
  const cells = useMemo(() => (record ? decodeBucas(record.params).cells : null), [record]);
  const viewerHref = useMemo(() => {
    if (!record) return null;
    const query = new URLSearchParams(toOurParams(parseParams(record.params))).toString();
    return `/viewer?${query}`;
  }, [record]);

  return (
    <div className="space-y-12">
      <section className="space-y-4">
        <Badge variant="secondary">{t.dateStamp}</Badge>
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">{t.title}</h1>
        <p className="max-w-3xl text-lg text-muted-foreground">{t.answer}</p>
      </section>

      <section className="grid items-start gap-6 md:grid-cols-2">
        <Card>
          <CardHeader className="pb-2">
            <div className="text-sm font-medium text-muted-foreground">{t.recordLabel}</div>
            <CardTitle className="font-mono text-4xl">{t.recordScore}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-3 text-sm text-muted-foreground">
            <div className="font-medium text-foreground">{t.recordWho}</div>
            <p>{t.recordNote}</p>
            {viewerHref && (
              <Button variant="outline" size="sm" render={<Link to={viewerHref} />}>
                {t.viewBoard}
              </Button>
            )}
          </CardContent>
        </Card>
        <div className="space-y-2">
          {cells && <BoardSvg width={16} height={16} cells={cells} className="w-full" />}
          <p className="text-xs text-muted-foreground">{t.boardCaption}</p>
        </div>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.factsTitle}</h2>
        <dl className="grid max-w-4xl gap-4 sm:grid-cols-2">
          {t.facts.map((f) => (
            <div key={f.term} className="rounded-lg border p-4">
              <dt className="font-medium">{f.term}</dt>
              <dd className="mt-1 text-sm text-muted-foreground">{f.desc}</dd>
            </div>
          ))}
        </dl>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.notCountTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.notCount}</p>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.whyTitle}</h2>
        <p className="text-sm leading-relaxed text-muted-foreground">{t.why}</p>
      </section>

      <section className="flex flex-wrap gap-3">
        <Button render={<Link to="/research/records" />}>{t.ctaRecords}</Button>
        <Button variant="outline" render={<Link to="/research/open-problems" />}>
          {t.ctaOpenProblems}
        </Button>
        <Button variant="outline" render={<Link to="/research/build/approaches-map" />}>
          {t.ctaApproaches}
        </Button>
        <Button variant="outline" render={<Link to="/puzzle" />}>
          {t.ctaPuzzle}
        </Button>
        <Button variant="outline" render={<Link to="/algorithms" />}>
          {t.ctaAlgorithms}
        </Button>
      </section>
      <PageFaq pageKey="status" />
    </div>
  );
}

export const meta = pageMeta("status");
