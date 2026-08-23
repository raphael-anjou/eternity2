// The regression this file exists for: events raised BEFORE the gtag script
// loads used to be dropped on the floor, which silently lost the landing page
// view of every session (measured: "(not set)" was the largest landing page in
// GA, 641 of 2183 sessions). track() must buffer until injection and replay in
// order, so an early page_view survives.
//
// analytics.ts reads VITE_GA_ID at module load, so each test imports it fresh
// via vi.resetModules() after stubbing the env.

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";

import type * as AnalyticsModule from "./analytics";

type Analytics = typeof AnalyticsModule;

// The suite runs in vitest's `node` environment (see vitest.config.ts: the unit
// tests deliberately avoid a DOM dependency). analytics.ts only needs a handful
// of window/document members, so we stub exactly those rather than pulling in
// jsdom for one file. requestIdleCallback is deliberately absent so the module
// takes its setTimeout path; injection is driven by the interaction listener,
// which is what a real first-interaction load does anyway.
type Listener = () => void;

function installWindow(): void {
  const listeners = new Map<string, Set<Listener>>();
  const win = {
    dataLayer: undefined as unknown[] | undefined,
    gtag: undefined as ((...a: unknown[]) => void) | undefined,
    addEventListener(type: string, fn: Listener) {
      const set = listeners.get(type) ?? new Set<Listener>();
      set.add(fn);
      listeners.set(type, set);
    },
    removeEventListener(type: string, fn: Listener) {
      listeners.get(type)?.delete(fn);
    },
    dispatchEvent(type: string) {
      for (const fn of [...(listeners.get(type) ?? [])]) fn();
    },
    setTimeout: (() => 1) as unknown as typeof setTimeout,
    clearTimeout: (() => {}) as unknown as typeof clearTimeout,
  };
  const doc = { createElement: () => ({} as Record<string, unknown>), head: { appendChild: () => {} } };
  Object.assign(globalThis, { window: win, document: doc });
}

/** The stub window, typed for assertions. */
function w(): { dataLayer?: unknown[]; gtag?: unknown; dispatchEvent(t: string): void } {
  return globalThis.window as unknown as ReturnType<typeof w>;
}

/** Every gtag call recorded on dataLayer, as plain arrays. */
function calls(): unknown[][] {
  return (w().dataLayer ?? []).map((a) => Array.from(a as ArrayLike<unknown>));
}

/** Fresh module instance with a valid measurement id. */
async function load(): Promise<Analytics> {
  vi.stubEnv("VITE_GA_ID", "G-TEST1234");
  vi.resetModules();
  return import("./analytics");
}

beforeEach(() => {
  installWindow();
});

afterEach(() => {
  vi.unstubAllEnvs();
  vi.restoreAllMocks();
  Reflect.deleteProperty(globalThis, "window");
  Reflect.deleteProperty(globalThis, "document");
});

describe("track() before the GA script has loaded", () => {
  it("buffers the event instead of dropping it, and replays it on injection", async () => {
    const { track, loadAnalyticsWhenIdle } = await load();

    // Nothing has injected yet: this is the exact moment the landing page view
    // is raised, and window.gtag does not exist.
    expect(w().gtag).toBeUndefined();
    track("page_view", { page_path: "/start/" });

    // Injection happens on the first user interaction.
    loadAnalyticsWhenIdle();
    w().dispatchEvent("pointerdown");

    expect(w().gtag).toBeDefined();
    const c = calls();
    // config must come before the replayed event, or GA attributes it to no
    // property.
    const configAt = c.findIndex((x) => x[0] === "config");
    const eventAt = c.findIndex((x) => x[0] === "event" && x[1] === "page_view");
    expect(configAt).toBeGreaterThanOrEqual(0);
    expect(eventAt).toBeGreaterThan(configAt);
    expect(c[eventAt]?.[2]).toEqual({ page_path: "/start/" });
  });

  it("preserves the order of several buffered events", async () => {
    const { track, loadAnalyticsWhenIdle } = await load();
    track("page_view", { page_path: "/" });
    track("language_switch", { from: "en", to: "fr" });

    loadAnalyticsWhenIdle();
    w().dispatchEvent("pointerdown");

    const names = calls()
      .filter((c) => c[0] === "event")
      .map((c) => c[1]);
    expect(names).toEqual(["page_view", "language_switch"]);
  });

  it("passes straight through once injected, without re-buffering", async () => {
    const { track, loadAnalyticsWhenIdle } = await load();
    loadAnalyticsWhenIdle();
    w().dispatchEvent("pointerdown");

    track("solver_run", { size: 6 });
    const events = calls().filter((c) => c[0] === "event");
    expect(events).toHaveLength(1);
    expect(events[0]?.[1]).toBe("solver_run");
  });

  it("bounds the buffer so a page that never loads GA cannot grow it forever", async () => {
    const { track, loadAnalyticsWhenIdle } = await load();
    for (let i = 0; i < 200; i++) track("page_view", { i });

    loadAnalyticsWhenIdle();
    w().dispatchEvent("pointerdown");

    const events = calls().filter((c) => c[0] === "event");
    expect(events.length).toBeLessThanOrEqual(50);
  });
});

describe("teardown", () => {
  it("cancels the pending schedule instead of injecting", async () => {
    const { loadAnalyticsWhenIdle } = await load();
    const teardown = loadAnalyticsWhenIdle();
    teardown();

    // The old code returned the injector itself as its "cleanup", so unmounting
    // loaded analytics rather than tearing it down.
    expect(w().gtag).toBeUndefined();

    // And the listeners are gone, so a later interaction does not inject either.
    w().dispatchEvent("pointerdown");
    expect(w().gtag).toBeUndefined();
  });
});

describe("when no measurement id is configured (dev, forks)", () => {
  it("does nothing at all", async () => {
    vi.stubEnv("VITE_GA_ID", "");
    vi.resetModules();
    const { track, loadAnalyticsWhenIdle } = await import("./analytics");

    track("page_view", { page_path: "/" });
    loadAnalyticsWhenIdle();
    w().dispatchEvent("pointerdown");

    expect(w().gtag).toBeUndefined();
    expect(w().dataLayer).toBeUndefined();
  });
});
