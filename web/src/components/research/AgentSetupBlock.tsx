// "Set this up with your coding agent" — a copy-paste prompt that points an
// agent (Claude Code, Cursor, Copilot, …) at the starter kit's PROMPT.md so it
// scaffolds the kit and helps write a solver. Modelled on the now-standard
// paste-and-go pattern: one line in, the agent does the rest.
import { useState } from "react";

import { Button } from "@/components/ui/button";
import { useT } from "@/i18n";

const PROMPT_URL =
  "https://raw.githubusercontent.com/raphael-anjou/eternity2/main/research/starter-kit/PROMPT.md";

const AGENT_PROMPT = `Follow the instructions from\n${PROMPT_URL}\nand ask me questions as needed.`;

export function AgentSetupBlock() {
  const t = useT(T);
  const [copied, setCopied] = useState(false);

  const copy = () => {
    void navigator.clipboard.writeText(AGENT_PROMPT);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  return (
    <div className="not-prose my-6 rounded-lg border border-primary/30 bg-primary/[0.04] p-4">
      <div className="mb-2 flex items-center gap-2 text-sm font-medium">
        <span aria-hidden>✨</span>
        {t.heading}
      </div>
      <p className="mb-3 text-sm text-muted-foreground">{t.intro}</p>
      <pre className="overflow-x-auto rounded-md border border-border bg-background p-3 text-xs leading-relaxed">
        <code>{AGENT_PROMPT}</code>
      </pre>
      <div className="mt-3 flex flex-wrap items-center gap-3">
        <Button onClick={copy} size="sm">
          {copied ? t.copied : t.copy}
        </Button>
        <a
          href={PROMPT_URL}
          target="_blank"
          rel="noreferrer"
          className="text-xs text-muted-foreground underline hover:text-foreground"
        >
          {t.view}
        </a>
      </div>
      <p className="mt-3 text-xs text-muted-foreground">{t.fallback}</p>
    </div>
  );
}

const T = {
  en: {
    heading: "Set this up with your coding agent",
    intro:
      "Using Claude Code, Cursor, or another coding agent? Paste this in — it fetches the kit's setup instructions, builds it, and helps you write your first solver.",
    copy: "Copy prompt",
    copied: "Copied ✓",
    view: "View PROMPT.md",
    fallback:
      "No web access in your agent? Clone the repo and open research/starter-kit/AGENTS.md instead.",
  },
  fr: {
    heading: "Configurez-le avec votre agent de code",
    intro:
      "Vous utilisez Claude Code, Cursor ou un autre agent de code ? Collez ceci : il récupère les instructions d'installation du kit, le compile et vous aide à écrire votre premier solveur.",
    copy: "Copier l'invite",
    copied: "Copié ✓",
    view: "Voir PROMPT.md",
    fallback:
      "Pas d'accès web dans votre agent ? Clonez le dépôt et ouvrez research/starter-kit/AGENTS.md à la place.",
  },
  es: {
    heading: "Configúralo con tu agente de código",
    intro:
      "¿Usas Claude Code, Cursor u otro agente de código? Pega esto: obtiene las instrucciones de instalación del kit, lo compila y te ayuda a escribir tu primer solucionador.",
    copy: "Copiar indicación",
    copied: "Copiado ✓",
    view: "Ver PROMPT.md",
    fallback:
      "¿Tu agente no tiene acceso web? Clona el repositorio y abre research/starter-kit/AGENTS.md.",
  },
};
