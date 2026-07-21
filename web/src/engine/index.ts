// The engine surface the whole app imports as "@/engine".
//
// The site runs the canonical Rust engine compiled to WebAssembly (via
// wasm-bindgen); this module simply re-exports it so the rest of the app never
// touches the backend module directly. Faithful reimplementations of this engine
// in other languages (TypeScript, C, C++, and more) live in the repo's
// `engine-ports/` collection, validated byte-for-byte against `golden.txt`; they
// are a study exhibit, not a build option.

export type { SolverHandle } from "@/lib/types";

export {
  initEngine,
  getOfficialPuzzle,
  getGeneratedPuzzle,
  getGeneratedSolvedPuzzle,
  getGeneratedPuzzleFramed,
  getGeneratedSolvedPuzzleFramed,
  getMaxColors,
  getPathKinds,
  getPath,
  getBoardScore,
  createSolver,
} from "./backends/rust.ts";
