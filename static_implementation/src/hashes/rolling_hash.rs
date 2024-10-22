use crate::traits::*;
use crate::utils::str::*;
use rand::prelude::*;

const DEFAULT_MODULO: usize = 1000000000 + 7;
const DEFAULT_BASE: usize = 37;

pub struct RollingHash {
    modulo: usize,
    base: usize,
}

impl Hash for RollingHash {
    type DomainType = Str;
    type HashType = usize;
    type State = (Vec<usize>, Vec<usize>);

    fn new() -> Self {
        RollingHash { base: DEFAULT_BASE, modulo: DEFAULT_MODULO }
    }

    fn hash(&self, s: &Self::DomainType) -> Self::HashType {
        self.slow_prefix_hash(s, s.len())
    }

    fn slow_prefix_hash(&self, s: &Self::DomainType, ind: usize) -> Self::HashType {
        assert!(ind <= s.len());
        let mut res: Self::HashType = self.base;
        let mut pot: Self::HashType = (self.base * self.base) % self.modulo;
        let mut currind: usize = 0;

        for x in s.as_ref() {
            if currind + WORD_SIZE > ind {
                break;
            }
            currind += WORD_SIZE;

            res += (((x + 1) % self.modulo) * pot) % self.modulo;
            res %= self.modulo;

            pot *= self.base;
            pot %= self.modulo;
        }
	
        while currind < ind {
            res += pot;
            res %= self.modulo;
            if s[currind] {
                res += pot;
                res %= self.modulo;
            }

            pot *= self.base;
            pot %= self.modulo;

            currind += 1;
        }

        res
    }

    fn compute_state(&self, s: &Self::DomainType) -> Self::State {
        let mut res: Self::HashType = self.base;
        let mut pot: Self::HashType = (self.base * self.base) % self.modulo;

        let mut v = vec![res];
        let mut pots = vec![pot];

        for x in s.as_ref() {
            res += (((x + 1) % self.modulo) * pot) % self.modulo;
            res %= self.modulo;
            v.push(res);

            pot *= self.base;
            pot %= self.modulo;
            pots.push(pot);
        }

        (v, pots)
    }

    fn fast_prefix_hash(
        &self,
        s: &Self::DomainType,
        state: &Self::State,
        ind: usize
    ) -> Self::HashType {
        let mut res: Self::HashType = state.0[ind / WORD_SIZE];
        let mut pot: Self::HashType = state.1[ind / WORD_SIZE];

        let mut currind = (ind / WORD_SIZE) * WORD_SIZE;

        while currind < ind {
            res += pot;
            if s[currind] {
                res += pot;
            }

            pot *= self.base;
            pot %= self.modulo;

            currind += 1;
        }

        res % self.modulo
    }
}

impl ParametricHash for RollingHash {
    fn new_parametric(domain_size: usize, seed: u64) -> Self {
        let num = ((SmallRng::seed_from_u64(seed).next_u32() as usize) % (domain_size - 1)) + 1;
        RollingHash {
            base: RollingHash::get_coprime(num, domain_size),
            modulo: domain_size,
        }
    }

    fn new_random(domain_size: usize) -> Self {
        let num = ((SmallRng::from_rng(thread_rng()).next_u32() as usize) % (domain_size - 1)) + 1;
        RollingHash {
            base: RollingHash::get_coprime(num, domain_size),
            modulo: domain_size,
        }
    }
}

impl RollingHash {
    fn get_coprime(num: usize, domain_size: usize) -> usize {
        let mut res = num;
        loop {
            let mut x = res;
            let mut y = domain_size;
            while y != 0 {
                let tmp = y;
                y = x % y;
                x = tmp;
            }
            if x == 1 {
                break;
            } else {
                res /= x;
            }
        }
        res
    }
}
