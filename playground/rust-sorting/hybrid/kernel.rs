//! In-place hybrid sorts on true strided chains of a 1024-key array.
//! No chain materialization, precomputed ranks or auxiliary array in these kernels.
pub type Data = [u64; 1024];
pub const QIN: &[usize] = &[239, 111, 51, 23, 10, 4, 1];
pub const SEDGEWICK: &[usize] = &[929, 505, 209, 109, 41, 19, 3, 1];
pub const TWO: &[usize] = &[2, 1];
pub const TABLES: [(&str, &[usize]); 3] =
    [("qin214", QIN), ("sedgewick_6_3", SEDGEWICK), ("two", TWO)];

pub const INSERT: u8 = 0;
pub const INTRO: u8 = 1;
pub const LENGTH: u8 = 2;
pub const SAMPLE: u8 = 3;
pub const FROZEN: u8 = 4;
pub const BUDGET: u8 = 5;
pub const NAMES: [&str; 6] = ["insert", "intro", "length", "sample", "frozen", "budget"];

#[derive(Copy, Clone, Default)]
pub struct Cost {
    pub comparisons: u64,
    pub selector: u64,
    pub writes: u64,
    pub shifts: u64,
    pub ins_chains: u64,
    pub intro_chains: u64,
    pub sampled_chains: u64,
    pub budget_fallbacks: u64,
    pub heap_fallbacks: u64,
}
#[inline(always)]
fn less<const COUNT: bool>(x: u64, y: u64, c: &mut Cost) -> bool {
    if COUNT {
        c.comparisons += 1;
    }
    x < y
}
#[inline(always)]
fn swap<const H: usize, const COUNT: bool>(a: &mut [u64], i: usize, j: usize, c: &mut Cost) {
    if i != j {
        a.swap(i * H, j * H);
        if COUNT {
            c.writes += 2;
        }
    }
}
#[inline(always)]
fn insert_key<const H: usize, const COUNT: bool>(
    a: &mut [u64],
    lo: usize,
    k: usize,
    c: &mut Cost,
) -> usize {
    let key = a[k * H];
    let mut j = k;
    while j > lo && less::<COUNT>(key, a[(j - 1) * H], c) {
        a[j * H] = a[(j - 1) * H];
        j -= 1;
    }
    let shifts = k - j;
    if shifts > 0 {
        a[j * H] = key;
        if COUNT {
            c.writes += 1 + shifts as u64;
            c.shifts += shifts as u64;
        }
    }
    shifts
}
fn insertion<const H: usize, const COUNT: bool>(a: &mut [u64], lo: usize, hi: usize, c: &mut Cost) {
    for k in lo + 1..hi {
        insert_key::<H, COUNT>(a, lo, k, c);
    }
}

fn sift<const H: usize, const COUNT: bool>(
    a: &mut [u64],
    lo: usize,
    n: usize,
    mut root: usize,
    c: &mut Cost,
) {
    while 2 * root + 1 < n {
        let mut child = 2 * root + 1;
        if child + 1 < n && less::<COUNT>(a[(lo + child) * H], a[(lo + child + 1) * H], c) {
            child += 1;
        }
        if !less::<COUNT>(a[(lo + root) * H], a[(lo + child) * H], c) {
            return;
        }
        swap::<H, COUNT>(a, lo + root, lo + child, c);
        root = child;
    }
}
fn heap<const H: usize, const COUNT: bool>(a: &mut [u64], lo: usize, hi: usize, c: &mut Cost) {
    let n = hi - lo;
    for root in (0..n / 2).rev() {
        sift::<H, COUNT>(a, lo, n, root, c);
    }
    for end in (1..n).rev() {
        swap::<H, COUNT>(a, lo, lo + end, c);
        sift::<H, COUNT>(a, lo, end, 0, c);
    }
}
fn median<const COUNT: bool>(x: u64, y: u64, z: u64, c: &mut Cost) -> u64 {
    if less::<COUNT>(x, y, c) {
        if less::<COUNT>(y, z, c) {
            y
        } else if less::<COUNT>(x, z, c) {
            z
        } else {
            x
        }
    } else if less::<COUNT>(x, z, c) {
        x
    } else if less::<COUNT>(y, z, c) {
        z
    } else {
        y
    }
}
fn introsort<const H: usize, const COUNT: bool>(
    a: &mut [u64],
    mut lo: usize,
    mut hi: usize,
    mut depth: u32,
    c: &mut Cost,
) {
    while hi - lo > 12 {
        if depth == 0 {
            if COUNT {
                c.heap_fallbacks += 1;
            }
            heap::<H, COUNT>(a, lo, hi, c);
            return;
        }
        depth -= 1;
        let pivot = median::<COUNT>(a[lo * H], a[(lo + (hi - lo) / 2) * H], a[(hi - 1) * H], c);
        let mut i = lo;
        let mut j = hi;
        while i < j {
            while i < j && less::<COUNT>(a[i * H], pivot, c) {
                i += 1;
            }
            while i < j && less::<COUNT>(pivot, a[(j - 1) * H], c) {
                j -= 1;
            }
            if i < j {
                j -= 1;
                swap::<H, COUNT>(a, i, j, c);
                i += 1;
            }
        }
        if j - lo < hi - i {
            introsort::<H, COUNT>(a, lo, j, depth, c);
            lo = i;
        } else {
            introsort::<H, COUNT>(a, i, hi, depth, c);
            hi = j;
        }
    }
    insertion::<H, COUNT>(a, lo, hi, c);
}

