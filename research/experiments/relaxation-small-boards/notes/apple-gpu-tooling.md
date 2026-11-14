# GPU tooling on Apple chips for the low-rank solver (2026-11-13)

> **Status (2026-11-13):** not independently checked. A read-only lookup of documentation and release notes, transcribed here. Items marked unverified were not confirmed from a primary source. Benchmark figures come from search snippets, not from pages opened in full. No measurement was run on our machines.

Use case: an iterative solver (NumPy today) whose inner loop is dense products of a tall matrix V (about 160,000 rows, 50 to 2,000 columns) with small matrices, row-pair dot products gathered from index lists of up to about 10^10 pairs in chunks with scatter-add of gradients, and elementwise work. Tolerances down to 1e-5 or 1e-6. Machines: M4 (10-core GPU, 32 GB) and M1 (8-core GPU, 16 GB).

## What the documentation says

| Route | Double precision on the GPU | Indexed add with repeated indices | State |
|---|---|---|---|
| Apple MLX 0.32.4 | No. float64 arrays work on CPU only; on the GPU they raise an exception | `a.at[idx].add(y)` accumulates duplicates correctly (documented example). Plain `a[idx] += 1` is nondeterministic for duplicates and has no bounds check | Active. Lazy evaluation: about one `eval()` per outer iteration. Memory limits settable; the Metal buffer limit is to be read from `mx.device_info()` on each machine (value not retrieved) |
| PyTorch MPS (docs 2.15) | No. Creating or moving a float64 tensor to MPS raises an error; the CPU fallback switch does not help | Unverified for `index_add_`, `scatter_add_`, `gather` on MPS; repeated indices are undefined behaviour for `index_add_` in general per a forum answer | Active. High watermark ratio 1.7 by default acts as a hard memory ceiling |
| jax-metal 0.1.1 (2024-10-08) | No | Not checked | Effectively stalled (a forum post says issues were closed in December 2025; unverified) |
| CPU with Accelerate | Yes | Yes (NumPy semantics) | The M4 is the first Apple chip with Arm SME; whether Accelerate uses it is unverified |

Sparse matrix support on MLX and on MPS: unverified, nothing found in the pages opened.

## Speed, as far as anyone has measured

- Dense float32 products, measured in a 2025 preprint (snippet only): M1 GPU about 1.36 TFLOPS, about equal to the M1 CPU; M4 GPU about 2.9 TFLOPS against about 1.5 for the M4 CPU.
- An informal single run: about 2.78 TFLOPS float32 on a fanless M4 laptop with MLX 0.28.
- Hand-written SME kernels on an M4 reach about 1.8 TFLOPS float32 at 512 x 512 (a university group's library, not Accelerate).
- Gather and scatter workloads: no benchmark found.
- Double precision on the M4 CPU: no published figure found. A two-line NumPy test would measure it.

## Reading for our solver

- The Apple GPU buys at most about a factor of 2 over the same chip's CPU on dense float32 work on the M4, and nothing on the M1. The scout's guess for the whole solver is 2 to 10 times over single-thread NumPy on the M4, possibly under 1 on scatter-heavy chunks; it is a guess with no measurement behind it.
- Single precision is the binding limit: a penalty or augmented-Lagrangian solver driven to 1e-5 or 1e-6 would need the multipliers, residual accumulation and convergence checks kept in double precision on the CPU. Techniques for this (mixed precision, iterative refinement, compensated summation) were not researched here.
- If a port is made: MLX first, PyTorch MPS as the fallback, not jax-metal. Float32 products, gathered dots and `at[].add` on the GPU; float64 bookkeeping on the CPU.
- Three likely pitfalls: float32 not reaching the tolerance; slow or inaccurate scatter-add over about 10^10 pairs; memory (V plus chunk temporaries on the 16 GB machine, and an unknown Metal buffer limit).

## Sources opened

- https://ml-explore.github.io/mlx/build/html/install.html
- https://ml-explore.github.io/mlx/build/html/python/data_types.html
- https://ml-explore.github.io/mlx/build/html/usage/indexing.html
- https://ml-explore.github.io/mlx/build/html/python/_autosummary/mlx.core.array.at.html
- https://ml-explore.github.io/mlx/build/html/usage/lazy_evaluation.html
- https://ml-explore.github.io/mlx/build/html/python/metal.html
- https://pypi.org/pypi/mlx/json
- https://docs.pytorch.org/docs/2.15/notes/mps.html
- https://docs.pytorch.org/docs/2.15/mps_environment_variables.html
- https://pypi.org/pypi/jax-metal/json
- https://developer.apple.com/forums/thread/815139
- https://arxiv.org/pdf/2502.05317v2 (snippet only)
- https://dev.to/tinyalg/278-tflops-on-a-fanless-macbook-air-benchmarking-apples-m4-with-mlx-38n4 (snippet only)
- https://scalable.uni-jena.de/research/2025/03/31/sme-kernels.html (snippet only)
- https://nanoreview.net/en/gpu-compare/apple-m4-gpu-10-core-vs-apple-m1-gpu-8-core (theoretical figures)
- https://discuss.pytorch.org/t/does-index-add-work-with-repeated-indexes/107496
