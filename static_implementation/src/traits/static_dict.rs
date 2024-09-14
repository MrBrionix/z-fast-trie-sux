use super::hash::ParametricHash;

pub trait StaticDict<K, V, H: ParametricHash<DomainType = K>> {
    type State;

    fn new() -> Self;
    fn build(&mut self, keys: &Vec<K>, values: &Vec<V>);
    fn get(&self, key: &K) -> Option<&V>;
    fn compute_state(&self, key: &K) -> Self::State;
    fn fast_prefix_get(&self, key: &K, state: &Self::State, ind: usize) -> Option<&V>;
}
