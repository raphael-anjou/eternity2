use crate::event::SolverEvent;

// EventSink is the only way events leave a solver. NullSink monomorphizes
// to no-ops so the solver writes `sink.emit(...)` unconditionally and the
// compiler erases the call when nothing is listening.
pub trait EventSink {
    fn emit(&mut self, event: SolverEvent);

    // Cooperative cancellation. Solvers check this at backtracks and
    // before entering propagator loops. Default true; sinks that want
    // to cancel override.
    #[inline]
    fn should_continue(&self) -> bool { true }
}

pub struct NullSink;

impl EventSink for NullSink {
    #[inline(always)]
    fn emit(&mut self, _event: SolverEvent) {}

    #[inline(always)]
    fn should_continue(&self) -> bool { true }
}

#[derive(Debug, Default)]
pub struct BufferSink {
    pub events: Vec<SolverEvent>,
    cancel: bool,
}

impl BufferSink {
    #[must_use]
    pub const fn new() -> Self { Self { events: Vec::new(), cancel: false } }

    pub fn cancel(&mut self) { self.cancel = true; }
}

impl EventSink for BufferSink {
    fn emit(&mut self, event: SolverEvent) { self.events.push(event); }
    fn should_continue(&self) -> bool { !self.cancel }
}
