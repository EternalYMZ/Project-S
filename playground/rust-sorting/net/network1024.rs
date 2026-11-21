//! Standalone Sedgewick_6_3 shellsort for exactly 1024 u64 keys.
//! Short chains (at most 16 keys) use truncated Batcher odd-even networks.

#[inline(always)]
fn compare_columns<const H: usize, const I: usize, const J: usize>(v: &mut [u64; 1024]) {
    let lanes = H.min(1024 - J * H);
    for r in 0..lanes {
        let a = v[I * H + r];
        let b = v[J * H + r];
        v[I * H + r] = a.min(b);
        v[J * H + r] = a.max(b);
    }
}

#[inline(never)]
fn insert<const H: usize>(v: &mut [u64; 1024]) {
    for i in H..1024 {
        let key = v[i];
        let mut j = i;
        if key < v[j - H] {
            loop {
                v[j] = v[j - H];
                j -= H;
                if j < H || key >= v[j - H] { break; }
            }
            v[j] = key;
        }
    }
}

// Expand each comparator at compile time. The innermost loop then runs
// across adjacent residue classes at a constant displacement.
macro_rules! compare_pairs {
    ($v:ident, $h:literal; $(($i:literal, $j:literal)),+ $(,)?) => {
        $(compare_columns::<$h, $i, $j>($v);)+
    };
}

/// Sort in ascending order. This specialization uses no heap allocation.
pub fn sort(v: &mut [u64; 1024]) {
    compare_pairs!(v, 929; (0,1));
    compare_pairs!(v, 505; (0,1), (0,2), (1,2));
    compare_pairs!(v, 209;
        (0,1), (2,3), (0,2), (1,3), (1,2), (0,4), (2,4), (1,2), (3,4));
    compare_pairs!(v, 109;
        (0,1), (2,3), (0,2), (1,3), (1,2), (4,5), (6,7), (4,6),
        (5,7), (5,6), (0,4), (2,6), (2,4), (1,5), (3,7), (3,5),
        (1,2), (3,4), (5,6), (8,9), (0,8), (4,8), (2,4), (6,8),
        (1,9), (5,9), (3,5), (7,9), (1,2), (3,4), (5,6), (7,8));
    insert::<41>(v);
    insert::<19>(v);
    insert::<3>(v);
    insert::<1>(v);
}

#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn agrees_with_std() {
        let mut state = 731u64;
        for kind in 0..4 {
            for _ in 0..64 {
                let mut keys = std::array::from_fn(|i| {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    match kind {
                        0 => state,
                        1 => state % 7,
                        2 => u64::MAX - i as u64,
                        _ => (i / 3) as u64,
                    }
                });
                let mut expected = keys;
                expected.sort_unstable();
                sort(&mut keys);
                assert_eq!(keys, expected);
            }
        }
    }
}
