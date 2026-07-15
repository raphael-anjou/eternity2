import { Popover } from "@base-ui/react/popover";
import { Bug, Lightbulb, BookOpen, MessageSquarePlus, ArrowUpRight } from "lucide-react";
import type { ComponentType } from "react";
import { REPO_URL } from "@/site";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";

// Floating "Feedback" button (bottom-left, every page). Opens a small popover
// that routes each kind of feedback straight to the matching GitHub issue form
// (.github/ISSUE_TEMPLATE/*.yml) in a new tab, so people can raise a bug, an
// idea or a content fix without hunting for the repo. "Something else" drops
// them on GitHub's own issue chooser (blank issue + Discussions link).
//
// The site has no backend — GitHub is the feedback inbox — so this is a set of
// pre-filled deep links, nothing is sent from the page itself.

const newIssue = (template: string) =>
  `${REPO_URL}/issues/new?template=${template}`;

type Item = {
  icon: ComponentType<{ className?: string }>;
  href: string;
  title: string;
  desc: string;
};

const COPY = {
  en: {
    trigger: "Feedback",
    heading: "Spotted something?",
    sub: "Everything opens a pre-filled issue on GitHub.",
    items: (): Item[] => [
      {
        icon: Bug,
        href: newIssue("bug_report.yml"),
        title: "Report a bug",
        desc: "Something's broken or wrong",
      },
      {
        icon: Lightbulb,
        href: newIssue("idea.yml"),
        title: "Suggest an idea",
        desc: "A feature or improvement",
      },
      {
        icon: BookOpen,
        href: newIssue("content_correction.yml"),
        title: "Fix the content",
        desc: "A research-wiki error or typo",
      },
    ],
    more: "Something else — all options on GitHub",
  },
  fr: {
    trigger: "Votre avis",
    heading: "Un souci, une idée ?",
    sub: "Chaque choix ouvre un ticket pré-rempli sur GitHub.",
    items: (): Item[] => [
      {
        icon: Bug,
        href: newIssue("bug_report.yml"),
        title: "Signaler un bug",
        desc: "Quelque chose ne marche pas",
      },
      {
        icon: Lightbulb,
        href: newIssue("idea.yml"),
        title: "Proposer une idée",
        desc: "Une fonctionnalité ou une amélioration",
      },
      {
        icon: BookOpen,
        href: newIssue("content_correction.yml"),
        title: "Corriger le contenu",
        desc: "Une erreur ou coquille du wiki",
      },
    ],
    more: "Autre chose — toutes les options sur GitHub",
  },
};

export function FeedbackButton() {
  const t = useT(COPY);
  const items = t.items();

  return (
    <Popover.Root>
      <Popover.Trigger
        className={cn(
          "fixed bottom-4 left-4 z-40 inline-flex items-center gap-2 rounded-full border",
          "bg-background/90 px-3.5 py-2 text-sm font-medium text-muted-foreground shadow-md backdrop-blur",
          "transition-colors hover:bg-muted hover:text-foreground",
          "focus-visible:outline-none focus-visible:ring-3 focus-visible:ring-ring/50",
          "print:hidden",
        )}
        aria-label={t.trigger}
      >
        <MessageSquarePlus className="size-4" aria-hidden />
        <span className="hidden sm:inline">{t.trigger}</span>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Positioner side="top" align="start" sideOffset={8} className="z-50">
          <Popover.Popup
            className={cn(
              "w-72 max-w-[calc(100vw-2rem)] origin-(--transform-origin) rounded-xl bg-popover p-2 text-popover-foreground shadow-lg ring-1 ring-foreground/10",
              "data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95",
              "data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95",
            )}
          >
            <div className="px-2 pt-1.5 pb-2">
              <p className="text-sm font-semibold text-foreground">{t.heading}</p>
              <p className="mt-0.5 text-xs text-muted-foreground">{t.sub}</p>
            </div>
            <div className="flex flex-col">
              {items.map((item) => (
                <a
                  key={item.href}
                  href={item.href}
                  target="_blank"
                  rel="noreferrer"
                  className="flex items-start gap-3 rounded-lg px-2 py-2 text-left transition-colors hover:bg-muted"
                >
                  <item.icon className="mt-0.5 size-4 shrink-0 text-muted-foreground" aria-hidden />
                  <span className="min-w-0">
                    <span className="block text-sm font-medium text-foreground">{item.title}</span>
                    <span className="block text-xs text-muted-foreground">{item.desc}</span>
                  </span>
                </a>
              ))}
            </div>
            <a
              href={`${REPO_URL}/issues/new/choose`}
              target="_blank"
              rel="noreferrer"
              className="mt-1 flex items-center gap-1.5 border-t px-2 pt-2 pb-1 text-xs text-muted-foreground transition-colors hover:text-foreground"
            >
              {t.more}
              <ArrowUpRight className="size-3.5" aria-hidden />
            </a>
          </Popover.Popup>
        </Popover.Positioner>
      </Popover.Portal>
    </Popover.Root>
  );
}
