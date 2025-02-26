use crate::traits::*;
use crate::utils::str::*;
use std::cmp::min;
use sux::prelude::*;

pub struct JacobsonRank {
    n: usize,
    block_dim: usize,
    super_block_dim: usize,
    block_num: usize,
    super_block_num: usize,
    block_bits: usize,
    super_block_bits: usize,
    n_bits: usize,
    block_ranks: BitFieldVec<usize>,
    super_block_ranks: BitFieldVec<usize>,
    partial_ranks: Vec<BitFieldVec<usize>>,
}

impl RankStructure for JacobsonRank {
    fn new() -> Self {
        JacobsonRank {
            n: 0,
            block_dim: 0,
            super_block_dim: 0,
            block_num: 0,
            super_block_num: 0,
            block_bits: 0,
            super_block_bits: 0,
            n_bits: 0,
            block_ranks: BitFieldVec::<usize>::new(1, 0),
            super_block_ranks: BitFieldVec::<usize>::new(1, 0),
            partial_ranks: vec![],
        }
    }

    fn build(&mut self, v0: &Str, v1: &Str) {
        assert!(v0.len()==v1.len());
        let mut sequence : Str = Str::new(0);
        for i in 0..v0.len() {
            sequence.push(v0[i]|v1[i]);
        }

        self.n = sequence.len() + 1;
        self.block_dim = (0.5 * (self.n as f64).log2()).ceil() as usize;
        self.super_block_dim = ((self.n as f64).log2() * (self.n as f64).log2()).ceil() as usize;
        while self.super_block_dim % self.block_dim != 0 {
            self.super_block_dim += 1;
        }
        self.block_num = (self.n + self.block_dim - 1) / self.block_dim;
        self.super_block_num = (self.n + self.super_block_dim - 1) / self.super_block_dim;

        let mut t = vec![];
        let mut curr_t = 0;
        let mut tmp = vec![];
        for x in &sequence {
            t.push(curr_t);
            if x {
                curr_t += 1;
                tmp.push(1);
            } else {
                tmp.push(0);
            }
        }
        t.push(curr_t);
        tmp.push(0);

        self.n_bits = (self.n as f64).log2().ceil() as usize;
        self.super_block_ranks = BitFieldVec::<usize>::with_capacity(
            self.n_bits,
            self.super_block_num
        );
        self.super_block_bits = (self.super_block_dim as f64).log2().ceil() as usize;
        self.block_ranks = BitFieldVec::<usize>::with_capacity(
            self.super_block_bits,
            self.block_num
        );
        self.block_bits = (self.block_dim as f64).log2().ceil() as usize;

        for i in 0..self.super_block_num {
            self.super_block_ranks.push(t[i * self.super_block_dim]);
        }
        for i in 0..self.block_num {
            self.block_ranks.push(
                t[i * self.block_dim] -
                    self.super_block_ranks.get((i * self.block_dim) / self.super_block_dim)
            );
        }
        for i in 0..1 << self.block_dim {
            self.partial_ranks.push(Self::compute_ranklist(i, self.block_dim, self.block_bits));
        }
    }

    fn rank(&self, i: usize, v0: &Str, v1: &Str) -> usize {
        (self.super_block_ranks.get(i / self.super_block_dim) +
            self.block_ranks.get(i / self.block_dim) +
            self.partial_ranks[
                Self::get_index(
                    i - (i % self.block_dim),
                    i - (i % self.block_dim) + self.block_dim,
                    v0,
                    v1
                )
            ].get(i % self.block_dim)) as usize
    }
}

impl JacobsonRank {
    fn compute_ranklist(mut x: usize, size: usize, k: usize) -> BitFieldVec<usize> {
        let mut ranklist = BitFieldVec::<usize>::with_capacity(k, size);
        let mut curr_t = 0;
        for _ in 0..size {
            ranklist.push(curr_t);
            if x % 2 == 1 {
                curr_t += 1;
            }
            x /= 2;
        }
        ranklist
    }
    fn get_index(l: usize, r: usize, v0: &Str, v1: &Str) -> usize {
        let mut res : usize = 0;
        for i in (l..min(r, v0.len())).rev() {
            res *= 2;
            if v0[i] | v1[i] {
                res += 1;
            }
        }
        res
    }
}
