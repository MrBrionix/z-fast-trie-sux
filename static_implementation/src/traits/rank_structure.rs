use crate::utils::str::*;

pub trait RankStructure {
    fn new() -> Self;
    fn build(&mut self, v0: &Str, v1: &Str);
    fn rank(&self, ind: usize, v0: &Str, v1: &Str) -> usize;
}
