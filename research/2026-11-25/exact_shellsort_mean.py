"""Exact mean shifts and comparisons for a preset Shellsort gap table.

Uniform permutations of distinct keys; ordinary linear insertion; key-key
comparisons only. The state algorithm is exponential in n. See chapter 01.

Usage:
    python3 exact_shellsort_mean.py --n 20 --gaps 11 7 3 1
"""
from collections import defaultdict
from fractions import Fraction as F
from itertools import product
from math import comb, prod
from decimal import Decimal, localcontext
import argparse
import json

def lengths(n, a):
    return tuple(len(range(j, n, a)) for j in range(a))

def encode(z, n, a):
    bits = [0] * n
    for j, count in enumerate(z):
        for q in range(count):
            bits[j + a * q] = 1
    return tuple(bits)

def step(state, n, a, b):
    z, w = state[:a], state[a:]
    bits = encode(z, n, a)
    new_z = [0] * b
    smaller_before = [0] * n
    for i, bit in enumerate(bits):
        r = i % b
        smaller_before[i] = new_z[r]
        new_z[r] += bit
    new_w = [0] * b
    shifts = failures = 0
    for j, weight in enumerate(w):
        if not weight:
            continue
        u = j + a * z[j]
        assert 0 <= u < n and bits[u] == 0
        small = smaller_before[u]
        shifts += weight * (u // b - small)
        failures += weight * int(small > 0)
        new_w[u % b] += weight
    assert sum(new_z) == sum(z)
    assert sum(new_w) == sum(w) == n - sum(z)
    return tuple(new_z + new_w), shifts, failures

def initial_distribution(n, a):
    lens = lengths(n, a)
    result = {}
    for z in product(*(range(length + 1) for length in lens)):
        w = tuple(length - count for length, count in zip(lens, z))
        result[z + w] = prod(comb(length, count) for length, count in zip(lens, z))
    assert sum(result.values()) == 2 ** n
    return result

def expected_cost(n, gaps, validate_histogram=True):
    assert gaps and all(1 <= h <= n for h in gaps)
    a = gaps[0]
    lens = lengths(n, a)
    first_shifts = sum((F(length * (length - 1), 4) for length in lens), F())
    first_failures = sum((F(length) - sum((F(1, j) for j in range(1, length + 1)), F())
                          for length in lens), F())
    distribution = initial_distribution(n, a)
    pass_costs = [(first_shifts, first_failures)]
    state_counts = [len(distribution)]
    beta = [F(1, n * comb(n - 1, count)) for count in range(n)]
    for b in gaps[1:]:
        next_distribution = defaultdict(int)
        shift_by_count = [0] * n
        failure_by_count = [0] * n
        for state, coefficient in distribution.items():
            N = sum(state[:a])
            new_state, shift_cost, failure_cost = step(state, n, a, b)
            next_distribution[new_state] += coefficient
            if N < n:
                shift_by_count[N] += coefficient * shift_cost
                failure_by_count[N] += coefficient * failure_cost
            else:
                assert shift_cost == failure_cost == 0
        shifts = sum((beta[N] * shift_by_count[N] for N in range(n)), F())
        failures = sum((beta[N] * failure_by_count[N] for N in range(n)), F())
        pass_costs.append((shifts, failures))
        distribution, a = dict(next_distribution), b
        if validate_histogram:
            histogram = [0] * (n + 1)
            for state, coefficient in distribution.items():
                histogram[sum(state[:a])] += coefficient
            assert histogram == [comb(n, N) for N in range(n + 1)]
        state_counts.append(len(distribution))
    total_shifts = sum((value[0] for value in pass_costs), F())
    total_failures = sum((value[1] for value in pass_costs), F())
    return total_shifts, total_shifts + total_failures, pass_costs, state_counts


def decimal_text(value):
    with localcontext() as context:
        context.prec = 22
        return str(Decimal(value.numerator) / Decimal(value.denominator))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--n", type=int, required=True)
    parser.add_argument("--gaps", type=int, nargs="+", required=True)
    args = parser.parse_args()
    if args.n < 1 or any(h < 1 or h > args.n for h in args.gaps):
        parser.error("n must be positive and every gap must be in 1..n")
    shifts, comparisons, pass_costs, state_counts = expected_cost(args.n, tuple(args.gaps))
    result = {
        "n": args.n,
        "gaps": args.gaps,
        "input_model": "uniform permutations of distinct keys",
        "cost_model": "ordinary linear insertion, key-key comparisons only",
        "arithmetic": "exact integers and Fraction; Decimal values are display approximations",
        "expected_shifts": str(shifts),
        "expected_comparisons": str(comparisons),
        "expected_shifts_decimal": decimal_text(shifts),
        "expected_comparisons_decimal": decimal_text(comparisons),
        "passes": [
            {"pass": index + 1,
             "gap": h,
             "expected_shifts": str(pair[0]),
             "expected_failed_comparisons": str(pair[1]),
             "expected_comparisons": str(pair[0] + pair[1]),
             "states_after_pass": count}
            for index, (h, pair, count) in enumerate(zip(args.gaps, pass_costs, state_counts))
        ],
    }
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
