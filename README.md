# Project S

**Miaozhen Yu (俞缪臻)** · [Project board](https://github.com/users/EternalYMZ/projects/1) · [Proof guide](brief.md) · [Lean source index](MODULES.md)

Project S studies Shellsort and ordered-partition sorting through information, intermediate array states, and the cost of legal movements and comparisons. This index links independently buildable Lean developments to the mathematical questions they address. The paper manuscripts are held for author review.

## Formal releases

The revision links point to fixed snapshots. Each repository README identifies its model and the scope of its formalization.

| Paper | Repository | Formal scope | Revision |
|---|---|---|---|
| P00 | [Chain-displacement-in-Shellsort](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort) | Single-pass chain displacement and execution identities. | [388e0c6](https://github.com/EternalYMZ/Chain-displacement-in-Shellsort/tree/388e0c6b8373e54c9b167a07999119d6dc8beb74) |
| P01 | [Optimal-mean-costs-of-three-pass-Shellsort](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort) | Uniform three-pass lower bounds, the natural upper-bound family and the separate optimal means. | [3eae7e3](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/tree/3eae7e3be9bf080ebdcb5c86d9f5a78b8d159449) |
| P02 | [Three-pass-Shellsort-inversion-error](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error) | Deterministic final-pass arc identities for coprime gaps. Formal coverage is partial. | [9f1f585](https://github.com/EternalYMZ/Three-pass-Shellsort-inversion-error/tree/9f1f585257f2962196eb398b9ca44aab1e718d85) |
| P03 | [Four-pass-Shellsort-bounds](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds) | Packing complete residue runs in long and short chains, cyclic support and conditional variance. The global four-pass lower bound remains a candidate. | [9d707f0](https://github.com/EternalYMZ/Four-pass-Shellsort-bounds/tree/9d707f0a648b466d7847c30578d441dcc8eadabb) |
| P04 | [Optimal-mean-comparisons-in-Shellsort](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort) | Optimal uniform mean key comparisons for predetermined tables, with rank and physical-key statements. | [5e544f6](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/tree/5e544f661ba7f65fdf47bfb04d2afdb8fdd5219a) |
| P05 | [Four-pass-Shellsort-reachability](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability) | Slot potentials, checkpoint capacity and a finite example. The four-pass separation awaits formalization. | [ff68599](https://github.com/EternalYMZ/Four-pass-Shellsort-reachability/tree/ff68599d362de7b4e0b70f438213af0c9725721e) |
| P06 | [Two-stage-partition-sorting](https://github.com/EternalYMZ/Two-stage-partition-sorting) | Threshold contraction and marked-key identities for gap words. Arbitrary first partitions remain outside these roots. | [79e433d](https://github.com/EternalYMZ/Two-stage-partition-sorting/tree/79e433da3fb815bbcd6cf9660252eeffb673d521) |
| P07 | [Rank-informed-pair-sorting](https://github.com/EternalYMZ/Rank-informed-pair-sorting) | Exact exchanges and insertion shifts for prescribed slot pairs, local order counts and optimal span sums. | [556838d](https://github.com/EternalYMZ/Rank-informed-pair-sorting/tree/556838d75026da16cf1810bd3b1193bc322c7979) |
| P08 | [Shellsort-checkpoint-writes](https://github.com/EternalYMZ/Shellsort-checkpoint-writes) | The exact minimum number of whole-record main-array writes for prescribed checkpoints when target records are available. | [3161ef8](https://github.com/EternalYMZ/Shellsort-checkpoint-writes/tree/3161ef8d7f221b1e3f41bf4392911c5d49e03d97) |
| P09 | [Shellsort-gap-table-generation](https://github.com/EternalYMZ/Shellsort-gap-table-generation) | Partial formalization of gap selection, including moments, pruning, occupancy and warm-start budgets. | [77f46fb](https://github.com/EternalYMZ/Shellsort-gap-table-generation/tree/77f46fb71b224bce1368a238f23c9f28ed2983ae) |
| P10 | [Two-large-gaps-in-Shellsort](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort) | Executed comparison formula for two large gaps. With the second gap fixed to ceil(n/2), a unique first gap minimizes the mean for n ≥ 237. | [69f5e10](https://github.com/EternalYMZ/Two-large-gaps-in-Shellsort/tree/69f5e101e88b7f27ae870593229eee2e0dab87b0) |
| P11 | [Rank-windows-in-Shellsort](https://github.com/EternalYMZ/Rank-windows-in-Shellsort) | Prefix structure, next-pass comparisons under read/influence bounds, and finite rank-window lemmas. The full analytical rank-window claim is outside these statements. | [2eaf4b9](https://github.com/EternalYMZ/Rank-windows-in-Shellsort/tree/2eaf4b96e66fff07d98f53ccabafbfac59991743) |

[Repository setup record](https://github.com/EternalYMZ/Optimal-mean-costs-of-three-pass-Shellsort/issues/1).

## Playground and current results

[Engineering playground](playground/) · [Eight-pass billion-key measurements](playground/eight-pass/) · [Pair-layer cache layout](playground/layout/) · [Finite comparison certificates](https://github.com/EternalYMZ/Optimal-mean-comparisons-in-Shellsort/tree/main/certificates)

The paper repositories now include their English proof guides. New Lean sources cover exact fiber packing (P03), ordered-block contraction and cancellation (P06), disjoint pair-layer costs (P07), and the unique winner over the full two-large-gap class for n ≥ 237 (P10). For a fixed billion-key size, the finite comparison certificates exclude every schedule with seven or fewer passes from the under-50-billion target. The playground gives per-pass counts for two completed runs of a predetermined eight-pass schedule.

## Acknowledgments

The author thanks OpenAI's ChatGPT and Codex for assistance in exploring derivations, organizing proofs, checking references, and writing programs for the finite computations.

[Fixed-size Shellsort and Rust ipnsort experiments](playground/rust-sorting/) include standalone kernels, paired measurements and exact comparison checks.
