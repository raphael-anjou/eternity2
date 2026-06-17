#!/usr/bin/env python3
"""bfc — a tiny tape macro-compiler that emits Brainfuck.

We author the Eternity II backtracking solver against the register/array API
here and compile it to one self-contained `solver.bf`. We keep this compiler
in the repo: it is what makes the Brainfuck *maintainable* — edit the readable
solver source, recompile, get a fresh (and verified) `.bf`.

Memory model: one flat byte tape (the Brainfuck machine). Named scalar
registers and fixed-size arrays get fixed tape offsets; the compiler tracks the
data pointer and emits the '>'/'<' moves. All values are 8-bit wrapping, which
is all Eternity II needs (colors 0..22, small-board indices, rotations 0..3).

Arrays are indexed by a *runtime* value via a LINEAR SCAN: to touch arr[i] we
walk all slots and act on the one whose position equals i. O(n), but trivially
correct and plenty fast for boards up to 16 cells — correctness over the
fragile moving-pointer idiom. Every primitive is covered by test_bfc.py.
"""

from __future__ import annotations


class BF:
    def __init__(self) -> None:
        self.out: list[str] = []
        self.pos = 0
        self.free = 0
        self.off: dict[str, int] = {}
        self.arr: dict[str, tuple[int, int]] = {}

    # ---- allocation -------------------------------------------------------
    def reg(self, *names: str) -> None:
        for n in names:
            if n in self.off:
                raise ValueError(f"dup register {n}")
            self.off[n] = self.free
            self.free += 1

    def array(self, name: str, length: int) -> None:
        # Tape-local layout so indexed access scans with RELATIVE motion and
        # never jumps back to far registers mid-scan (that is what made a naive
        # scan emit O(length * tape_width) pointer moves). Per array:
        #   [w_idx][w_acc][w_flag][w_gap] then `length` data cells.
        # The 4 work lanes sit immediately left of the data and travel with the
        # pointer during a walk; see aload/astore.
        self.reg(f"__{name}_wi", f"__{name}_wa", f"__{name}_wf", f"__{name}_wg")
        self.arr[name] = (self.free, length)
        self.free += length

    # ---- raw / pointer ----------------------------------------------------
    def _e(self, s: str) -> None:
        self.out.append(s)

    def _goto(self, off: int) -> None:
        d = off - self.pos
        self._e(">" * d if d >= 0 else "<" * (-d))
        self.pos = off

    def at(self, name: str) -> None:
        self._goto(self.off[name])

    def _atoff(self, off: int) -> None:
        self._goto(off)

    # ---- scalars ----------------------------------------------------------
    def zero(self, name: str) -> None:
        self.at(name)
        self._e("[-]")

    def set(self, name: str, v: int) -> None:
        self.zero(name)
        self._e("+" * (v & 0xFF))

    def inc(self, name: str, n: int = 1) -> None:
        self.at(name)
        self._e("+" * (n & 0xFF))

    def dec(self, name: str, n: int = 1) -> None:
        self.at(name)
        self._e("-" * (n & 0xFF))

    def _drain(self, src: int, dsts: list[int], op: str = "+") -> None:
        self._goto(src)
        self._e("[-")
        for d in dsts:
            self._goto(d)
            self._e(op)
        self._goto(src)
        self._e("]")

    def move(self, src: str, dst: str) -> None:
        self.zero(dst)
        self._drain(self.off[src], [self.off[dst]])

    def copy(self, src: str, dst: str, tmp: str) -> None:
        s, d, t = self.off[src], self.off[dst], self.off[tmp]
        self.zero(dst)
        self.zero(tmp)
        self._drain(s, [d, t])
        self._drain(t, [s])

    def addto(self, dst: str, src: str, tmp: str) -> None:
        s, d, t = self.off[src], self.off[dst], self.off[tmp]
        self.zero(tmp)
        self._drain(s, [d, t])
        self._drain(t, [s])

    def subto(self, dst: str, src: str, tmp: str) -> None:
        s, d, t = self.off[src], self.off[dst], self.off[tmp]
        self.zero(tmp)
        self._goto(s)
        self._e("[-")
        self._goto(d); self._e("-")
        self._goto(t); self._e("+")
        self._goto(s)
        self._e("]")
        self._drain(t, [s])

    # ---- logic ------------------------------------------------------------
    def logical_not(self, name: str, dst: str) -> None:
        """dst = (name == 0); CONSUMES name (name := 0)."""
        self.set(dst, 1)
        n = self.off[name]
        self._goto(n)
        self._e("[")
        self.at(dst); self._e("-")
        self._goto(n); self._e("[-]")
        self._e("]")

    def boolify(self, name: str, dst: str) -> None:
        """dst = (name != 0); CONSUMES name."""
        self.zero(dst)
        n = self.off[name]
        self._goto(n)
        self._e("[")
        self.at(dst); self._e("[-]+")
        self._goto(n); self._e("[-]")
        self._e("]")

    def eq(self, a: str, b: str, dst: str, tmp: str) -> None:
        """dst = (a == b). a,b preserved; tmp scratch."""
        self.copy(a, tmp, dst)
        self.subto(tmp, b, dst)
        self.logical_not(tmp, dst)

    def ifnz(self, cond_copy: str):
        """Run body when cond_copy != 0; CONSUMES cond_copy. Use as a `with`.
        Pass a disposable copy of the real flag."""
        return _If(self, cond_copy)

    def while_nz(self, name: str):
        """Loop while `name` != 0; body must drive it toward 0. `with`."""
        return _While(self, name)

    # ---- arrays (linear-scan, runtime index) ------------------------------
    # Helpers carry an index register `idx` and a position counter `k`,
    # comparing k to idx at each slot. They need 4 scratch regs (declared once).
    def _ensure_ascratch(self) -> None:
        if "__a_k" not in self.off:
            self.reg("__a_k", "__a_hit", "__a_t1", "__a_t2")

    def ainit_const(self, arr: str, i: int, value_name: str, tmp: str) -> None:
        """arr[i] = value_name with a COMPILE-TIME index. value preserved."""
        base, _ = self.arr[arr]
        cell = base + i
        v, t = self.off[value_name], self.off[tmp]
        self._atoff(cell); self._e("[-]")
        self.zero(tmp)
        self._goto(v); self._e("[-")
        self._atoff(cell); self._e("+")
        self._goto(t); self._e("+")
        self._goto(v); self._e("]")
        self._drain(t, [v])

    # Compile-time-constant index: direct cell access, no scan. Use these
    # wherever the index is known at build time (table construction, I/O) — it
    # is what keeps the emitted program small.
    def aget_const(self, arr: str, i: int, dst: str, tmp: str) -> None:
        """dst = arr[i] for a CONSTANT i. arr preserved."""
        base, _ = self.arr[arr]
        self._copy_cell_to_reg(base + i, dst, tmp)

    def aset_const(self, arr: str, i: int, value_name: str, tmp: str) -> None:
        """arr[i] = value_name for a CONSTANT i. value preserved."""
        base, _ = self.arr[arr]
        cell = base + i
        self._atoff(cell)
        self._e("[-]")
        self._copy_to_cell(value_name, cell, tmp)

    def ainit_const(self, arr: str, i: int, value_name: str, tmp: str) -> None:
        """Alias kept for readability at puzzle-load time."""
        self.aset_const(arr, i, value_name, tmp)

    # Runtime index: linear scan. Each array carries its own scan scratch lanes
    # (__{arr}_wi/_wa/_wf/_wg) ADJACENT to the data, so the per-slot pointer
    # moves are short (a few cells) rather than spanning the whole tape — that
    # adjacency is what makes the emitted code O(length) per access instead of
    # O(length * tape_width).
    def aload(self, arr: str, idx: str, dst: str) -> None:
        """dst = arr[idx]. idx preserved."""
        base, length = self.arr[arr]
        wi, wa, wf = f"__{arr}_wi", f"__{arr}_wa", f"__{arr}_wf"
        wg = f"__{arr}_wg"
        self.copy(idx, wi, wg)              # wi = countdown from idx
        self.zero(dst)
        for slot in range(length):
            self.copy(wi, wa, wg)
            self.logical_not(wa, wf)        # wf = (wi == 0)  (consumes wa)
            with self.ifnz_copy(wf):
                self._add_cell_to(base + slot, dst, wg)
            self.dec(wi)

    def astore(self, arr: str, idx: str, value_name: str) -> None:
        """arr[idx] = value_name. idx and value preserved."""
        base, length = self.arr[arr]
        wi, wa, wf = f"__{arr}_wi", f"__{arr}_wa", f"__{arr}_wf"
        wg = f"__{arr}_wg"
        self.copy(idx, wi, wg)
        for slot in range(length):
            self.copy(wi, wa, wg)
            self.logical_not(wa, wf)
            with self.ifnz_copy(wf):
                self._atoff(base + slot)
                self._e("[-]")
                self._copy_to_cell(value_name, base + slot, wg)
            self.dec(wi)

    def ifnz_copy(self, flag_name: str):
        """`with` that runs the body iff flag_name != 0, WITHOUT consuming it
        (copies into a fresh scratch first)."""
        return _IfPreserve(self, flag_name)

    # cell-level helpers (absolute offsets)
    def _add_cell_to(self, cell: int, dst: str, tmp: str) -> None:
        d, t = self.off[dst], self.off[tmp]
        self.zero(tmp)
        self._atoff(cell); self._e("[-")
        self._goto(d); self._e("+")
        self._goto(t); self._e("+")
        self._atoff(cell); self._e("]")
        self._drain(t, [cell])

    def _copy_to_cell(self, src: str, cell: int, tmp: str) -> None:
        s, t = self.off[src], self.off[tmp]
        self.zero(tmp)
        self._goto(s); self._e("[-")
        self._atoff(cell); self._e("+")
        self._goto(t); self._e("+")
        self._goto(s); self._e("]")
        self._drain(t, [s])

    def _copy_cell_to_reg(self, cell: int, dst: str, tmp: str) -> None:
        """dst = tape[cell] (cell preserved) via scratch tmp."""
        d, t = self.off[dst], self.off[tmp]
        self.zero(dst)
        self.zero(tmp)
        self._atoff(cell); self._e("[-")
        self._goto(d); self._e("+")
        self._goto(t); self._e("+")
        self._atoff(cell); self._e("]")
        self._drain(t, [cell])

    # ---- I/O --------------------------------------------------------------
    def read(self, name: str) -> None:
        self.at(name); self._e(",")

    def write(self, name: str) -> None:
        self.at(name); self._e(".")

    def write_digit(self, name: str, tmp: str) -> None:
        """Print name (0..9) as an ASCII digit. name preserved."""
        self.copy(name, tmp, "__a_t1" if "__a_t1" in self.off else self._tmp())
        self.inc(tmp, 48)
        self.write(tmp)

    def _tmp(self) -> str:
        if "__io_t" not in self.off:
            self.reg("__io_t")
        return "__io_t"

    def code(self) -> str:
        return "".join(self.out)


