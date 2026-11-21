"""Uniform permutations of distinct ranks. Standard library only; no timing."""
from fractions import Fraction
from itertools import permutations
from math import comb, factorial


def closed_form(n):
    m, odd = divmod(n, 2)
    a = Fraction(m * 4 ** (m - 1), comb(2 * m, m))
    return a * (Fraction(2 * (m + 1), 2 * m + 1) if odd else 1)


def merge_words(n):
    """Enumerate paths by dynamic programming; aggregate cross inversions exactly."""
    even, odd = (n + 1) // 2, n // 2
    counts = [0] * (odd + 1)
    costs = [0] * (odd + 1)
    counts[0] = 1
    for a in range(even + 1):
        for b in range(odd + 1):
            if a > 0:  # Append the a-th even-chain rank to the preceding row.
                costs[b] += counts[b] * abs(a - 1 - b)
            if b > 0:  # Append one odd-chain rank. Charge crossings on even steps.
                counts[b] += counts[b - 1]
                costs[b] += costs[b - 1]
    assert counts[-1] == comb(n, even)
    return Fraction(costs[-1], counts[-1])


def pass_cost(keys, gap):
    c = m = reached_head = 0
    for i in range(gap, len(keys)):
        x, j = keys[i], i
        while j >= gap:
            c += 1
            if keys[j - gap] <= x:
                break
            m += 1
            keys[j] = keys[j - gap]
            j -= gap
        keys[j] = x
        reached_head += j < gap
    assert c == m + len(keys) - gap - reached_head
    return m


def exhaustive(n):
    even, odd = (n + 1) // 2, n // 2
    first = last = 0
    expected = list(range(n))
    for p in permutations(expected):
        keys = list(p)
        first += pass_cost(keys, 2)
        shifts = pass_cost(keys, 1)
        assert keys == expected
        first_state = list(p)
        first_state[::2] = sorted(p[::2])
        first_state[1::2] = sorted(p[1::2])
        assert shifts == sum(x > y for i, x in enumerate(first_state)
                             for y in first_state[i + 1:])
        last += shifts
    assert Fraction(first, factorial(n)) == Fraction(comb(even, 2) + comb(odd, 2), 2)
    return Fraction(last, factorial(n))


if __name__ == "__main__":
    for n in (*range(2, 17), 32, 63, 64, 1023, 1024):
        mean = closed_form(n)
        assert merge_words(n) == mean
        check = "DP"
        if n <= 8:
            assert exhaustive(n) == mean
            check += "+all_permutations"
        print(n, check, str(mean) if n <= 16 else f"{float(mean):.9f}")
