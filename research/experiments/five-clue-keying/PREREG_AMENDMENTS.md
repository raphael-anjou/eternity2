# five-clue-keying: amendments

## Amendment 1 (2026-11-13, about 02:50 local): the Track 13 judge's findings

What, why, when: the lead passed on three findings of Track 13's judge
(`quota-keying/JUDGE.md` at 17c5688 on Track 13's branch) at 02:47. PREREG.md
was committed at 6bcad1c (02:41); at the time of this amendment only the port
gate had been started (stage `gate`, this Mac); no screening and no
confirmation run had started. PREREG.md is unchanged; this file adds to it.

1. **Wall clock.** Per-node gains can be wall-clock losses (Track 13: on dell
   a published-keying run with no clue cost about 1,500 s per 1e9 nodes
   against about 64 s for the default). Every row already records `seconds`
   (wall time of the search). Added:
   - Confirmation runs are dealt to hosts **by opening**: all arms of one seed
     run on the same host (`scripts/jobs.py confirm`, fixed deal: of seeds
     401..420, dell takes 10, mac-m1 6, this Mac 4, by a fixed pattern in the
     script). Pairing is therefore by seed and by host.
   - Per-second statistic: Ysec = (sum over openings of legal nodes at depth
     >= 201) / (sum over openings of run seconds). Every confirmation ratio
     (each arm against D; best screened against best random) is reported
     twice, per node spent (the preregistered statistic) and per second, each
     with the same paired bootstrap over openings.
   - Reading, added before any confirmation run: if the per-node rule of
     PREREG.md says "lever" but the per-second point ratio of the best
     screened keying is below 1 against D or against the best random key, the
     reading is **"not a lever: wins per node, loses per second"**. The other
     branches of the reading are unchanged.
   - Screening: Ys per second is reported for every keying, descriptive only;
     the ranking stays per node as preregistered (screening runs are spread
     over hosts, so per-second screening numbers mix machines).
2. **Port quirk.** Some openings spend the whole budget in an illegal subtree
   from the first moves (the port's no-break path may place a piece showing
   grey toward an interior cell; Track 13 seeds 105 and 108, 2 to 5 legal
   nodes per 1e9). Runs with **fewer than 100 legal nodes** are flagged, kept
   in every preregistered statistic, and the confirmation statistics are
   also reported without them (a flagged run contributes to neither sum of
   its arm; descriptive only). The engine is not changed (no fix variant).
3. **Scope wording.** Every reading holds "at 1e9 nodes per run, one attempt
   per opening, on this port, with a row scan from the top-left corner
   without border interleaving", and is written that way.

## Amendment 2 (2026-11-13, 03:48 local): runner fix only

What: `scripts/host_run.sh` skipped writing the confirmation job list (a line
meant for an earlier plan where the list was written locally), so a remote
host would have had no job file. Fixed in f21d8f5, before the first
confirmation run; no arm, seed, cap, statistic or reading changed. The
screening finished at 03:47 with all 2,043 runs (no reduction applied); the
confirmation arms are those printed in `results/screen.txt` (11 distinct
keyings: P is also screening rank 4).
