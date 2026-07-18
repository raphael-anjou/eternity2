import { Popover } from "@base-ui/react/popover";
import { Bug, Lightbulb, BookOpen, MessageSquarePlus, ArrowUpRight } from "lucide-react";
import type { ComponentType } from "react";
import { REPO_URL } from "@/site";
import { cn } from "@/lib/utils";
import { TRIGGER_CLASS, type FeedbackCopy } from "./feedback-shared";

// The base-ui Popover half of the feedback button. Split out from
// FeedbackButton so this (and the base-ui Popover it pulls in, ~30KB gzip) is
// lazy-loaded on first interaction instead of shipping on the landing-page
// critical path. `defaultOpen` opens it immediately on the first mount, so the
// click that swapped the plain button for this one also opens the popover.

const newIssue = (template: string) => `${REPO_URL}/issues/new?template=${template}`;

type Item = { icon: ComponentType<{ className?: string }>; href: string; title: string; desc: string };

export default function FeedbackPopover({ t }: { t: FeedbackCopy }) {
  const items: Item[] = [
    { icon: Bug, href: newIssue("bug_report.yml"), title: t.bugTitle, desc: t.bugDesc },
    { icon: Lightbulb, href: newIssue("idea.yml"), title: t.ideaTitle, desc: t.ideaDesc },
    { icon: BookOpen, href: newIssue("content_correction.yml"), title: t.contentTitle, desc: t.contentDesc },
  ];
  return (
    <Popover.Root defaultOpen>
      <Popover.Trigger className={TRIGGER_CLASS} aria-label={t.trigger}>
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
