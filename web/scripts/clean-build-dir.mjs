#!/usr/bin/env node
// Remove build/ before a build.
//
// `react-router build` runs Vite more than once (a client pass and a server
// pass). Each pass empties the out dir it owns, and against a build/ tree left
// over from a previous run those cleanups race the writes: the build fails
// intermittently with ENOTEMPTY, or with ERR_MODULE_NOT_FOUND /
// "ENOENT ... build/client/.vite/manifest.json" when one pass deletes an
// intermediate the next pass is still reading. It is a flake, not a code
// error: the identical source builds fine against a clean tree.
//
// Starting from nothing removes the race entirely and costs a few milliseconds.
// It also means a stale artifact from an older commit can never survive into a
// new build, which is what made "it works locally" hard to trust.

import { rmSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const dir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "build");
rmSync(dir, { recursive: true, force: true });
