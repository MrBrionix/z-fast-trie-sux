use rand::prelude::*;
use std::cmp::min;
use std::mem::swap;

use z_fast_trie_static_sux::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::BenchmarkId;

type Ds1 = NaiveTrie;
type Ds2 = CompactTrie;
type Ds3 = ZFastTrie<RollingHash>;
type Ds4 = ZFastTrieSux<RollingHash>;

pub fn bench_compact_fixed(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds2 = Ds2::new();

    bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"bench_compact_fixed");
}

pub fn bench_z_fast_fixed(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds3 = Ds3::new();

    bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"bench_z_fast_fixed");
}

pub fn bench_z_fast_variable(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = true;
    let fixed_seed = true;

    let mut ds: Ds3 = Ds3::new();

    bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"bench_z_fast_variable");
}

pub fn bench_z_fast_sux_fixed(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds4 = Ds4::new();

    bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"bench_z_fast_sux_fixed");
}

pub fn bench_z_fast_sux_variable(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = true;
    let fixed_seed = true;

    let mut ds: Ds4 = Ds4::new();

    bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"bench_z_fast_sux_variable");
}

/*
//#[test]
fn test_z_fast_fixed_small() {
    let t = 1;
    let bits = 40;
    let n = 1000000;
    let m = 1;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds3 = Ds3::new();

    test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds);
}

//#[test]
fn test_z_fast_sux_fixed_small() {
    let t = 1;
    let bits = 40;
    let n = 1000000;
    let m = 1;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds4 = Ds4::new();

    test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds);
}
*/

fn gen_bin_str(rng: &mut SmallRng, n: u32) -> Str {
    let mut s = Str::new();
    for _ in 0..n {
        s.push(rng.next_u32() % 2 == 0);
    }
    s
}

pub fn bench<T: Trie>(
    t: u32,
    bits: u32,
    n: u32,
    m: u32,
    deb: bool,
    variablelen: bool,
    fixed_seed: bool,
    ds: &mut T,
    c: &mut Criterion,
    name: &str
) {
    let mut rng = {
        if fixed_seed { SmallRng::seed_from_u64(0) } else { SmallRng::from_rng(thread_rng()) }
    };

    for _ in 0..t {
        let mut v: Vec<Str> = vec![];
        for _ in 0..n {
            let len = {
                if variablelen { (rng.next_u32() % ((bits / 4) * 3)) + bits / 4 } else { bits }
            };
            let s = gen_bin_str(&mut rng, len);
            let mut flag = true;

            if variablelen {
                //da ottimizzare
                for i in &v {
                    if i[0..min(i.len(), s.len())] == *i || i[0..min(i.len(), s.len())] == s {
                        flag = false;
                        continue;
                    }
                }
            }
            if flag {
                v.push(s);
            }
        }

        if deb {
            print!("genero:\n");
            for i in &v {
                print!("{}\n", i);
            }
            print!("testo:\n");
        }
        
        ds.build(&v);
        

        let mut group = c.benchmark_group(name);
        for i in 0..m {
            let len = {
                if variablelen { (rng.next_u32() % ((bits / 4) * 3)) + bits / 4 } else { bits }
            };
            let mut s1 = gen_bin_str(&mut rng, len);
            let len2 = {
                if variablelen { (rng.next_u32() % ((bits / 4) * 3)) + bits / 4 } else { bits }
            };
            let mut s2 = gen_bin_str(&mut rng, len2);
            if s1 > s2 {
                swap(&mut s1, &mut s2);
            }
            if deb {
                print!("query: {} & {}\n", s1, s2);
            }

            group.bench_with_input(BenchmarkId::from_parameter(&s1), &s1, |b, s1| b.iter(|| ds.pred_query(&s1)));
            //c.bench_function(name,|b| b.iter(|| ds.pred_query(&s1)));
            
            /*let pred = ds.pred_query(&s1);
            if deb {
                if let Some(t) = pred {
                    print!("pred: {}\n", t);
                } else {
                    print!("non ha predecessori\n");
                }
            }

            let succ = ds.succ_query(&s1);
            if deb {
                if let Some(t) = succ {
                    print!("succ: {}\n", t);
                } else {
                    print!("non ha successori\n");
                }
            }

            let flag = ds.ex_range_query(&s1, &s2);
            if deb {
                print!("range query: {}\n", flag);
                print!("------------\n");
            }*/
        }
        group.finish();
    }
}

criterion_group!(benches,
bench_compact_fixed,
//bench_z_fast_fixed,
//bench_z_fast_variable,
//bench_z_fast_sux_fixed,
//bench_z_fast_sux_variable
);
criterion_main!(benches);

