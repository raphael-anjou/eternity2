import { lazy, Suspense, useState } from "react";
import { MessageSquarePlus } from "lucide-react";
import { useT } from "@/i18n";
import { TRIGGER_CLASS, COPY } from "./feedback-shared";

// Floating "Feedback" button (bottom-left, every page). Opens a small popover
// that routes each kind of feedback straight to the matching GitHub issue form
// (.github/ISSUE_TEMPLATE/*.yml) in a new tab, so people can raise a bug, an
// idea or a content fix without hunting for the repo.
//
// The site has no backend — GitHub is the feedback inbox — so this is a set of
// pre-filled deep links, nothing is sent from the page itself.
//
// Perf: the popover uses base-ui (~30KB gzip) and is on every page including the
// landing page, but it only matters once someone clicks. So until the first
// click this renders a plain <button> (no base-ui in the critical path); the
// click swaps in the lazy-loaded FeedbackPopover, which opens itself.

const FeedbackPopover = lazy(() => import("./FeedbackPopover"));

export function FeedbackButton() {
  const t = useT(COPY);
  const [opened, setOpened] = useState(false);

  if (opened) {
    return (
      <Suspense fallback={null}>
        <FeedbackPopover t={t} />
      </Suspense>
    );
  }

  return (
    <button type="button" className={TRIGGER_CLASS} aria-label={t.trigger} onClick={() => setOpened(true)}>
      <MessageSquarePlus className="size-4" aria-hidden />
      <span className="hidden sm:inline">{t.trigger}</span>
    </button>
  );
}
