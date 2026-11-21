"""Count logical comparators for each real chain of the <=16 Batcher policy."""
from fractions import Fraction


def merge(lo, size, step):
    if 2 * step == size:
        yield lo, lo + step
        return
    yield from merge(lo, size, 2 * step)
    yield from merge(lo + step, size, 2 * step)
    for i in range(lo + step, lo + size - step, 2 * step):
        yield i, i + step


def network(lo, size):
    if size > 1:
        yield from network(lo, size // 2)
        yield from network(lo + size // 2, size // 2)
        yield from merge(lo, size, 1)


if __name__ == "__main__":
    worst = Fraction()
    print("max_chain,actual_chain,comparators,ratio_to_insertion_floor")
    for longest in range(2, 17):
        pairs = list(network(0, 1 << (longest - 1).bit_length()))
        for actual in range(max(2, longest - 1), longest + 1):
            count = sum(j < actual for _, j in pairs)
            ratio = Fraction(count, actual - 1)
            worst = max(worst, ratio)
            print(f"{longest},{actual},{count},{ratio}")
    assert worst == Fraction(59, 14)
