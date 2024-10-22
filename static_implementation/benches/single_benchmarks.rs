use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use std::cmp::min;
use std::cmp::Ordering::*;
use std::mem::swap;
use z_fast_trie_static_sux::prelude::*;

type Ds2 = CompactTrie;
type Ds3 = ZFastTrie<RollingHash>;
type Ds4 = ZFastTrieSux<RollingHash>;

pub fn single_bench_compact_fixed(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds2 = Ds2::new();

    single_bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"single_bench_compact_fixed");
}

pub fn single_bench_z_fast_fixed(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds3 = Ds3::new();

    single_bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"single_bench_z_fast_fixed");
}

pub fn single_bench_z_fast_variable(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = true;
    let fixed_seed = true;

    let mut ds: Ds3 = Ds3::new();

    single_bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"single_bench_z_fast_variable");
}

pub fn single_bench_z_fast_sux_fixed(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds4 = Ds4::new();

    single_bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"single_bench_z_fast_sux_fixed");
}

pub fn single_bench_z_fast_sux_variable(c: &mut Criterion) {
    let t = 1;
    let bits = 10000;
    let n = 5000;
    let m = 10;
    let deb = false;
    let variablelen = true;
    let fixed_seed = true;

    let mut ds: Ds4 = Ds4::new();

    single_bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"single_bench_z_fast_sux_variable");
}

pub fn single_bench_z_fast_fixed_small(c: &mut Criterion) {
    let t = 1;
    let bits = 40;
    let n = 100000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds3 = Ds3::new();

    single_bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"single_bench_z_fast_fixed_small");
}

fn single_bench_z_fast_sux_fixed_small(c: &mut Criterion) {
    let t = 1;
    let bits = 40;
    let n = 100000;
    let m = 10;
    let deb = false;
    let variablelen = false;
    let fixed_seed = true;

    let mut ds: Ds4 = Ds4::new();

    single_bench(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, c, &"single_bench_z_fast_sux_fixed_small");
}


fn gen_bin_str(rng: &mut SmallRng, n: u32) -> Str {
    let mut s = Str::new(0);
    for _ in 0..n {
        s.push(rng.next_u32() % 2 == 0);
    }
    s
}

pub fn single_bench<T: Trie>(
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
                    if get_substr(i,0,min(i.len(), s.len())) == *i || get_substr(i,0,min(i.len(), s.len())) == s {
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
        for _i in 0..m {
            let len = {
                if variablelen { (rng.next_u32() % ((bits / 4) * 3)) + bits / 4 } else { bits }
            };
            let mut s1 = gen_bin_str(&mut rng, len);
            let len2 = {
                if variablelen { (rng.next_u32() % ((bits / 4) * 3)) + bits / 4 } else { bits }
            };
            let mut s2 = gen_bin_str(&mut rng, len2);
            if cmp(&s1,&s2) == Greater {
                swap(&mut s1, &mut s2);
            }
            if deb {
                print!("query: {} & {}\n", s1, s2);
            }

            group.bench_with_input(BenchmarkId::from_parameter(&get_substr(&s1,0,min(20,s1.len()))), &s1, |b, s1| b.iter(|| ds.pred_query(&s1)));
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
single_bench_compact_fixed,
single_bench_z_fast_fixed,
single_bench_z_fast_variable,
single_bench_z_fast_sux_fixed,
single_bench_z_fast_sux_variable,
single_bench_z_fast_fixed_small,
single_bench_z_fast_sux_fixed_small
);
criterion_main!(benches);

