import { useCallback, useEffect, useRef, useState } from "react";
import type { SolverHandle, SolverReport } from "@/lib/types";

// Shared machinery for the animated / live-solver components embedded in the
// research pages. Two problems it solves:
//
//  1. Loops must not run while the reader can't see them: every demo on a page
//     ticking at once (or a hidden tab ticking forever) starves the main thread
//     and makes navigation clicks feel dead. `useRunWhileVisible` reports
//     whether a component is actually on screen in a visible tab, so its
//     animation effect can simply bail out (and resume seamlessly) on that flag.
//
//  2. Solver work must be time-boxed by wall clock, not by step counts: a fixed
//     "N steps per tick" can blow way past a frame on a slow machine. Loops
//     should slice work with `performance.now()` against `FRAME_BUDGET_MS` and
//     yield back to the browser between slices (`yieldToBrowser`).

/** Max synchronous work per slice, in milliseconds (~half a 60fps frame). */
export const FRAME_BUDGET_MS = 8;

/** Await a macrotask so the browser can process input / paint / navigation. */
export function yieldToBrowser(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

/**
 * Drive a step-able solver to completion off the main thread: run `step(batch)`
 * in `FRAME_BUDGET_MS`-bounded slices, yielding to the browser between slices so
 * input, paint and navigation stay responsive. Stops when the solver is no
 * longer running, when `cancelled()` flips (component unmounted, new run
 * started), or when a supplied cap is reached:
 *   - `keepGoing(report)` returns false — a cap on a report field (e.g. nodes);
 *   - `budgetCap` is exceeded — a cap on the total step budget *requested*
 *     (the sum of `batch` over all slices), matching the labs' old
 *     `spent += batch; spent < CAP` loops exactly.
 *
 * The live research labs share this loop; each supplies only its own batch size
 * and cap. The caller owns the solver's lifecycle (`free()`), since some want to
 * read `board()`/`score()` off it afterward.
 *
 * Returns the final report.
 */
export async function driveSolver(
  solver: SolverHandle,
  {
    batch,
    cancelled,
    keepGoing = () => true,
    budgetCap = Infinity,
  }: {
    batch: number;
    cancelled: () => boolean;
    keepGoing?: (report: SolverReport) => boolean;
    budgetCap?: number;
  },
): Promise<SolverReport> {
  let r = solver.report();
  let spent = 0;
  const running = () =>
    r.status === "running" && spent < budgetCap && keepGoing(r) && !cancelled();
  while (running()) {
    const deadline = performance.now() + FRAME_BUDGET_MS;
    do {
      r = solver.step(batch);
      spent += batch;
    } while (
      r.status === "running" &&
      spent < budgetCap &&
      keepGoing(r) &&
      performance.now() < deadline
    );
    if (running()) await yieldToBrowser();
  }
  return r;
}

/**
 * Reports whether the user has asked the OS to reduce motion
 * (`prefers-reduced-motion: reduce`), reacting to changes. `false` during SSR /
 * prerender and where matchMedia is unavailable.
 */
export function usePrefersReducedMotion(): boolean {
  const [reduced, setReduced] = useState(false);
  useEffect(() => {
    if (typeof matchMedia === "undefined") return;
    const mq = matchMedia("(prefers-reduced-motion: reduce)");
    const onChange = () => setReduced(mq.matches);
    onChange();
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, []);
  return reduced;
}

/**
 * Tracks whether a component is worth animating: its root element intersects
 * the viewport (IntersectionObserver) AND the tab is visible (visibilitychange).
 *
 * Attach the returned `ref` to the component's root element (it is a callback
 * ref, so it also works on elements that mount late, e.g. after a client-only
 * or engine-ready guard) and gate the animation effect on `visible`.
 *
 * Pass `{ respectReducedMotion: true }` for animations that auto-play (loops
 * that start on scroll-into-view, with no user gesture): they then also stop
 * when the user has asked the OS to reduce motion (WCAG 2.2.2). Leave it off for
 * animations the user explicitly starts with a control — reducing motion should
 * not disable a solver the reader just pressed "run" on.
 */
export function useRunWhileVisible(opts?: { respectReducedMotion?: boolean }): {
  ref: (el: Element | null) => void;
  visible: boolean;
} {
  const [inView, setInView] = useState(false);
  const [tabVisible, setTabVisible] = useState(true);
  const reducedMotion = usePrefersReducedMotion();
  const observerRef = useRef<IntersectionObserver | null>(null);

  const ref = useCallback((el: Element | null) => {
    observerRef.current?.disconnect();
    observerRef.current = null;
    if (!el) {
      setInView(false);
      return;
    }
    if (typeof IntersectionObserver === "undefined") {
      // Ancient browser / test environment: never block the animation.
      setInView(true);
      return;
    }
    const io = new IntersectionObserver((entries) => {
      setInView(entries.some((e) => e.isIntersecting));
    });
    io.observe(el);
    observerRef.current = io;
  }, []);

  useEffect(() => {
    const onChange = () => setTabVisible(document.visibilityState !== "hidden");
    onChange();
    document.addEventListener("visibilitychange", onChange);
    return () => {
      document.removeEventListener("visibilitychange", onChange);
      observerRef.current?.disconnect();
      observerRef.current = null;
    };
  }, []);

  const gateOnMotion = opts?.respectReducedMotion ? !reducedMotion : true;
  return { ref, visible: inView && tabVisible && gateOnMotion };
}
