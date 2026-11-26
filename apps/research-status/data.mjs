export const repository = 'https://github.com/raphael-anjou/eternity2';
export const agendaPath = 'research/program/AGENDA.md';
export const ledgerPath = 'research/LEDGER.md';

export function sections(markdown) {
  return [...markdown.matchAll(/^## (.+)\r?\n([\s\S]*?)(?=^## |$(?![\s\S]))/gm)]
    .map((match) => ({ title: match[1], body: match[2].trim() }));
}

export function clean(text) {
  return text.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1').replace(/[*`]/g, '').replace(/\s+/g, ' ').trim();
}

export function fields(body) {
  return [...body.matchAll(/^- \*\*([^*]+)\*\*\s*([\s\S]*?)(?=^- \*\*|$(?![\s\S]))/gm)]
    .map((match) => ({ label: match[1].replace(/\.$/, ''), text: match[2].trim() }));
}

function slug(text) {
  // Render inline markup before removing heading punctuation, as GitHub does.
  return text.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1').toLowerCase()
    .replace(/[^\p{L}\p{N}\p{M}\s_-]/gu, '').replace(/ /g, '-');
}

export function parseResearch({ agenda, ledger, experiments, revision, source, sourceDate, checkedAt, warning = null }) {
  const blob = `${repository}/blob/${revision}`;
  const threads = sections(agenda).flatMap((section) => {
    const heading = /^(A\d+)\. (.+?) · (.+)$/.exec(section.title);
    if (!heading) return [];
    const [, id, title, statusText] = heading;
    const status = /^(proposed|running|killed|promoted|published|open)\b/.exec(statusText)?.[1] ?? 'unknown';
    // Third-level headings introduce the next agenda group, not thread fields.
    const body = section.body.split(/^### /m)[0];
    const detail = fields(body);
    const paths = experiments.filter((name) => body.includes(`research/experiments/${name}`)
      || (name.includes('-') && new RegExp(`\\b${name}\\b`, 'i').test(body)));
    return [{
      id, title: clean(title), status, statusText, fields: detail,
      hypothesis: clean(detail.find((field) => field.label === 'Hypothesis')?.text ?? ''),
      results: detail.filter((field) => /result|follow-up|judge|measurement/i.test(field.label)),
      experiments: paths,
      url: `${blob}/${agendaPath}#${slug(section.title)}`,
      group: Number(id.slice(1)) >= 35 ? 'Closing arc' : Number(id.slice(1)) >= 23 ? 'Night studies' : 'Initial rounds',
      kind: title.startsWith('Reopening condition:') ? 'Reopening condition' : 'Research hypothesis',
    }];
  });
  if (!threads.length) throw new Error('No agenda threads found; keeping the previous source snapshot.');
  const log = sections(ledger).filter((section) => /^\d{4}-\d{2}-\d{2}/.test(section.title)).map((section, index) => ({
    id: `log-${index}`, title: clean(section.title), body: section.body,
    date: section.title.slice(0, 10), url: `${blob}/${ledgerPath}#${slug(section.title)}`,
    threadIds: threads.filter((thread) => new RegExp(`\\b${thread.id}\\b`).test(`${section.title} ${section.body}`)
      || thread.experiments.some((name) => section.body.includes(`research/experiments/${name}`))).map((thread) => thread.id),
  })).reverse();
  const arc = sections(agenda).find((section) => section.title.startsWith('Arc of '));
  const context = arc?.body.split(/\n\s*\n/)[0] ?? '';
  const decision = log.find((entry) => /Decision: stop the hunt/.test(entry.title));
  return {
    threads, log, experiments: experiments.map((name) => ({ name, url: `${repository}/tree/${revision}/research/experiments/${name}` })),
    program: { context: clean(context), stopped: /hunt for a method stopped/.test(context), stoppedAt: /hunt for a method stopped on (\d{4}-\d{2}-\d{2})/.exec(context)?.[1] ?? null,
      reason: /The one-sentence reason: ([^.]+)\./.exec(decision?.body ?? '')?.[1] ?? '',
      tripwires: threads.filter((thread) => thread.kind === 'Reopening condition').map((thread) => thread.id),
      url: decision?.url ?? `${blob}/${agendaPath}${arc ? `#${slug(arc.title)}` : ''}` },
    meta: { revision, source, sourceDate, checkedAt, warning, repository, agendaUrl: `${blob}/${agendaPath}`, ledgerUrl: `${blob}/${ledgerPath}` },
  };
}
