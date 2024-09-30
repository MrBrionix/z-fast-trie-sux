#![feature(test)]
extern crate test;

pub mod hashes;
pub mod rank_structures;
pub mod static_dicts;
pub mod traits;
pub mod tries;
pub mod utils;

pub mod prelude {
    pub use crate::hashes::*;
    pub use crate::rank_structures;
    pub use crate::static_dicts::*;
    pub use crate::traits::*;
    pub use crate::tries::*;
    pub use crate::utils::*;
}

use crate::prelude::*;
use rand::prelude::*;
use std::cmp::min;
use std::mem::swap;
use test::Bencher;

#[cfg(test)]
mod random_tests {
    use crate::*;
    use test::Bencher;

    type Ds1 = NaiveTrie;
    type Ds2 = CompactTrie;
    type Ds3 = ZFastTrie<RollingHash>;
    type Ds4 = ZFastTrieSux<RollingHash>;

    #[bench]
    fn test_compact_fixed(b: &mut Bencher) {
        let t = 1;
        let bits = 10000;
        let n = 5000;
        let m = 10;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds2 = Ds2::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, b);
    }

    #[bench]
    fn test_z_fast_fixed(b: &mut Bencher) {
        let t = 1;
        let bits = 10000;
        let n = 5000;
        let m = 10;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds3 = Ds3::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, b);
    }

    #[bench]
    fn test_z_fast_variable(b: &mut Bencher) {
        let t = 1;
        let bits = 10000;
        let n = 5000;
        let m = 10;
        let deb = false;
        let variablelen = true;
        let fixed_seed = true;

        let mut ds: Ds3 = Ds3::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, b);
    }

    #[bench]
    fn test_z_fast_sux_fixed(b: &mut Bencher) {
        let t = 1;
        let bits = 10000;
        let n = 5000;
        let m = 10;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds4 = Ds4::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, b);
    }

    #[bench]
    fn test_z_fast_sux_variable(b: &mut Bencher) {
        let t = 1;
        let bits = 10000;
        let n = 5000;
        let m = 10;
        let deb = false;
        let variablelen = true;
        let fixed_seed = true;

        let mut ds: Ds4 = Ds4::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, b);
    }

    #[bench]
    fn test_z_fast_fixed_small(b: &mut Bencher) {
        let t = 1;
        let bits = 40;
        let n = 1000000;
        let m = 1;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds3 = Ds3::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, b);
    }

    #[bench]
    fn test_z_fast_sux_fixed_small(b: &mut Bencher) {
        let t = 1;
        let bits = 40;
        let n = 1000000;
        let m = 1;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds4 = Ds4::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds, b);
    }
}

fn gen_bin_str(rng: &mut SmallRng, n: u32) -> Str {
    let mut s = Str::new();
    for _ in 0..n {
        s.push(rng.next_u32() % 2 == 0);
    }
    s
}

pub fn test<T: Trie>(
    t: u32,
    bits: u32,
    n: u32,
    m: u32,
    deb: bool,
    variablelen: bool,
    fixed_seed: bool,
    ds: &mut T,
    b: &mut Bencher
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

        for _ in 0..m {
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

            b.iter(|| { ds.pred_query(&s1) });
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
    }
}
