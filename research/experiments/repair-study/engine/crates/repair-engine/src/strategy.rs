//! The four independently-toggleable strategy pieces a repair variant composes:
//! where to destroy, how to repair, when to accept, and when to restart — plus
//! the starting board the loop begins from. Each is a small enum with cheap
//! implementations, so a variant is a record naming one choice per axis and a
//! neighbour differs by exactly one.

pub mod accept;
pub mod destroy;
pub mod repair;
pub mod restart;
pub mod start;
