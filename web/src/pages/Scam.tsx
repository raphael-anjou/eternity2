import { pageMeta } from "@/seo";
import type { ReactNode } from "react";
import { LocalizedLink as Link } from "@/components/LocalizedLink";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useT } from "@/i18n";

function msg(n: number): string {
  return `https://groups.io/g/eternity2/message/${n}`;
}

type Misconception = {
  claim: string;
  verdict: string;
  body: ReactNode;
};

const T = {
  en: {
    title: "Is Eternity II a scam? Can it even be solved?",
    lede: (
      <>
        The question comes up on Hacker News, on Discord, and it came up on the
        players' own mailing list back in 2007. A $2,000,000 prize, a deadline
        that passed with no winner, and a rule that you may not tell anyone
        you've cracked it: it does look suspicious.
      </>
    ),
    shortAnswerTitle: "Short answer",
    shortAnswer: (
      <>
        No, it is not a scam, and yes, a complete solution almost certainly
        exists. The competition ran from 28 July 2007 to noon on 31 December
        2010 and expired with no winner, but that is a story about difficulty,
        not deception. The puzzle was built to have very few solutions, so close
        to one solution by the maker's account, that finding one among roughly
        1.15×10<sup>557</sup> arrangements is astronomically hard. "Few solutions
        by design" is not the same claim as "no solution exists."
      </>
    ),
    communityHookTitle: "The community asked this first",
    communityHook: (
      <>
        You are in good company. On 6 August 2007, a player writing as jagbrain
        put it bluntly on the official list:{" "}
        <a className="underline" href={msg(1407)} target="_blank" rel="noreferrer">
          "How do we know Eternity II isn't a big SCAM? ... Are there a notary in
          the Committee? If there is, the name must be public."
        </a>{" "}
        That is the same worry people voice today: who holds the solution, and is
        anyone impartial checking? Here is what the record actually shows.
      </>
    ),
    misconceptionsTitle: "Three things people get wrong",
    misconceptions: [
      {
        claim: "“Even the creator has no solution.”",
        verdict: "Half true, and it cuts the other way.",
        body: (
          <>
            A TOMY leaflet quoted on the mailing list says{" "}
            <a className="underline" href={msg(902)} target="_blank" rel="noreferrer">
              "the ETERNITY II solution was printed between pages of random text
              whilst all parties were out of the room"
            </a>{" "}
            (the leaflet was relayed by Dave Clark in{" "}
            <a className="underline" href={msg(901)} target="_blank" rel="noreferrer">
              a related message
            </a>
            ). The design deliberately avoided anyone keeping a readable master
            solution. That is a safeguard against insider cheating, not evidence
            that no solution exists. The puzzle was engineered around a solution;
            the makers chose not to hold it in plain sight.
          </>
        ),
      },
      {
        claim: "“Only a quantum computer could solve it.”",
        verdict: "A misframing.",
        body: (
          <>
            Quantum hardware is not a magic key for this kind of search, and no
            quantum machine is needed for the claim "a solution exists" to hold.
            Eternity I, the 1999 predecessor, was solved within about 18 months
            by Alex Selby and Oliver Riordan on ordinary computers, using better
            reasoning rather than more raw speed. Eternity II was then hardened
            against the exact tricks that cracked Eternity I, which is why it is
            harder, not why it is impossible.
          </>
        ),
      },
      {
        claim: "“It's cruel, or plain impossible.”",
        verdict: "Neither.",
        body: (
          <>
            The best partial board found so far matches 470 of the 480 interior
            edges (Joshua Blackwood, 2021). Ten unmatched edges out of 480 is a
            narrow gap, and progress has been real: 467 in 2008, 469 in 2020, 470
            in 2021. A search this hard can go a very long time without a full hit
            while a solution still sits somewhere in the space. Hard is not the
            same as impossible.
          </>
        ),
      },
    ] as Misconception[],
    verificationTitle: "Was there any real verification?",
    verification: (
      <>
        Yes, and it had a precedent that worked. Eternity I was checked by{" "}
        <a className="underline" href={msg(901)} target="_blank" rel="noreferrer">
          "specialist loss adjusters acting for the insurers,"
        </a>{" "}
        and the prize was paid. For Eternity II the plan was annual "scrutineering"
        dates when submitted boards would be judged (see{" "}
        <a className="underline" href={msg(1129)} target="_blank" rel="noreferrer">
          the players' notes on the scrutiny schedule
        </a>{" "}
        and{" "}
        <a className="underline" href={msg(1135)} target="_blank" rel="noreferrer">
          the follow-up
        </a>
        ). The "don't tell anyone you've solved it" rule (raised on the list by{" "}
        <a className="underline" href={msg(314)} target="_blank" rel="noreferrer">
          simon.chapple
        </a>
        ) protected the prize from leaks, and it also fed the suspicion. A real
        solution had a real path to being verified and paid; nobody ever produced
        one before the deadline.
      </>
    ),
    distinctionTitle: "Few by design is not impossible",
    distinction: (
      <>
        This is the point worth keeping. Selby and Riordan helped design Eternity
        II precisely so it would not have the abundance of near-solutions that let
        them beat Eternity I. Fewer target solutions means the search has almost
        nothing to home in on, so brute force and clever heuristics alike wander
        an enormous space. A solution exists by construction; what is missing is a
        method fast enough to find it. That is why the puzzle is famous, and why it
        is not a trick.
      </>
    ),
    learnMoreTitle: "Read further",
    learnMoreMath: (
      <>
        For the combinatorics behind that 10<sup>557</sup>, the NP-completeness of
        edge matching, and how solvers actually attack the board, see{" "}
        <Link className="underline" to="/algorithms">
          Algorithms
        </Link>{" "}
        and the{" "}
        <Link className="underline" to="/research">
          Research
        </Link>{" "}
        pages.
      </>
    ),
    learnMoreRecord: (
      <>
        For the current best boards, including the 470/480 record, see the records
        table on{" "}
        <Link className="underline" to="/puzzle">
          The Puzzle
        </Link>
        .
      </>
    ),
  },
  fr: {
    title: "Eternity II, une arnaque ? A-t-il seulement une solution ?",
    lede: (
      <>
        La question revient sur Hacker News, sur Discord, et elle s'était déjà
        posée sur la liste de diffusion des joueurs dès 2007. Un prix de 2 millions
        de dollars, une date limite passée sans vainqueur, et une règle qui vous
        interdit d'annoncer que vous avez trouvé : forcément, ça intrigue.
      </>
    ),
    shortAnswerTitle: "Réponse courte",
    shortAnswer: (
      <>
        Non, ce n'est pas une arnaque, et oui, une solution complète existe
        presque à coup sûr. Le concours s'est tenu du 28 juillet 2007 au 31
        décembre 2010 à midi, et s'est éteint sans gagnant, mais c'est une
        histoire de difficulté, pas de tromperie. Le puzzle a été conçu pour
        n'avoir que très peu de solutions, si peu que, d'après son créateur, on
        frôle l'unicité : dénicher l'une d'elles parmi environ 1,15×10<sup>557</sup>{" "}
        agencements relève de l'astronomique. « Peu de solutions par conception »
        n'est pas la même affirmation que « aucune solution n'existe ».
      </>
    ),
    communityHookTitle: "La communauté a posé la question la première",
    communityHook: (
      <>
        Vous êtes en bonne compagnie. Le 6 août 2007, un joueur sous le pseudo
        jagbrain l'a lancée sans détour sur la liste officielle :{" "}
        <a className="underline" href={msg(1407)} target="_blank" rel="noreferrer">
          « Qui nous dit qu'Eternity II n'est pas une grosse ARNAQUE ? ... Y a-t-il
          un notaire au sein du comité ? Si oui, son nom doit être rendu public. »
        </a>{" "}
        C'est exactement l'inquiétude que l'on entend aujourd'hui : qui détient la
        solution, et un tiers impartial vérifie-t-il quoi que ce soit ? Voici ce
        que disent réellement les faits.
      </>
    ),
    misconceptionsTitle: "Trois idées reçues à corriger",
    misconceptions: [
      {
        claim: "« Même le créateur n'a pas de solution. »",
        verdict: "À moitié vrai, et l'argument se retourne.",
        body: (
          <>
            Un dépliant de TOMY, cité sur la liste, indique que{" "}
            <a className="underline" href={msg(902)} target="_blank" rel="noreferrer">
              « la solution d'ETERNITY II a été imprimée entre des pages de texte
              aléatoire, toutes les parties ayant quitté la pièce »
            </a>{" "}
            (dépliant relayé par Dave Clark dans{" "}
            <a className="underline" href={msg(901)} target="_blank" rel="noreferrer">
              un message associé
            </a>
            ). Le procédé évitait sciemment que quiconque conserve une solution
            maîtresse lisible. C'est une précaution contre la triche interne, pas
            la preuve qu'aucune solution n'existe. Le puzzle a été bâti autour
            d'une solution ; ses auteurs ont seulement choisi de ne pas la garder
            sous les yeux.
          </>
        ),
      },
      {
        claim: "« Seul un ordinateur quantique pourrait le résoudre. »",
        verdict: "Un raccourci trompeur.",
        body: (
          <>
            Le quantique n'est pas une clé miracle pour ce type de recherche, et
            aucune machine quantique n'est nécessaire pour que l'affirmation « une
            solution existe » tienne. Eternity I, son prédécesseur de 1999, a été
            résolu en dix-huit mois environ par Alex Selby et Oliver Riordan sur
            des ordinateurs ordinaires, grâce à un meilleur raisonnement plutôt
            qu'à une puissance brute. Eternity II a ensuite été blindé contre les
            astuces mêmes qui avaient eu raison d'Eternity I : voilà pourquoi il
            est plus dur, non pourquoi il serait impossible.
          </>
        ),
      },
      {
        claim: "« C'est cruel, voire tout bonnement impossible. »",
        verdict: "Ni l'un ni l'autre.",
        body: (
          <>
            Le meilleur plateau partiel trouvé à ce jour apparie 470 des 480 côtés
            intérieurs (Joshua Blackwood, 2021). Dix côtés non appariés sur 480,
            c'est un écart mince, et les progrès sont bien réels : 467 en 2008, 469
            en 2020, 470 en 2021. Une recherche aussi ardue peut rester longtemps
            sans aboutir alors qu'une solution attend quelque part dans l'espace.
            Difficile ne veut pas dire impossible.
          </>
        ),
      },
    ] as Misconception[],
    verificationTitle: "Y avait-il une vraie vérification ?",
    verification: (
      <>
        Oui, et le procédé avait déjà fait ses preuves. Eternity I avait été
        contrôlé par des{" "}
        <a className="underline" href={msg(901)} target="_blank" rel="noreferrer">
          « experts en sinistres mandatés par les assureurs »
        </a>
        , et le prix avait été versé. Pour Eternity II, des dates annuelles
        d'examen (le « scrutineering ») étaient prévues pour juger les plateaux
        soumis (voir{" "}
        <a className="underline" href={msg(1129)} target="_blank" rel="noreferrer">
          les échanges des joueurs sur le calendrier d'examen
        </a>{" "}
        et{" "}
        <a className="underline" href={msg(1135)} target="_blank" rel="noreferrer">
          la suite
        </a>
        ). La règle « ne dites à personne que vous avez résolu le puzzle »
        (soulevée sur la liste par{" "}
        <a className="underline" href={msg(314)} target="_blank" rel="noreferrer">
          simon.chapple
        </a>
        ) protégeait le prix des fuites, et alimentait du même coup les soupçons.
        Une vraie solution avait donc un vrai chemin pour être vérifiée et payée ;
        personne n'en a produit avant l'échéance.
      </>
    ),
    distinctionTitle: "Peu de solutions par conception, ce n'est pas impossible",
    distinction: (
      <>
        Voilà le point à retenir. Selby et Riordan ont participé à la conception
        d'Eternity II justement pour lui ôter la profusion de quasi-solutions qui
        leur avait permis de battre Eternity I. Moins de solutions cibles, c'est
        une recherche qui n'a presque rien vers quoi converger : force brute comme
        heuristiques rusées errent alors dans un espace démesuré. La solution
        existe par construction ; ce qui manque, c'est une méthode assez rapide
        pour la trouver. D'où la célébrité du puzzle, et pourquoi ce n'est pas un
        piège.
      </>
    ),
    learnMoreTitle: "Pour aller plus loin",
    learnMoreMath: (
      <>
        Pour la combinatoire derrière ce 10<sup>557</sup>, la NP-complétude de
        l'appariement de côtés et la façon dont les solveurs attaquent réellement
        le plateau, voir{" "}
        <Link className="underline" to="/algorithms">
          Algorithmes
        </Link>{" "}
        et les pages{" "}
        <Link className="underline" to="/research">
          Recherche
        </Link>
        .
      </>
    ),
    learnMoreRecord: (
      <>
        Pour les meilleurs plateaux du moment, dont le record de 470/480, voir le
        tableau des records sur{" "}
        <Link className="underline" to="/puzzle">
          Le Puzzle
        </Link>
        .
      </>
    ),
  },
};

