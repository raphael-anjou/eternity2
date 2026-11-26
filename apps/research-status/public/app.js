const $ = (selector) => document.querySelector(selector);
const statuses = ['running', 'open', 'proposed', 'promoted', 'published', 'killed'];
const labels = { running: 'Running', open: 'Open', proposed: 'Proposed', promoted: 'Promoted', published: 'Published', killed: 'Killed' };
const state = { data: null, view: 'threads', scope: 'all', status: null, query: '', group: 'all', sort: 'number' };
let opener;

function el(tag, className, text) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}
function link(text, url, className) {
  const node = el('a', className, text);
  node.href = url; node.target = '_blank'; node.rel = 'noopener'; return node;
}
function badge(status, text) {
  const node = el('span', `badge ${status}`);
  node.append(el('span', 'status-dot'), document.createTextNode(text ?? labels[status] ?? status)); return node;
}
function plain(text) { return text.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1').replace(/[*`]/g, '').replace(/\s+/g, ' ').trim(); }
function rich(text) {
  const container = el('div', 'prose');
  for (const paragraph of text.split(/\n\s*\n/)) {
    const p = el('p');
    const pattern = /\[([^\]]+)\]\((https?:\/\/[^\s)]+)\)|`([^`]+)`|\*\*([^*]+)\*\*/g;
    let cursor = 0;
    for (const match of paragraph.matchAll(pattern)) {
      p.append(document.createTextNode(paragraph.slice(cursor, match.index)));
      p.append(match[1] ? link(match[1], match[2]) : el(match[3] ? 'code' : 'strong', '', match[3] ?? match[4]));
      cursor = match.index + match[0].length;
    }
    p.append(document.createTextNode(paragraph.slice(cursor))); container.append(p);
  }
  return container;
}
function date(text) {
  if (!text) return 'Unknown';
  return new Date(text).toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric', timeZone: 'UTC' });
}
function setView(view) {
  state.view = view; state.query = ''; $('#search').value = '';
  const titles = { threads: ['Research threads', 'The questions being explored. The decisions already made.'], log: ['Research log', 'Decisions and findings, newest first. Including the paths that did not work.'], evidence: ['Evidence folders', 'The experiments behind the notebook. Scripts, committed results, and reproduction notes.'] };
  $('#page-title').textContent = titles[view][0]; $('#breadcrumb').textContent = titles[view][0]; $('#page-description').textContent = titles[view][1];
  $('#search').placeholder = view === 'threads' ? 'Search threads, ideas, evidence…' : view === 'log' ? 'Search the research log…' : 'Search experiment folders…';
  $('#tabs').hidden = view !== 'threads'; $('#status-nav').hidden = view !== 'threads';
  $('#group-label').hidden = view !== 'threads'; $('#sort-label').hidden = view !== 'threads';
  document.querySelectorAll('[data-view]').forEach((node) => { node.classList.toggle('active', node.dataset.view === view); node.setAttribute('aria-current', node.dataset.view === view ? 'page' : 'false'); });
  render();
}
function renderOverview() {
  const threads = state.data.threads;
  const cards = [
    ['All threads', threads.length, 'Recorded in the research agenda', null],
    ['Unresolved', threads.filter((t) => ['open', 'proposed'].includes(t.status)).length, 'Open questions + proposed ideas', 'unresolved'],
    ['Advanced', threads.filter((t) => ['promoted', 'published'].includes(t.status)).length, 'Promoted or published in agenda', 'advanced'],
    ['Closed', threads.filter((t) => t.status === 'killed').length, 'Negative results are results', 'closed'],
  ];
  $('#overview').replaceChildren(...cards.map(([label, count, caption, scope]) => {
    const node = el('button', `metric ${state.scope === (scope ?? 'all') && !state.status ? 'selected' : ''}`);
    node.append(el('span', 'metric-label', label), el('strong', 'metric-number', count), el('span', 'metric-caption', caption));
    node.addEventListener('click', () => { state.scope = scope ?? 'all'; state.status = null; setView('threads'); }); return node;
  }));
  $('#thread-total').textContent = threads.length;
  $('#status-list').replaceChildren(...statuses.map((status) => {
    const count = threads.filter((t) => t.status === status).length;
    const button = el('button', `status-filter ${state.status === status ? 'active' : ''}`);
    button.append(badge(status), el('span', 'mono', count)); button.addEventListener('click', () => { state.status = state.status === status ? null : status; state.scope = 'all'; render(); });
    return button;
  }));
  const note = $('#program-note'); note.hidden = !state.data.program.context;
  note.replaceChildren();
  if (state.data.program.context) {
    const text = el('div');
    text.append(el('strong', '', state.data.program.stopped ? 'Full-solve method hunt stopped' : 'Program context'), el('p', '', state.data.program.stopped ? 'The agenda records a stop on 14 Nov 2026. Open questions and reopening conditions remain; they are not active runs.' : state.data.program.context));
    note.append(el('span', 'note-symbol', 'Ⅱ'), text, link('Read the decision ↗', state.data.program.url));
  }
}
function filteredThreads() {
  return state.data.threads.filter((thread) => {
    const scope = state.scope === 'all' || (state.scope === 'unresolved' && ['open', 'proposed'].includes(thread.status)) || (state.scope === 'advanced' && ['promoted', 'published'].includes(thread.status)) || (state.scope === 'closed' && thread.status === 'killed');
    return scope && (!state.status || thread.status === state.status) && (state.group === 'all' || state.group === thread.group) && JSON.stringify(thread).toLowerCase().includes(state.query);
  }).sort((a, b) => state.sort === 'recent' ? Number(b.id.slice(1)) - Number(a.id.slice(1)) : state.sort === 'status' ? statuses.indexOf(a.status) - statuses.indexOf(b.status) || Number(a.id.slice(1)) - Number(b.id.slice(1)) : Number(a.id.slice(1)) - Number(b.id.slice(1)));
}
function openThread(thread, button) {
  opener = button;
  const body = $('#detail-body'); body.replaceChildren();
  $('#detail-id').textContent = `${thread.id} / ${thread.group}`;
  body.append(badge(thread.status, thread.statusText), el('h2', '', thread.title), el('p', 'detail-kind', thread.kind));
  body.append(link('Open original agenda entry ↗', thread.url, 'source-link'));
  const disclaimer = el('p', 'detail-disclaimer', 'Recorded agenda status, not live execution. Later research log entries may narrow or supersede this result.'); body.append(disclaimer);
  for (const field of thread.fields) {
    const section = el('section', 'detail-section'); section.append(el('h3', '', field.label), rich(field.text)); body.append(section);
  }
  if (thread.experiments.length) {
    const section = el('section', 'detail-section'); section.append(el('h3', '', 'Evidence folders'));
    for (const name of thread.experiments) section.append(link(`${name} ↗`, state.data.experiments.find((e) => e.name === name).url, 'evidence-link'));
    body.append(section);
  }
  $('#detail').showModal(); $('#detail').scrollTop = 0;
  history.replaceState(null, '', `#${thread.id}`);
}
function renderThreads() {
  const threads = filteredThreads();
  $('#result-count').textContent = `${threads.length} of ${state.data.threads.length} threads${state.status ? ` · ${labels[state.status]}` : ''}`;
  if (!threads.length) return empty('No threads match these filters.', 'Try another idea or clear the filters.');
  const table = el('table', 'threads-table');
  const head = el('thead'); const tr = el('tr');
  for (const title of ['Thread', 'Recorded status', 'Latest note in entry', '']) { const th = el('th', '', title); th.scope = 'col'; tr.append(th); }
  head.append(tr); table.append(head);
  const tbody = el('tbody');
  for (const thread of threads) {
    const row = el('tr');
    const main = el('td', 'thread-main'); const button = el('button', 'thread-button');
    button.append(el('span', 'thread-id mono', thread.id), el('span', 'thread-title', thread.title));
    button.addEventListener('click', () => openThread(thread, button));
    const meta = el('div', 'thread-meta', thread.group);
    if (thread.kind === 'Reopening condition') meta.append(el('span', 'kind-label', 'Reopening condition'));
    main.append(button, meta);
    const status = el('td', 'status-cell'); status.append(badge(thread.status)); status.title = thread.statusText;
    const result = thread.results.at(-1);
    const outcome = el('td', 'outcome-cell'); outcome.append(el('p', 'outcome', result ? plain(result.text) : thread.hypothesis));
    const detail = el('td', 'arrow-cell'); const open = el('button', 'icon-button', '↗'); open.setAttribute('aria-label', `Open ${thread.id} details`); open.addEventListener('click', () => openThread(thread, open)); detail.append(open);
    row.append(main, status, outcome, detail); tbody.append(row);
  }
  table.append(tbody); $('#content').replaceChildren(table);
}
function empty(title, subtitle) {
  const box = el('div', 'empty'); box.append(el('strong', '', title), el('p', '', subtitle)); $('#content').replaceChildren(box);
}
function renderLog() {
  const entries = state.data.log.filter((entry) => `${entry.title} ${entry.body}`.toLowerCase().includes(state.query));
  $('#result-count').textContent = `${entries.length} research log entries · newest first`;
  if (!entries.length) return empty('No log entries match.', 'Try a method name or an experiment folder.');
  const list = el('div', 'log-list');
  for (const entry of entries) {
    const details = el('details', 'log-entry'); const summary = el('summary');
    summary.append(el('span', 'log-date mono', date(entry.date)), el('strong', '', entry.title.replace(/^\d{4}-\d{2}-\d{2}\s*[·:—-]?\s*/, '')), el('span', 'log-expand', '+'));
    details.append(summary, rich(entry.body), link('View source entry ↗', entry.url, 'source-link')); list.append(details);
  }
  $('#content').replaceChildren(list);
}
function renderEvidence() {
  const entries = state.data.experiments.filter((entry) => entry.name.includes(state.query));
  $('#result-count').textContent = `${entries.length} experiment folders · source repository`;
  if (!entries.length) return empty('No folders match.', 'Try a shorter experiment name.');
  const grid = el('div', 'evidence-grid');
  for (const entry of entries) { const node = link('', entry.url, 'evidence-card'); const related = state.data.threads.filter((t) => t.experiments.includes(entry.name)); node.append(el('span', 'folder-icon', '▱'), el('strong', '', entry.name), el('small', '', related.length ? `Referenced by ${related.map((t) => t.id).join(', ')}` : 'Browse source files and results'), el('span', 'folder-arrow', '↗')); grid.append(node); }
  $('#content').replaceChildren(grid);
}
function render() {
  if (!state.data) return;
  renderOverview();
  document.querySelectorAll('[data-scope]').forEach((node) => { const active = node.dataset.scope === state.scope && !state.status; node.classList.toggle('active', active); node.setAttribute('aria-pressed', String(active)); });
  $('#clear').hidden = !(state.query || state.status || state.scope !== 'all' || state.group !== 'all');
  if (state.view === 'threads') renderThreads(); else if (state.view === 'log') renderLog(); else renderEvidence();
  const meta = state.data.meta;
  const footer = $('#source-footer'); footer.replaceChildren();
  const status = el('span', `sync-state ${meta.warning ? 'warning' : ''}`, meta.source === 'github' ? 'GitHub source' : 'Local snapshot');
  footer.append(status, el('span', '', `Checked ${new Date(meta.checkedAt).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit' })} · refreshes every 5 min`), link(meta.revision.slice(0, 7), `${meta.repository}/commit/${meta.revision}`, 'mono'), el('span', '', `Source commit ${date(meta.sourceDate)}`));
  if (meta.warning) footer.append(el('p', 'source-warning', meta.warning));
  $('#content').setAttribute('aria-busy', 'false');
}
async function load() {
  const button = $('#refresh'); button.disabled = true;
  try {
    const response = await fetch('/api/research');
    if (!response.ok) throw new Error(`Source request failed (${response.status})`);
    state.data = await response.json(); $('#load-error').hidden = true; render();
    const id = location.hash.slice(1); const thread = state.data.threads.find((t) => t.id === id);
    if (thread && !$('#detail').open) openThread(thread, null);
  } catch (error) {
    $('#load-error').hidden = false; $('#load-error').textContent = `${error.message}. ${state.data ? 'Showing the last loaded snapshot.' : 'Use Refresh source to try again.'}`;
    if (!state.data) { $('#content').setAttribute('aria-busy', 'false'); empty('The source is unavailable.', 'Refresh to try again.'); }
  } finally { button.disabled = false; }
}
$('#navigation').addEventListener('click', (event) => { const button = event.target.closest('[data-view]'); if (button) setView(button.dataset.view); });
$('#tabs').addEventListener('click', (event) => { const button = event.target.closest('[data-scope]'); if (button) { state.scope = button.dataset.scope; state.status = null; render(); } });
$('#search').addEventListener('input', (event) => { state.query = event.target.value.trim().toLowerCase(); render(); });
$('#group').addEventListener('change', (event) => { state.group = event.target.value; render(); });
$('#sort').addEventListener('change', (event) => { state.sort = event.target.value; render(); });
$('#clear').addEventListener('click', () => { state.query = ''; state.scope = 'all'; state.status = null; state.group = 'all'; $('#search').value = ''; $('#group').value = 'all'; render(); });
$('#refresh').addEventListener('click', load);
$('#close-detail').addEventListener('click', () => $('#detail').close());
$('#detail').addEventListener('click', (event) => { if (event.target === $('#detail')) $('#detail').close(); });
$('#detail').addEventListener('close', () => { history.replaceState(null, '', location.pathname); opener?.focus(); });
document.addEventListener('keydown', (event) => { if (event.key === '/' && !['INPUT', 'TEXTAREA', 'SELECT'].includes(document.activeElement.tagName) && !$('#detail').open) { event.preventDefault(); $('#search').focus(); } });
setInterval(load, 60000);
void load();
