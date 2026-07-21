// The component map handed to every research MDX page. Kept apart from the
// component definitions so mdx-components.tsx stays a components-only module
// (react-refresh wants that).

import { Callout, Figure, H2, H3, H4, ProseLink, ProseTable } from "./mdx-components";
import { Term } from "./Term";

export const mdxComponents = {
  a: ProseLink,
  h2: H2,
  h3: H3,
  h4: H4,
  table: ProseTable,
  Callout,
  Figure,
  // Emitted by the build-time glossary auto-linker (plugins/
  // research-glossary-autolink.ts), which wraps the first prose mention of each
  // glossary term per article.
  Term,
};