export default function ScamPage() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t.title}</h1>
        <p className="mt-2 max-w-3xl text-muted-foreground">{t.lede}</p>
      </div>

      <Card className="border-primary/40 bg-primary/5">
        <CardHeader className="pb-2">
          <CardTitle className="text-xl">{t.shortAnswerTitle}</CardTitle>
        </CardHeader>
        <CardContent className="max-w-3xl text-sm text-muted-foreground">
          {t.shortAnswer}
        </CardContent>
      </Card>

      <section className="space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.communityHookTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.communityHook}</p>
      </section>

      <section className="space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.misconceptionsTitle}</h2>
        <div className="grid gap-4 md:grid-cols-3">
          {t.misconceptions.map((m, i) => (
            <Card key={i} className="h-full">
              <CardHeader className="pb-2">
                <CardTitle className="text-base">{m.claim}</CardTitle>
                <div className="text-sm font-semibold text-primary">{m.verdict}</div>
              </CardHeader>
              <CardContent className="text-sm text-muted-foreground">{m.body}</CardContent>
            </Card>
          ))}
        </div>
      </section>

      <section className="space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.verificationTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.verification}</p>
      </section>

      <section className="space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.distinctionTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.distinction}</p>
      </section>

      <section className="space-y-3">
        <h2 className="text-2xl font-semibold tracking-tight">{t.learnMoreTitle}</h2>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.learnMoreMath}</p>
        <p className="max-w-3xl text-sm text-muted-foreground">{t.learnMoreRecord}</p>
      </section>
    </div>
  );
}

export const meta = pageMeta("scam");
