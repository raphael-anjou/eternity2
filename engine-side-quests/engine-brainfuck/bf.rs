//! A fast, dependency-free Brainfuck interpreter — the runner for solver.bf.
//!
//! Python's bf.py is fine for the unit tests, but a full backtracking search
//! executes tens of millions of Brainfuck instructions; this runs them in a
//! fraction of a second. Same semantics as bf.py: 8-bit wrapping cells, a tape
//! that grows on '>' past the end, stdin bytes for ',' and stdout bytes for '.'.
//!
//! Build & run (no Cargo needed):
//!     rustc -O bf.rs -o bf
//!     ./bf solver.bf < data/bf_3_3_2.bin | xxd
//!
//! Pre-matched brackets make loops O(1) jumps; the hot loop is a flat match.

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::exit;
use std::time::Instant;

fn main() {
    // Parse args: [--trace[=N]] <program.bf>. --trace prints a heartbeat to
    // stderr every N executed instructions (default 50,000,000) so long runs
    // visibly make progress without polluting stdout (which carries the board).
    let mut trace_every: u64 = 0;
    let mut path: Option<String> = None;
    for arg in env::args().skip(1) {
        if arg == "--trace" {
            trace_every = 50_000_000;
        } else if let Some(n) = arg.strip_prefix("--trace=") {
            trace_every = n.parse().unwrap_or(50_000_000);
        } else {
            path = Some(arg);
        }
    }
    let path = match path {
        Some(p) => p,
        None => {
            eprintln!("usage: bf [--trace[=N]] <program.bf>  (input on stdin)");
            exit(2);
        }
    };
    let src = fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        exit(1);
    });

    // Keep only the 8 instructions; pre-match brackets.
    let prog: Vec<u8> = src.into_iter().filter(|c| b"+-<>[],.".contains(c)).collect();
    let mut jump = vec![0usize; prog.len()];
    let mut stack = Vec::new();
    for (i, &c) in prog.iter().enumerate() {
        if c == b'[' {
            stack.push(i);
        } else if c == b']' {
            let j = stack.pop().expect("unmatched ']'");
            jump[i] = j;
            jump[j] = i;
        }
    }
    assert!(stack.is_empty(), "unmatched '['");

    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).ok();
    let mut inp = 0usize;

    let mut tape: Vec<u8> = vec![0; 1 << 16];
    let mut ptr = 0usize;
    let mut pc = 0usize;

    let start = Instant::now();
    let mut steps: u64 = 0;
    let mut next_tick = trace_every;

    while pc < prog.len() {
        if trace_every != 0 {
            steps += 1;
            if steps >= next_tick {
                let secs = start.elapsed().as_secs_f64();
                eprintln!(
                    "[bf] {steps} instructions  {:.1}s  {:.1}M ips",
                    secs,
                    steps as f64 / secs / 1e6,
                );
                next_tick += trace_every;
            }
        }
        match prog[pc] {
            b'>' => {
                ptr += 1;
                if ptr == tape.len() {
                    tape.push(0);
                }
            }
            b'<' => ptr -= 1,
            b'+' => tape[ptr] = tape[ptr].wrapping_add(1),
            b'-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            b'.' => {
                // Stream each output byte immediately so progress (and the
                // --trace heartbeat) is visible live even on a long/looping run.
                let buf = [tape[ptr]];
                io::stdout().write_all(&buf).ok();
                io::stdout().flush().ok();
            }
            b',' => {
                tape[ptr] = input.get(inp).copied().unwrap_or(0);
                inp += 1;
            }
            b'[' => {
                if tape[ptr] == 0 {
                    pc = jump[pc];
                }
            }
            b']' => {
                if tape[ptr] != 0 {
                    pc = jump[pc];
                }
            }
            _ => unreachable!(),
        }
        pc += 1;
    }
    io::stdout().flush().ok();
}
