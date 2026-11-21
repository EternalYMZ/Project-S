//! In-place Rust u64 kernels for exactly 1024 slots.
//! Call sort(&mut data) directly, or resolve a table once with kernel(name).
pub const N: usize = 1024;
pub type Data = [u64; N];
#[inline(always)]
fn insertion_pass(v: &mut Data, h: usize) {
    for i in h..N {
        let key = v[i];
        let mut j = i;
        if key < v[j - h] {
            loop {
                v[j] = v[j - h];
                j -= h;
                if j < h || key >= v[j - h] { break }
            }
            v[j] = key;
        }
    }
}
// Share a gap's specialized machine code across schedules.
#[inline(never)]
fn fixed_pass<const H: usize>(v: &mut Data) { insertion_pass(v, H) }

pub fn sort_ciura(data: &mut Data) {
    fixed_pass::<701>(data);
    fixed_pass::<301>(data);
    fixed_pass::<132>(data);
    fixed_pass::<57>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_ciura_0_705(data: &mut Data) {
    fixed_pass::<705>(data);
    fixed_pass::<301>(data);
    fixed_pass::<132>(data);
    fixed_pass::<57>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_tokuda(data: &mut Data) {
    fixed_pass::<525>(data);
    fixed_pass::<233>(data);
    fixed_pass::<103>(data);
    fixed_pass::<46>(data);
    fixed_pass::<20>(data);
    fixed_pass::<9>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_tokuda_0_472(data: &mut Data) {
    fixed_pass::<472>(data);
    fixed_pass::<233>(data);
    fixed_pass::<103>(data);
    fixed_pass::<46>(data);
    fixed_pass::<20>(data);
    fixed_pass::<9>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_tokuda_0_578(data: &mut Data) {
    fixed_pass::<578>(data);
    fixed_pass::<233>(data);
    fixed_pass::<103>(data);
    fixed_pass::<46>(data);
    fixed_pass::<20>(data);
    fixed_pass::<9>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_sedgewick(data: &mut Data) {
    fixed_pass::<929>(data);
    fixed_pass::<505>(data);
    fixed_pass::<209>(data);
    fixed_pass::<109>(data);
    fixed_pass::<41>(data);
    fixed_pass::<19>(data);
    fixed_pass::<5>(data);
    fixed_pass::<1>(data);
}
pub fn sort_sedgewick_0_883(data: &mut Data) {
    fixed_pass::<883>(data);
    fixed_pass::<505>(data);
    fixed_pass::<209>(data);
    fixed_pass::<109>(data);
    fixed_pass::<41>(data);
    fixed_pass::<19>(data);
    fixed_pass::<5>(data);
    fixed_pass::<1>(data);
}
pub fn sort_sedgewick_1_556(data: &mut Data) {
    fixed_pass::<929>(data);
    fixed_pass::<556>(data);
    fixed_pass::<209>(data);
    fixed_pass::<109>(data);
    fixed_pass::<41>(data);
    fixed_pass::<19>(data);
    fixed_pass::<5>(data);
    fixed_pass::<1>(data);
}
pub fn sort_sedgewick_6_3(data: &mut Data) {
    fixed_pass::<929>(data);
    fixed_pass::<505>(data);
    fixed_pass::<209>(data);
    fixed_pass::<109>(data);
    fixed_pass::<41>(data);
    fixed_pass::<19>(data);
    fixed_pass::<3>(data);
    fixed_pass::<1>(data);
}
pub fn sort_qin214(data: &mut Data) {
    fixed_pass::<239>(data);
    fixed_pass::<111>(data);
    fixed_pass::<51>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_qin214_0_251(data: &mut Data) {
    fixed_pass::<251>(data);
    fixed_pass::<111>(data);
    fixed_pass::<51>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_transfer1000(data: &mut Data) {
    fixed_pass::<995>(data);
    fixed_pass::<409>(data);
    fixed_pass::<156>(data);
    fixed_pass::<57>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_transfer1000_0_895(data: &mut Data) {
    fixed_pass::<895>(data);
    fixed_pass::<409>(data);
    fixed_pass::<156>(data);
    fixed_pass::<57>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_ciura_half(data: &mut Data) {
    fixed_pass::<301>(data);
    fixed_pass::<132>(data);
    fixed_pass::<57>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_tokuda_half(data: &mut Data) {
    fixed_pass::<233>(data);
    fixed_pass::<103>(data);
    fixed_pass::<46>(data);
    fixed_pass::<20>(data);
    fixed_pass::<9>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_sedgewick_half(data: &mut Data) {
    fixed_pass::<505>(data);
    fixed_pass::<209>(data);
    fixed_pass::<109>(data);
    fixed_pass::<41>(data);
    fixed_pass::<19>(data);
    fixed_pass::<5>(data);
    fixed_pass::<1>(data);
}
pub fn sort_qin214_full(data: &mut Data) {
    fixed_pass::<513>(data);
    fixed_pass::<239>(data);
    fixed_pass::<111>(data);
    fixed_pass::<51>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_qin_prepend_d1(data: &mut Data) {
    fixed_pass::<1023>(data);
    fixed_pass::<239>(data);
    fixed_pass::<111>(data);
    fixed_pass::<51>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_qin_prepend_d4(data: &mut Data) {
    fixed_pass::<1020>(data);
    fixed_pass::<239>(data);
    fixed_pass::<111>(data);
    fixed_pass::<51>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub fn sort_qin_prepend_d10(data: &mut Data) {
    fixed_pass::<1014>(data);
    fixed_pass::<239>(data);
    fixed_pass::<111>(data);
    fixed_pass::<51>(data);
    fixed_pass::<23>(data);
    fixed_pass::<10>(data);
    fixed_pass::<4>(data);
    fixed_pass::<1>(data);
}
pub use self::sort_sedgewick_6_3 as sort;
pub fn kernel(name: &str) -> Option<fn(&mut Data)> {
    match name {
        "ciura" => Some(sort_ciura),
        "ciura_0_705" => Some(sort_ciura_0_705),
        "tokuda" => Some(sort_tokuda),
        "tokuda_0_472" => Some(sort_tokuda_0_472),
        "tokuda_0_578" => Some(sort_tokuda_0_578),
        "sedgewick" => Some(sort_sedgewick),
        "sedgewick_0_883" => Some(sort_sedgewick_0_883),
        "sedgewick_1_556" => Some(sort_sedgewick_1_556),
        "sedgewick_6_3" => Some(sort_sedgewick_6_3),
        "qin214" => Some(sort_qin214),
        "qin214_0_251" => Some(sort_qin214_0_251),
        "transfer1000" => Some(sort_transfer1000),
        "transfer1000_0_895" => Some(sort_transfer1000_0_895),
        "ciura_half" => Some(sort_ciura_half),
        "tokuda_half" => Some(sort_tokuda_half),
        "sedgewick_half" => Some(sort_sedgewick_half),
        "qin214_full" => Some(sort_qin214_full),
        "qin_prepend_d1" => Some(sort_qin_prepend_d1),
        "qin_prepend_d4" => Some(sort_qin_prepend_d4),
        "qin_prepend_d10" => Some(sort_qin_prepend_d10),
        _ => None,
    }
}
#[cfg(test)]
#[test]
fn all_tables_sort() {
    for name in ["ciura","ciura_0_705","tokuda","tokuda_0_472","tokuda_0_578","sedgewick","sedgewick_0_883","sedgewick_1_556","sedgewick_6_3","qin214","qin214_0_251","transfer1000","transfer1000_0_895","ciura_half","tokuda_half","sedgewick_half","qin214_full","qin_prepend_d1","qin_prepend_d4","qin_prepend_d10"] {
        let mut x = std::array::from_fn(|i| ((i*239+51) % 97) as u64);
        let mut expected = x; expected.sort_unstable();
        kernel(name).unwrap()(&mut x); assert_eq!(x, expected);
    }
}
