//! Gap 3 review, item 4: subtree sizes inside a level of the real row-scan tree.
//!
//! Built on Track 10's independently assessed child function (tree.rs and puzzle.rs of commit dc71ad56,
//! exported unchanged by build.sh). Everything else is written here and shares no code with
//! the memo's g3tree.c: the population sampler (uniform subset of the children, without
//! replacement, Floyd's algorithm as in Track 10's `smc`), the member draw (without
//! replacement) and the exact count of the subtree below a member.
//!
//!   r3_shape <csv> <np> <seed> <d1,d2,..> <m1,m2,..> <cap per subtree> <node limit>
//!
//! Output: "LV k log10width" per level, "US d size capped" per sampled member (size = valid
//! partial boards strictly below the member, counted to cap), "END nodes" at the end.
mod puzzle;
mod tree;
use puzzle::{load_csv, Rng};
use tree::{rows_bottom_up, State, Tree};

fn count(t: &Tree, st: &mut State, k: usize, nodes: &mut u64, cap: u64) {
    if k == t.nfree || *nodes >= cap {
        return;
    }
    let mut kids = [0u32; 96];
    let mut n = 0usize;
    t.for_each_child(st, k, |i| {
        kids[n] = i;
        n += 1;
    });
    for &i in &kids[..n] {
        if *nodes >= cap {
            return;
        }
        *nodes += 1;
        t.apply(st, k, i);
        count(t, st, k + 1, nodes, cap);
        t.undo(st, k, i);
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let p = load_csv(&a[1]);
    let np: usize = a[2].parse().unwrap();
    let seed: u64 = a[3].parse().unwrap();
    let depths: Vec<usize> = a[4].split(',').map(|x| x.parse().unwrap()).collect();
    let ms: Vec<usize> = a[5].split(',').map(|x| x.parse().unwrap()).collect();
    let cap: u64 = a[6].parse().unwrap();
    let limit: u64 = a[7].parse().unwrap();
    let fixed = p.hints.clone();
    assert_eq!(fixed.len(), 5);
    let order: Vec<usize> = rows_bottom_up(p.n)
        .into_iter()
        .filter(|q| !fixed.iter().any(|f| f.0 == *q))
        .collect();
    let t = Tree::new(&p, &order, &fixed);
    let mut rng = Rng::new(seed);
    let mut pop: Vec<State> = vec![t.root()];
    let mut log10w = 0f64;
    let mut spent = 0u64;
    println!("LV 0 0");
    for k in 0..t.nfree {
        let mut counts: Vec<u32> = Vec::with_capacity(pop.len());
        let mut e = 0u64;
        for st in &pop {
            let mut c = 0u32;
            t.for_each_child(st, k, |_| c += 1);
            counts.push(c);
            e += c as u64;
        }
        if e == 0 {
            break;
        }
        log10w += (e as f64 / pop.len() as f64).log10();
        println!("LV {} {:.6}", k + 1, log10w);
        let mut chosen: Vec<u64> = if e as usize <= np {
            (0..e).collect()
        } else {
            let mut set = std::collections::HashSet::with_capacity(np * 2);
            for j in (e - np as u64)..e {
                let r = rng.next_u64() % (j + 1);
                if !set.insert(r) {
                    set.insert(j);
                }
            }
            set.into_iter().collect()
        };
        chosen.sort_unstable();
        let mut next: Vec<State> = Vec::with_capacity(chosen.len());
        let (mut ci, mut base) = (0usize, 0u64);
        for (j, st) in pop.iter().enumerate() {
            let c = counts[j] as u64;
            if ci < chosen.len() && chosen[ci] < base + c {
                let mut off = 0u64;
                let mut kids: Vec<u32> = Vec::new();
                t.for_each_child(st, k, |i| kids.push(i));
                for &i in &kids {
                    if ci < chosen.len() && chosen[ci] == base + off {
                        let mut ns = st.clone();
                        t.apply(&mut ns, k, i);
                        next.push(ns);
                        ci += 1;
                    }
                    off += 1;
                }
            }
            base += c;
        }
        pop = next;
        if let Some(di) = depths.iter().position(|&d| d == k + 1) {
            let m = ms[di].min(pop.len());
            // m distinct members: partial Fisher-Yates over the indices
            let mut idx: Vec<usize> = (0..pop.len()).collect();
            for i in 0..m {
                let j = i + rng.below(idx.len() - i);
                idx.swap(i, j);
            }
            for &i in &idx[..m] {
                if spent >= limit {
                    println!("STOP {} node limit reached", k + 1);
                    break;
                }
                let mut st = pop[i].clone();
                let mut nodes = 0u64;
                count(&t, &mut st, k + 1, &mut nodes, cap);
                spent += nodes;
                println!("US {} {} {}", k + 1, nodes, (nodes >= cap) as u8);
            }
        }
    }
    println!("END {}", spent);
}
