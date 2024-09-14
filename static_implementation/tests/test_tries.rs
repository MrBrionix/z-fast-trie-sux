use rand::prelude::*;
use z_fast_trie_static_sux::prelude::*;
use std::cmp::min;
use std::mem::swap;

type Ds1 = NaiveTrie;
type Ds2 = CompactTrie;
type Ds3 = ZFastTrie<RollingHash>;
type Ds4 = ZFastTrieSux<RollingHash>;

#[test]
fn abcd() {
    let t = 1;
    let bits = 30;
    let n = 20;
    let m = 500;
    let deb = true;
    let variablelen = false;
    let fixed_seed = true;
    let mut ds1: Ds3 = Ds3::new();
    let mut ds2: Ds4 = Ds4::new();
    crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
}

#[cfg(test)]
mod cross_tests {
    use crate::*;

    #[test]
    fn test_compact_and_naive_fixed() {
        let t = 5;
        let bits = 100;
        let n = 100;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = false;

        let mut ds1: Ds1 = Ds1::new();
        let mut ds2: Ds2 = Ds2::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }

    #[test]
    fn test_compact_and_naive_variable() {
        let t = 5;
        let bits = 100;
        let n = 1000;
        let m = 5000;
        let deb = false;
        let variablelen = true;
        let fixed_seed = true;

        let mut ds1: Ds1 = Ds1::new();
        let mut ds2: Ds2 = Ds2::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }

    #[test]
    fn test_compact_and_z_fast_variable() {
        let t = 5;
        let bits = 100;
        let n = 1000;
        let m = 5000;
        let deb = false;
        let variablelen = true;
        let fixed_seed = true;

        let mut ds1: Ds2 = Ds2::new();
        let mut ds2: Ds3 = Ds3::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }

    #[test]
    fn test_compact_and_z_fast_fixed() {
        let t = 5;
        let bits = 200;
        let n = 1000;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds1: Ds2 = Ds2::new();
        let mut ds2: Ds3 = Ds3::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }

    #[test]
    fn test_compact_and_z_fast_fixed_small() {
        let t = 5;
        let bits = 30;
        let n = 1000;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds1: Ds2 = Ds2::new();
        let mut ds2: Ds3 = Ds3::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }

    #[test]
    fn test_z_fast_and_z_fast_sux_variable() {
        let t = 5;
        let bits = 200;
        let n = 1000;
        let m = 5000;
        let deb = false;
        let variablelen = true;
        let fixed_seed = true;

        let mut ds1: Ds3 = Ds3::new();
        let mut ds2: Ds4 = Ds4::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }

    #[test]
    fn test_z_fast_and_z_fast_sux_fixed() {
        let t = 5;
        let bits = 200;
        let n = 1000;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds1: Ds3 = Ds3::new();
        let mut ds2: Ds4 = Ds4::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }

    #[test]
    fn test_z_fast_and_z_fast_sux_fixed_small() {
        let t = 5;
        let bits = 30;
        let n = 1000;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds1: Ds3 = Ds3::new();
        let mut ds2: Ds4 = Ds4::new();

        crosstest(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds1, &mut ds2);
    }
}

#[cfg(test)]
mod random_tests {
    use crate::*;

    #[test]
    fn test_compact_fixed() {
        let t = 1;
        let bits = 1000;
        let n = 5000;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds2 = Ds2::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds);
    }

    #[test]
    fn test_z_fast_fixed() {
        let t = 1;
        let bits = 1000;
        let n = 5000;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds3 = Ds3::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds);
    }

    #[test]
    fn test_z_fast_variable() {
        let t = 1;
        let bits = 1000;
        let n = 5000;
        let m = 5000;
        let deb = false;
        let variablelen = true;
        let fixed_seed = true;

        let mut ds: Ds3 = Ds3::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds);
    }

    #[test]
    fn test_z_fast_sux_fixed() {
        let t = 1;
        let bits = 1000;
        let n = 5000;
        let m = 5000;
        let deb = false;
        let variablelen = false;
        let fixed_seed = true;

        let mut ds: Ds4 = Ds4::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds);
    }

    #[test]
    fn test_z_fast_sux_variable() {
        let t = 1;
        let bits = 1000;
        let n = 5000;
        let m = 5000;
        let deb = false;
        let variablelen = true;
        let fixed_seed = true;

        let mut ds: Ds4 = Ds4::new();

        test(t, bits, n, m, deb, variablelen, fixed_seed, &mut ds);
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
    ds: &mut T
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

            let pred = ds.pred_query(&s1);
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
            }
        }
    }
}

pub fn crosstest<T1: Trie, T2: Trie>(
    t: u32,
    bits: u32,
    n: u32,
    m: u32,
    deb: bool,
    variablelen: bool,
    fixed_seed: bool,
    ds1: &mut T1,
    ds2: &mut T2
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

        v.sort();
        if deb {
            print!("genero:\n");
            for i in &v {
                print!("{}\n", i);
            }
            print!("testo:\n");
        }
        ds1.build(&v);
        ds2.build(&v);
		
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

            let pred1 = ds1.pred_query(&s1);
            let pred2 = ds2.pred_query(&s1);
            if deb {
                if let Some(ref t) = pred1 {
                    print!("pred: {}\n", t);
                } else {
                    print!("non ha predecessori\n");
                }
            }
            assert!(pred1 == pred2, "answers (pred) don't match\n {:?}\n {:?}\n", pred1, pred2);

            let succ1 = ds1.succ_query(&s1);
            let succ2 = ds2.succ_query(&s1);
            if deb {
                if let Some(ref t) = succ1 {
                    print!("succ: {}\n", t);
                } else {
                    print!("non ha successori\n");
                }
            }
            assert!(succ1 == succ2, "answers (succ) don't match");

            let flag1 = ds1.ex_range_query(&s1, &s2);
            let flag2 = ds2.ex_range_query(&s1, &s2);
            if deb {
                print!("range query: {}\n", flag1);
                print!("------------\n");
            }
            assert!(flag1 == flag2, "answer (range query) don't match\n {}\n {}\n", flag1, flag2);
        }
    }
}
