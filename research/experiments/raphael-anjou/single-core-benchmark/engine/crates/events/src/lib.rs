#![forbid(unsafe_code)]

mod event;
mod sink;
mod throttle;

pub use event::{
    BacktrackCause, EventBody, EventCategory, FinalStats, SamplingCount, SelectionReason,
    SolverEvent, SolverRunId,
};
pub use sink::{BufferSink, EventSink, NullSink};
pub use throttle::{ThrottleConfig, ThrottleRule, ThrottledSink};
