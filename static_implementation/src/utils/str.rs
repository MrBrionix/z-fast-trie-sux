use bitvec::prelude::*;
use std::cmp::*;

pub type Str = BitVec<u32>;
pub const WORD_SIZE: usize = 32;

pub fn lcp(x: &Str, y: &Str) -> Str {
    let mut currind: usize = 0;
    let mut chunkx = x.chunks_exact(WORD_SIZE);
    let mut chunky = y.chunks_exact(WORD_SIZE);
    let mut res = Str::new();
    while currind + WORD_SIZE <= min(x.len(), y.len()) {
        let a = chunkx.next().unwrap();
        let b = chunky.next().unwrap();

        if a == b {
            res.extend_from_bitslice(a);
            currind += WORD_SIZE;
        } else {
            break;
        }
    }

    while currind < min(x.len(), y.len()) && x[currind] == y[currind] {
        res.push(x[currind]);
        currind += 1;
    }
    res
}
