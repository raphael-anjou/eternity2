// Old → new research URLs, for pages that moved when the Build section was
// reorganised into per-technique-family sub-hubs (2026-07). The catch-all doc
// route consults this before rendering, so an old bookmark or external link
// lands on the new page instead of a 404. Language-neutral paths (no /fr, no
// trailing slash), matching neutralPath() in the doc route.
export const RESEARCH_REDIRECTS: Record<string, string> = {
  // The "All themes" index duplicated the Overview page (which already lists every
  // topic hub in its left rail), so it was removed and its url now lands on
  // Overview. The per-topic hubs themselves stay, reachable from that rail.
  "/research/topics": "/research",
  // The records topic hub was just a list of links; the rich record timeline and
  // score chart already live at /research/records, so that url is where the old
  // hub now lands. The `records` tag stays valid on pages for search and metadata.
  "/research/topics/records": "/research/records",
  // The engines section had a second, thinner CSP-presets page (a concept stub)
  // duplicating the measured one under single-core-benchmark. It was removed so
  // there is one canonical, leaderboard-backed page; its old url lands there.
  "/research/lab/experiments/raphael-anjou/engines/csp-presets":
    "/research/lab/experiments/single-core-benchmark/csp-presets",

  // Experiments re-homed under a per-author folder, so the lab opens to more
  // than one researcher (2026-07), then sorted into per-kind sub-hubs under
  // that author (runs / analyses / engines) so the sidebar can fold them.
  // Targets below point at the CURRENT url, so a pre-author bookmark still
  // lands in one hop.
  "/research/lab/experiments/prior": "/research/lab/experiments/raphael-anjou/pipelines/prior",
  "/research/lab/experiments/keyring": "/research/lab/experiments/raphael-anjou/pipelines/keyring",
  "/research/lab/experiments/gauntlet": "/research/lab/experiments/raphael-anjou/pipelines/gauntlet",
  "/research/lab/experiments/lodestone": "/research/lab/experiments/raphael-anjou/pipelines/lodestone",
  "/research/lab/experiments/staged": "/research/lab/experiments/raphael-anjou/pipelines/staged",
  "/research/lab/experiments/palimpsest": "/research/lab/experiments/raphael-anjou/pipelines/palimpsest",
  "/research/lab/experiments/ladder": "/research/lab/experiments/raphael-anjou/pipelines/ladder",
  "/research/lab/experiments/cloister": "/research/lab/experiments/raphael-anjou/pipelines/cloister",
  "/research/lab/experiments/midden": "/research/lab/experiments/raphael-anjou/pipelines/midden",
  "/research/lab/experiments/bandsaw": "/research/lab/experiments/raphael-anjou/analyses/bandsaw",
  "/research/lab/experiments/mosaic": "/research/lab/experiments/raphael-anjou/pipelines/mosaic",
  "/research/lab/experiments/replay": "/research/lab/experiments/raphael-anjou/analyses/replay",
  // …and from the flat per-author urls those pages held until the sub-hubs
  // landed (2026-07). Same pages, one level deeper.
  "/research/lab/experiments/raphael-anjou/prior": "/research/lab/experiments/raphael-anjou/pipelines/prior",
  "/research/lab/experiments/raphael-anjou/keyring": "/research/lab/experiments/raphael-anjou/pipelines/keyring",
  "/research/lab/experiments/raphael-anjou/gauntlet": "/research/lab/experiments/raphael-anjou/pipelines/gauntlet",
  "/research/lab/experiments/raphael-anjou/lodestone": "/research/lab/experiments/raphael-anjou/pipelines/lodestone",
  "/research/lab/experiments/raphael-anjou/staged": "/research/lab/experiments/raphael-anjou/pipelines/staged",
  "/research/lab/experiments/raphael-anjou/palimpsest": "/research/lab/experiments/raphael-anjou/pipelines/palimpsest",
  "/research/lab/experiments/raphael-anjou/ladder": "/research/lab/experiments/raphael-anjou/pipelines/ladder",
  "/research/lab/experiments/raphael-anjou/cloister": "/research/lab/experiments/raphael-anjou/pipelines/cloister",
  "/research/lab/experiments/raphael-anjou/midden": "/research/lab/experiments/raphael-anjou/pipelines/midden",
  "/research/lab/experiments/raphael-anjou/mosaic": "/research/lab/experiments/raphael-anjou/pipelines/mosaic",
  "/research/lab/experiments/raphael-anjou/bandsaw": "/research/lab/experiments/raphael-anjou/analyses/bandsaw",
  "/research/lab/experiments/raphael-anjou/replay": "/research/lab/experiments/raphael-anjou/analyses/replay",
  // The "runs" sub-hub was renamed to "pipelines" (2026-07): tagging each run
  // with its stages made clear these are multi-engine compositions, not single
  // algorithms, so the folder and section name now say so. Old /runs/ urls land
  // on the same page under /pipelines/.
  "/research/lab/experiments/raphael-anjou/runs": "/research/lab/experiments/raphael-anjou/pipelines",
  "/research/lab/experiments/raphael-anjou/runs/prior": "/research/lab/experiments/raphael-anjou/pipelines/prior",
  "/research/lab/experiments/raphael-anjou/runs/keyring": "/research/lab/experiments/raphael-anjou/pipelines/keyring",
  "/research/lab/experiments/raphael-anjou/runs/lodestone": "/research/lab/experiments/raphael-anjou/pipelines/lodestone",
  "/research/lab/experiments/raphael-anjou/runs/gauntlet": "/research/lab/experiments/raphael-anjou/pipelines/gauntlet",
  "/research/lab/experiments/raphael-anjou/runs/palimpsest": "/research/lab/experiments/raphael-anjou/pipelines/palimpsest",
  "/research/lab/experiments/raphael-anjou/runs/cloister": "/research/lab/experiments/raphael-anjou/pipelines/cloister",
  "/research/lab/experiments/raphael-anjou/runs/midden": "/research/lab/experiments/raphael-anjou/pipelines/midden",
  "/research/lab/experiments/raphael-anjou/runs/mosaic": "/research/lab/experiments/raphael-anjou/pipelines/mosaic",
  "/research/lab/experiments/raphael-anjou/runs/staged": "/research/lab/experiments/raphael-anjou/pipelines/staged",
  "/research/lab/experiments/raphael-anjou/runs/ladder": "/research/lab/experiments/raphael-anjou/pipelines/ladder",
  // The publishing methodology moved under Experiments (2026-07).
  "/research/lab/methodology": "/research/lab/experiments/methodology",
  // Pages moved into the History & community section (2026-07).
  "/research/lab/boards": "/research/community/boards",
  "/research/build/history": "/research/community/hunt",
  "/research/build/history-2": "/research/community/hunt-part-2",
  // Build reorg into per-technique-family sub-hubs.
  "/research/build/concepts": "/research/build/techniques",
  "/research/build/concepts/approaches-map": "/research/build/approaches-map",
  // Reduce the search
  "/research/build/concepts/arc-consistency": "/research/build/reduce/arc-consistency",
  "/research/build/concepts/alldiff-regin": "/research/build/reduce/alldiff-regin",
  "/research/build/concepts/nogood-learning": "/research/build/reduce/nogood-learning",
  "/research/build/concepts/edge-slipping": "/research/build/reduce/edge-slipping",
  // Backtracking
  "/research/build/concepts/fill-order": "/research/build/backtracking/fill-order",
  "/research/build/concepts/restarts": "/research/build/backtracking/restarts",
  // Go faster
  "/research/build/concepts/solver-engineering": "/research/build/faster/solver-engineering",
  "/research/build/concepts/distributed-solving": "/research/build/faster/distributed-solving",
  // Build boards up
  "/research/build/concepts/beam-search": "/research/build/construct/beam-search",
  // Local search
  "/research/build/concepts/local-search-alns": "/research/build/local-search/local-search-alns",
  "/research/build/concepts/parallel-tempering": "/research/build/local-search/parallel-tempering",
  "/research/build/concepts/evolutionary": "/research/build/local-search/evolutionary",
  // Exact methods
  "/research/build/concepts/sat-csp-encodings": "/research/build/exact/sat-csp-encodings",
  "/research/build/concepts/lp-relaxations": "/research/build/exact/lp-relaxations",
  "/research/build/concepts/exact-cover-dlx": "/research/build/exact/exact-cover-dlx",
  "/research/build/concepts/meet-in-the-middle": "/research/build/exact/meet-in-the-middle",
  "/research/build/concepts/iterated-maps": "/research/build/exact/iterated-maps",
  // Analysis
  "/research/build/concepts/parity-arguments": "/research/build/analysis/parity-arguments",
  "/research/build/concepts/solution-counting": "/research/build/analysis/solution-counting",
  // GPU & hardware
  "/research/build/concepts/gpu-solving": "/research/build/hardware/gpu-solving",
  "/research/build/concepts/fpga-solving": "/research/build/hardware/fpga-solving",
  "/research/build/concepts/quantum": "/research/build/hardware/quantum",
};
