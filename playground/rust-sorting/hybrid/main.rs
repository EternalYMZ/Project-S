mod kernel;
use kernel::*;
use std::fs::File;
use std::hint::black_box;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut x = self.0;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }
    fn index(&mut self, n: usize) -> usize {
        let n = n as u64;
        loop {
            let x = self.next();
            if x >= n.wrapping_neg() % n {
                return (x % n) as usize;
            }
        }
    }
    fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in (1..a.len()).rev() {
            a.swap(i, self.index(i + 1));
        }
    }
}
const PATTERNS: [&str; 4] = ["random", "near_adj8", "near_far4", "reverse"];
fn input(seed: u64, pattern: &str) -> Data {
    let mut a = std::array::from_fn(|i| i as u64);
    let mut rng = Rng(seed);
    match pattern {
        "random" => rng.shuffle(&mut a),
        "near_adj8" => {
            let mut used = [false; 1024];
            let mut changed = 0;
            while changed < 8 {
                let i = rng.index(1023);
                if !used[i] && !used[i + 1] {
                    a.swap(i, i + 1);
                    used[i] = true;
                    used[i + 1] = true;
                    changed += 1;
                }
            }
        }
        "near_far4" => {
            let mut used = [false; 1024];
            let mut changed = 0;
            while changed < 4 {
                let i = rng.index(1024);
                let j = rng.index(1024);
                if i.abs_diff(j) >= 256 && !used[i] && !used[j] {
                    a.swap(i, j);
                    used[i] = true;
                    used[j] = true;
                    changed += 1;
                }
            }
        }
        "reverse" => a.reverse(),
        _ => panic!("unknown input"),
    }
    a
}
fn checked(v: &Data) {
    assert!(v.iter().enumerate().all(|(i, &x)| x == i as u64));
}
fn run_pass(p: usize, a: &mut Data, h: usize, bit: bool) -> Cost {
    let mut c = Cost::default();
    macro_rules! dispatch { ($($code:ident),*)=>{match p as u8 {$($code=>one_pass::<$code,true>(a,h,bit,&mut c),)*_=>unreachable!()}} }
    dispatch!(INSERT, INTRO, LENGTH, SAMPLE, FROZEN, BUDGET);
    c
}
fn oracle(a: &Data, h: usize) -> (Data, u64, u64) {
    // Independent stable sort on gathered chains ONLY in the offline oracle.
    // Local ranks and Fenwick prefix counts give F_h and I_h before the pass.
    let mut out = *a;
    let mut f = 0u64;
    let mut inv = 0u64;
    let mut rank = [0usize; 1024];
    for r in 0..h {
        let mut chain: Vec<_> = a[r..].iter().step_by(h).copied().collect();
        chain.sort();
        for (j, &x) in chain.iter().enumerate() {
            out[r + j * h] = x;
            rank[x as usize] = j;
        }
        let mut tree = vec![0usize; chain.len() + 1];
        for (j, &x) in a[r..].iter().step_by(h).enumerate() {
            let target = rank[x as usize];
            f += target.abs_diff(j) as u64;
            let mut t = target + 1;
            let mut below = 0;
            while t > 0 {
                below += tree[t];
                t &= t - 1;
            }
            inv += (j - below) as u64;
            t = target + 1;
            while t < tree.len() {
                tree[t] += 1;
                t += t & t.wrapping_neg();
            }
        }
    }
    assert!(inv <= f && f <= 2 * inv);
    (out, f, inv)
}
fn file(path: &str) -> BufWriter<File> {
    BufWriter::new(File::create(path).unwrap())
}
fn train(out: &mut dyn Write) -> [u16; 3] {
    writeln!(
        out,
        "table,gap,train_arrays,insertion_comparisons,intro_comparisons,choose_intro"
    )
    .unwrap();
    let mut masks = [0u16; 3];
    for (idx, &(table, gaps)) in TABLES.iter().enumerate() {
        let mut rng = Rng(0x29ee64000203a745);
        let mut winner = vec![(0, 0); gaps.len()];
        for _ in 0..64 {
            let mut a = input(rng.next(), "random");
            for (j, &h) in gaps.iter().enumerate() {
                let mut b = a;
                let i = run_pass(0, &mut a, h, false);
                let q = run_pass(1, &mut b, h, false);
                assert_eq!(a, b);
                winner[j].0 += i.comparisons;
                winner[j].1 += q.comparisons;
            }
        }
        for (j, (&(i, q), &h)) in winner.iter().zip(gaps).enumerate() {
            let choose = q < i && h != 1;
            if choose {
                masks[idx] |= 1 << j;
            }
            writeln!(out, "{table},{h},64,{i},{q},{}", choose as u8).unwrap();
        }
    }
    masks
}
fn audit() {
    verify_heap_fallback();
    let masks = train(&mut file("training.csv"));
    let mut out = file("audit.csv");
    writeln!(out,"table,pattern,seed,policy,gap,F_h,I_h,comparisons,selector_cmp,slot_writes,shift_writes,insert_chains,intro_chains,sampled_chains,budget_fallbacks,heap_fallbacks").unwrap();
    let mut stdout = file("std-counts.csv");
    writeln!(stdout, "pattern,seed,comparisons").unwrap();
    let mut assertions = 0;
    let mut rng = Rng(0x7fd220500325737f);
    for _ in 0..192 {
        let seed = rng.next();
        for pattern in PATTERNS {
            let initial = input(seed, pattern);
            let mut std = initial;
            let mut cmp = 0u64;
            std.sort_unstable_by(|a, b| {
                cmp += 1;
                a.cmp(b)
            });
            checked(&std);
            writeln!(stdout, "{pattern},{seed},{cmp}").unwrap();
            for (t, &(table, gaps)) in TABLES.iter().enumerate() {
                let mut a = initial;
                for (j, &h) in gaps.iter().enumerate() {
                    let (want, f, inv) = oracle(&a, h);
                    for (p, policy) in NAMES.iter().enumerate() {
                        let mut v = a;
                        let c = run_pass(p, &mut v, h, masks[t] & (1 << j) != 0);
                        assert_eq!(v, want, "{table} {pattern} {seed} {policy} {h}");
                        if p == 0 || h == 1 {
                            assert_eq!(c.shifts, inv);
                        }
                        assert!(c.selector <= c.comparisons);
                        assertions += 1;
                        writeln!(out,"{table},{pattern},{seed},{policy},{h},{f},{inv},{},{},{},{},{},{},{},{},{}",c.comparisons,c.selector,c.writes,c.shifts,c.ins_chains,c.intro_chains,c.sampled_chains,c.budget_fallbacks,c.heap_fallbacks).unwrap();
                    }
                    a = want;
                }
                checked(&a);
            }
        }
    }
    println!(
        "{assertions} phase-by-phase comparisons to independent oracle passed; forced heap fallback passed; frozen masks={masks:?}"
    );
}
type Job = fn(&mut Data, &[usize], u16);
fn std_sort(a: &mut Data, _: &[usize], _: u16) {
    a.sort_unstable()
}
const JOBS: [Job; 6] = [
    sort::<INSERT>,
    sort::<INTRO>,
    sort::<LENGTH>,
    sort::<SAMPLE>,
    sort::<FROZEN>,
    sort::<BUDGET>,
];
#[inline(never)]
fn timed(job: Job, input: &[Data], gaps: &[usize], mask: u16, buf: &mut [Data]) -> u128 {
    buf.copy_from_slice(input);
    let start = Instant::now();
    for a in buf.iter_mut() {
        job(black_box(a), black_box(gaps), mask);
    }
    black_box(&*buf);
    let dt = start.elapsed().as_nanos();
    for a in buf {
        checked(a);
    }
    dt
}
fn timing() {
    let masks = train(&mut io::sink());
    let mut out = file("time.csv");
    writeln!(
        out,
        "table,pattern,policy,seed,trial,stream,batch,pass,order,std_ns,insertion_ns,candidate_ns"
    )
    .unwrap();
    let mut rng = Rng(0x8fa4bb000273dfc9);
    let k = 8;
    let mut buf = vec![[0; 1024]; k];
    for _ in 0..20 {
        let seed = rng.next();
        let mut stream_rng = Rng(seed);
        let mut order_rng = Rng(seed ^ 0x22f753babe921aa7);
        for trial in 0..=4 {
            let stream = stream_rng.next();
            let mut tasks: Vec<_> = (0..TABLES.len() * PATTERNS.len()).collect();
            order_rng.shuffle(&mut tasks);
            for task in tasks {
                let t = task % TABLES.len();
                let pat = PATTERNS[task / TABLES.len()];
                let (table, gaps) = TABLES[t];
                let mut local = Rng(stream);
                let data: Vec<_> = (0..k).map(|_| input(local.next(), pat)).collect();
                let mut policies = [0, 1, 2, 3, 4, 5];
                order_rng.shuffle(&mut policies);
                for p in policies {
                    let jobs = [std_sort as Job, JOBS[0], JOBS[p]];
                    let mut order = [0, 1, 2];
                    order_rng.shuffle(&mut order);
                    for pass in 0..2 {
                        let mut times = [0; 3];
                        if pass == 1 {
                            order.reverse();
                        }
                        for &arm in &order {
                            times[arm] = timed(jobs[arm], &data, gaps, masks[t], &mut buf);
                        }
                        if trial > 0 {
                            writeln!(out,"{table},{pat},{},{seed},{trial},{stream},{k},{pass},{}{}{},{},{},{}",NAMES[p],order[0],order[1],order[2],times[0],times[1],times[2]).unwrap();
                        }
                    }
                }
            }
        }
        out.flush().unwrap();
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("time") => timing(),
        Some("audit") => audit(),
        _ => panic!("supply audit or time"),
    }
}
