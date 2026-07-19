// Old → new research URLs. The catch-all doc route consults this before
// rendering, so a moved URL lands on its new home instead of 404ing.
// Language-neutral paths (no /fr, no /es, no trailing slash), matching
// neutralPath() in the doc route; the route re-applies the visitor's language.
//
// The site's URLs were reshuffled heavily during the July 2026 rebuild, but the
// site had no external inbound links then, so those transient reorg URLs need no
// redirect — they're gone. The only entries kept are the two whose OLD path
// still names a page a reader might reach for: the retired "topics index" and
// its records theme, both of which now fold into an existing page.
export const RESEARCH_REDIRECTS: Record<string, string> = {
  // The "All themes" index duplicated the Overview page (which already lists every
  // topic hub in its left rail), so it was removed and its url now lands on
  // Overview. The per-topic hubs themselves stay, reachable from that rail.
  "/research/topics": "/research",
  // The records topic hub was just a list of links; the rich record timeline and
  // score chart already live at /research/records, so that url is where the old
  // hub now lands. The `records` tag stays valid on pages for search and metadata.
  "/research/topics/records": "/research/records",
};
