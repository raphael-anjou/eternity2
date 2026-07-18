import { useLang, useT } from "@/i18n";
import { pageFaq } from "@/seo";

// Visible FAQ section for the pages that also emit FAQPage JSON-LD
// (see PAGE_FAQ / faqLd in seo.ts). Google requires the questions and answers
// in FAQ structured data to be visible on the page; this renders the SAME data
// the JSON-LD is built from, so the two are guaranteed identical. Kept as plain,
// always-visible text (no accordion) because content hidden behind interaction
// is weighted less and can fail the "visible" requirement.

const T = {
  en: { heading: "Common questions" },
  fr: { heading: "Questions fréquentes" },
  es: { heading: "Preguntas frecuentes" },
};

export function PageFaq({ pageKey }: { pageKey: Parameters<typeof pageFaq>[0] }) {
  const { lang } = useLang();
  const t = useT(T);
  const qa = pageFaq(pageKey, lang);
  if (qa.length === 0) return null;
  return (
    <section aria-labelledby="faq-heading" className="mt-12 border-t pt-8">
      <h2 id="faq-heading" className="text-2xl font-semibold tracking-tight">
        {t.heading}
      </h2>
      <dl className="mt-6 space-y-6">
        {qa.map((item) => (
          <div key={item.q}>
            <dt className="text-lg font-medium text-foreground">{item.q}</dt>
            <dd className="mt-1 max-w-prose text-muted-foreground">{item.a}</dd>
          </div>
        ))}
      </dl>
    </section>
  );
}
