import test from 'node:test';
import assert from 'node:assert/strict';
import { connect } from 'node:net';

process.env.SOURCE_MODE = 'local';
const { start } = await import('./server.mjs');

test('malformed URL returns 400 without terminating the server', async () => {
  const server = await start(0);
  try {
    const port = server.address().port;
    const response = await new Promise((resolve, reject) => {
      const socket = connect(port, '127.0.0.1', () => socket.write('GET http://[ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n'));
      let data = '';
      socket.on('data', (chunk) => { data += chunk; }); socket.on('error', reject); socket.on('end', () => resolve(data));
    });
    assert.match(response, /400 Bad Request/);
    assert.equal((await fetch(`http://127.0.0.1:${port}/api/research`)).status, 200);
    const encoded = await fetch(`http://127.0.0.1:${port}/api/research`, { headers: { 'Accept-Encoding': 'gzip' } });
    assert.equal(encoded.headers.get('content-encoding'), 'gzip');
    assert.ok((await encoded.json()).threads.length);
    const identity = await fetch(`http://127.0.0.1:${port}/api/research`, { headers: { 'Accept-Encoding': 'gzip; q=0, identity' } });
    assert.equal(identity.headers.get('content-encoding'), null);
    assert.ok((await identity.json()).threads.length);
    assert.equal((await fetch(`http://127.0.0.1:${port}/server.mjs`)).status, 404);
  } finally { await new Promise((done) => server.close(done)); }
});
