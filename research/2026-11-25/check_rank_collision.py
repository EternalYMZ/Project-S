"""Check the rank-pair/value-bin relation on complete finite sample spaces."""
from fractions import Fraction
from itertools import combinations, permutations, product
from math import comb, factorial


def run_prefix(values, prefix):
    a = list(values)
    for gap in prefix:
        for r in range(min(gap, len(a))):
            a[r::gap] = sorted(a[r::gap])
    return a


def adjacent_targets(n, prefix, pairs):
    total = left_bins = right_bins = 0
    for ranks in permutations(range(n)):
        a = run_prefix(ranks, prefix)
        total += sum(abs(a[i] - a[j]) == 1 for i, j in pairs)
        left_bins += sum(a[i] // 2 == a[j] // 2 for i, j in pairs)
        right_bins += sum((a[i] + 1) // 2 == (a[j] + 1) // 2 for i, j in pairs)
    assert total == left_bins + right_bins
    return Fraction(total, factorial(n))


def check(n, prefix, h, grids):
    pairs = [(i, j) for i, j in combinations(range(n), 2) if (j - i) % h == 0]
    adjacent = adjacent_targets(n, prefix, pairs)
    all_pairs = comb(n, 2)
    for grid in grids:
        total = single = 0
        for values in product(range(grid), repeat=n):
            a = run_prefix(values, prefix)
            restricted = sum(a[i] == a[j] for i, j in pairs)
            total += restricted
            if sum(comb(a.count(v), 2) for v in set(a)) == 1:
                single += restricted
        samples = grid**n
        assignments = 0 if grid < n - 1 else factorial(grid) // factorial(grid - n + 1)
        single_probability = Fraction(all_pairs * assignments, samples)
        assert Fraction(single, samples) == single_probability * adjacent / (n - 1)
        observed = Fraction(total, samples)
        residual = Fraction(all_pairs, grid) - single_probability
        assert abs(observed - n * adjacent / (2 * grid)) <= residual
        assert residual <= Fraction(all_pairs * (all_pairs - 1), grid**2)
        if (n, prefix, h) == (6, (5, 4), 3):
            assert adjacent == Fraction(19, 18)
            assert observed == Fraction(3, grid) + Fraction((grid - 1) * (grid - 2), 6 * grid**3)
        if h == 1:
            assert observed == Fraction(all_pairs, grid)
    print(f"n={n}, prefix={prefix}, h={h}, E[adjacent]={adjacent}, grids={list(grids)}")


if __name__ == "__main__":
    if not __debug__:
        raise SystemExit("Run without -O so all checks are enabled.")
    check(2, (), 1, range(1, 5))
    check(4, (), 2, range(1, 6))
    check(5, (3,), 2, range(1, 5))
    check(6, (5, 4), 3, range(1, 7))
    check(6, (5, 4), 1, range(1, 5))
    check(6, (2, 5, 1), 4, range(1, 5))
