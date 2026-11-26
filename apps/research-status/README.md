# Research threads

A read-only workspace for the Eternity II research agenda, ledger, and experiment folders. Runs separately from the public static site; no web dependencies or credentials required.

```sh
cd apps/research-status
npm run build
npm test
npm start
```

Open port 4317 on localhost. Set `PORT` to change it. Requires Node 22 or newer.
The server binds to `127.0.0.1` by default. Set `HOST=0.0.0.0` only behind an authenticated reverse proxy or on a trusted network.

## Source and freshness

- Reads the committed local `origin/main` snapshot immediately, explicitly labelled **Local snapshot**. Uncommitted working-tree notes are never served.
- Checks the canonical upstream GitHub repository every five minutes, avoiding unauthenticated API limits and fork-specific data. `SOURCE_BRANCH` selects another upstream branch. Git and network access are required for updates.
- A cached verified upstream commit is preferred at startup. If a tracking ref is missing or the checkout is a fork, the server fetches upstream before serving data. If no source is available it serves the app with an explicit unavailable state, rather than exiting.
- Fetches the agenda, ledger, and experiment list at one immutable commit, so the source links match the displayed data.
- If GitHub is unavailable, retains the last good snapshot and displays a warning. A restart falls back to the committed local snapshot.
- The browser polls the server snapshot each minute using conditional requests; unchanged snapshots return no body. **Refresh source** also checks GitHub. Network checks are limited to once per minute across all viewers.
- `SOURCE_MODE=local npm start` disables GitHub and rereads committed `HEAD` every five minutes. Use it only for local previews of unpublished branches.

Statuses are read literally from numbered agenda headings, not inferred from ledger prose or running processes. Qualifiers (such as "killed for the narrow family priced") remain visible in thread details. **Unresolved** groups open and proposed; **Advanced** groups promoted and published; **Closed** groups killed. Reopening conditions remain labelled separately from hypotheses. Later ledger entries can supersede agenda text, so the details warn about that and the research log stays accessible.

The log includes dated level-two and level-three headings as separate entries with their own source anchors. Related entries use explicit thread IDs, dated result-track citations, and non-baseline experiment references; the broad `approach-pricing` folder alone is not enough. Folder references retain their agenda field labels so baselines are not presented as measured outcomes. Later negative ledger results can be highlighted without rewriting an agenda status.

The app makes no new scientific claims or changes to research records. It uses the site's neutral styling and system-font fallback, semantic tables, keyboard search (`/`), native modal dialogs (Escape to close), and responsive layouts. It has no service worker and does not promise offline access.
