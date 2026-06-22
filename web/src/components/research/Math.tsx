import katex from "katex";

// Math rendering via KaTeX's string renderer. It produces static HTML + CSS, so
// it works identically during the static prerender and in the browser (no layout
// flash, no client-only gate). `throwOnError: false` makes a malformed formula
// render in red rather than crash the page.

export function Math({ children }: { children: string }) {
  const html = katex.renderToString(children, {
    throwOnError: false,
    displayMode: false,
  });
  return <span dangerouslySetInnerHTML={{ __html: html }} />;
}

export function MathBlock({ children }: { children: string }) {
  const html = katex.renderToString(children, {
    throwOnError: false,
    displayMode: true,
  });
  return (
    <div
      className="my-4 overflow-x-auto text-center"
      dangerouslySetInnerHTML={{ __html: html }}
    />
  );
}
