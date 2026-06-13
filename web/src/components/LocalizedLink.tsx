// A drop-in replacement for react-router's <Link> that keeps in-content links
// in the current language: <LocalizedLink to="/research"> resolves to
// "/research" in English and "/fr/research" in French. Absolute URLs (http...)
// and hash/anchor links pass through untouched.

import { Link, type LinkProps } from "react-router";
import { useLang, pathForLang } from "@/i18n";

export function LocalizedLink({ to, ...rest }: LinkProps) {
  const { lang } = useLang();
  const localized =
    typeof to === "string" && to.startsWith("/") ? pathForLang(to, lang) : to;
  return <Link to={localized} {...rest} />;
}
