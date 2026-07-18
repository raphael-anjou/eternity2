import { Card, CardContent } from "@/components/ui/card";
import { useT } from "@/i18n";

// Academic citations (authors, titles, venues, years, URLs) are not translated.
// Only the per-entry notes are localized, via T.notes, keyed by paper id.
// `tier` drives a small "how useful in practice" badge; `section` groups them.
type Tier = "foundational" | "practical" | "context";
type Section = "complexity" | "solvers" | "cp" | "global" | "crossdomain";

type Paper = {
  id: string;
  authors: string;
  year: number;
  title: string;
  venue: string;
  url: string;
  tier: Tier;
  section: Section;
};

// Curated from our research vault's academic bibliography (200+ volumes of
// notes) plus the community reading list. Ordered within each section roughly
// by influence on how people actually attack Eternity II.
const PAPERS: Paper[] = [
  // --- Complexity foundations ---
  {
    id: "demaine2007",
    authors: "E. D. Demaine, M. L. Demaine",
    year: 2007,
    title: "Jigsaw Puzzles, Edge Matching, and Polyomino Packing: Connections and Complexity",
    venue: "Graphs and Combinatorics 23",
    url: "https://erikdemaine.org/papers/Jigsaw_GC/",
    tier: "foundational",
    section: "complexity",
  },
  {
    id: "demaine2019",
    authors: "Z. Abel, E. D. Demaine, M. L. Demaine, et al.",
    year: 2019,
    title: "Edge Matching with Inequalities, Triangles, Unknown Shape, and Two Players",
    venue: "JCDCG³G 2019 · arXiv:2002.03887",
    url: "https://arxiv.org/abs/2002.03887",
    tier: "context",
    section: "complexity",
  },
  {
    id: "rotations2017",
    authors: "J. Bosboom, E. D. Demaine, et al.",
    year: 2017,
    title: "Edge-Matching Problems with Rotations",
    venue: "arXiv:1703.09421",
    url: "https://arxiv.org/abs/1703.09421",
    tier: "context",
    section: "complexity",
  },
  {
    id: "mateu2012",
    authors: "C. Ansótegui, R. Béjar, C. Fernández, C. Mateu",
    year: 2012,
    title: "On the hardness of solving edge matching puzzles as SAT or CSP problems",
    venue: "Constraints 18 (Springer)",
    url: "https://link.springer.com/article/10.1007/s10601-012-9128-9",
    tier: "foundational",
    section: "complexity",
  },
  {
    id: "ansotegui2008cp",
    authors: "C. Ansótegui, R. Béjar, C. Fernández, C. Mateu",
    year: 2008,
    title: "Edge Matching Puzzles as Hard SAT/CSP Benchmarks",
    venue: "CP 2008",
    url: "https://link.springer.com/chapter/10.1007/978-3-540-85958-1_39",
    tier: "context",
    section: "complexity",
  },
  {
    id: "bejar2009",
    authors: "R. Béjar, C. Fernàndez, C. Mateu, N. Pascual",
    year: 2009,
    title: "Bounding the Phase Transition on Edge Matching Puzzles",
    venue: "ISMVL 2009 (IEEE)",
    url: "https://ieeexplore.ieee.org/document/5010379/",
    tier: "context",
    section: "complexity",
  },
  {
    id: "benoist2008edges",
    authors: "T. Benoist",
    year: 2008,
    title: "How many edges can be shared by N square tiles on a board?",
    venue: "e-lab Research Report (Bouygues SA)",
    url: "https://groups.io/g/eternity2/files/Puzzle%20Theory/HowManyEdges.pdf",
    tier: "context",
    section: "complexity",
  },
  {
    id: "ansotegui2008ccia",
    authors: "C. Ansótegui, R. Béjar, C. Fernández, C. Mateu",
    year: 2008,
    title: "How Hard is a Commercial Puzzle: the Eternity II Challenge",
    venue: "CCIA 2008",
    url: "https://repositori.udl.cat/bitstreams/0b6533fe-54e5-4070-85fe-80f7d35837d8/download",
    tier: "context",
    section: "complexity",
  },

  // --- E2-specific solvers ---
  {
    id: "heule2008",
    authors: "M. J. H. Heule",
    year: 2008,
    title: "Solving edge-matching problems with satisfiability solvers",
    venue: "SAT 2008 workshop (Guangzhou)",
    url: "https://www.cs.cmu.edu/~mheule/publications/eternity.pdf",
    tier: "practical",
    section: "solvers",
  },
  {
    id: "benoist2008",
    authors: "T. Benoist, E. Bourreau",
    year: 2008,
    title: "Fast Global Filtering for Eternity II",
    venue: "Constraint Programming Letters 3",
    url: "https://hal-lirmm.ccsd.cnrs.fr/lirmm-00364330v1/document",
    tier: "practical",
    section: "solvers",
  },
  {
    id: "schaus2008",
    authors: "P. Schaus, Y. Deville",
    year: 2008,
    title: "Hybridization of CP and VLNS for Eternity II",
    venue: "JFPC 2008",
    url: "https://dial.uclouvain.be/pr/boreal/object/boreal:85275/datastream/PDF_01/view",
    tier: "practical",
    section: "solvers",
  },
  {
    id: "coelho2010",
    authors: "I. Coelho, B. Coelho, V. Coelho, M. Haddad, M. Souza, L. Ochi",
    year: 2010,
    title: "A general variable neighborhood search approach for the resolution of the Eternity II puzzle",
    venue: "META 2010 · reference implementation on GitHub",
    url: "https://github.com/anzdani/vnsEternity2",
    tier: "context",
    section: "solvers",
  },
  {
    id: "niang2011",
    authors: "A. Niang",
    year: 2011,
    title: "Solving the Eternity II Puzzle using Evolutionary Computing Techniques",
    venue: "MASc thesis, Concordia University",
    url: "https://spectrum.library.concordia.ca/id/eprint/7487/1/Niang_MASc_S2011.pdf",
    tier: "context",
    section: "solvers",
  },
  {
    id: "wang2010",
    authors: "W.-S. Wang, T.-C. Chiang",
    year: 2010,
    title: "Solving Eternity-II puzzles with a tabu search algorithm",
    venue: "META 2010",
    url: "https://web.ntnu.edu.tw/~tcchiang/publications/meta2010.htm",
    tier: "context",
    section: "solvers",
  },
  {
    id: "wauters2012",
    authors: "T. Wauters, W. Vancroonenburg, G. Vanden Berghe",
    year: 2012,
    title: "A guide-and-observe hyper-heuristic approach to the Eternity II puzzle",
    venue: "J. Math. Modelling and Algorithms 11",
    url: "https://link.springer.com/article/10.1007/s10852-012-9178-4",
    tier: "practical",
    section: "solvers",
  },
  {
    id: "salassa2017",
    authors: "F. Salassa, W. Vancroonenburg, T. Wauters, F. Della Croce, G. Vanden Berghe",
    year: 2017,
    title: "MILP and Max-Clique based heuristics for the Eternity II puzzle",
    venue: "arXiv:1709.00252",
    url: "https://arxiv.org/abs/1709.00252",
    tier: "practical",
    section: "solvers",
  },
  {
    id: "harris2018",
    authors: "G. Harris, B. J. Vanstone, A. Gepp",
    year: 2018,
    title: "Automatically Generating and Solving Eternity II Style Puzzles",
    venue: "IEA/AIE 2018, LNCS 10868",
    url: "https://research.bond.edu.au/en/publications/automatically-generating-and-solving-eternity-ii-style-puzzles",
    tier: "practical",
    section: "solvers",
  },

  // --- Constraint-programming machinery ---
  {
    id: "regin1994",
    authors: "J.-C. Régin",
    year: 1994,
    title: "A filtering algorithm for constraints of difference in CSPs",
    venue: "AAAI 1994",
    url: "https://cdn.aaai.org/AAAI/1994/AAAI94-055.pdf",
    tier: "foundational",
    section: "cp",
  },
  {
    id: "compacttable2016",
    authors: "J. Demeulenaere, R. Hartert, C. Lecoutre, et al.",
    year: 2016,
    title: "Compact-Table: Efficiently Filtering Table Constraints with Reversible Sparse Bit-Sets",
    venue: "CP 2016 · arXiv:1604.06641",
    url: "https://arxiv.org/abs/1604.06641",
    tier: "context",
    section: "cp",
  },

  // --- Global optimization (a different paradigm) ---
  {
    id: "kovalsky2014",
    authors: "S. Z. Kovalsky, R. Basri, D. Glasner",
    year: 2014,
    title: "Solving Jigsaw Puzzles with Linear Programming",
    venue: "preprint",
    url: "https://shaharkov.github.io/projects/GlobalPuzzles.pdf",
    tier: "context",
    section: "global",
  },

  // --- Cross-domain methods we've actually tried on E2 ---
  {
    id: "helsgaun2009",
    authors: "K. Helsgaun",
    year: 2009,
    title: "General k-opt submoves for the Lin–Kernighan TSP heuristic",
    venue: "Mathematical Programming Computation 1",
    url: "https://link.springer.com/article/10.1007/s12532-009-0004-6",
    tier: "context",
    section: "crossdomain",
  },
  {
    id: "gomes2004",
    authors: "C. P. Gomes, M. Sellmann",
    year: 2004,
    title: "Streamlined Constraint Reasoning",
    venue: "CP 2004, LNCS 3258",
    url: "https://www.cs.cornell.edu/gomes/pdf/2004_gomes_cp_streamlined.pdf",
    tier: "context",
    section: "crossdomain",
  },
  {
    id: "braunstein2005",
    authors: "A. Braunstein, M. Mézard, R. Zecchina",
    year: 2005,
    title: "Survey propagation: an algorithm for satisfiability",
    venue: "Random Structures & Algorithms 27 · arXiv:cs/0212002",
    url: "https://arxiv.org/abs/cs/0212002",
    tier: "context",
    section: "crossdomain",
  },
];

