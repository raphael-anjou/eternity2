# Amendments

## 1. Second machine and thread count (19:55, 2026-11-12, after tier 1 and part of tier 2 had run)

What: the lead allowed more threads (`dell` up to 12 busy in total, `mac-m1`
up to 7 busy in total). The Dell run (4 threads, already running) was not
restarted. Tiers 4, 5 and 6 (entrants 33 to 49, seeds 2 and 3 of every
stochastic entrant except `scb-gacolor_ac3_random`, and the two check arms)
moved to `mac-m1` (Apple M1, 3 threads, shared with 4 single-thread runs of
Track 1), listed in `mac-m1-part.txt` and run with `run.sh part`. Empty
placeholder files on the Dell make its tier-order script skip that work; they
are not results and the analysis ignores empty files. Each (entrant, seed,
rung) runs wholly on one machine; every row records `machine`, `cpu`, `load`.

Why: at 4 threads the preregistered order would not reach tiers 4 to 6 before
the 22:00 launch deadline.

Check before trusting mac-m1 rows: `mac-m1-part.txt` first reruns
`scb-gacolor_ac3_random` run seed 1 on all three rungs (90 rows the Dell
already produced); the analysis compares them node for node
(`results/check-mac-m1/`). Wall clock is a per-machine column and is never
compared across machines (the G1 wall ratio of a mac-m1 entrant against a
Dell baseline is reported as not comparable).

Code change for this: the machine description in each row comes from
`TOURNAMENT_MACHINE`, `TOURNAMENT_CPU`, `TOURNAMENT_LOAD` (defaults = Dell);
`run.sh` gained the `part` stage and a `TAG` suffix for log files. The search
code is unchanged.

## 2. One extra Dell thread for the A16 arms at R6 (19:58, 2026-11-12)

What: with 7 Track 1 runs live on the Dell, the lead's rule (at most 12 busy)
left one thread beyond the 4 in use. A second Dell process with 1 thread runs
`bb-impact`, `bb-learned`, `bb-single` at R6 only, writing to
`results/raw-dell2/` (log `results/log-dell2.txt`). Empty placeholders make the
main Dell script skip those three R6 files; their R7 and R8 stay in the main
tier order.

Why: the three A16 arms are last in tier 3 and each costs up to 60 runs of the
240 s guard at R7 and R8; R6 is cheap and would otherwise likely not be reached.

## 3. A16 arms at R7 and R8 split across the free threads (20:24, 2026-11-12)

What: Track 1 now holds 7 runs on the Dell and 2 on mac-m1, which frees 1
Dell thread and 2 mac-m1 threads under the lead's rule. `bb-single` R7 and R8
run on mac-m1 (2 threads, `results/raw/` there, log
`results/log-mac-m1-b.txt`); `bb-learned` R8 runs in a second 1-thread Dell
process (`results/raw-dell2/`). Placeholders make the main Dell script skip
them; `bb-impact` (R7, R8) and `bb-learned` R7 stay in the main tier order.

Why: at up to 240 s per run (A16's guard), the three arms would not all launch
before the 22:00 deadline on 4 threads. The guard makes these rows depend on
the machine and its load; the time-censored count is reported per row.

## 4. Fixed thread quotas (20:38, 2026-11-12)

What: the lead replaced the free-thread rule by fixed quotas for this track:
Dell 4 threads in total, mac-m1 2 threads in total, effective from the next
launch, nothing running to be killed. Before that message this track had 5
busy threads on each machine (Dell: 4 + the 1-thread `bb-learned` R8 process;
mac-m1: the 3-thread `run.sh part` driver + the 2-thread `bb-single` process).
Actions: the two tier-order drivers (`run.sh run` on the Dell, `run.sh part`
on mac-m1) were stopped between launches; the solver processes they had
started were left to finish. The Dell's remaining preregistered work is driven
by `dell-driver-a4.sh` with 3 threads (4 in total with the 1-thread process):
it waits for `bb-impact` to finish, then runs `bb-learned` R7, then the tier 5
and 6 items mac-m1 can no longer take (`cmp-hall-all-k4`, `cmp-hall-all`,
`cmp-hall-frontier-k4` seeds 2 and 3; `scb-fork-off`, `cmp-fork-off`), in that
order, under the same 22:00 launch deadline. mac-m1 launches nothing new
(its 2-thread `bb-single` process fills its quota).

Why: quota compliance; the order and deadline are unchanged.
