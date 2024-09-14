use crate::traits::*;
use crate::utils::str::*;

pub struct NaiveRank;

impl RankStructure for NaiveRank {
    fn new() -> Self {
        NaiveRank {}
    }

    fn build(&mut self, _v0: &Str, _v1: &Str) {}

    fn rank(&self, ind: usize, v0: &Str, v1: &Str) -> usize {
        assert!(ind < v0.len());
        let mut res = 0;
        for i in 0..ind {
            if v0[i] || v1[i] {
                res += 1;
            }
        }
        res
    }
}
