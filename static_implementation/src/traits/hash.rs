use std::cmp::Eq;

pub trait Hash {
    type DomainType;
    type HashType: Eq + std::hash::Hash;
    type State;

    fn new() -> Self;
    fn hash(&self, s: &Self::DomainType) -> Self::HashType;
    fn slow_prefix_hash(&self, s: &Self::DomainType, ind: usize) -> Self::HashType;
    fn compute_state(&self, s: &Self::DomainType) -> Self::State;
    fn fast_prefix_hash(
        &self,
        s: &Self::DomainType,
        state: &Self::State,
        ind: usize
    ) -> Self::HashType;
}

pub trait ParametricHash: Hash<HashType = usize> {
    fn new_parametric(domain_size: usize, seed: u64) -> Self;
    fn new_random(domain_size: usize) -> Self;
}
