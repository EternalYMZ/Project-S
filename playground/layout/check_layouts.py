"""Reproduce the cache model and report the paired medians in the native CSV."""
from collections import OrderedDict, defaultdict
import csv
from itertools import permutations
from math import factorial
from pathlib import Path
from statistics import median

HERE = Path(__file__).resolve().parent


def scrambled(size):
    order = list(range(size))
    state = 5819
    for s in range(size, 1, -1):
        state ^= (state << 13) & 0xffffffff
        state ^= state >> 17
        state ^= (state << 5) & 0xffffffff
        j = state % s
        order[s-1], order[j] = order[j], order[s-1]
    return order


def schedules(k, b):
    lines = k // b
    line_order = scrambled(lines)
    return {
        "forward": range(k),
        "reverse": list(reversed(range(k))),
        "shuffled": scrambled(k),
        "striped": [b*(i % lines)+i//lines for i in range(k)],
        "line_shuffled": [b*line_order[i//b]+i%b for i in range(k)]
    }


def tree_order(k, leaf):
    state = 7523
    order = []

    def visit(start, size):
        nonlocal state
        if size == leaf:
            order.extend(range(start, start+size))
            return
        state ^= (state << 13) & 0xffffffff
        state ^= state >> 17
        state ^= (state << 5) & 0xffffffff
        half = size//2
        a, b = (half, 0) if state & 1 else (0, half)
        visit(start+a, half)
        visit(start+b, half)

    visit(0, k)
    return order


def is_tree_order(order, start):
    if len(order) <= 1:
        return True
    h = len(order)//2
    a, b = (start, start+h) if order[0] < start+h else (start+h, start)
    if not all(a <= j < a+h for j in order[:h]):
        return False
    return is_tree_order(order[:h], a) and is_tree_order(order[h:], b)


def fills(order, b, capacity, with_table=False):
    assert capacity >= 2 + int(with_table)
    lines = len(order) // b
    lru = OrderedDict()
    misses = 0

    def touch(address):
        nonlocal misses
        if address in lru:
            lru.move_to_end(address)
        else:
            misses += 1
            if len(lru) == capacity:
                lru.popitem(last=False)
            lru[address] = None

    for i, j in enumerate(order):
        if with_table:
            touch(2*lines + i//16)  # a separate aligned uint32 table
        touch(i//b)
        touch(lines + j//b)
        # Left/read, right/read, left/write, right/write leaves the same recency.
    return misses


def exhaustive():
    for k, b in [(4, 2), (6, 2), (6, 3), (8, 4)]:
        lines = k//b
        best_count = 0
        for order in permutations(range(k)):
            right = [j//b for j in order]
            runs = 1 + sum(a != c for a, c in zip(right, right[1:]))
            cost = fills(order, b, 2)
            assert cost == lines + runs
            best_count += cost == 2*lines
        assert best_count == factorial(lines)*factorial(b)**lines
    for k in (2, 4, 8):
        tree_count = 0
        levels = [1 << j for j in range(k.bit_length())]
        for order in permutations(range(k)):
            all_minimal = all(fills(order, b, 2) == 2*k//b for b in levels)
            assert is_tree_order(order, 0) == all_minimal
            tree_count += all_minimal
        assert tree_count == 2**(k-1)


def cache_table():
    k, b = 32768, 8
    lines = k//b
    with (HERE/"cache.csv").open("w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["n", "k", "keys_per_line", "capacity_lines", "right_layout",
                         "data_fills", "fills_including_index"])
        for cap in (3, 64, 512, lines, 16384):
            for name, order in schedules(k, b).items():
                data = fills(order, b, cap)
                total = fills(order, b, cap, with_table=True)
                if name in ("forward", "reverse", "line_shuffled"):
                    assert (data, total) == (2*lines, 2*lines+k//16)
                if name == "striped" and cap <= lines:
                    assert (data, total) == (k+lines, k+lines+k//16)
                writer.writerow([2*k, k, b, cap, name, data, total])


def hierarchy_cache_table():
    k = 32768
    order = schedules(k, 8)
    order["dyadic"] = tree_order(k, 8)
    with (HERE/"hierarchy_cache.csv").open("w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["n", "keys_per_block", "capacity_blocks", "layout", "data_fills", "compulsory"])
        for name in ("forward", "line_shuffled", "dyadic"):
            for b in (1, 8, 64, 512, 4096):
                for cap in (2, 16):
                    q = fills(order[name], b, cap)
                    if name != "line_shuffled" or b <= 8:
                        assert q == 2*k//b
                    writer.writerow([2*k, b, cap, name, q, 2*k//b])


def native_summary(source, result, repeats, num_groups):
    groups = defaultdict(list)
    with (HERE/source).open() as file:
        for row in csv.DictReader(file):
            groups[row["scope"], int(row["n"]), row["layout"]].append(
                (int(row["trial"]), int(row["elapsed_ns"]), int(row["rows"])))
    assert len(groups) == num_groups
    with (HERE/result).open("w", newline="") as file:
        writer = csv.writer(file)
        writer.writerow(["scope", "n", "layout", "median_ns_per_array",
                         "paired_time_over_forward", "paired_min", "paired_max", "slower_count"])
        for (scope, n, layout), samples in groups.items():
            ref = sorted(groups[scope, n, "forward"])
            data = sorted(samples)
            assert len(data) == repeats and [r[0] for r in data] == list(range(repeats))
            ratios = [value/base for (_, value, _), (_, base, _) in zip(data, ref)]
            writer.writerow([scope, n, layout, round(median(v/b for _, v, b in data), 3),
                             round(median(ratios), 4), round(min(ratios), 4), round(max(ratios), 4),
                             sum(q > 1 for q in ratios)])


if __name__ == "__main__":
    exhaustive()
    cache_table()
    hierarchy_cache_table()
    native_summary("timings.csv", "summary.csv", 10, 25)
    native_summary("hierarchy.csv", "hierarchy_summary.csv", 12, 3)
