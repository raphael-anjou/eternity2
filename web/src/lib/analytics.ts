// Google Analytics (GA4), loaded lazily so it stays off the critical render
// path. The gtag script used to sit in <head> and, even async, its inline
// config ran during initial parse and delayed first paint. Instead we inject it
// once the page is idle (or on the first user interaction, whichever comes
// first), after hydration. Page views are sent by the router (see PageTracking
// in layout.tsx).
//
// Events raised BEFORE the script loads are buffered here and flushed on
// injection, right after `config`. Without that buffer they were dropped
// outright: the app raises its first page_view during mount, but `window.gtag`
// only exists once inject() has run (idle, i.e. hundreds of ms later), so an
// optional-chained `window.gtag?.(...)` silently discarded it and the effect
// never retried. That lost the landing page view of EVERY session — GA then
// reported the second page as the landing page, and single-page sessions (most
// organic search traffic here) recorded nothing at all. Measured before the
// fix: "(not set)" was the single largest landing page, 641 of 2183 sessions.
//
// So: never call window.gtag directly from app code. Call track() / trackPageView(),
// which are safe before, during and after loading.

// The gtag/dataLayer globals GA installs on window, declared once so the whole
// app can read window.gtag without per-call casts. (requestIdleCallback and its
// canceller are standard DOM lib types; only these two need augmenting.)
declare global {
  interface Window {
    dataLayer?: unknown[];
    gtag?: (...args: unknown[]) => void;
  }
}

const GA_ID = import.meta.env["VITE_GA_ID"];
const VALID = typeof GA_ID === "string" && /^G-[A-Z0-9]{4,}$/.test(GA_ID);

let started = false;

/** Events raised before the GA script was injected, in order. Flushed by
 *  inject() after `config` (GA processes dataLayer in order, so an event pushed
 *  before its config would be attributed to no property). Bounded so a page that
 *  never loads GA — a bot, a blocked request, a user who leaves immediately —
 *  cannot grow it without limit. */
const pending: unknown[][] = [];
const MAX_PENDING = 50;

/** Inject the gtag script + base config exactly once, then flush any events
 *  raised while it was still loading. */
function inject(): void {
  if (started || !VALID) return;
  started = true;

  window.dataLayer = window.dataLayer || [];
  // Match GA's own bootstrap: gtag pushes its argument list onto dataLayer.
  window.gtag = function gtag() {
    // eslint-disable-next-line prefer-rest-params
    window.dataLayer?.push(arguments);
  };
  window.gtag("js", new Date());
  // send_page_view:false because the router sends one page_view per route
  // change itself (SPA navigations are invisible to GA's automatic pageview).
  window.gtag("config", GA_ID, { send_page_view: false });

  // Replay what the app raised before the script existed. Order matters and is
  // preserved; this is what keeps the landing page view.
  for (const args of pending) window.gtag(...args);
  pending.length = 0;

  const s = document.createElement("script");
  s.async = true;
  s.src = `https://www.googletagmanager.com/gtag/js?id=${GA_ID}`;
  document.head.appendChild(s);
}

/** Raise a GA event. Safe to call at any time: before the GA script has loaded
 *  the call is buffered and replayed on injection, after it the call goes
 *  straight through. No-op when analytics is disabled (dev, forks). */
export function track(name: string, params?: Record<string, unknown>): void {
  if (!VALID || typeof window === "undefined") return;
  const args: unknown[] = params ? ["event", name, params] : ["event", name];
  if (window.gtag) {
    window.gtag(...args);
  } else if (pending.length < MAX_PENDING) {
    pending.push(args);
  }
}

/** Raise a page_view for a route. `path` should be the canonical, trailing-slash
 *  form (matching <link rel="canonical"> and the sitemap) so GA's page paths
 *  agree with Search Console's rather than fragmenting into slashed and
 *  unslashed variants of the same page. */
export function trackPageView(path: string, location: string, title: string): void {
  track("page_view", { page_path: path, page_location: location, page_title: title });
}

/**
 * Schedule GA to load after the page is idle, or on the first user interaction,
 * whichever comes first. No-op when there is no valid measurement id (dev,
 * forks) or when called outside the browser. Returns a teardown that cancels
 * the pending schedule (it does NOT inject: injecting from a cleanup would load
 * analytics precisely when the component is going away).
 */
export function loadAnalyticsWhenIdle(): () => void {
  if (!VALID || typeof window === "undefined") return () => {};

  const events = ["pointerdown", "keydown", "scroll", "touchstart"] as const;
  // Track WHICH scheduler was used: cancelling a setTimeout id with
  // cancelIdleCallback (or vice versa) is a silent no-op on the browsers that
  // have only one of the two.
  let idleHandle: number | undefined;
  let timeoutHandle: number | undefined;

  const unschedule = () => {
    for (const e of events) window.removeEventListener(e, go);
    if (idleHandle !== undefined) window.cancelIdleCallback(idleHandle);
    if (timeoutHandle !== undefined) window.clearTimeout(timeoutHandle);
    idleHandle = timeoutHandle = undefined;
  };

  function go() {
    unschedule();
    inject();
  }

  if (typeof window.requestIdleCallback === "function") {
    // `timeout` is a hard guarantee: if no idle period comes up within it, the
    // callback runs anyway. So GA loads within ~4s even with zero interaction.
    idleHandle = window.requestIdleCallback(go, { timeout: 4000 });
  } else {
    timeoutHandle = window.setTimeout(go, 2500);
  }

  for (const e of events) window.addEventListener(e, go, { once: true, passive: true });

  return unschedule;
}
