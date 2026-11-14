# Draft protocol and kit amendments, night 2026-11-12

Proposals only. `research/experiments/hard-ladder/PROTOCOL.md` and
`research/program/EXPLORED.md` were not edited (a night shift may not edit
them). Each draft adds a rule and weakens none. Each names the event of the
night that motivates it.

## P1. Run-seed control for any arm that changes search order

**Rule.** An arm that changes the order of the search (cell order, value order,
or a pruning rule that changes which cell the ordering picks) is compared with
a random-order twin of its parent over at least 3 run seeds, on the same
instances. A G1 pass counts only if the arm's ratio lies outside the range of
the twin's run seeds.

**Why.** Track 3 (A21), judged: at 8x8 c=10 the random-value-order control
`mrv-rand-s3` passes G1 against `bb-mrv` (0.495, CI [0.409, 0.768]) with no
mechanism, while seed 1 sits at 1.623. `bb-mrv`'s index order is one draw from
that spread.

## P2. A growth stage for every method that claims relevance beyond small boards

**Rule.** A method track that passes at R7 or 8x8 c=10 must, before promotion,
report the slope of log work on board size over a fixed colour track
(6x6 c=7, 7x7 c=8, 8x8 c=10, and 9x9 c=11 where the baseline solves), paired
against its baseline, with a CI. No statement about the 16x16 without it.

**Why.** PROTOCOL_TARGETS_PROPOSAL.md already says only growth with n bears on
the 16x16; none of the 13 idea-slot proposals preregistered a growth stage
(tournament judge, section 3c).

## P3. Evidence grade on every "explored" or "dead end" claim