const SECTION_ORDER: Section[] = ["complexity", "solvers", "cp", "global", "crossdomain"];

const T = {
  en: {
    usefulnessTitle: "Which ones are actually useful?",
    usefulness: [
      [
        "If you want to understand the wall",
        "Start with Demaine & Demaine (the NP-completeness proof) and Ansótegui–Béjar–Fernández–Mateu — together they explain why no fast general algorithm is known and, crucially, why Eternity II's exact parameters were chosen: the puzzle sits at the SAT/CSP phase transition (≈17 interior colors) where instances have about one expected solution and are hardest to find.",
      ],
      [
        "If you want ideas that make a solver faster",
        "Benoist & Bourreau's O(1)-per-node global filtering and Schaus & Deville's CP + very-large-neighborhood search are the lineage behind most record boards. Wauters et al.'s hyper-heuristic and Salassa et al.'s MILP + max-clique formulations are the strongest published metaheuristics. Harris/Vanstone/Gepp lean on structural knowledge and the uniform color distribution rather than generic forward-checking, reporting up to three orders of magnitude over earlier solvers.",
      ],
      [
        "The constraint-propagation core",
        "If you implement real pruning, Régin's AllDifferent filtering (generalized arc consistency via bipartite matching) is the cornerstone — Eternity II's bounded per-color piece supply makes a per-color AllDifferent the most powerful global propagator. Compact-Table is the algorithm to reach for if you ever encode precomputed patch tables.",
      ],
      [
        "A different paradigm worth knowing",
        "Kovalsky–Basri–Glasner solve jigsaw assembly as one global linear program instead of sequential placement. It does NOT transfer to E2 directly (image compatibility is real-valued; E2 edges are discrete) — the authors say as much — but the global-optimization framing is a useful contrast to backtracking and the seed of any SDP-relaxation attempt.",
      ],
      [
        "A practical trick the papers under-sell",
        "Precomputing legal two-piece pairs (corner+edge, edge+interior, interior+interior) and searching over pairs instead of single pieces is a recurring community speedup: it front-loads the cheapest constraint check and shrinks the branching factor. It's folklore as much as literature, but worth knowing before you reinvent it.",
      ],
      [
        "Where the literature stops",
        "No published method actually solves the 16×16 board. SAT/CSP encodings empirically cap around 10×10 sub-puzzles; the best heuristics plateau well short of 480 matched edges. The community record (470/480, Blackwood 2021, using his own solver; tied since) beats every academic-published solver. The frontier today lives in the community (groups.io, Discord) and in open lab notebooks like this one — not in a paper.",
      ],
    ],
    sections: {
      complexity: "Complexity & why it's hard",
      solvers: "Eternity II solvers",
      cp: "Constraint-propagation machinery",
      global: "A different paradigm: global optimization",
      crossdomain: "Cross-domain methods we've tried on E2",
    } satisfies Record<Section, string>,
    tier: {
      foundational: "Foundational",
      practical: "Practical",
      context: "Context",
    } satisfies Record<Tier, string>,
    notes: {
      demaine2007:
        "Proves edge-matching puzzles NP-complete — the theoretical license for 'there is no known fast algorithm'.",
      demaine2019:
        "Maps the complexity of edge-matching variants (inequalities, triangles, rotations); useful when you consider relaxations.",
      rotations2017:
        "Edge matching stays NP-hard even with rotations and in restricted settings — closes off an obvious 'maybe rotations make it easy' hope.",
      mateu2012:
        "The phase-transition result: E2's ~17 interior / 5 border color split puts it exactly at the SAT/CSP hardness peak, with about one expected solution. This is *why* the parameters are what they are.",
      ansotegui2008cp:
        "Encodes Eternity-style puzzles for SAT/CSP solvers and shows generated instances sit right at the hardness peak.",
      bejar2009:
        "Bounds the phase transition analytically (via second-moment statistical measures), not just empirically: an upper bound that doubles as a good estimator for locating the hardest instances. The theory backbone under the 'E2 sits at the hardness peak' claim.",
      benoist2008edges:
        "A short, elegant result from the e-lab (same author as Fast Global Filtering): the exact maximum number of shared edges N square tiles can achieve on a board, with a closed-form h(N) and a proof. A necessary feasibility condition, and a clean building block for parity and solution-count arguments.",
      ansotegui2008ccia:
        "The Eternity II instance itself, dissected as a hard CSP benchmark — an early, accessible analysis of the real puzzle.",
      heule2008:
        "The canonical SAT encoding (piece-position-rotation variables; edge, uniqueness clauses). Ground truth that pure SAT caps near 10×10 sub-puzzles — not the path to 480.",
      benoist2008:
        "Constraint-propagation machinery purpose-built for E2: O(1)-per-node global filtering that prunes impossible regions early and cheaply. A direct ancestor of fast solvers.",
      schaus2008:
        "Constraint programming plus very-large-neighborhood repair — the family of methods behind most record boards.",
      coelho2010:
        "A general variable-neighborhood-search metaheuristic with five move types; robust across sizes but doesn't reach optimality.",
      wang2010:
        "A two-phase tabu search: solve the frame first, then the interior conditioned on it. A clean illustration of the border-first decomposition many solvers rely on.",
      niang2011:
        "A thorough thesis-length treatment of evolutionary/genetic approaches to E2 — useful as a survey of what GA-style methods can and can't do here.",
      wauters2012:
        "Hyper-heuristics: an algorithm that learns which of its own moves are working, then leans on them.",
      salassa2017:
        "Mixed-integer programming and clique-finding as repair tools for near-complete boards; also publishes new hard benchmark instances.",
      harris2018:
        "Generates authentic E2-style instances and solves them using structural knowledge and the uniform color distribution — reporting large speedups over earlier published solvers.",
      regin1994:
        "Generalized arc consistency for AllDifferent via maximum bipartite matching. The single most powerful propagator for E2: enforce it per color, where supply is bounded.",
      compacttable2016:
        "State-of-the-art table-constraint filtering via reversible sparse bit-sets. The algorithm to use if you encode precomputed patch tables.",
      kovalsky2014:
        "Assembles a whole jigsaw as one linear program rather than placing pieces in sequence. Doesn't transfer to E2 (continuous vs. discrete edges) but frames the global-optimization alternative to backtracking.",
      helsgaun2009:
        "LKH — the state-of-the-art Lin–Kernighan TSP heuristic with general k-opt moves. We built a 2D edge-matching variant (variable-depth swap chains); it lost to simulated-annealing repair on equal budget, but the idea is sound.",
      gomes2004:
        "Streamlining: impose conjectured structural regularities as hard constraints to collapse the search space (distinct from symmetry-breaking). Untried on E2 in earnest; a real open direction.",
      braunstein2005:
        "Survey propagation — message passing built for clustered solution spaces near the SAT phase transition. Plain belief propagation gives ~19% interior reduction on E2 but pure SP was refuted early; the backtracking-SP variant remains untested here.",
    } as Record<string, string>,
  },
  fr: {
    usefulnessTitle: "Lesquels sont vraiment utiles ?",
    usefulness: [
      [
        "Pour comprendre le mur",
        "Commencez par Demaine & Demaine (la preuve de NP-complétude) et Ansótegui–Béjar–Fernández–Mateu : ensemble, ils expliquent pourquoi aucun algorithme rapide général n'est connu et, surtout, pourquoi les paramètres exacts d'Eternity II ont été choisis : le puzzle se situe à la transition de phase SAT/CSP (≈17 couleurs intérieures), là où les instances n'ont qu'environ une solution attendue et sont les plus dures à trouver.",
      ],
      [
        "Pour des idées qui accélèrent un solveur",
        "Le filtrage global en O(1) par nœud de Benoist & Bourreau et la recherche CP + très grand voisinage de Schaus & Deville constituent la lignée derrière la plupart des plateaux records. L'hyper-heuristique de Wauters et al. et les formulations PLNE + clique maximale de Salassa et al. sont les meilleures métaheuristiques publiées. Harris/Vanstone/Gepp s'appuient sur la connaissance structurelle et la distribution uniforme des couleurs plutôt que sur du forward-checking générique, avec jusqu'à trois ordres de grandeur de mieux que les solveurs antérieurs.",
      ],
      [
        "Le cœur de la propagation de contraintes",
        "Si vous implémentez un vrai élagage, le filtrage AllDifferent de Régin (cohérence d'arc généralisée par couplage biparti) est la pierre angulaire — le stock borné de pièces par couleur d'Eternity II fait d'un AllDifferent par couleur le propagateur global le plus puissant. Compact-Table est l'algorithme à choisir si vous encodez un jour des tables de motifs précalculées.",
      ],
      [
        "Un autre paradigme à connaître",
        "Kovalsky–Basri–Glasner résolvent l'assemblage de puzzles comme un seul programme linéaire global plutôt que par placement séquentiel. Cela ne se transpose PAS directement à E2 (compatibilité d'image réelle vs bords discrets) — les auteurs le disent eux-mêmes — mais le cadrage en optimisation globale est un contraste utile au retour sur trace et le germe de toute tentative de relaxation SDP.",
      ],
      [
        "Une astuce pratique que les articles sous-estiment",
        "Précalculer les paires de deux pièces légales (coin+bord, bord+intérieur, intérieur+intérieur) et chercher sur des paires plutôt que sur des pièces isolées est une accélération récurrente dans la communauté : elle avance le contrôle de contrainte le moins cher et réduit le facteur de branchement. C'est autant du folklore que de la littérature, mais bon à savoir avant de le réinventer.",
      ],
      [
        "Là où la littérature s'arrête",
        "Aucune méthode publiée ne résout réellement le plateau 16×16. Les encodages SAT/CSP plafonnent empiriquement vers des sous-puzzles 10×10 ; les meilleures heuristiques stagnent bien en deçà de 480 bords appariés. Le record de la communauté (470/480, Blackwood 2021, avec son propre solveur ; égalé depuis) bat tous les solveurs publiés. La frontière se trouve aujourd'hui dans la communauté (groups.io, Discord) et dans des carnets de laboratoire ouverts comme celui-ci — pas dans un article.",
      ],
    ],
    sections: {
      complexity: "Complexité & pourquoi c'est dur",
      solvers: "Solveurs Eternity II",
      cp: "Mécanique de propagation de contraintes",
      global: "Un autre paradigme : l'optimisation globale",
      crossdomain: "Méthodes d'autres domaines testées sur E2",
    } satisfies Record<Section, string>,
    tier: {
      foundational: "Fondateur",
      practical: "Pratique",
      context: "Contexte",
    } satisfies Record<Tier, string>,
    notes: {
      demaine2007:
        "Prouve que les casse-tête d'assemblage par les bords sont NP-complets — la caution théorique du « on ne connaît aucun algorithme rapide ».",
      demaine2019:
        "Cartographie la complexité des variantes d'edge-matching (inégalités, triangles, rotations) ; utile quand on envisage des relaxations.",
      rotations2017:
        "L'edge-matching reste NP-difficile même avec rotations et dans des cadres restreints — referme l'espoir « les rotations rendent peut-être le problème facile ».",
      mateu2012:
        "Le résultat de transition de phase : le partage ≈17 couleurs intérieures / 5 de bord place E2 pile au pic de difficulté SAT/CSP, avec environ une solution attendue. C'est *pourquoi* les paramètres sont ce qu'ils sont.",
      ansotegui2008cp:
        "Traduit les puzzles de type Eternity pour les solveurs SAT/CSP et montre que les instances générées tombent pile au pic de difficulté.",
      bejar2009:
        "Borne la transition de phase de façon analytique (mesures statistiques du second moment), et pas seulement empirique : une borne supérieure qui sert aussi de bon estimateur pour situer les instances les plus dures. La charpente théorique sous l'affirmation « E2 est pile au pic de difficulté ».",
      benoist2008edges:
        "Un résultat court et élégant de l'e-lab (même auteur que Fast Global Filtering) : le nombre maximal exact d'arêtes partagées que N tuiles carrées peuvent atteindre sur un plateau, avec un h(N) en forme close et une preuve. Une condition nécessaire de faisabilité, et une brique nette pour les arguments de parité et de comptage de solutions.",
      ansotegui2008ccia:
        "L'instance Eternity II elle-même, disséquée comme benchmark CSP difficile — une analyse précoce et accessible du vrai puzzle.",
      heule2008:
        "L'encodage SAT canonique (variables pièce-position-rotation ; clauses de bord et d'unicité). Vérité de terrain : le SAT pur plafonne vers des sous-puzzles 10×10 — ce n'est pas la voie vers 480.",
      benoist2008:
        "Une mécanique de propagation taillée pour E2 : un filtrage global en O(1) par nœud qui écarte tôt et à bas coût les régions impossibles. Un ancêtre direct des solveurs rapides.",
      schaus2008:
        "Programmation par contraintes et réparation par très grands voisinages — la famille de méthodes derrière la plupart des plateaux records.",
      coelho2010:
        "Une métaheuristique générale de recherche à voisinage variable avec cinq types de mouvements ; robuste selon les tailles mais n'atteint pas l'optimalité.",
      wang2010:
        "Une recherche tabou en deux phases : résoudre d'abord le cadre, puis l'intérieur conditionné dessus. Une illustration nette de la décomposition « bord d'abord » sur laquelle reposent beaucoup de solveurs.",
      niang2011:
        "Un traitement approfondi (mémoire) des approches évolutionnaires/génétiques pour E2 — utile comme panorama de ce que les méthodes de type AG peuvent ou non faire ici.",
      wauters2012:
        "Les hyper-heuristiques : un algorithme qui apprend lesquels de ses propres coups portent leurs fruits, puis s'appuie dessus.",
      salassa2017:
        "PLNE et recherche de cliques comme outils de réparation pour les plateaux presque complets ; publie aussi de nouvelles instances-benchmark difficiles.",
      harris2018:
        "Génère des instances authentiques de type E2 et les résout en exploitant la connaissance structurelle et la distribution uniforme des couleurs — avec d'importants gains sur les solveurs publiés antérieurs.",
      regin1994:
        "Cohérence d'arc généralisée pour AllDifferent par couplage biparti maximal. Le propagateur le plus puissant pour E2 : à appliquer par couleur, là où le stock est borné.",
      compacttable2016:
        "Filtrage de contraintes en table à l'état de l'art via des bit-sets creux réversibles. L'algorithme à utiliser si vous encodez des tables de motifs précalculées.",
      kovalsky2014:
        "Assemble un puzzle entier comme un seul programme linéaire plutôt que pièce par pièce. Ne se transpose pas à E2 (bords continus vs discrets) mais incarne l'alternative en optimisation globale au retour sur trace.",
      helsgaun2009:
        "LKH — l'heuristique TSP Lin–Kernighan à l'état de l'art, avec mouvements k-opt généraux. Nous en avons bâti une variante 2D pour l'edge-matching (chaînes d'échanges à profondeur variable) ; elle perd contre la réparation par recuit simulé à budget égal, mais l'idée est saine.",
      gomes2004:
        "Streamlining : imposer des régularités structurelles conjecturées comme contraintes dures pour effondrer l'espace de recherche (distinct du symmetry-breaking). Jamais vraiment essayé sur E2 ; une vraie piste ouverte.",
      braunstein2005:
        "Survey propagation — passage de messages conçu pour les espaces de solutions en grappes près de la transition de phase SAT. La propagation de croyance simple donne ≈19 % de réduction intérieure sur E2 mais la SP pure a été réfutée tôt ; la variante backtracking-SP reste à tester ici.",
    } as Record<string, string>,
  },
  es: {
    usefulnessTitle: "¿Cuáles resultan realmente útiles?",
    usefulness: [
      [
        "Para entender el muro",
        "Empieza por Demaine & Demaine (la demostración de NP-completitud) y Ansótegui–Béjar–Fernández–Mateu: juntos explican por qué no se conoce ningún algoritmo rápido general y, sobre todo, por qué se eligieron los parámetros exactos de Eternity II: el puzzle se sitúa en la transición de fase SAT/CSP (≈17 colores interiores), donde las instancias tienen alrededor de una solución esperada y son las más difíciles de hallar.",
      ],
      [
        "Para ideas que aceleran un solucionador",
        "El filtrado global en O(1) por nodo de Benoist & Bourreau y la búsqueda CP + vecindario muy grande de Schaus & Deville son el linaje detrás de la mayoría de los tableros récord. La hiperheurística de Wauters et al. y las formulaciones MILP + clique máxima de Salassa et al. son las mejores metaheurísticas publicadas. Harris/Vanstone/Gepp se apoyan en el conocimiento estructural y en la distribución uniforme de colores en lugar del forward-checking genérico, y reportan hasta tres órdenes de magnitud sobre los solucionadores anteriores.",
      ],
      [
        "El núcleo de la propagación de restricciones",
        "Si implementas una poda de verdad, el filtrado AllDifferent de Régin (consistencia de arco generalizada mediante emparejamiento bipartito) es la piedra angular: el suministro acotado de piezas por color en Eternity II convierte un AllDifferent por color en el propagador global más potente. Compact-Table es el algoritmo al que recurrir si algún día codificas tablas de parches precalculadas.",
      ],
      [
        "Otro paradigma que conviene conocer",
        "Kovalsky–Basri–Glasner resuelven el ensamblaje de puzzles como un único programa lineal global en vez de por colocación secuencial. NO se traslada directamente a E2 (la compatibilidad de imagen es de valor real; las aristas de E2 son discretas) —los propios autores lo dicen—, pero el enfoque de optimización global es un contraste útil frente al backtracking y la semilla de cualquier intento de relajación SDP.",
      ],
      [
        "Un truco práctico que los artículos infravaloran",
        "Precalcular los pares de dos piezas legales (esquina+borde, borde+interior, interior+interior) y buscar sobre pares en lugar de sobre piezas sueltas es una aceleración recurrente en la comunidad: adelanta la comprobación de restricción más barata y reduce el factor de ramificación. Es tanto folclore como literatura, pero conviene conocerlo antes de reinventarlo.",
      ],
      [
        "Hasta dónde llega la literatura",
        "Ningún método publicado resuelve realmente el tablero de 16×16. Las codificaciones SAT/CSP topan empíricamente en torno a subpuzzles de 10×10; las mejores heurísticas se estancan muy por debajo de 480 aristas coincidentes. El récord de la comunidad (470/480, Blackwood 2021, con su propio solucionador; igualado desde entonces) supera a todos los solucionadores publicados en la academia. La frontera vive hoy en la comunidad (groups.io, Discord) y en cuadernos de laboratorio abiertos como este, no en un artículo.",
      ],
    ],
    sections: {
      complexity: "Complejidad y por qué es difícil",
      solvers: "Solucionadores de Eternity II",
      cp: "Maquinaria de propagación de restricciones",
      global: "Otro paradigma: la optimización global",
      crossdomain: "Métodos de otros dominios que hemos probado en E2",
    } satisfies Record<Section, string>,
    tier: {
      foundational: "Fundacional",
      practical: "Práctico",
      context: "Contexto",
    } satisfies Record<Tier, string>,
    notes: {
      demaine2007:
        "Demuestra que los puzzles de encaje de bordes son NP-completos: el aval teórico de «no se conoce ningún algoritmo rápido».",
      demaine2019:
        "Mapea la complejidad de las variantes de edge-matching (desigualdades, triángulos, rotaciones); útil cuando planteas relajaciones.",
      rotations2017:
        "El edge-matching sigue siendo NP-difícil incluso con rotaciones y en entornos restringidos: cierra la esperanza obvia de que «quizá las rotaciones lo vuelvan fácil».",
      mateu2012:
        "El resultado de transición de fase: el reparto de ≈17 colores interiores / 5 de borde coloca a E2 justo en el pico de dificultad SAT/CSP, con alrededor de una solución esperada. Este es el *porqué* de que los parámetros sean los que son.",
      ansotegui2008cp:
        "Codifica los puzzles de tipo Eternity para solucionadores SAT/CSP y muestra que las instancias generadas caen justo en el pico de dificultad.",
      bejar2009:
        "Acota la transición de fase de forma analítica (mediante medidas estadísticas de segundo momento), y no solo empírica: una cota superior que sirve además como buen estimador para localizar las instancias más difíciles. El armazón teórico bajo la afirmación de que «E2 está justo en el pico de dificultad».",
      benoist2008edges:
        "Un resultado breve y elegante del e-lab (mismo autor que Fast Global Filtering): el número máximo exacto de aristas compartidas que N piezas cuadradas pueden alcanzar en un tablero, con una h(N) en forma cerrada y una demostración. Una condición necesaria de factibilidad y un ladrillo limpio para los argumentos de paridad y conteo de soluciones.",
      ansotegui2008ccia:
        "La propia instancia de Eternity II, diseccionada como benchmark CSP difícil: un análisis temprano y accesible del puzzle real.",
      heule2008:
        "La codificación SAT canónica (variables pieza-posición-rotación; cláusulas de borde y de unicidad). Verdad de terreno: el SAT puro topa cerca de subpuzzles de 10×10, no es el camino hacia 480.",
      benoist2008:
        "Maquinaria de propagación diseñada a medida para E2: un filtrado global en O(1) por nodo que descarta pronto y a bajo coste las regiones imposibles. Un ancestro directo de los solucionadores rápidos.",
      schaus2008:
        "Programación con restricciones más reparación por vecindarios muy grandes: la familia de métodos detrás de la mayoría de los tableros récord.",
      coelho2010:
        "Una metaheurística general de búsqueda por vecindario variable con cinco tipos de movimiento; robusta a distintos tamaños, pero no alcanza la optimalidad.",
      wang2010:
        "Una búsqueda tabú en dos fases: resolver primero el marco y luego el interior condicionado a él. Una ilustración clara de la descomposición «borde primero» en la que se apoyan muchos solucionadores.",
      niang2011:
        "Un tratamiento extenso (a nivel de tesina) de los enfoques evolutivos/genéticos para E2, útil como panorámica de lo que los métodos de tipo AG pueden y no pueden hacer aquí.",
      wauters2012:
        "Hiperheurísticas: un algoritmo que aprende cuáles de sus propios movimientos funcionan y luego se apoya en ellos.",
      salassa2017:
        "Programación entera mixta y búsqueda de cliques como herramientas de reparación para tableros casi completos; publica además nuevas instancias-benchmark difíciles.",
      harris2018:
        "Genera instancias auténticas de tipo E2 y las resuelve aprovechando el conocimiento estructural y la distribución uniforme de colores, reportando grandes aceleraciones sobre solucionadores publicados anteriormente.",
      regin1994:
        "Consistencia de arco generalizada para AllDifferent mediante emparejamiento bipartito máximo. El propagador más potente para E2: aplícalo por color, donde el suministro está acotado.",
      compacttable2016:
        "Filtrado de restricciones en tabla al estado del arte mediante conjuntos de bits dispersos reversibles. El algoritmo a usar si codificas tablas de parches precalculadas.",
      kovalsky2014:
        "Ensambla un puzzle entero como un único programa lineal en vez de colocar las piezas en secuencia. No se traslada a E2 (aristas continuas frente a discretas), pero enmarca la alternativa de optimización global al backtracking.",
      helsgaun2009:
        "LKH: la heurística TSP de Lin–Kernighan al estado del arte, con movimientos k-opt generales. Construimos una variante 2D para el edge-matching (cadenas de intercambio de profundidad variable); perdió frente a la reparación por recocido simulado con igual presupuesto, pero la idea es sólida.",
      gomes2004:
        "Streamlining: imponer regularidades estructurales conjeturadas como restricciones duras para colapsar el espacio de búsqueda (distinto del symmetry-breaking). Sin probar en serio en E2; una dirección abierta real.",
      braunstein2005:
        "Survey propagation: paso de mensajes concebido para espacios de solución agrupados cerca de la transición de fase SAT. La propagación de creencias simple da ≈19 % de reducción interior en E2, pero la SP pura fue refutada pronto; la variante backtracking-SP sigue sin probarse aquí.",
    } as Record<string, string>,
  },
};

