// Engine backend dispatcher.
//
// The website can be built against any of four interchangeable engine
// implementations, all of which expose the identical surface re-exported
// below:
//
//   VITE_ENGINE=rust  (default)  the canonical Rust engine -> WebAssembly
//   VITE_ENGINE=ts               a pure-TypeScript port (no WASM at all)
//   VITE_ENGINE=c                a C port -> WebAssembly (clang, freestanding)
//   VITE_ENGINE=cpp              a C++ port -> WebAssembly (clang++)
//
// `vite.config.ts` resolves the `virtual:engine-backend` import to the chosen
// backend module at build time, so the rest of the app imports from
// "@/engine" and is completely unaware of which engine it is running. See
// web/README.md for the switch and the engine ports themselves.
//
// All backends are validated byte-for-byte against the same Rust golden data
// (see ../../engine-side-quests/ALGORITHM.md §10).

export type { SolverHandle } from "@/lib/types";

export {
  initEngine,
  getOfficialPuzzle,
  getGeneratedPuzzle,
  getGeneratedSolvedPuzzle,
  getMaxColors,
  getPathKinds,
  getPath,
  getBoardScore,
  createSolver,
} from "virtual:engine-backend";
