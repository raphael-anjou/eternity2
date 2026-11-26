import { createServer } from 'node:http';
import { readFile, readdir } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';
import { parseResearch, agendaPath, ledgerPath } from './data.mjs';

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, '../..');
const branch = process.env.SOURCE_BRANCH ?? 'main';
const localOnly = process.env.SOURCE_MODE === 'local';
const refreshInterval = 5 * 60 * 1000;
let snapshot;
let refreshing;

async function localSnapshot() {
  const [agenda, ledger, entries] = await Promise.all([
    readFile(resolve(repoRoot, agendaPath), 'utf8'), readFile(resolve(repoRoot, ledgerPath), 'utf8'),
    readdir(resolve(repoRoot, 'research/experiments'), { withFileTypes: true }),
  ]);
  const revision = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' }).trim();
  const sourceDate = execFileSync('git', ['log', '-1', '--format=%cI', '--', agendaPath, ledgerPath, 'research/experiments'], { cwd: repoRoot, encoding: 'utf8' }).trim();
  return parseResearch({ agenda, ledger, experiments: entries.filter((entry) => entry.isDirectory()).map((entry) => entry.name), revision, sourceDate,
    checkedAt: new Date().toISOString(), source: 'local', warning: localOnly ? null : 'Local checkout snapshot. Checking the published source…' });
}

async function getJson(url) {
  const response = await fetch(url, { headers: { Accept: 'application/vnd.github+json', 'User-Agent': 'eternity-research-status' }, signal: AbortSignal.timeout(15000) });
  if (!response.ok) throw new Error(`GitHub returned ${response.status}`);
  return response.json();
}

async function getText(path, revision) {
  const response = await fetch(`https://raw.githubusercontent.com/raphael-anjou/eternity2/${revision}/${path}`, { signal: AbortSignal.timeout(15000) });
  if (!response.ok) throw new Error(`Source returned ${response.status}`);
  return response.text();
}

export async function refresh() {
  if (refreshing) return refreshing;
  refreshing = (async () => {
    try {
      if (localOnly) { snapshot = await localSnapshot(); return; }
      const commit = await getJson(`https://api.github.com/repos/raphael-anjou/eternity2/commits/${encodeURIComponent(branch)}`);
      if (snapshot?.meta.source === 'github' && snapshot.meta.revision === commit.sha) {
        snapshot.meta.checkedAt = new Date().toISOString(); snapshot.meta.warning = null; return;
      }
      const [agenda, ledger, directories] = await Promise.all([
        getText(agendaPath, commit.sha), getText(ledgerPath, commit.sha),
        getJson(`https://api.github.com/repos/raphael-anjou/eternity2/contents/research/experiments?ref=${commit.sha}`),
      ]);
      snapshot = parseResearch({ agenda, ledger, experiments: directories.filter((entry) => entry.type === 'dir').map((entry) => entry.name),
        revision: commit.sha, sourceDate: commit.commit.committer.date, source: 'github', checkedAt: new Date().toISOString() });
    } catch (error) {
      if (!snapshot) throw error;
      snapshot.meta.warning = `Could not check GitHub (${error.message}). Showing the last available snapshot, not live activity.`;
    }
  })().finally(() => { refreshing = null; });
  return refreshing;
}

const assets = new Map([
  ['/', ['index.html', 'text/html; charset=utf-8']], ['/app.js', ['app.js', 'text/javascript; charset=utf-8']],
  ['/style.css', ['style.css', 'text/css; charset=utf-8']], ['/icon.svg', ['icon.svg', 'image/svg+xml']],
  ['/manifest.webmanifest', ['manifest.webmanifest', 'application/manifest+json']],
]);

export async function start(port = Number(process.env.PORT ?? 4317)) {
  snapshot = await localSnapshot();
  const server = createServer(async (req, res) => {
    res.setHeader('X-Content-Type-Options', 'nosniff');
    res.setHeader('Referrer-Policy', 'no-referrer');
    res.setHeader('Content-Security-Policy', "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; font-src 'self'; object-src 'none'; base-uri 'none'");
    let path;
    try { path = new URL(req.url, 'http://localhost').pathname; }
    catch { res.writeHead(400); res.end('Bad request'); return; }
    if (!['GET', 'HEAD'].includes(req.method)) { res.writeHead(405); res.end(); return; }
    if (path === '/api/research') {
      res.setHeader('Content-Type', 'application/json'); res.setHeader('Cache-Control', 'no-store');
      res.end(req.method === 'HEAD' ? '' : JSON.stringify(snapshot)); return;
    }
    const asset = assets.get(path);
    if (!asset) { res.writeHead(404); res.end('Not found'); return; }
    try {
      res.setHeader('Content-Type', asset[1]); res.setHeader('Cache-Control', 'no-cache');
      const bytes = await readFile(resolve(root, 'public', asset[0]));
      res.end(req.method === 'HEAD' ? '' : bytes);
    } catch { res.writeHead(500); res.end('Unable to load app'); }
  });
  await new Promise((done) => server.listen(port, '0.0.0.0', done));
  void refresh();
  const timer = setInterval(() => void refresh(), refreshInterval);
  timer.unref();
  server.on('close', () => clearInterval(timer));
  console.log(`Research threads listening on ${server.address().port}`);
  return server;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) await start();
