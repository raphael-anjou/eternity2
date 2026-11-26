import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { parseResearch, datedSections } from './data.mjs';

const input = {
  agenda: '# Agenda\n\n## A1. First idea · killed for the narrow family priced\n\n- **Hypothesis.** A claim.\n  A continuation.\n- **Kill.** Fails cheaply.\n- **Result (round 1).** Negative. `research/experiments/example/`.\n\n## A2. Reopening condition: new evidence · open\n\n- **Hypothesis.** More data.\n\n## Unnumbered notes\nNot a thread.\n',
  ledger: '# Log\n\n## Plan and conventions\nIgnore.\n\n## 2026-01-01 · First result\nFirst evidence.\n\n## 2026-01-02 · Second result\nSecond evidence.\n',
  experiments: ['example'], revision: 'abc123', source: 'local', sourceDate: '2026-01-02', checkedAt: '2026-01-03',
};
test('keeps scoped statuses and multiline fields without inventing activity', () => {
  const data = parseResearch(input);
  assert.equal(data.threads.length, 2);
  assert.equal(data.threads[0].status, 'killed');
  assert.equal(data.threads[0].statusText, 'killed for the narrow family priced');
  assert.equal(data.threads[0].hypothesis, 'A claim. A continuation.');
  assert.deepEqual(data.threads[0].experiments, ['example']);
  assert.equal(data.threads[0].results.length, 1);
  assert.equal(data.threads[1].kind, 'Reopening condition');
  assert.ok(data.threads[0].url.includes('/blob/abc123/'));
  assert.ok(data.threads[0].url.endsWith('#a1-first-idea--killed-for-the-narrow-family-priced'));
});
test('ledger is newest first and excludes undated conventions', () => {
  const data = parseResearch(input);
  assert.equal(data.log.length, 2);
  assert.equal(data.log[0].date, '2026-01-02');
  assert.ok(data.log[0].url.endsWith('#2026-01-02--second-result'));
});
test('related log entries include a thread mentioned only in the heading', () => {
  const data = parseResearch({ ...input, ledger: '# Log\n\n## 2026-01-02 · A2 proposal\nMore data needed.\n' });
  assert.deepEqual(data.log[0].threadIds, ['A2']);
});
test('dated level-three log entries retain their own body, date and source anchor', () => {
  const ledger = '# Log\n\n## Undated conventions\nIgnore.\n\n### 2026-01-01 · Pass 2\nFirst audit.\n\n## 2026-01-02 · Parent result\nParent only.\n\n### 2026-01-03 · Child result\nChild only.\n';
  assert.equal(datedSections(ledger).length, 3);
  const data = parseResearch({ ...input, ledger });
  assert.deepEqual(data.log.map((entry) => entry.date), ['2026-01-03', '2026-01-02', '2026-01-01']);
  assert.equal(data.log[1].body, 'Parent only.');
  assert.ok(data.log[2].url.endsWith('#2026-01-01--pass-2'));
});
test('malformed agenda cannot silently replace good data', () => {
  assert.throws(() => parseResearch({ ...input, agenda: 'No headings' }), /No agenda threads/);
});
test('agenda group introductions cannot become the preceding thread result', () => {
  const data = parseResearch({ ...input, agenda: input.agenda.replace('## A2.', '### Next agenda group\n\nIntroduction, not an A1 result.\n\n## A2.') });
  assert.ok(data.threads[0].fields.every((field) => !field.text.includes('Next agenda group') && !field.text.includes('Introduction')));
});
test('parses every numbered heading in the real agenda and retains every dated ledger entry', async () => {
  const [agenda, ledger, entries] = await Promise.all([
    readFile(new URL('../../research/program/AGENDA.md', import.meta.url), 'utf8'),
    readFile(new URL('../../research/LEDGER.md', import.meta.url), 'utf8'),
    readdir(new URL('../../research/experiments/', import.meta.url), { withFileTypes: true }),
  ]);
  const data = parseResearch({ ...input, agenda, ledger, experiments: entries.filter((entry) => entry.isDirectory()).map((entry) => entry.name) });
  assert.equal(data.threads.length, [...agenda.matchAll(/^## A\d+\./gm)].length);
  assert.equal(new Set(data.threads.map((thread) => thread.id)).size, data.threads.length);
  assert.equal(data.log.length, [...ledger.matchAll(/^#{2,3} \d{4}-\d{2}-\d{2}/gm)].length);
  assert.ok(data.threads.every((thread) => thread.status !== 'unknown' && thread.hypothesis));
  assert.deepEqual(data.program.tripwires, ['A40', 'A53', 'A54', 'A55']);
  assert.ok(data.program.url.includes('/research/LEDGER.md#'));
  assert.match(data.program.reason, /works too late/);
  const latestLearning = data.log.find((entry) => entry.title.includes('Learning against propagation on real-piece prefixes'));
  assert.ok(latestLearning.threadIds.includes('A43'));
  assert.match(data.threads.find((thread) => thread.id === 'A43').laterResult.title, /real-piece prefixes/);
  for (const id of ['A27', 'A28']) {
    assert.ok(data.threads.find((thread) => thread.id === id).experiments.includes('dead-node-census'));
    assert.ok(data.log.find((entry) => /Night track 6,/i.test(entry.title)).threadIds.includes(id));
  }
  const linearisation = data.log.find((entry) => /Linearisation degree on toy boards/.test(entry.title));
  assert.ok(linearisation.threadIds.includes('A37'));
  assert.ok(!linearisation.threadIds.includes('A35'));
  assert.ok(!data.threads.find((thread) => thread.id === 'A39').fields.at(-1).text.includes('Open after the arc'));
});
