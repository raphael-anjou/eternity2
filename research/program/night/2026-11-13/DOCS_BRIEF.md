# Common brief for the documentation sessions of 2026-11-13

The owner gave the lead ownership of the notes, the experiments and the published documentation, to make them more accurate and more trustworthy. Site pages and program files may now be edited directly. Nothing is pushed.

## Where and how
- Shared worktree: <repo>, branch `night/2026-11-13-docs`. Never touch the main checkout (<repo>, the owner works in it) or another worktree; read other branches with `git show branch:path`.
- Several sessions edit this worktree at once. You own ONLY the files listed in your own brief. If a correction belongs in a file you do not own, do not edit it: list it in your final message.
- Commit only your own paths (`git add <explicit paths>`, never `git add -A` or `.`), one logical step per commit, one-line messages with a conventional prefix (`docs:`, `fix:`, `research:`), no attribution lines of any kind, no Co-Authored-By. If git reports an index lock, wait a few seconds and retry.
- At most ONE CPU core on this Mac, shared by everyone: no `pnpm install`, no `pnpm build`, no cargo. `web/node_modules` is absent in this worktree on purpose; the lead runs the full build and the gates once at the end on another machine. You may run a stdlib-only node or python script (`python3 -I`) single threaded. Read `web/scripts/check-research-style.mjs` (or wherever `check:research` lives) and obey its rules by hand.
- Shell discipline: no `eval`, no nested command substitutions; multi-step work in script files. If a command does not return in two minutes, rephrase it as a plain script call. Do not wait on any person.

## Rules of the repo that bind you (read AGENTS.md first, all of it)
- No em dash anywhere, in any language, in prose, code or comments. No banned phrase ("honest", "honestly" and the rest of the gate's list).
- Three languages: `page.mdx` (EN), `page.fr.mdx`, `page.es.mdx`. Every change lands in all three, and the sidecars mirror the EN frontmatter. French and Spanish are written, not translated word for word; keep full diacritics.
- Every number is tied to a committed result or an opened source. If you cannot tie it, remove the number or mark the statement with its grade (measured, reported, argued) in plain words. Do not add a number whose source is a memo that its referee struck.
- `/research/records` is the single home of the headline numbers; other pages link to it and do not restate them. The expected number of solutions is about 14,702 with the starter clue and about 1 with all five (ensemble estimate); never "one solution".
- Attribution-neutral: state findings and methods plainly; do not describe sessions, agents, nights, judges or the process that produced them. On site pages, cite evidence by committed files and public sources only. Branch names of unmerged work are not evidence a reader can open: when a correction rests on unmerged work, state the correction that the committed data and public sources support and leave the rest out.
- `sources[].url` must be an absolute URL; internal pages go in `related[]`.
- Update the `updated:` frontmatter date (2026-11-13) on every page whose claims change.
- Scientific rigor checklist: `.claude/commands/rigor-check.md`. Apply it to every page you change: null model, title and description no stronger than the body, numbers recomputed, site-wide grep for the same claim (report other pages that repeat a claim you corrected, even if you do not own them).

## Material from the night of 2026-11-12 (all in this worktree under research/program/night/2026-11-12/)
`REPORT.md`, `OBSERVATIONS.md`, `EVIDENCE_AUDIT.md`, `DRAFTS.md`, `PROTOCOL_AMENDMENT_DRAFTS.md`, `memos/` (each memo has a `.REVIEW.md`; the review wins over the memo), `checks/verify-466/`. Under `research/program/night/2026-11-13/`: `ROUTES.md` (draft index of every route examined, with flagged disagreements), `STATUS.md`, `memos/`.

## Final message to the lead
Files changed with one line each on what changed and why; every number you changed with old value, new value and its source; what you removed because it could not be tied to evidence; corrections needed in files you do not own; anything you were unsure about and left alone.
