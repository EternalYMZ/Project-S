//! Fixed, eight-comparison dispatch on the Sedgewick_6_3 schedule.
#[path = "../net/network1024.rs"]
mod network1024;

pub const SAMPLES: [usize; 8] = [64, 192, 320, 448, 576, 704, 832, 960];

#[inline(always)]
pub fn uses_network(data: &[u64; 1024]) -> bool {
    let mut descending = 0;
    for i in SAMPLES { descending += usize::from(data[i] > data[i + 1]); }
    (2..=6).contains(&descending)
}

#[inline(never)]
pub fn network(data: &mut [u64; 1024]) { network1024::sort(data); }

#[inline(never)]
fn insertion_pass<const H: usize>(v: &mut [u64; 1024]) {
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

#[inline(never)]
pub fn insertion(v: &mut [u64; 1024]) {
    insertion_pass::<929>(v); insertion_pass::<505>(v);
    insertion_pass::<209>(v); insertion_pass::<109>(v);
    insertion_pass::<41>(v); insertion_pass::<19>(v);
    insertion_pass::<3>(v); insertion_pass::<1>(v);
}

#[inline(never)]
pub fn sort(data: &mut [u64; 1024]) {
    if uses_network(data) { network(data) } else { insertion(data) }
}
