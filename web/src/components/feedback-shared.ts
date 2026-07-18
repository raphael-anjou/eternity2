import { cn } from "@/lib/utils";

// Shared bits between the lightweight FeedbackButton (always rendered) and the
// lazy-loaded FeedbackPopover (base-ui, loaded on first interaction). Keeping
// the copy dict and the trigger styling here means the two halves render an
// identical-looking trigger, so swapping one for the other on first click is
// seamless.

export const TRIGGER_CLASS = cn(
  "fixed bottom-4 left-4 z-40 inline-flex items-center gap-2 rounded-full border",
  "bg-background/90 px-3.5 py-2 text-sm font-medium text-muted-foreground shadow-md backdrop-blur",
  "transition-colors hover:bg-muted hover:text-foreground",
  "focus-visible:outline-none focus-visible:ring-3 focus-visible:ring-ring/50",
  "print:hidden",
);

export type FeedbackCopy = {
  trigger: string;
  heading: string;
  sub: string;
  bugTitle: string;
  bugDesc: string;
  ideaTitle: string;
  ideaDesc: string;
  contentTitle: string;
  contentDesc: string;
  more: string;
};

export const COPY: { en: FeedbackCopy; fr: FeedbackCopy; es: FeedbackCopy } = {
  en: {
    trigger: "Feedback",
    heading: "Spotted something?",
    sub: "Everything opens a pre-filled issue on GitHub.",
    bugTitle: "Report a bug",
    bugDesc: "Something's broken or wrong",
    ideaTitle: "Suggest an idea",
    ideaDesc: "A feature or improvement",
    contentTitle: "Fix the content",
    contentDesc: "A research-wiki error or typo",
    more: "Something else — all options on GitHub",
  },
  fr: {
    trigger: "Votre avis",
    heading: "Un souci, une idée ?",
    sub: "Chaque choix ouvre un ticket pré-rempli sur GitHub.",
    bugTitle: "Signaler un bug",
    bugDesc: "Quelque chose ne marche pas",
    ideaTitle: "Proposer une idée",
    ideaDesc: "Une fonctionnalité ou une amélioration",
    contentTitle: "Corriger le contenu",
    contentDesc: "Une erreur ou coquille du wiki",
    more: "Autre chose — toutes les options sur GitHub",
  },
  es: {
    trigger: "Opiniones",
    heading: "¿Viste algo?",
    sub: "Cada opción abre una incidencia ya rellenada en GitHub.",
    bugTitle: "Reportar un error",
    bugDesc: "Algo está roto o funciona mal",
    ideaTitle: "Proponer una idea",
    ideaDesc: "Una función o una mejora",
    contentTitle: "Corregir el contenido",
    contentDesc: "Un error o errata del wiki",
    more: "Otra cosa — todas las opciones en GitHub",
  },
};
