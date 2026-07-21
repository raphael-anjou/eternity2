---
description: Multi-lens adversarial review of a research article or experiment — fan out specialized skeptic subagents (design, scope, prose, code) in parallel, then consolidate and verify their findings against the data.
argument-hint: "[path or topic to review, e.g. a study dir or 'the hint study']"
---

# Adversarial review

Run a rigorous, multi-perspective adversarial review of the work identified by
`$ARGUMENTS` (a research article, experiment, study directory, or the current
changes if none given). The goal is to find **weak, overstated, inconsistent, or
unsupported claims, and real bugs** — before publication, not after.

This works best as **parallel specialized reviewers**, each with ONE sharp lens so
they don't collude on the same easy findings. Adapt the lenses to the work, but the
default four for a research/experiment deliverable are:

1. **Experiment-design validity** — is what's measured a valid test of what's
   claimed? Hunt for confounds, metric validity, whether effects are within noise,
   baseline/control gaps, correlation-from-few-points, budget/threshold artifacts.
   Cross-check every quantitative prose claim against the raw data (results.jsonl /
   the committed record), not the summary.
2. **Scope & completeness** — does the work test enough to support its claims, and
   does it honestly acknowledge what it did NOT test? Hunt for over-reach (one
   solver → "solvers"; one size → "in general"; one layout's law universalized),
   silently-dropped conditions, and disclaimers the body then steps past.
3. **Prose & pedagogy** — is the writing precise, honest in tone, well-structured,
   genuinely illuminating? Hunt for hype/intensifiers doing a number's job, vague
   quantifiers where an exact figure exists, redundancy across pages, causal
   overreach, terminology drift, and headings that oversell. Match the house
   register (compare to a sibling article).
4. **Code & reproducibility** — will the results actually reproduce and are the
   computations correct? The highest-risk area is **cross-language / cross-file
   reimplementations of the same logic** (e.g. a geometry computed in Rust, Python,
   AND TypeScript) — check they agree exactly. Also: silent failure/drop paths,
   determinism, feature-flag traps, and analysis code matching the engine.

## How to run it

1. **Snapshot the ground truth first** (data state, n, what's partial) so reviewers
   judge against reality and you can tell "still-running" from "broken."
2. **Launch the reviewers in parallel**, each as its own subagent, in a single
   message so they run concurrently. Give each: the exact files to read in full, the
   raw data path, the specific claims to interrogate, and a demand for
   `file:line` + severity (blocker / should-fix / nitpick). Tell them NOT to fix
   anything — report only.
3. **Verify before acting.** Reviewers miscount and misread. For every serious
   finding, confirm it against the data/code yourself before treating it as real —
   and do NOT "fix" something a reviewer flagged that is actually correct.
4. **Consolidate** into tiers (reshapes-the-result / integrity / polish), noting
   reviewer misfires you're deliberately ignoring and why.
5. If findings are big enough to change the headline or require re-analysis,
   **surface the decision** rather than silently rewriting the story.

The value is in the fan-out (independent lenses catch more than one pass) and in
the verify step (skepticism about the reviewers themselves). Scale the number and
kind of lenses to the work — a quick check needs two, a publication needs four-plus.
