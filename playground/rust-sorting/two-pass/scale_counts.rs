#[allow(dead_code)]
mod kernel;
use kernel::{one_pass, Cost, INSERT, INTRO};
use std::io::{self, Write};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> usize {
        loop {
            let x = self.next();
            if x >= n.wrapping_neg() % n { return (x % n) as usize; }
        }
    }
}

fn pass<const P: u8>(a: &mut [u64], h: usize) -> Cost {
    let mut c = Cost::default();
    one_pass::<P, true>(a, h, false, &mut c);
    c
}

fn main() -> io::Result<()> {
    let mut stream = Rng(0x51c8ab3752f03d09);
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out,"n,pattern,seed,insertion_first_cmp,intro_first_cmp,insertion_first_writes,intro_first_writes,final_cmp,final_writes,final_shifts")?;
    for n in [32, 64, 128, 256, 512, 1024, 2048, 4096] {
        for trial in 0..=128 {
            let seed = stream.next();
            let mut rng = Rng(seed);
            let mut a: Vec<_> = (0..n as u64).collect();
            let pattern = if trial < 128 {
                for i in (1..n).rev() { a.swap(i, rng.below((i + 1) as u64)); }
                "random"
            } else {
                for i in 0..n / 2 {
                    a[2*i] = (n/2 + i) as u64;
                    a[2*i+1] = i as u64;
                }
                "separated_parities"
            };
            let mut b = a.clone();
            let x = pass::<INSERT>(&mut a, 2);
            let y = pass::<INTRO>(&mut b, 2);
            assert_eq!(a, b);
            let z = pass::<INSERT>(&mut a, 1);
            let t = pass::<INTRO>(&mut b, 1);
            assert_eq!((z.comparisons,z.writes,z.shifts),(t.comparisons,t.writes,t.shifts));
            assert_eq!(a, b);
            assert!(a.iter().enumerate().all(|(i,&x)| x == i as u64));
            if trial == 128 {
                let m = (n/2) as u64;
                assert_eq!(z.shifts, m*(m+1)/2);
            }
            writeln!(out,"{n},{pattern},{seed},{},{},{},{},{},{},{}",x.comparisons,y.comparisons,x.writes,y.writes,z.comparisons,z.writes,z.shifts)?;
        }
    }
    out.flush()
}
