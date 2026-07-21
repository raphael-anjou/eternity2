// The MDX component map for research wiki pages. Typography comes from the
// `prose` classes on the article wrapper (see DocsShell); this map only adds
// behavior the CSS can't: language-aware internal links, anchor links on
// headings, and a few authoring shortcodes (Callout, Figure).

import type { ComponentPropsWithoutRef, ReactNode } from "react";
import { LocalizedLink } from "@/components/LocalizedLink";
import { useT } from "@/i18n";
import { cn } from "@/lib/utils";

/** Internal absolute links stay in the reader's language and use SPA
 *  navigation; external links open in a new tab. */
function A({ href = "", children, ...rest }: ComponentPropsWithoutRef<"a">) {
  if (href.startsWith("/")) {
    return (
      <LocalizedLink to={href} {...rest}>
        {children}
      </LocalizedLink>
    );
  }
  if (/^https?:\/\//.test(href)) {
    return (
      <a href={href} target="_blank" rel="noreferrer" {...rest}>
        {children}
      </a>
    );
  }
  // Anchors and relative links pass through.
  return (
    <a href={href} {...rest}>
      {children}
    </a>
  );
}

/** Heading with a hover anchor link (ids are assigned by rehype-slug). */
function Heading({
  as: Tag,
  id,
  children,
  ...rest
}: ComponentPropsWithoutRef<"h2"> & { as: "h2" | "h3" | "h4" }) {
  return (
    <Tag id={id} className="group scroll-mt-24" {...rest}>
      {children}
      {id && (
        <a
          href={`#${id}`}
          aria-label="Link to this section"
          className="ml-2 align-middle text-muted-foreground no-underline opacity-0 transition-opacity group-hover:opacity-100"
        >
          #
        </a>
      )}
    </Tag>
  );
}

export function H2(props: ComponentPropsWithoutRef<"h2">) {
  return <Heading as="h2" {...props} />;
}
export function H3(props: ComponentPropsWithoutRef<"h3">) {
  return <Heading as="h3" {...props} />;
}
export function H4(props: ComponentPropsWithoutRef<"h4">) {
  return <Heading as="h4" {...props} />;
}

/** Callout box: <Callout kind="note">…</Callout> (kinds: note, warn, insight,
 *  open). `open` is the open-question variant: an amber dashed outline, so a
 *  still-unresolved question reads as deliberately open rather than a warning. */
export function Callout({
  kind = "note",
  title,
  children,
}: {
  kind?: "note" | "warn" | "insight" | "open";
  title?: string;
  children: ReactNode;
}) {
  const styles: Record<string, string> = {
    note: "border-sky-500/30 bg-sky-500/5",
    warn: "border-amber-500/40 bg-amber-500/5",
    insight: "border-violet-500/30 bg-violet-500/5",
    open: "border-dashed border-amber-500/50 bg-transparent",
  };
  return (
    <div className={cn("my-6 rounded-lg border p-4 text-sm leading-relaxed", styles[kind])}>
      {title && <div className="mb-1 font-semibold">{title}</div>}
      {children}
    </div>
  );
}

/** Figure wrapper for embedded visuals: centers content, adds a caption.
 *
 *  Interactive labs are heavy: full-height cards that a reader skimming the
 *  argument has to scroll past. Pass `collapsible` (optionally `open` to start
 *  expanded) to render the visual inside a native <details>, so the caption
 *  stays visible as a jump target while the widget itself folds away until the
 *  reader chooses to explore it. `title` is the summary label; `caption` is the
 *  small line beneath the content. */
export function Figure({
  caption,
  title,
  collapsible = false,
  open = false,
  children,
}: {
  caption?: string;
  title?: string;
  collapsible?: boolean;
  open?: boolean;
  children: ReactNode;
}) {
  const t = useT(FIGURE_T);
  const body = (
    <>
      {children}
      {caption && (
        <figcaption className="mt-2 text-center text-xs text-muted-foreground">
          {caption}
        </figcaption>
      )}
    </>
  );

  if (!collapsible) {
    return <figure className="my-8">{body}</figure>;
  }

  return (
    <figure className="my-8">
      <details className="group rounded-lg border bg-card/40 open:bg-transparent" open={open}>
        <summary className="flex cursor-pointer list-none items-center gap-2 rounded-lg px-4 py-3 text-sm font-medium transition-colors hover:bg-muted/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring group-open:rounded-b-none group-open:border-b">
          <span
            aria-hidden
            className="text-muted-foreground transition-transform group-open:rotate-90 motion-reduce:transition-none"
          >
            ▶
          </span>
          <span className="flex-1">{title ?? caption ?? t.fallback}</span>
          <span className="text-xs font-normal text-muted-foreground group-open:hidden">
            {t.explore}
          </span>
        </summary>
        <div className="p-4">{body}</div>
      </details>
    </figure>
  );
}

const FIGURE_T = {
  en: { explore: "Explore →", fallback: "Interactive figure" },
  fr: { explore: "Explorer →", fallback: "Figure interactive" },
  es: { explore: "Explorar →", fallback: "Figura interactiva" },
};

/** Responsive table wrapper (wide research tables scroll, not overflow). */
export function ProseTable(props: ComponentPropsWithoutRef<"table">) {
  return (
    <div className="my-6 overflow-x-auto">
      <table {...props} className="my-0" />
    </div>
  );
}

export { A as ProseLink };