const TIER_CLASS: Record<Tier, string> = {
  foundational: "bg-amber-100 text-amber-900 dark:bg-amber-950 dark:text-amber-200",
  practical: "bg-emerald-100 text-emerald-900 dark:bg-emerald-950 dark:text-emerald-200",
  context: "bg-muted text-muted-foreground",
};

function PaperCard({ p, note, tierLabel }: { p: Paper; note: string; tierLabel: string }) {
  return (
    <a href={p.url} target="_blank" rel="noreferrer" className="group">
      <Card className="transition-shadow group-hover:shadow-md">
        <CardContent className="pt-4">
          <div className="flex flex-wrap items-baseline gap-x-2">
            <span
              className={`rounded px-1.5 py-0.5 text-[10px] font-medium uppercase tracking-wide ${TIER_CLASS[p.tier]}`}
            >
              {tierLabel}
            </span>
            <span className="font-medium group-hover:underline">{p.title}</span>
            <span className="text-xs text-muted-foreground">
              {p.authors} · {p.venue} · {p.year}
            </span>
          </div>
          <p className="mt-1 text-sm text-muted-foreground">{note}</p>
        </CardContent>
      </Card>
    </a>
  );
}

export function PapersView() {
  const t = useT(T);
  return (
    <div className="space-y-12">
      <section className="max-w-3xl space-y-4">
        <h2 className="text-2xl font-semibold tracking-tight">{t.usefulnessTitle}</h2>
        <dl className="space-y-3 text-sm">
          {t.usefulness.map(([term, desc]) => (
            <div key={term}>
              <dt className="font-medium">{term}</dt>
              <dd className="mt-0.5 text-muted-foreground">{desc}</dd>
            </div>
          ))}
        </dl>
      </section>

      {SECTION_ORDER.map((sec) => {
        const items = PAPERS.filter((p) => p.section === sec);
        if (items.length === 0) return null;
        return (
          <section key={sec} className="space-y-4">
            <h2 className="text-2xl font-semibold tracking-tight">{t.sections[sec]}</h2>
            <div className="grid gap-3">
              {items.map((p) => (
                <PaperCard key={p.id} p={p} note={t.notes[p.id] ?? ""} tierLabel={t.tier[p.tier]} />
              ))}
            </div>
          </section>
        );
      })}
    </div>
  );
}
