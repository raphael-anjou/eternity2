use std::collections::HashMap;

use crate::event::{EventCategory, SolverEvent};
use crate::sink::EventSink;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThrottleRule {
    pub first_n_full: u64,
    pub then_one_in_k: u64,        // 0 ⇒ suppress all after first_n_full
    pub min_interval_ms: u64,      // 0 ⇒ no time floor
    pub max_per_window: u64,       // 0 ⇒ no cap
}

impl ThrottleRule {
    pub const FULL: Self = Self {
        first_n_full: u64::MAX,
        then_one_in_k: 1,
        min_interval_ms: 0,
        max_per_window: 0,
    };

    pub const SUPPRESS: Self = Self {
        first_n_full: 0,
        then_one_in_k: 0,
        min_interval_ms: 0,
        max_per_window: 0,
    };

    // Default profiles as specified in V2_DESIGN.md §"Default rates by sink type".
    #[must_use]
    pub const fn wire_default(category: EventCategory) -> Self {
        match category {
            EventCategory::VariableSelected
            | EventCategory::ValueTried
            | EventCategory::ConstraintPropagated
            | EventCategory::DomainWipeout => Self {
                first_n_full: 1000,
                then_one_in_k: 200,
                min_interval_ms: 0,
                max_per_window: 0,
            },
            EventCategory::BacktrackShallow => Self {
                first_n_full: 0,
                then_one_in_k: 1,
                min_interval_ms: 50,
                max_per_window: 0,
            },
            EventCategory::PartialSolution => Self {
                first_n_full: 0,
                then_one_in_k: 1,
                min_interval_ms: 100,
                max_per_window: 50,
            },
            EventCategory::Stats => Self {
                first_n_full: 0,
                then_one_in_k: 1,
                min_interval_ms: 250,
                max_per_window: 0,
            },
        }
    }

    #[must_use]
    pub const fn benchmark_default(category: EventCategory) -> Self {
        match category {
            EventCategory::Stats => Self {
                first_n_full: 0,
                then_one_in_k: 1,
                min_interval_ms: 500,
                max_per_window: 0,
            },
            _ => Self::SUPPRESS,
        }
    }

