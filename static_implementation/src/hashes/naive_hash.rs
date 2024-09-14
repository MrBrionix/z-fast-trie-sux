use crate::traits::*;
use crate::utils::str::*;

pub struct NaiveHash;

impl Hash for NaiveHash {
    type DomainType = Str;
    type HashType = Str;
    type State = ();

    fn new() -> Self {
        NaiveHash {}
    }

    fn hash(&self, s: &Self::DomainType) -> Self::HashType {
        self.slow_prefix_hash(s, s.len())
    }

    fn slow_prefix_hash(&self, s: &Self::DomainType, ind: usize) -> Self::HashType {
        s[0..ind].to_bitvec()
    }

    fn compute_state(&self, _s: &Self::DomainType) {}

    fn fast_prefix_hash(
        &self,
        s: &Self::DomainType,
        _state: &Self::State,
        ind: usize
    ) -> Self::HashType {
        self.slow_prefix_hash(s, ind)
    }
}
