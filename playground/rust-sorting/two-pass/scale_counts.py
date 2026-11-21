"""python scale_counts.py [rustc]: reuse the adjacent experiment's strided kernel."""
import csv
import math
import os
import shutil
import statistics
import subprocess
import sys
import tempfile
from pathlib import Path
from verify_two_pass import closed_form


def run(root, rustc):
    kernel = (root.parent / "hybrid" / "kernel.rs").read_text(encoding="utf-8")
    # Only relax the argument from &mut [u64; 1024] to &mut [u64] in the two
    # functions used by the driver. All kernel bodies and counters stay identical.
    for marker in ("fn pass<const", "pub fn one_pass<const"):
        start = kernel.index(marker)
        end = kernel.index(") {", start)
        header = kernel[start:end]
        assert header.count("&mut Data") == 1
        kernel = kernel[:start] + header.replace("&mut Data", "&mut [u64]") + kernel[end:]
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        (tmp / "kernel.rs").write_text(kernel, encoding="utf-8")
        shutil.copy(root / "scale_counts.rs", tmp)
        exe = tmp / ("counts.exe" if os.name == "nt" else "counts")
        subprocess.run([rustc, "--edition=2024", "-O", "scale_counts.rs", "-o", str(exe)],
                       cwd=tmp, check=True)
        with (root / "scale-raw.csv").open("wb") as stream:
            subprocess.run([str(exe)], stdout=stream, check=True)


def summarize(root):
    with (root / "scale-raw.csv").open(newline="") as stream:
        rows = list(csv.DictReader(stream))
    with (root / "scale-summary.csv").open("w", newline="") as stream:
        out = csv.writer(stream)
        out.writerow(["n", "pattern", "samples", "insertion_total_comparisons",
                      "intro_total_comparisons", "insertion_total_slot_writes",
                      "intro_total_slot_writes", "final_comparisons", "final_slot_writes",
                      "final_shifts", "final_shifts_se", "uniform_exact_mean", "error_over_se"])
        for n in (32, 64, 128, 256, 512, 1024, 2048, 4096):
            for pattern in ("random", "separated_parities"):
                group = [r for r in rows if int(r["n"]) == n and r["pattern"] == pattern]
                assert len(group) == (128 if pattern == "random" else 1)
                def mean(key):
                    return statistics.mean(int(r[key]) for r in group)
                shifts = [int(r["final_shifts"]) for r in group]
                avg = statistics.mean(shifts)
                se = statistics.stdev(shifts) / math.sqrt(len(shifts)) if len(shifts) > 1 else None
                exact = float(closed_form(n)) if se is not None else None
                error = (avg - exact) / se if se is not None else None
                costs = [mean("insertion_first_cmp") + mean("final_cmp"),
                         mean("intro_first_cmp") + mean("final_cmp"),
                         mean("insertion_first_writes") + mean("final_writes"),
                         mean("intro_first_writes") + mean("final_writes"),
                         mean("final_cmp"), mean("final_writes"), avg, se, exact, error]
                out.writerow([n, pattern, len(group),
                              *["" if v is None else f"{v:.6f}" for v in costs]])


if __name__ == "__main__":
    root = Path(__file__).resolve().parent
    run(root, sys.argv[1] if len(sys.argv) > 1 else "rustc")
    summarize(root)
