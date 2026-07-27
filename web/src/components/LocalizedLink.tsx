// A drop-in replacement for react-router's <Link> that keeps in-content links
// in the current language: <LocalizedLink to="/research"> resolves to
// "/research/" in English and "/fr/research/" in French. Absolute URLs (http...)
// and hash/anchor links pass through untouched.
//
// The resolved path is canonicalized to the trailing-slash form the host serves
// at 200 (canonicalPath), so a crawler following the link lands on the page
// directly instead of taking GitHub Pages' 301 to the slash variant. React
// Router matches the route with or without the slash, so client navigation is
// unaffected; only the emitted href changes.

import { Link, type LinkProps } from "react-router";
import { useLang, pathForLang } from "@/i18n";
import { canonicalPath } from "@/site";

export function LocalizedLink({ to, ...rest }: LinkProps) {
  const { lang } = useLang();
  const localized =
    typeof to === "string" && to.startsWith("/") ? canonicalPath(pathForLang(to, lang)) : to;
  return <Link to={localized} {...rest} />;
}
