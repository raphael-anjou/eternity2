// Google Analytics (GA4), loaded lazily so it stays off the critical render
// path. The gtag script used to sit in <head> and, even async, its inline
// config ran during initial parse and delayed first paint. Instead we inject it
// once the page is idle (or on the first user interaction, whichever comes
// first), after hydration. Page views are sent by the router (see PageTracking
// in layout.tsx); window.gtag safely no-ops until this has run.

const GA_ID = import.meta.env["VITE_GA_ID"];
const VALID = typeof GA_ID === "string" && /^G-[A-Z0-9]{4,}$/.test(GA_ID);

let started = false;

/** Inject the gtag script + base config exactly once. */
function inject(): void {
  if (started || !VALID) return;
  started = true;

  const w = window as unknown as {
    dataLayer: unknown[];
    gtag: (...args: unknown[]) => void;
  };
  w.dataLayer = w.dataLayer || [];
  // Match GA's own bootstrap: gtag pushes its argument list onto dataLayer.
  w.gtag = function gtag() {
    // eslint-disable-next-line prefer-rest-params
    w.dataLayer.push(arguments);
  };
  w.gtag("js", new Date());
  w.gtag("config", GA_ID, { anonymize_ip: true, send_page_view: false });

  const s = document.createElement("script");
  s.async = true;
  s.src = `https://www.googletagmanager.com/gtag/js?id=${GA_ID}`;
  document.head.appendChild(s);
}

/**
 * Schedule GA to load after the page is idle, or on the first user interaction,
 * whichever comes first. No-op when there is no valid measurement id (dev,
 * forks) or when called outside the browser. Returns a cleanup function.
 */
export function loadAnalyticsWhenIdle(): () => void {
  if (!VALID || typeof window === "undefined") return () => {};

  const events = ["pointerdown", "keydown", "scroll", "touchstart"] as const;
  let idleHandle: number | undefined;

  const go = () => {
    inject();
    for (const e of events) window.removeEventListener(e, go);
    if (idleHandle !== undefined) {
      const ric = (window as unknown as { cancelIdleCallback?: (h: number) => void })
        .cancelIdleCallback;
      ric?.(idleHandle);
    }
  };

  const ric = (
    window as unknown as {
      requestIdleCallback?: (cb: () => void, opts?: { timeout: number }) => number;
    }
  ).requestIdleCallback;
  if (ric) idleHandle = ric(go, { timeout: 4000 });
  else idleHandle = window.setTimeout(go, 2500);

  for (const e of events) window.addEventListener(e, go, { once: true, passive: true });

  return go;
}
