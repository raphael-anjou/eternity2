import { useCallback, useEffect, useRef, useState } from "react";

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
 * Tracks whether a component is worth animating: its root element intersects
 * the viewport (IntersectionObserver) AND the tab is visible (visibilitychange).
 *
 * Attach the returned `ref` to the component's root element (it is a callback
 * ref, so it also works on elements that mount late, e.g. after a client-only
 * or engine-ready guard) and gate the animation effect on `visible`.
 */
export function useRunWhileVisible(): {
  ref: (el: Element | null) => void;
  visible: boolean;
} {
  const [inView, setInView] = useState(false);
  const [tabVisible, setTabVisible] = useState(true);
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

  return { ref, visible: inView && tabVisible };
}
