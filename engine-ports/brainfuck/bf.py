#!/usr/bin/env python3
"""Minimal Brainfuck interpreter — the harness that runs the .bf engine.

8 instructions over a byte tape (wrapping 0..255), 30,000 cells, '>' past the
end grows the tape. Input comes from stdin bytes; output is written as bytes.
This is deliberately tiny: the interesting code is the Brainfuck, not this.

Usage:
    python3 bf.py program.bf            # input from stdin
    echo -n '...' | python3 bf.py program.bf
"""
import sys


def run_tape(src: str, stdin: bytes = b"", tape_len: int = 30000):
    """Like run(), but also returns the final tape and pointer — for testing
    primitives by inspecting memory directly. Returns (out, tape, ptr)."""
    out, tape, ptr = _exec(src, stdin, tape_len)
    return bytes(out), list(tape), ptr


def run(src: str, stdin: bytes = b"") -> bytes:
    out, _, _ = _exec(src, stdin, 30000)
    return bytes(out)


def _exec(src: str, stdin: bytes, tape_len: int):
    # Pre-match brackets so loops are O(1) jumps.
    jump = {}
    stack = []
    for i, c in enumerate(src):
        if c == "[":
            stack.append(i)
        elif c == "]":
            j = stack.pop()
            jump[i] = j
            jump[j] = i
    if stack:
        raise SyntaxError("unmatched '['")

    tape = bytearray(tape_len)
    ptr = 0
    pc = 0
    inp = 0
    out = bytearray()
    n = len(src)
    while pc < n:
        c = src[pc]
        if c == ">":
            ptr += 1
            if ptr == len(tape):
                tape.append(0)
        elif c == "<":
            ptr -= 1
        elif c == "+":
            tape[ptr] = (tape[ptr] + 1) & 0xFF
        elif c == "-":
            tape[ptr] = (tape[ptr] - 1) & 0xFF
        elif c == ".":
            out.append(tape[ptr])
        elif c == ",":
            tape[ptr] = stdin[inp] if inp < len(stdin) else 0
            inp += 1
        elif c == "[":
            if tape[ptr] == 0:
                pc = jump[pc]
        elif c == "]":
            if tape[ptr] != 0:
                pc = jump[pc]
        pc += 1
    return out, tape, ptr


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit("usage: bf.py program.bf")
    with open(sys.argv[1]) as f:
        program = f.read()
    data = sys.stdin.buffer.read()
    sys.stdout.buffer.write(run(program, data))