**Rule.** EXPLORED.md and the dead-ends page carry, per row or entry, one of:
`measured` (a committed result or reproducible command in this repo, with its
path), `reported` (someone else's number, with the citation), `argued`
(reasoning, or one small attempt with nothing committed). Only a `measured`
stop may be used to reject a new idea as a variant; `reported` and `argued`
stops are risks to state. The page's own word "Measured" is reserved for the
first grade.

**Why.** The GPU page presented GPUs as a dead end until the 2026-11-11
rewrite. The tournament judge graded 21 claims its ranking leaned on: 7
measured, 8 reported, 6 argued, including entries labelled "Measured" with no
committed data. See `EVIDENCE_AUDIT.md` when the audit completes.

## P4. Long runs are readable and resumable

**Rule.** A run with a cap above 1e7 nodes writes a progress record at a fixed
node interval (nodes so far, deepest placement, share of the search tree
closed, computed from the stack) and, at the cap, a snapshot of its search
stack, so the same run can be continued to a higher cap. A continued run must
be node-identical to an uninterrupted one (hash-gated, as an optimization
track).

**Why.** Track 1's capped 9x9 runs say only "more than 1e8 nodes" after two
hours each, and cannot be extended without starting again.

## P5. R6 is a smoke test, not a decision rung

**Rule.** 6x6 c=5 (about 951 baseline nodes) keeps its place as the
implementation and dropping check, but a wall-clock claim needs a rung where
runs last at least 100 ms, and tie-breaking evidence comes from R7 and the
growth track.

**Why.** Tournament judge: a factor of 2 at R6 is noise in wall clock; most
proposals and three of tonight's tracks lean on it.

## P6. Machine placement

**Rule.** Node-count and work-count tracks may run on any machine that first
reproduces a committed reference run node for node; the machine, CPU and
binary checksum are recorded per row. Tracks with a wall-clock gate run on one
machine, arms interleaved. Wall clock is never compared across machines.

**Why.** Tonight Tracks 1, 5, 6 and 7 ran on `machine-b` and `machine-c` after each
reproduced A22's `bb-mrv` rows exactly on 8x8 c=10 seeds 21 to 25.

## P7. Decide on work or time, never on nodes alone

**Rule.** G1 is computed on a work measure that charges propagation (counted
propagation operations, or instructions, or wall clock with arms interleaved on
one machine), and plain row-major DFS (`hp-dfs`) is a mandatory arm at every
rung, reported in nodes, work and seconds. A node gain that is a work or time
loss is labelled NO-GAIN. Nodes stay a reported column.

**Why.** PROTOCOL.md counts one node per assignment whatever the assignment
costs. A referee tonight ran plain row-major DFS on 9x9 c=11: median 1.56e8
nodes in 2 to 6 s per instance, while the pilot's `bb-mrv` took 6,772 s for
one solve of 7.8e7 nodes (about half the nodes, about a thousand times the
time). A17 had already measured plain DFS at 26 times lower wall time at 8x8
c=10. PROTOCOL_TARGETS_PROPOSAL.md proposes the work measure; this draft makes
the plain-DFS arm and the label rule explicit.

## P8. A mechanical verdict issued through a proxy names the proxy

**Rule.** When an experiment issues a kill or pass on a proposal by measuring a
stand-in for its mechanism (a one-shot check for a persistent filter, a share
for a ratio), the PREREG says so in those words, and the verdict is recorded as
"killed by proxy" or "not decided", never as a kill of the proposal. Controls
are matched on every parameter that changes cost (number of partitions, budget).
A criterion that compares medians defines what happens on ties and on infinite
values, and a tie never fires a kill.

**Why.** Track 6's census: one kill rested on a tie of two infinite medians, and
one on a random control with a median of 16 partitions against the candidate's
6. Its judge overturned both.

## P9. A memo is a claim until refereed

**Rule.** A derivation or literature memo enters the ledger, the agenda or a
brief only after an independent referee has ruled on its claims one by one
(holds, holds with a correction, not established, wrong). Until then it is
quoted as "claimed in".

**Why.** Tonight's growth-law memo claimed that no ladder rung is critical. The
lead forwarded it as a hypothesis within minutes. A referee refuted it by
enumeration three quarters of an hour later.

## P10. Censored pairs are not ties

**Rule.** A paired statistic never scores "both runs hit the cap" as a tie at
zero. Either the statistic is defined on per-solver Kaplan-Meier medians, or the
PREREG fixes a cap at which the baseline is expected to finish on a stated share
of instances and declares the run uninformative, not killed, below that share.

**Why.** Track 1: the baseline hit the cap on 8 of 8 seeds, six pairs scored 0,
and the kill criterion on the median fired as a side effect. Its judge ruled
that the cap, not the ratio, decided.

## P11. A model of a search tree is checked against an enumerated profile

**Rule.** A cost model is validated on the per-depth node counts of trees small
enough to enumerate, and, where a sampler exists, on a sampled profile of the
target instance. Reproducing the solution count is not a validation.

**Why.** Two memos spread the full-board match probability evenly over joints.
Both reproduced the 14,702 solution count, the one depth where the wrong and
right models agree, and both were 4.6 to 4.8 digits off on the tree. Track 10's
sampler then confirmed the corrected model on the real pieces to 0.03 digits.

## P12. A port is checked on its shape, not on its total

**Rule.** When an engine is ported or instrumented, the equivalence gate compares
per-depth histograms, or the deepest depth at several caps, against the
original. A node total at the cap is equal by construction and proves nothing.

**Why.** Track 10's preregistered gate compared the node total at the cap. Its
judge extended it to deepest depth at six caps on three seeds (18 of 18 agree).

## P13. An unattended runner never waits on a person

**Rule.** In an unattended shift a runner puts every multi-step launcher in a
script file and runs the file; no `eval`, no inline shell functions. A command
silent for two minutes is treated as held by a permission prompt and rephrased.
The lead checks every running track's last commit and last action at each
hourly check, not only its notifications, and a runner that waits on remote
jobs sets a fallback timer.

**Why.** Tonight: Track 11 lost an hour to a command that never started;
Track 5's wait loop missed the end of its runs for an hour; Track 1's runner
session was lost and went unnoticed for 50 minutes; one earlier wait loop
looked for a word that zsh's `time` never prints.

## P14. Seeds are checked for collisions

**Rule.** A stochastic entrant reports the number of distinct runs its run seeds
produce on a calibration instance. Equal rows across seeds are a defect of the
entrant, reported in the table.

**Why.** Track 5's judge found `fc-luby` seeds with `(seed ^ hash) | 1`, so
seeds 2k and 2k+1 are the same run and "three run seeds" were two.

## Kit changes already made on the lead branch (reviewable in the diff)

- `.claude/agents/runner.md`: correct path to PROTOCOL.md; PREREG as a commit
  of its own; the run-seed control of P1; `target/` excluded from em-dash greps.
- `.claude/agents/judge.md`: check result timestamps when PREREG and results
  are committed minutes apart; look for the missing run-seed control; grade any
  explored or dead-end claim the verdict leans on.
- `.claude/agents/scout.md`: open the cited page before calling an idea
  explored; grade "does not work" claims.
- `scripts/night-eta.py`: local tracker (stage, progress, ETA per track, other
  machines over SSH).
