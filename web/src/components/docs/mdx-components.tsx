// The MDX component map for research wiki pages. Typography comes from the
// `prose` classes on the article wrapper (see DocsShell); this map only adds
// behavior the CSS can't: language-aware internal links, anchor links on
// headings, and a few authoring shortcodes (Callout, Figure).

import type { ComponentPropsWithoutRef, ReactNode } from "react";
import { LocalizedLink } from "@/components/LocalizedLink";
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

/** Callout box: <Callout kind="note">…</Callout> (kinds: note, warn, insight). */
export function Callout({
  kind = "note",
  title,
  children,
}: {
  kind?: "note" | "warn" | "insight";
  title?: string;
  children: ReactNode;
}) {
  const styles: Record<string, string> = {
    note: "border-sky-500/30 bg-sky-500/5",
    warn: "border-amber-500/40 bg-amber-500/5",
    insight: "border-violet-500/30 bg-violet-500/5",
  };
  return (
    <div className={cn("my-6 rounded-lg border p-4 text-sm leading-relaxed", styles[kind])}>
      {title && <div className="mb-1 font-semibold">{title}</div>}
      {children}
    </div>
  );
}

/** Figure wrapper for embedded visuals: centers content, adds a caption. */
export function Figure({ caption, children }: { caption?: string; children: ReactNode }) {
  return (
    <figure className="my-8">
      {children}
      {caption && (
        <figcaption className="mt-2 text-center text-xs text-muted-foreground">
          {caption}
        </figcaption>
      )}
    </figure>
  );
}

/** Responsive table wrapper (wide research tables scroll, not overflow). */
export function ProseTable(props: ComponentPropsWithoutRef<"table">) {
  return (
    <div className="my-6 overflow-x-auto">
      <table {...props} className="my-0" />
    </div>
  );
}

export { A as ProseLink };
