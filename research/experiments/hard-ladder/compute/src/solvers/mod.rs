//! Built-in solvers: the main engine's plain DFS (hardness-peak), the
//! single-core registry engines (wrapped through an event sink that counts the
//! common metric), and the minimal plug-in example.

pub mod engine_dfs;
pub mod example;
pub mod scb;
