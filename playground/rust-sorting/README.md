# Rust sorting experiments

Windows x64, Intel Core Ultra 7 270K Plus, Rust 1.100.0. Timing figures below are archived measurements. Each subsection has its own input distribution and baseline.

| Experiment | Result and evidence |
|---|---|
| [Fixed schedules](schedules/) | A 280-table search on 1,024 distinct keys picked 929, 505, 209, 109, 41, 19, 3, 1 for elapsed time. Replacing the penultimate 5 with 3 reduced held-out time by 5.39% and 6.77% in two protocols, while adding 430.33 comparisons and 426.79 shifts on average. The directory includes the candidate table list, holdout records and standalone insertion kernels. |
| [Per-pass implementations](hybrid/) | On the modified Sedgewick table the first four network passes gave a paired speedup of 1.237× on random permutations. Using introsort on all middle passes of the qin table increased elapsed time by 29.5%. The audit checks complete pass outputs. The CSVs separate comparisons, slot writes and time. |
| [Short-chain network](net/) and [adaptive dispatch](net-adaptive/) | With at most 16 keys per chain the truncated network costs at most 59/14 times the insertion comparison count per chain. The dispatcher reads eight predetermined adjacent pairs. On random inputs the fixed network took 23.06 µs and the adaptive kernel 23.52 µs. With four adjacent swaps the corresponding times were 2.93 and 0.98 µs. |
| [Two-pass calibration](two-pass/) | After sorting odd and even slots separately, the classical Hunt–Knuth formula gives 5,134.828173387 expected final shifts at n=1,024. The checker implements the finite formula, an independent interleaving recurrence and small permutation enumeration. The cross-size counts compare insertion and introsort implementations of the first pass. |
| [ipnsort ordered-region patch](ipnsort/) | For 16,384 u64 keys with the middle 128 shuffled, the public sorting interface went from 73.65 to 4.30 µs. Across 18 real datasets in original order typical timings were near parity. The wide results also include two-valued inputs with slowdowns. |
| [ipnsort local repair](ipnsort/guard/) | A separate patch repairs isolated adjacent inversions and hands control back when the same large key would move right twice. On the 1,024-key adj8 workload, the public API went from 3,309.77 to 274.61 ns over 32 paired timings. |

## Reproduce selected results

Run the commands from this directory. Python scripts use the standard library. Use Rust 1.100.0 for comparison with the archived implementation.

    python two-pass/verify_two_pass.py
    python net/network_bound.py
    rustc --edition=2024 --test -O net/network1024.rs -o network-test
    rustc --edition=2024 -O hybrid/main.rs -o hybrid-bench

Run the generated network-test executable for correctness tests. The hybrid-bench executable accepts audit or time; run in a scratch directory to keep its output separate from the archived CSVs. To rebuild the cross-size driver, use scale_counts.py in a working copy: it prepares the variable-length version of the strided kernel before compiling and regenerates CSV files.

## Patch provenance

Both patches target [Rust 1.100.0](https://github.com/rust-lang/rust/tree/1.100.0). Apply each separately to a clean checkout from its root with git apply. Upstream Rust code in the patch context retains its original copyright and licensing. The broad patch used for the extended measurements has SHA-256 c7850073740f9654401f54adbeb32de7192a7a7c050cf766a6819e07389520c3. The archived CSVs give workload-specific engineering evidence. The fixed-length table search is empirical.
