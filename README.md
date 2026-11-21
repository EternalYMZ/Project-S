# Project S

**Miaozhen Yu (俞缪臻)** · [Project board](https://github.com/users/EternalYMZ/projects/1) · [Lean source index](MODULES.md)

Project S studies Shellsort and ordered-partition sorting through information, intermediate array states, and the cost of legal movements and comparisons. This index links independently buildable Lean developments to the mathematical questions they address. The paper manuscripts are held for author review.

## Formal releases

The revision links point to fixed snapshots. Each repository README identifies its model and the scope of its formalization.

| Paper | Repository | Formal scope | Revision |
|---|---|---|---|
| P00 | [Chain-displacement-in-Shellsort](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort) | Single-pass chain displacement and execution identities. | [51421b8](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort/tree/51421b8d6708f3de0b4b5b6c41e38db775b95f9c) |
| P01 | [Optimal-mean-costs-of-three-pass-Shellsort](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort) | Uniform three-pass lower bounds, the natural upper-bound family and the separate optimal means. | [f83ba5c](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/tree/f83ba5cca0512e1406e71107487a9034cdf5bda1) |
| P02 | [Three-pass-Shellsort-inversion-error](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error) | Deterministic final-pass arc identities for coprime gaps. Formal coverage is partial. | [afe8a5e](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/tree/afe8a5e00022093474444bc1d3b6aebc3481a940) |
| P03 | [Four-pass-Shellsort-bounds](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds) | Packing complete residue runs in long and short chains, cyclic support and conditional variance. The global four-pass lower bound remains a candidate. | [6bc015e](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds/tree/6bc015e6b704f5911713a58c4f476dcf71514998) |
| P04 | [Optimal-mean-comparisons-in-Shellsort](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort) | Optimal uniform mean key comparisons for predetermined tables, with rank and physical-key statements. | [fe3fbeb](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/tree/fe3fbebaa0bada514b2fe3e6c2cd61cd99a858f4) |
| P05 | [Four-pass-Shellsort-reachability](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability) | Slot potentials, checkpoint capacity and a finite example. The four-pass separation awaits formalization. | [f9c50e7](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability/tree/f9c50e72c0bd6a3496b86e5c9d02acd86a8cb805) |
| P06 | [Two-stage-partition-sorting](https://github.com/EternalYMZ/Two-stage-partition-sorting) | Threshold contraction and marked-key identities for gap words. Arbitrary first partitions remain outside these roots. | [c367251](https://github.com/EternalYMZ/Two-stage-partition-sorting/tree/c367251171e3378458427db376408b9a2bdb7e4a) |
| P07 | [Rank-informed-pair-sorting](https://github.com/EternalYMZ/Rank-informed-pair-sorting) | Exact exchanges and insertion shifts for prescribed slot pairs, local order counts and optimal span sums. | [4e033ec](https://github.com/EternalYMZ/Rank-informed-pair-sorting/tree/4e033ec5d8a09d311e3a52c44e2d919436073443) |
| P08 | [Shellsort-checkpoint-writes](https://github.com/EternalYMZ/Shellsort-checkpoint-writes) | The exact minimum number of whole-record main-array writes for prescribed checkpoints when target records are available. | [7920dab](https://github.com/EternalYMZ/Shellsort-checkpoint-writes/tree/7920dab6af0fbe89b72d2389582556253a89189f) |
| P09 | [Shellsort-gap-table-generation](https://github.com/EternalYMZ/Shellsort-gap-table-generation) | Partial formalization of gap selection, including moments, pruning, occupancy and warm-start budgets. | [73b376a](https://github.com/EternalYMZ/Shellsort-gap-table-generation/tree/73b376a6ada92ee96a2e1af83878cd27bdae2c33) |
| P10 | [Two-large-gaps-in-Shellsort](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort) | Executed comparison formula for two large gaps. With the second gap fixed to ceil(n/2), a unique first gap minimizes the mean for n ≥ 237. | [ab5b5b7](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort/tree/ab5b5b784e5487ceca89e2b17abcd7fa4d418f67) |
| P11 | [Rank-windows-in-Shellsort](https://github.com/EternalYMZ/Rank-windows-in-Shellsort) | Prefix structure, next-pass comparisons under read/influence bounds, and finite rank-window lemmas. The full analytical rank-window claim is outside these statements. | [a8795aa](https://github.com/EternalYMZ/Rank-windows-in-Shellsort/tree/a8795aae4f3a2619844bcfcd6b74896544a7a048) |

[Repository setup record](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/issues/1).
