import { describe, expect, it } from "vitest";
import { driveSolver } from "./useRunWhileVisible";
import type { SolverHandle, SolverReport } from "./types";

// A minimal fake solver: it "solves" after `stepsToSolve` calls to step(),
// counting the total budget requested. Only the SolverHandle surface driveSolver
// touches (report/step) is real; the rest throw if called.
function fakeSolver(stepsToSolve: number): SolverHandle & { calls: number; budget: number } {
  let calls = 0;
  let budget = 0;
  const mk = (status: SolverReport["status"], nodes: number): SolverReport => ({
    status,
    nodes,
    attempts: nodes,
    backtracks: 0,
    placed: nodes,
    bestPlaced: nodes,
  });
  const handle = {
    calls: 0,
    budget: 0,
    report: () => mk("running", calls),
    step(b: number) {
      calls += 1;
      budget += b;
      handle.calls = calls;
      handle.budget = budget;
      return mk(calls >= stepsToSolve ? "solved" : "running", calls);
    },
    board: () => new Int32Array(),
    bestBoard: () => new Int32Array(),
    score: () => 0,
    bestScore: () => 0,
    reset: () => {},
    free: () => {},
  };
  return handle;
}

describe("driveSolver", () => {
  it("steps until the solver reports solved", async () => {
    const s = fakeSolver(5);
    const r = await driveSolver(s, { batch: 1000, cancelled: () => false });
    expect(r.status).toBe("solved");
    expect(s.calls).toBe(5);
  });

  it("stops at budgetCap even if the solver never solves", async () => {
    const s = fakeSolver(1_000_000); // effectively never solves
    const r = await driveSolver(s, {
      batch: 1000,
      cancelled: () => false,
      budgetCap: 4000, // 4 steps' worth of budget
    });
    expect(r.status).toBe("running");
    // The loop stops once the requested budget reaches the cap: exactly 4 steps.
    expect(s.calls).toBe(4);
    expect(s.budget).toBe(4000);
  });

  it("honors a keepGoing cap on a report field", async () => {
    const s = fakeSolver(1_000_000);
    const r = await driveSolver(s, {
      batch: 1000,
      cancelled: () => false,
      keepGoing: (report) => report.nodes < 3, // nodes == call count here
    });
    expect(r.status).toBe("running");
    expect(s.calls).toBe(3);
  });

  it("stops promptly when cancelled", async () => {
    const s = fakeSolver(1_000_000);
    let n = 0;
    const r = await driveSolver(s, {
      batch: 1000,
      // cancel after the first slice's worth of stepping
      cancelled: () => {
        n += 1;
        return n > 1;
      },
    });
    expect(r.status).toBe("running");
  });
});
