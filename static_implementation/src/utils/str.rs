use std::cmp::*;
use std::cmp::Ordering::*;
use sux::prelude::*;
 
pub type Str = BitVec<Vec<usize>>;
pub const WORD_SIZE: usize = 64;

pub fn cmp(x: &Str, y: &Str) -> Ordering {
    for i in 0..min(x.len(),y.len()) {
        if x[i] < y[i] {
            return Less;
        }
        if x[i] > y[i] {
            return Greater;
        }
    }
    return x.len().cmp(&y.len());
}

pub fn get_substr(x: &Str, start: usize, end: usize) -> Str {
    let mut y : Str = Str::new(0);
    for i in start..end {
        y.push(x[i]);
    }
    y
}

pub fn push_front(x: &mut Str, val: bool) {
    let mut tmp : Str = Str::new(0);
    tmp.push(val);
    for i in 0..sux::traits::BitLength::len(&x) {
        tmp.push(x[i]);
    }
    *x = tmp;
}

pub fn lcp(x: &Str, y: &Str) -> Str {
    let mut currind: usize = 0;
    let mut res = Str::new(0);

    while currind < min(x.len(), y.len()) && x[currind] == y[currind] {
        res.push(x[currind]);
        currind += 1;
    }
    res
}
