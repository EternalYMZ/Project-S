# Project S

**Miaozhen Yu (俞缪臻)** · [Project board](https://github.com/users/EternalYMZ/projects/1) · [Proof guide](brief.md) · [Lean source index](MODULES.md)

Project S studies Shellsort and ordered-partition sorting through information, intermediate array states, and the cost of legal movements and comparisons. This index links independently buildable Lean developments to the mathematical questions they address. The paper manuscripts are held for author review.

## Formal releases

The revision links point to fixed snapshots. Each repository README identifies its model and the scope of its formalization.

| Paper | Repository | Formal scope | Revision |
|---|---|---|---|
| P00 | [Chain-displacement-in-Shellsort](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort) | Single-pass chain displacement and execution identities. | [2a25235](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort/tree/2a252351408af84dde3c9047c0f6606ef85c2100) |
| P01 | [Optimal-mean-costs-of-three-pass-Shellsort](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort) | Uniform three-pass lower bounds, the natural upper-bound family and the separate optimal means. | [56c82f3](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/tree/56c82f3b81947a58dc246d12f5721ad5588da1fa) |
| P02 | [Three-pass-Shellsort-inversion-error](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error) | Deterministic final-pass arc identities for coprime gaps. Formal coverage is partial. | [5ef33d2](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/tree/5ef33d2bf60eb8c0d1df7207f205c15cbad5621b) |
| P03 | [Four-pass-Shellsort-bounds](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds) | Packing complete residue runs in long and short chains, cyclic support and conditional variance. The global four-pass lower bound remains a candidate. | [f81e757](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds/tree/f81e75726533406bc78fac5026b42d407b116ff5) |
| P04 | [Optimal-mean-comparisons-in-Shellsort](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort) | Optimal uniform mean key comparisons for predetermined tables, with rank and physical-key statements. | [1475f91](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/tree/1475f916ce27924070b171444d7a1e42408828c1) |
| P05 | [Four-pass-Shellsort-reachability](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability) | Slot potentials, checkpoint capacity and a finite example. The four-pass separation awaits formalization. | [118bfa5](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability/tree/118bfa51b3c2bcfd1179cec1d004c2e114406a12) |
| P06 | [Two-stage-partition-sorting](https://github.com/EternalYMZ/Two-stage-partition-sorting) | Threshold contraction and marked-key identities for gap words. Arbitrary first partitions remain outside these roots. | [019df87](https://github.com/EternalYMZ/Two-stage-partition-sorting/tree/019df87ddaa8040160fc6308eb00e46505f918d6) |
| P07 | [Rank-informed-pair-sorting](https://github.com/EternalYMZ/Rank-informed-pair-sorting) | Exact exchanges and insertion shifts for prescribed slot pairs, local order counts and optimal span sums. | [b68ad8a](https://github.com/EternalYMZ/Rank-informed-pair-sorting/tree/b68ad8a3fc6eacef5dd1af69ad1886caca3626d0) |
| P08 | [Shellsort-checkpoint-writes](https://github.com/EternalYMZ/Shellsort-checkpoint-writes) | The exact minimum number of whole-record main-array writes for prescribed checkpoints when target records are available. | [07e56b9](https://github.com/EternalYMZ/Shellsort-checkpoint-writes/tree/07e56b92530e5911767ab02bcd2939a9cb158611) |
| P09 | [Shellsort-gap-table-generation](https://github.com/EternalYMZ/Shellsort-gap-table-generation) | Partial formalization of gap selection, including moments, pruning, occupancy and warm-start budgets. | [9698284](https://github.com/EternalYMZ/Shellsort-gap-table-generation/tree/96982841b026f7348ea8dadcc52207f27b1fec17) |
| P10 | [Two-large-gaps-in-Shellsort](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort) | Executed comparison formula for two large gaps. With the second gap fixed to ceil(n/2), a unique first gap minimizes the mean for n ≥ 237. | [690ec26](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort/tree/690ec26844a525e79b5f6842aed6107a105b2b5d) |
| P11 | [Rank-windows-in-Shellsort](https://github.com/EternalYMZ/Rank-windows-in-Shellsort) | Prefix structure, next-pass comparisons under read/influence bounds, and finite rank-window lemmas. The full analytical rank-window claim is outside these statements. | [74ee986](https://github.com/EternalYMZ/Rank-windows-in-Shellsort/tree/74ee98617f5f5e1394521befc96c8b3e11bb6340) |

[Repository setup record](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/issues/1).

## Playground and current results

[Engineering playground](playground/) · [Eight-pass billion-key measurements](playground/eight-pass/) · [Pair-layer cache layout](playground/layout/) · [Finite comparison certificates](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/tree/main/certificates)

The paper repositories now include their English proof guides. New Lean sources cover exact fiber packing (P03), ordered-block contraction and cancellation (P06), disjoint pair-layer costs (P07), and the unique winner over the full two-large-gap class for n ≥ 237 (P10). For a fixed billion-key size, the finite comparison certificates exclude every schedule with seven or fewer passes from the under-50-billion target. The playground gives per-pass counts for two completed runs of a predetermined eight-pass schedule.

## Acknowledgments

The author thanks OpenAI's ChatGPT and Codex for assistance in exploring derivations, organizing proofs, checking references, and writing programs for the finite computations.

[Fixed-size Shellsort and Rust ipnsort experiments](playground/rust-sorting/) include standalone kernels, paired measurements and exact comparison checks.
