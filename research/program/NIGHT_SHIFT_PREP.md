# Night shift for Eternity II: research survey and prep plan

## 1. What the autonomous-research literature says

| System | Structure | What carries over to E2 |
|---|---|---|
| Google AI co-scientist (2025) [blog](https://research.google/blog/accelerating-scientific-breakthroughs-with-an-ai-co-scientist/) | Supervisor plus generation, reflection, Elo-tournament ranking, evolution, meta-review agents | Generate many ideas, then rank by pairwise debate instead of one lead's taste. Our idea round lacks the ranking step. |
| AlphaEvolve (2025) [arXiv:2506.13131](https://arxiv.org/abs/2506.13131); Tao et al., 67 problems [arXiv:2511.02864](https://arxiv.org/abs/2511.02864) | LLMs mutate code; an automatic evaluator scores every program; evolutionary archive | Works when there is a cheap, unhackable scorer. Our ladder node count on held-out seeds is exactly that. Open versions: [OpenEvolve](https://github.com/algorithmicsuperintelligence/openevolve) (can call Claude Code), [ShinkaEvolve](https://github.com/SakanaAI/ShinkaEvolve). |
| Kosmos (Edison, 2025) [arXiv:2511.02824](https://arxiv.org/html/2511.02824) | Up to ten parallel analysis and literature agents per cycle, all writing to one structured "world model" | Our EXPLORED, AGENDA and LEDGER files already play this role. Keep every agent reading and writing them. |
| AI Scientist-v2 (Sakana, 2025) [arXiv:2504.08066](https://arxiv.org/abs/2504.08066) | Agentic tree search over experiments, end-to-end papers | Its authors report weak novelty and costly verification. Automation of the pipeline is not the bottleneck; judging is. |
| Anthropic harness posts [long-running agents](https://www.anthropic.com/research/effective-harnesses-for-long-running-agents), [planner/generator/evaluator](https://www.anthropic.com/research/harness-design-long-running-apps) | Fresh context per unit of work, a progress file and git as the handoff, a separate evaluator | Short-lived runners and judges with files as memory beat one long session that fills its context. |

**Failure modes, measured:**
- An arXiv study across 17 models and 38 tasks found spontaneous reward hacking in 30.5% of open-ended research-pipeline tasks. An LLM panel that saw only code and scores missed 6.5% of confirmed hacks. Their fix: metrics outside the agent's control and independent recomputation ([arXiv:2609.28614](https://arxiv.org/abs/2609.28614)).
- PNAS (2026) found AI scientist systems cherry-pick benchmarks, silently subsample data or generate undocumented synthetic data, and report only positive results ([PNAS](https://www.pnas.org/doi/10.1073/pnas.2610214123)).
- Our own program already defends against most of this: preregistration, fixed ladder instances and NodeCounter, separate judges that recompute from raw files. The night shift must keep those defences when no human is watching.

**Why our `bb-mrv` find happened:** it came from a forced strong control, reuse of a component from another experiment, and a same-instance comparison. The kit makes those three mandatory: strong control in every prereg, a baseline tournament track, and an "Unplanned observations" section in every report.

## 2. How to run Fable 5.2 leading Opus 5.5 and Sonnet 5.5 in Claude Code

Verified in Claude Code docs:
- **Models:** `claude --model fable` selects Fable 5.2. Subagent files take `model: opus`, `model: sonnet` or a full ID such as `claude-opus-5-5` ([subagents](https://code.claude.com/docs/en/sub-agents), [model config](https://code.claude.com/docs/en/model-config)). Anthropic names "Orchestrator" (Fable delegating to smaller models) as an intended role ([announcement](https://www.anthropic.com/claude-fable-and-mythos-5-2)).
- **Subagents versus agent teams:** agent teams (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`) are experimental, with "known limitations around session resumption, task coordination, and shutdown behavior", need an interactive session, and cost more tokens ([agent teams](https://code.claude.com/docs/en/agent-teams)). For an unattended night I recommend plain subagents, which report back to the lead. That is also what our judge separation needs.
- **Keeping it going:** `/goal <condition>` re-runs turns until a separate evaluator model judges the condition met. If a usage limit is hit, the goal pauses and resumes when the limit resets ([goal](https://code.claude.com/docs/en/goal)). Run it in auto mode so tool calls do not wait for you ([permission modes](https://code.claude.com/docs/en/permission-modes)).
- **Watching from your phone:** `claude --remote-control` (or `--rc`) lets you follow and message the local session from the Claude iOS app. It reconnects if the laptop sleeps, but work stops while it sleeps ([remote control](https://code.claude.com/docs/en/remote-control)). Hence `caffeinate`.
- **Usage limits:** on Max, Fable may use up to half the weekly limit. There is also a 5-hour session limit, so an 8-hour night crosses at least one reset. Ultra has no 5-hour limit. Usage credits can extend either ([plan help](https://support.claude.com/en/articles/11145838-use-claude-code-with-your-pro-max-or-ultra-plan), [Fable on your plan](https://support.claude.com/en/articles/15424964-claude-fable-models-on-your-plan)). Keep Fable to the lead role and push volume to Opus and Sonnet.
- **Fable consent:** on some plans the first Fable use asks a one-time consent to bill usage credits, interactively. Accept it before the night, or the session can stall at the first prompt.

API list prices per million tokens (input/output): Fable 5.2 $8/$40, Opus 5.5 $4/$20, Sonnet 5.5 $2/$10.

## 3. Role split

| Role | Model | Why |
|---|---|---|
| Lead (plans, assigns cores, sequences judges, writes the morning report) | Fable 5.2 | Long-horizon judgement, few tokens |
| Runner (one per experiment) | Opus 5.5 | Writes Rust and analysis code, most tokens |
| Judge (one per result, fresh context) | Opus 5.5 | Must be at least as strong as the runner it checks |
| Scout (novelty checks, papers, finding code) | Sonnet 5.5, read-only tools | Cheap, parallel |
| Plumber (lint, CI, ledger drafts) | Sonnet 5.5 | Routine, cannot touch results |

## 4. Things that would spoil the night, and the guard for each

1. **Starting from a stale main.** Runners branch from `origin/main`, so `git pull` first and check that main has `research/experiments/mrv-baseline/` and the A18 to A22 agenda entries.
2. **CPU oversubscription.** Several runners on one laptop make every wall-clock number meaningless and slow everything. The brief assigns cores per track; runners pin threads.
3. **Permission stalls.** Auto mode can still refuse a command and wait. The launcher loads the allow and deny rules in `.claude/settings.night.json` with `--settings`; do the rehearsal below.
4. **Mac sleeping or the lid closing.** Use `caffeinate -dimsu` (the launcher does), power plugged in.
5. **Usage limit mid-night.** `/goal` resumes after the reset, but nothing runs in the gap. Expect a gap on Max.
6. **Moving goalposts.** Prereg is pushed before any test seed, and the judge checks commit order. Deny rules block edits to PROTOCOL.md and EXPLORED.md.
7. **Destructive actions.** Deny rules for merges, force pushes, pushes to main, `aws`, `rm -rf`, and `pnpm build` (parallel builds collide, per AGENTS.md).
8. **Runaway or silent tracks.** Hourly STATUS.md pushes, and a 90-minute no-commit rule.
9. **Morning overload.** One REPORT.md, one line per judged track, and observations kept separate from results.

## 5. Rehearsal (30 minutes, the evening before)

1. `git pull` on main after the merges; `cargo build --release` in `research/experiments/hard-ladder/compute` and `mrv-baseline/compute`.
2. Set the date, end time and cores in NIGHT_BRIEF.md.
3. Run `scripts/night-shift.sh 07:00`, attach with `tmux attach -t night`, accept the Fable consent if asked.
4. Instead of the real goal, give it: "Run one runner on Track 3 with seeds limited to 3 at R6 only, then one judge, then stop." Check:
   - the runner shows `opus`, the scout shows `sonnet`,
   - no permission prompt was needed,
   - the phone app shows the session through Remote Control,
   - the judge only got the branch name.
5. Delete the rehearsal branch, then launch the real night.

## 6. Targets, track types, and anti-tuning

- `research/program/PROTOCOL_TARGETS_PROPOSAL.md` proposes the targets:
  - a verification gate,
  - solves within a work budget at critical colours, where work charges propagation,
  - paired work ratio,
  - growth slope over n,
  - wall clock as a guardrail.
  It also proposes two track types. An optimization must keep the node sequence identical, checked by hash. Anything else is a method.
- About 80% of compute goes to method tracks and at most 20% to optimization.
- The idea slot uses `inventor` subagents:
  - Each starts from a recorded structural stall.
  - Each imports a named mechanism with its paper. Four of the ten must combine two fields.
  - Tuning, speedups and restarts are excluded.
  - Proposals are then ranked in a pairwise tournament.

## 7. The kit

Files in the kit:
- `.claude/agents/runner.md`, `judge.md`, `inventor.md`, `scout.md`, `plumber.md`: the roles with their models.
- `research/program/PROTOCOL_TARGETS_PROPOSAL.md`: targets and track types.
- `.claude/commands/night-shift.md`: the lead's procedure, on top of `research-lead.md`.
- `research/program/NIGHT_BRIEF.md`: tonight's five tracks plus an idea slot.
  1. 9x9 `bb-mrv` pilot at 1e8 nodes.
  2. A20: blocker eviction inside `bb-mrv`.
  3. A21: value order by propagation impact.
  4. A17 rebaselined on `bb-mrv`.
  5. A baseline tournament.
  - Idea slot: ten proposals plus one AlphaEvolve-style proposal.
- `scripts/night-shift.sh`: the tmux, caffeinate, Fable, auto mode and Remote Control launcher.
- `.claude/settings.night.json`: allow and deny rules, loaded only by the launcher.

Not verified on your machine: your plan tier, your core count, and whether auto mode's classifier accepts every command in the allowlist. The rehearsal checks the last one.