    #[must_use]
    pub const fn json_dump_default(category: EventCategory) -> Self {
        match category {
            EventCategory::PartialSolution => Self {
                first_n_full: 0,
                then_one_in_k: 1,
                min_interval_ms: 500,
                max_per_window: 0,
            },
            EventCategory::Stats => Self {
                first_n_full: 0,
                then_one_in_k: 1,
                min_interval_ms: 1000,
                max_per_window: 0,
            },
            _ => Self::FULL,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ThrottleConfig {
    rules: HashMap<EventCategory, ThrottleRule>,
}

impl ThrottleConfig {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    #[must_use]
    pub fn wire() -> Self { Self::from_default(ThrottleRule::wire_default) }
    #[must_use]
    pub fn benchmark() -> Self { Self::from_default(ThrottleRule::benchmark_default) }
    #[must_use]
    pub fn json_dump() -> Self { Self::from_default(ThrottleRule::json_dump_default) }

    fn from_default(f: fn(EventCategory) -> ThrottleRule) -> Self {
        let categories = [
            EventCategory::VariableSelected,
            EventCategory::ValueTried,
            EventCategory::ConstraintPropagated,
            EventCategory::DomainWipeout,
            EventCategory::BacktrackShallow,
            EventCategory::PartialSolution,
            EventCategory::Stats,
        ];
        let mut rules = HashMap::with_capacity(categories.len());
        for c in categories {
            rules.insert(c, f(c));
        }
        Self { rules }
    }

    pub fn set(&mut self, category: EventCategory, rule: ThrottleRule) {
        self.rules.insert(category, rule);
    }

    #[must_use]
    pub fn rule(&self, category: EventCategory) -> ThrottleRule {
        self.rules.get(&category).copied().unwrap_or(ThrottleRule::FULL)
    }
}

#[derive(Debug, Default)]
struct CategoryState {
    seen: u64,
    last_emit_us: u64,
    window_emits: u64,
    window_start_us: u64,
    suppressed: u64,
}

pub struct ThrottledSink<S> {
    inner: S,
    config: ThrottleConfig,
    state: HashMap<EventCategory, CategoryState>,
}

impl<S: EventSink> ThrottledSink<S> {
    pub fn new(inner: S, config: ThrottleConfig) -> Self {
        Self { inner, config, state: HashMap::new() }
    }

    pub fn into_inner(self) -> S { self.inner }

    pub fn inner(&self) -> &S { &self.inner }

    pub fn inner_mut(&mut self) -> &mut S { &mut self.inner }

    // True ⇒ event should be passed to inner sink.
    fn admit(&mut self, category: EventCategory, timestamp_us: u64) -> bool {
        let rule = self.config.rule(category);
        let state = self.state.entry(category).or_default();
        state.seen = state.seen.saturating_add(1);

        if state.seen <= rule.first_n_full {
            state.last_emit_us = timestamp_us;
            return true;
        }
        if rule.then_one_in_k == 0 {
            state.suppressed = state.suppressed.saturating_add(1);
            return false;
        }
        if (state.seen - rule.first_n_full) % rule.then_one_in_k != 0 {
            state.suppressed = state.suppressed.saturating_add(1);
            return false;
        }
        if rule.min_interval_ms > 0 {
            let interval_us = rule.min_interval_ms * 1000;
            if state.last_emit_us != 0 && timestamp_us.saturating_sub(state.last_emit_us) < interval_us {
                state.suppressed = state.suppressed.saturating_add(1);
                return false;
            }
            // Window window enforcement.
            if state.window_start_us == 0
                || timestamp_us.saturating_sub(state.window_start_us) >= interval_us
            {
                state.window_start_us = timestamp_us;
                state.window_emits = 0;
            }
            if rule.max_per_window > 0 && state.window_emits >= rule.max_per_window {
                state.suppressed = state.suppressed.saturating_add(1);
                return false;
            }
            state.window_emits = state.window_emits.saturating_add(1);
        }
        state.last_emit_us = timestamp_us;
        true
    }
}

impl<S: EventSink> EventSink for ThrottledSink<S> {
    fn emit(&mut self, event: SolverEvent) {
        if event.is_terminal() {
            self.inner.emit(event);
            return;
        }
        let Some(cat) = event.category() else {
            // Started and other non-categorized non-terminal events always pass.
            self.inner.emit(event);
            return;
        };
        if self.admit(cat, event.timestamp_us) {
            self.inner.emit(event);
        }
    }

    fn should_continue(&self) -> bool { self.inner.should_continue() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{EventBody, FinalStats, SelectionReason};
    use crate::sink::BufferSink;

    fn ev(cat_id: u32, t_us: u64) -> SolverEvent {
        SolverEvent {
            schema_version: 1,
            solver_run_id: 1,
            node_id: u64::from(cat_id),
            depth: 1,
            timestamp_us: t_us,
            body: EventBody::VariableSelected {
                position: 0,
                domain_size: 1,
                score: 0.0,
                reason: SelectionReason::Unspecified,
            },
        }
    }

    #[test]
    fn first_n_full_then_sampled() {
        let mut cfg = ThrottleConfig::new();
        cfg.set(EventCategory::VariableSelected, ThrottleRule {
            first_n_full: 3, then_one_in_k: 2, min_interval_ms: 0, max_per_window: 0,
        });
        let mut sink = ThrottledSink::new(BufferSink::new(), cfg);
        for i in 0..10 {
            sink.emit(ev(i, u64::from(i) * 1000));
        }
        let buf = sink.into_inner();
        // First 3 pass, then 1/2 of the remaining 7 starting from offset 1.
        // Indices accepted: 0,1,2 (first_n_full), then 4,6,8 (every 2nd after).
        let accepted: Vec<u64> = buf.events.iter().map(|e| e.node_id).collect();
        assert_eq!(accepted, vec![0, 1, 2, 4, 6, 8]);
    }

    #[test]
    fn terminals_always_pass() {
        let cfg = ThrottleConfig::benchmark();
        let mut sink = ThrottledSink::new(BufferSink::new(), cfg);
        sink.emit(SolverEvent {
            schema_version: 1, solver_run_id: 1, node_id: 0, depth: 0, timestamp_us: 0,
            body: EventBody::Solved {
                board: Board::empty(&minimal_puzzle()),
                final_stats: FinalStats::default(),
            },
        });
        assert_eq!(sink.into_inner().events.len(), 1);
    }

    fn minimal_puzzle() -> eternity2_core::Puzzle {
        use eternity2_core::{Edges, Piece};
        eternity2_core::Puzzle::new(
            1, 1, 1,
            vec![Piece::new(0, Edges::new(0, 0, 0, 0))],
        ).unwrap()
    }

    use eternity2_core::Board;
}