class _While:
    def __init__(self, bf: BF, name: str):
        self.bf, self.name = bf, name

    def __enter__(self):
        self.bf.at(self.name); self.bf._e("[")
        return self

    def __exit__(self, *a):
        self.bf.at(self.name); self.bf._e("]")
        return False


class _If:
    """Runs body iff cond_copy != 0; consumes cond_copy."""
    def __init__(self, bf: BF, cond_copy: str):
        self.bf, self.c = bf, cond_copy

    def __enter__(self):
        self.bf.at(self.c); self.bf._e("[")
        return self

    def __exit__(self, *a):
        self.bf.at(self.c); self.bf._e("[-]")  # force single execution
        self.bf._e("]")
        return False


class _IfPreserve:
    """Runs body iff flag != 0, preserving flag (copies to a private scratch)."""
    _n = 0

    def __init__(self, bf: BF, flag: str):
        self.bf, self.flag = bf, flag
        _IfPreserve._n += 1
        self.scratch = f"__ifp_{_IfPreserve._n}"
        if self.scratch not in bf.off:
            bf.reg(self.scratch)
        self.t = "__ifp_t"
        if self.t not in bf.off:
            bf.reg(self.t)

    def __enter__(self):
        self.bf.copy(self.flag, self.scratch, self.t)
        self.bf.at(self.scratch); self.bf._e("[")
        return self

    def __exit__(self, *a):
        self.bf.at(self.scratch); self.bf._e("[-]")
        self.bf._e("]")
        return False
