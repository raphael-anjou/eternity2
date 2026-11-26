# Research threads

A read-only workspace for the Eternity II research agenda, ledger, and experiment folders. Runs separately from the public static site; no web dependencies or credentials required.

```sh
cd apps/research-status
npm run build
npm test
npm start
```

Open port 4317. Set `PORT` to change it. Requires Node 22 or newer.

## Source and freshness

- Reads the local checkout immediately, explicitly labelled **Local snapshot**.
- Checks public GitHub `main` every five minutes. `SOURCE_BRANCH` selects another branch.
- Fetches the agenda, ledger, and experiment list at one immutable commit, so the source links match the displayed data.
- If GitHub is unavailable or rate-limited, retains the last good snapshot and displays a warning. A restart falls back to the local checkout.
- The browser polls the server snapshot each minute; **Refresh source** reloads that snapshot. It does not bypass the five-minute GitHub interval.
- `SOURCE_MODE=local npm start` disables GitHub and rereads local files every five minutes.

Statuses are read literally from numbered agenda headings, not inferred from ledger prose or running processes. Qualifiers (such as "killed for the narrow family priced") remain visible in thread details. **Unresolved** groups open and proposed; **Advanced** groups promoted and published; **Closed** groups killed. Reopening conditions remain labelled separately from hypotheses. Later ledger entries can supersede agenda text, so the details warn about that and the research log stays accessible.

The app makes no new scientific claims or changes to research records. It uses the site's neutral styling and system-font fallback, semantic tables, keyboard search (`/`), native modal dialogs (Escape to close), and responsive layouts. It has no service worker and does not promise offline access.
