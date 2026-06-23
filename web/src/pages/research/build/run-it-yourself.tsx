import { pageMeta } from "@/seo";
import { useT } from "@/i18n";
import { LocalizedLink } from "@/components/LocalizedLink";

const REPO_URL = "https://github.com/raphael-anjou/eternity2";

function Cmd({ children }: { children: string }) {
  return (
    <pre className="overflow-x-auto rounded-md border bg-muted/40 p-3 text-xs leading-relaxed">
      <code>{children}</code>
    </pre>
  );
}

const T = {
  en: {
    backLabel: "← Build a solver",
    title: "Run it yourself",
    lede: "The whole site, the engine, and every result in this section run from one repository. Here is how to get it going and reproduce the numbers.",
    cloneTitle: "Get the code",
    cloneBody: "Clone the repository. Everything lives here: the Rust engine, the website, and the research topics with their compute scripts.",
    needTitle: "What you need",
    needBody: "Three toolchains. The engine is Rust, the site is a Node app, and the engine ships to the browser as WebAssembly.",
    needList: [
      ["Rust", "stable, with the wasm32-unknown-unknown target"],
      ["Node.js 22+ and pnpm", "for the website"],
      ["wasm-pack", "to compile the engine to WebAssembly"],
      ["just (optional)", "a small task runner that wraps the commands below"],
    ] as [string, string][],
    runTitle: "Build and run the site",
    runBody: "With just installed, two commands. The first builds the engine to WebAssembly and installs the web dependencies; the second starts the site at localhost:5173 with hot reload.",
    runManualNote: "Prefer to run things directly? Every just recipe is a one-line wrapper, so you can skip just entirely:",
    engineTitle: "Work on the engine",
    engineBody: "The engine is a normal Rust crate that also compiles to WebAssembly. Run its tests, then rebuild the WASM whenever you change it. The tests include cross-checks against real community boards, so they catch any change to the piece set, rotation, or scoring.",
    reproTitle: "Reproduce a research result",
    reproBody: "Each topic in this section is self-contained: an article, a compute script, and the committed output it produces. Re-running the script regenerates that output. Deterministic results come back byte-for-byte identical; runs that depend on randomness or take hours say so plainly, and ship the board they found so you can still check it in the viewer.",
    reproExample: "For example, the forbidden-patterns counts (about twenty seconds):",
    checkTitle: "Run every check",
    checkBody: "One command runs what continuous integration runs: engine tests, type-check, lint, and the production build.",
    repoLink: "Browse the repository on GitHub",
  },
  fr: {
    backLabel: "← Construire un solveur",
    title: "À vous de jouer",
    lede: "Tout le site, le moteur et chaque résultat de cette section proviennent d'un seul dépôt. Voici comment le lancer et reproduire les chiffres.",
    cloneTitle: "Récupérer le code",
    cloneBody: "Clonez le dépôt. Tout s'y trouve : le moteur Rust, le site web et les sujets de recherche avec leurs scripts de calcul.",
    needTitle: "Ce qu'il vous faut",
    needBody: "Trois chaînes d'outils. Le moteur est en Rust, le site est une application Node, et le moteur est livré au navigateur sous forme de WebAssembly.",
    needList: [
      ["Rust", "stable, avec la cible wasm32-unknown-unknown"],
      ["Node.js 22+ et pnpm", "pour le site web"],
      ["wasm-pack", "pour compiler le moteur en WebAssembly"],
      ["just (optionnel)", "un petit lanceur de tâches qui enrobe les commandes ci-dessous"],
    ] as [string, string][],
    runTitle: "Compiler et lancer le site",
    runBody: "Avec just installé, deux commandes. La première compile le moteur en WebAssembly et installe les dépendances web ; la seconde démarre le site sur localhost:5173 avec rechargement à chaud.",
    runManualNote: "Vous préférez lancer les choses directement ? Chaque recette just tient en une ligne, vous pouvez donc vous en passer :",
    engineTitle: "Travailler sur le moteur",
    engineBody: "Le moteur est un crate Rust classique qui se compile aussi en WebAssembly. Lancez ses tests, puis recompilez le WASM dès que vous le modifiez. Les tests incluent des vérifications croisées avec de vrais plateaux de la communauté : ils repèrent tout changement du jeu de pièces, de la rotation ou du score.",
    reproTitle: "Reproduire un résultat de recherche",
    reproBody: "Chaque sujet de cette section est autonome : un article, un script de calcul et la sortie qu'il produit, archivée. Relancer le script régénère cette sortie. Les résultats déterministes reviennent identiques au bit près ; les exécutions qui dépendent du hasard ou durent des heures le disent clairement, et fournissent le plateau trouvé pour que vous puissiez quand même le vérifier dans le visualiseur.",
    reproExample: "Par exemple, les comptages des motifs interdits (une vingtaine de secondes) :",
    checkTitle: "Lancer toutes les vérifications",
    checkBody: "Une commande exécute ce que fait l'intégration continue : tests du moteur, vérification de types, lint et build de production.",
    repoLink: "Parcourir le dépôt sur GitHub",
  },
};

export default function RunItYourself() {
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
        <p className="mt-3 max-w-3xl text-lg text-muted-foreground">{t.lede}</p>
      </div>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.cloneTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.cloneBody}</p>
        <Cmd>{`git clone https://github.com/raphael-anjou/eternity2
cd eternity2`}</Cmd>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.needTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.needBody}</p>
        <ul className="space-y-1 text-sm text-muted-foreground">
          {t.needList.map(([name, note]) => (
            <li key={name}>
              <span className="font-medium text-foreground">{name}</span> — {note}
            </li>
          ))}
        </ul>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.runTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.runBody}</p>
        <Cmd>{`just setup
just dev`}</Cmd>
        <p className="text-sm text-muted-foreground">{t.runManualNote}</p>
        <Cmd>{`# build the engine to WebAssembly
cd engine && wasm-pack build --target web --out-dir ../web/src/engine/pkg --release
# install and run the site
cd ../web && pnpm install && pnpm dev`}</Cmd>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.engineTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.engineBody}</p>
        <Cmd>{`just test    # cargo test --release
just wasm    # rebuild the WebAssembly after a change`}</Cmd>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.reproTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.reproBody}</p>
        <p className="text-sm text-muted-foreground">{t.reproExample}</p>
        <Cmd>{`just research-forbidden-patterns
# or directly:
cd research/topics/forbidden-patterns/compute
cargo run --release > ../results/feasibility.json`}</Cmd>
      </section>

      <section className="max-w-3xl space-y-3">
        <h2 className="text-xl font-semibold tracking-tight">{t.checkTitle}</h2>
        <p className="text-sm text-muted-foreground">{t.checkBody}</p>
        <Cmd>{`just check   # engine tests, typecheck, lint, build`}</Cmd>
      </section>

      <section className="max-w-3xl">
        <a
          href={REPO_URL}
          target="_blank"
          rel="noreferrer"
          className="text-sm font-medium underline hover:text-foreground"
        >
          {t.repoLink}
        </a>
      </section>
    </div>
  );
}

export const meta = pageMeta("run-it-yourself");
