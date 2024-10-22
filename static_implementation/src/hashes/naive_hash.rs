use crate::traits::*;
use crate::utils::str::*;

pub struct NaiveHash;

impl Hash for NaiveHash {
    type DomainType = Str;
    type HashType = Vec<usize>;
    type State = ();

    fn new() -> Self {
        NaiveHash {}
    }

    fn hash(&self, s: &Self::DomainType) -> Self::HashType {
        self.slow_prefix_hash(s, s.len())
    }

    fn slow_prefix_hash(&self, s: &Self::DomainType, ind: usize) -> Self::HashType {
        let mut res : Self::HashType = vec![];
        for i in get_substr(s,0,ind).as_ref() {
            res.push(*i);
        }
        res
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
