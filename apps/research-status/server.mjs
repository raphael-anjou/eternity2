import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { execFile, execFileSync } from 'node:child_process';
import { promisify } from 'node:util';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';
import { parseResearch, agendaPath, ledgerPath } from './data.mjs';

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, '../..');
const branch = process.env.SOURCE_BRANCH ?? 'main';
const upstream = 'https://github.com/raphael-anjou/eternity2.git';
const cacheRef = `refs/research-status/${branch}`;
const localOnly = process.env.SOURCE_MODE === 'local';
const refreshInterval = 5 * 60 * 1000;
const run = promisify(execFile);
const git = async (args) => (await run('git', args, { cwd: repoRoot, encoding: 'utf8', timeout: 30000, maxBuffer: 8 * 1024 * 1024 })).stdout.trim();
let snapshot;
let refreshing;
let lastAttempt = 0;

async function localSnapshot() {
  const git = (args) => execFileSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).trim();
  // Never serve uncommitted notes. Online mode starts from the known public branch.
  let revision;
  if (localOnly) revision = git(['rev-parse', 'HEAD']);
  else {
    try { revision = git(['rev-parse', '--verify', cacheRef]); }
    catch {
      const origin = git(['remote', 'get-url', 'origin']);
      if (!/[:/]raphael-anjou\/eternity2(?:\.git)?$/.test(origin)) throw new Error('No verified upstream snapshot cached');
      revision = git(['rev-parse', '--verify', `refs/remotes/origin/${branch}`]);
    }
  }
  const agenda = git(['show', `${revision}:${agendaPath}`]);
  const ledger = git(['show', `${revision}:${ledgerPath}`]);
  const experiments = git(['ls-tree', '-d', '--name-only', `${revision}:research/experiments`]).split('\n').filter(Boolean);
  const sourceDate = git(['log', '-1', '--format=%cI', revision, '--', agendaPath, ledgerPath, 'research/experiments']);
  return parseResearch({ agenda, ledger, experiments, revision, sourceDate,
    checkedAt: new Date().toISOString(), source: 'local', warning: localOnly ? null : 'Local checkout snapshot. Checking the published source…' });
}

export async function refresh() {
  if (refreshing) return refreshing;
  if (Date.now() - lastAttempt < 60000) return;
  lastAttempt = Date.now();
  refreshing = (async () => {
    try {
      if (localOnly) { snapshot = await localSnapshot(); return; }
      // Use Git's public transport, not the unauthenticated GitHub API rate limit.
      const ref = await git(['ls-remote', upstream, `refs/heads/${branch}`]);
      const revision = ref.split(/\s/)[0];
      if (!/^[a-f0-9]{40}$/.test(revision)) throw new Error('Source branch was not found');
      if (snapshot?.meta.source === 'github' && snapshot.meta.revision === revision) {
        snapshot.meta.checkedAt = new Date().toISOString(); snapshot.meta.warning = null; return;
      }
      try { await git(['cat-file', '-e', `${revision}^{commit}`]); }
      catch { await git(['fetch', '--no-tags', upstream, revision]); }
      const [agenda, ledger, directories, sourceDate] = await Promise.all([
        git(['show', `${revision}:${agendaPath}`]), git(['show', `${revision}:${ledgerPath}`]),
        git(['ls-tree', '-d', '--name-only', `${revision}:research/experiments`]), git(['show', '-s', '--format=%cI', revision]),
      ]);
      const next = parseResearch({ agenda, ledger, experiments: directories.split('\n').filter(Boolean),
        revision, sourceDate, source: 'github', checkedAt: new Date().toISOString() });
      await git(['update-ref', cacheRef, revision]);
      snapshot = next;
    } catch (error) {
      if (snapshot) snapshot.meta.warning = 'Could not check GitHub. Showing the last available snapshot, not live activity.';
    }
  })().finally(() => { refreshing = null; });
  return refreshing;
}

const assets = new Map([
  ['/', ['index.html', 'text/html; charset=utf-8']], ['/app.js', ['app.js', 'text/javascript; charset=utf-8']],
  ['/style.css', ['style.css', 'text/css; charset=utf-8']], ['/icon.svg', ['icon.svg', 'image/svg+xml']],
  ['/manifest.webmanifest', ['manifest.webmanifest', 'application/manifest+json']],
  ['/icon-192.png', ['icon-192.png', 'image/png']], ['/icon-512.png', ['icon-512.png', 'image/png']],
]);

export async function start(port = Number(process.env.PORT ?? 4317)) {
  snapshot = await localSnapshot().catch(() => null);
  if (!snapshot) await refresh();
  const server = createServer(async (req, res) => {
    res.setHeader('X-Content-Type-Options', 'nosniff');
    res.setHeader('Referrer-Policy', 'no-referrer');
    res.setHeader('Content-Security-Policy', "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; font-src 'self'; object-src 'none'; base-uri 'none'");
    let path;
    try { path = new URL(req.url, 'http://localhost').pathname; }
    catch { res.writeHead(400); res.end('Bad request'); return; }
    if (!['GET', 'HEAD'].includes(req.method)) { res.writeHead(405); res.end(); return; }
    if (path === '/api/research') {
      if (new URL(req.url, 'http://localhost').searchParams.has('refresh')) await refresh();
      if (!snapshot) { res.writeHead(503, { 'Content-Type': 'application/json' }); res.end(JSON.stringify({ error: 'No source snapshot available; retry shortly.' })); return; }
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
  await new Promise((done) => server.listen(port, process.env.HOST ?? '127.0.0.1', done));
  void refresh();
  const timer = setInterval(() => void refresh(), refreshInterval);
  timer.unref();
  server.on('close', () => clearInterval(timer));
  console.log(`Research threads listening on ${server.address().port}`);
  return server;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) await start();
