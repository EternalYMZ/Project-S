# Project S

**Miaozhen Yu (俞缪臻)** · [Project board](https://github.com/users/EternalYMZ/projects/1) · [Proof guide](brief.md) · [Lean source index](MODULES.md)

Project S studies Shellsort and ordered-partition sorting through information, intermediate array states, and the cost of legal movements and comparisons. This index links independently buildable Lean developments to the mathematical questions they address. The paper manuscripts are held for author review.

The [November 25 research update](research/2026-11-25/README.md) traces the new three-pass constants, four-pass lower bounds, random-table guarantees and billion-key certificates to their proofs, Lean statements and replay programs.

## Formal releases

The revision links point to fixed snapshots. Each repository README identifies its model and the scope of its formalization.

| Paper | Repository | Formal scope | Revision |
|---|---|---|---|
| P00 | [Chain-displacement-in-Shellsort](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort) | Single-pass chain displacement and execution identities. | [388e0c6](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort/tree/388e0c6b8373e54c9b167a07999119d6dc8beb74) |
| P01 | [Optimal-mean-costs-of-three-pass-Shellsort](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort) | Sharp three-pass coefficient for shifts and comparisons, fixed-divisor limits, all optimal-order families, and the compact set of optimal coprime shapes. | [F09](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/tree/59213850d62ca725acdbf2a66a875d03c715ac56) |
| P02 | [Three-pass-Shellsort-inversion-error](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error) | Deterministic final-pass arc identities for coprime gaps. Formal coverage is partial. | [9f1f585](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/tree/9f1f585257f2962196eb398b9ca44aab1e718d85) |
| P03 | [Four-pass-Shellsort-bounds](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds) | Exact fiber packing, cyclic support and conditional variance. The 25/18 all-table bound remains a candidate. The November 25 research note records a separate analytical 29/21 bound with logarithmic loss. | [8b2d624](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds/tree/8b2d624140dfdc5214860088b8d43dcd2d69114a) |
| P04 | [Optimal-mean-comparisons-in-Shellsort](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort) | Optimal uniform mean comparisons for predetermined tables. New results cover menu density, legality, endpoint costs, a finite range of spectral contraction and selection for prescribed input laws under a supplied bound. | [d439356](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/tree/d439356fd87e130d86b879a68360d14acdbcbc30) |
| P05 | [Four-pass-Shellsort-reachability](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability) | Slot potentials, checkpoint capacity and a finite example. The four-pass separation awaits formalization. | [ff68599](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability/tree/ff68599d362de7b4e0b70f438213af0c9725721e) |
| P06 | [Two-stage-partition-sorting](https://github.com/EternalYMZ/Two-stage-partition-sorting) | Threshold contraction for ordered blocks, exact trajectory discrepancy and target footrule identities. The general two-stage cost theorem awaits a formal proof. | [85f82f2](https://github.com/EternalYMZ/Two-stage-partition-sorting/tree/85f82f2a9be844b63d0981489392b1816bd756b9) |
| P07 | [Rank-informed-pair-sorting](https://github.com/EternalYMZ/Rank-informed-pair-sorting) | Exact exchange identities and the uniform mean movement formula for a prescribed disjoint pair layer, with optimal spans at fixed pair count. | [656437d](https://github.com/EternalYMZ/Rank-informed-pair-sorting/tree/656437d450ad1b5cb90a3bee22b8e438999e5b2c) |
| P08 | [Shellsort-checkpoint-writes](https://github.com/EternalYMZ/Shellsort-checkpoint-writes) | The exact minimum number of whole-record main-array writes for prescribed checkpoints when target records are available. | [3161ef8](https://github.com/EternalYMZ/Shellsort-checkpoint-writes/tree/3161ef8d7f221b1e3f41bf4392911c5d49e03d97) |
| P09 | [Shellsort-gap-table-generation](https://github.com/EternalYMZ/Shellsort-gap-table-generation) | Gap selection, pruning and conditional moment bounds. Seven- and eight-pass billion-key comparison bounds have analytical proofs with replayable rational certificates. | [6161ae6](https://github.com/EternalYMZ/Shellsort-gap-table-generation/tree/6161ae68c6354f86aa0f51a7ed11befa1b1d4344) |
| P10 | [Two-large-gaps-in-Shellsort](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort) | Unique minimum mean key comparisons over the full class n > h > g ≥ ceil(n/2) for n ≥ 237, with a counterexample to the formula at n = 236. | [4447d6c](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort/tree/4447d6c738cee73163140cac34f0734b872ad613) |
| P11 | [Rank-windows-in-Shellsort](https://github.com/EternalYMZ/Rank-windows-in-Shellsort) | Prefix structure, next-pass comparisons under read/influence bounds, and finite rank-window lemmas. The full analytical rank-window claim is outside these statements. | [2eaf4b9](https://github.com/EternalYMZ/Rank-windows-in-Shellsort/tree/2eaf4b96e66fff07d98f53ccabafbfac59991743) |

[Repository setup record](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/issues/1).

## Playground and current results

[Engineering playground](playground/) · [Eight-pass billion-key measurements](playground/eight-pass/) · [Pair-layer cache layout](playground/layout/) · [Finite comparison certificates](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/tree/main/certificates)

The paper repositories now include their English proof guides. New Lean sources cover exact fiber packing (P03), ordered-block contraction and cancellation (P06), disjoint pair-layer costs (P07), and the unique winner over the full two-large-gap class for n ≥ 237 (P10). The [seven-pass entropy proof](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/blob/7041c672477840b81d4b407fde22f86d694d53fe/certificates/seven-pass-proof.md) and its rational certificate exclude every schedule of seven or fewer passes from the under-50-billion target at n = 1,000,000,000. The playground gives per-pass counts for two completed runs of a predetermined eight-pass schedule.

## Acknowledgments

The author thanks OpenAI's ChatGPT and Codex for assistance in exploring derivations, organizing proofs, checking references, and writing programs for the finite computations.

[Fixed-size Shellsort and Rust ipnsort experiments](playground/rust-sorting/) include standalone kernels, paired measurements and exact comparison checks.