// Pairs of scaled positions. The rule is deterministic and sees values only
// through the 8 in-chain comparisons. For len >=32 all pairs are distinct.
const PAIRS: [(usize, usize); 8] = [
    (0, 5),
    (6, 12),
    (2, 14),
    (7, 8),
    (3, 9),
    (1, 4),
    (10, 15),
    (5, 11),
];
fn choose_intro<const H: usize, const COUNT: bool>(a: &[u64], len: usize, c: &mut Cost) -> bool {
    if COUNT {
        c.sampled_chains += 1;
        c.selector += 8;
    }
    let mut inverted = 0;
    for (x, y) in PAIRS {
        let i = (len - 1) * x / 15;
        let j = (len - 1) * y / 15;
        inverted += less::<COUNT>(a[j * H], a[i * H], c) as usize;
    }
    inverted >= 2
}
#[inline(never)]
fn pass<const H: usize, const P: u8, const COUNT: bool>(a: &mut Data, frozen: bool, c: &mut Cost) {
    for r in 0..H {
        let chain = &mut a[r..];
        let m = chain.len().div_ceil(H);
        if m < 2 {
            continue;
        }
        if P == BUDGET && H != 1 {
            if COUNT {
                c.ins_chains += 1;
            }
            let mut shifts = 0;
            let budget = m * (usize::BITS - (m - 1).leading_zeros()) as usize;
            for k in 1..m {
                shifts += insert_key::<H, COUNT>(chain, 0, k, c);
                if k + 1 < m && shifts >= budget {
                    if COUNT {
                        c.budget_fallbacks += 1;
                    }
                    introsort::<H, COUNT>(chain, 0, m, 2 * m.ilog2(), c);
                    break;
                }
            }
            continue;
        }
        let use_intro = H != 1
            && match P {
                INTRO => true,
                LENGTH => m >= 32,
                SAMPLE => m >= 32 && choose_intro::<H, COUNT>(chain, m, c),
                FROZEN => frozen,
                _ => false,
            };
        if use_intro {
            if COUNT {
                c.intro_chains += 1;
            }
            introsort::<H, COUNT>(chain, 0, m, 2 * m.ilog2(), c);
        } else {
            if COUNT {
                c.ins_chains += 1;
            }
            insertion::<H, COUNT>(chain, 0, m, c);
        }
    }
}
pub fn one_pass<const P: u8, const COUNT: bool>(
    a: &mut Data,
    h: usize,
    frozen: bool,
    c: &mut Cost,
) {
    macro_rules! gaps {($($g:literal),*)=>{match h {$($g=>pass::<$g,P,COUNT>(a,frozen,c),)* _=>unreachable!()}}}
    gaps!(239, 111, 51, 23, 10, 4, 1, 929, 505, 209, 109, 41, 19, 3, 2)
}
pub fn sort<const P: u8>(a: &mut Data, gaps: &[usize], mask: u16) {
    let mut c = Cost::default();
    for (j, &h) in gaps.iter().enumerate() {
        one_pass::<P, false>(a, h, mask & (1 << j) != 0, &mut c);
    }
}
// Standalone qin entry. Final gap 1 always uses linear insertion.
#[allow(dead_code)]
pub fn adaptive_qin(a: &mut Data) {
    sort::<BUDGET>(a, QIN, 0)
}

pub fn verify_heap_fallback() {
    let mut xs: Data = std::array::from_fn(|i| ((i * 911) % 1024) as u64 % 37);
    let reference = xs;
    let mut c = Cost::default();
    introsort::<4, true>(&mut xs, 0, 256, 0, &mut c);
    assert_eq!(c.heap_fallbacks, 1);
    let mut oracle: Vec<_> = reference.iter().step_by(4).copied().collect();
    oracle.sort();
    for i in 0..1024 {
        assert_eq!(
            xs[i],
            if i % 4 == 0 {
                oracle[i / 4]
            } else {
                reference[i]
            }
        );
    }
}
