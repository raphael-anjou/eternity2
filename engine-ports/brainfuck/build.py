#!/usr/bin/env python3
"""Compile the Brainfuck solver: solver_src.py -> solver.bf.

Run:  python3 build.py
"""
import solver_src

bf = solver_src.build()
with open("solver.bf", "w") as f:
    f.write(bf)
print(f"wrote solver.bf ({len(bf)} Brainfuck instructions, SIZE={solver_src.SIZE})")
