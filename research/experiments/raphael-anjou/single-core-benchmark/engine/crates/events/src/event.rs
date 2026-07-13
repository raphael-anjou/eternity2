use eternity2_core::{Board, CellAssignment, Position};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type SolverRunId = u64;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SolverEvent {
    pub schema_version: u32,
    pub solver_run_id: SolverRunId,
    pub node_id: u64,
    pub depth: u32,
    pub timestamp_us: u64,
    pub body: EventBody,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EventBody {
    Started {
        solver_id: String,
        heuristic_profile: String,
        puzzle_fingerprint: u64,
        seed: u64,
        started_wall_us: u64,
    },
    VariableSelected {
        position: Position,
        domain_size: u32,
        score: f64,
        reason: SelectionReason,
    },
    ValueTried {
        position: Position,
        piece_id: eternity2_core::PieceId,
        rotation: eternity2_core::Rotation,
    },
    ConstraintPropagated {
        from_pos: Position,
        to_pos: Position,
        removed_count: u32,
    },
    DomainWipeout {
        position: Position,
    },
    Backtrack {
        from_depth: u32,
        to_depth: u32,
        cause: BacktrackCause,
    },
    PartialSolution {
        snapshot_id: u64,
        positions_cleared: Vec<Position>,
        cells: Vec<CellAssignment>,
    },
    Stats(FinalStats),
    SampledOut {
        since_timestamp_us: u64,
        counts: Vec<SamplingCount>,
    },
    Solved {
        board: Board,
        final_stats: FinalStats,
    },
    Exhausted {
        final_stats: FinalStats,
        solutions_found: u64,
    },
    TimedOut {
        final_stats: FinalStats,
        best_partial: Board,
        best_depth: u32,
    },
    Cancelled {
        final_stats: FinalStats,
        best_partial: Board,
        best_depth: u32,
        solutions_so_far: Vec<Board>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SelectionReason {
    Unspecified,
    Mrv,
    Degree,
    BorderFirst,
    RareColor,
    PathStrict,
    PathPrior,
    PathPrefix,
    Random,
    DlxColumn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BacktrackCause {
    Unspecified,
    DomainWipeout,
    NoMatch,
    ParityFail,
    IslandFail,
    ExhaustedBranch,
}

// Sink-side categorization for throttling. Terminal events and deep
// backtracks are always passed through and not represented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EventCategory {
    VariableSelected,
    ValueTried,
    ConstraintPropagated,
    DomainWipeout,
    BacktrackShallow,
    PartialSolution,
    Stats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FinalStats {
    pub time_ms: u64,
    pub nodes: u64,
    pub backtracks: u64,
    pub propagations: u64,
    pub domain_wipeouts: u64,
    pub current_depth: u32,
    pub max_depth_seen: u32,
    pub solutions_found: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplingCount {
    pub category: EventCategory,
    pub count: u64,
}

impl SolverEvent {
    #[must_use]
    pub fn category(&self) -> Option<EventCategory> {
        match &self.body {
            EventBody::VariableSelected { .. } => Some(EventCategory::VariableSelected),
            EventBody::ValueTried { .. } => Some(EventCategory::ValueTried),
            EventBody::ConstraintPropagated { .. } => Some(EventCategory::ConstraintPropagated),
            EventBody::DomainWipeout { .. } => Some(EventCategory::DomainWipeout),
            EventBody::Backtrack { .. } => Some(EventCategory::BacktrackShallow),
            EventBody::PartialSolution { .. } => Some(EventCategory::PartialSolution),
            EventBody::Stats(_) => Some(EventCategory::Stats),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self.body,
            EventBody::Solved { .. }
                | EventBody::Exhausted { .. }
                | EventBody::TimedOut { .. }
                | EventBody::Cancelled { .. }
        )
    }
}
