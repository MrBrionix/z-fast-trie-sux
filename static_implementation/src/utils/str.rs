use std::cmp::*;
use std::cmp::Ordering::*;
use sux::prelude::*;
 
pub type Str = BitVec<Vec<usize>>;
pub const WORD_SIZE: usize = 64;

pub fn cmp(x: &Str, y: &Str) -> Ordering {
    let mut currind: usize = 0;
    let mut blockind: usize = 0;
    let refx = x.as_ref();
    let refy = y.as_ref();

    while currind+WORD_SIZE <= min(x.len(),y.len()) {
        if refx[blockind] != refy[blockind] {
            let mut tmp = refx[blockind]^refy[blockind];
            tmp = tmp & ((!tmp) + 1);
            if (tmp & refx[blockind]) != 0 {
                return Greater;
            } else {
                return Less;
            }
        }
        currind+=WORD_SIZE;
        blockind+=1;
    }

    while currind < min(x.len(), y.len()) {
        if x[currind] < y[currind] {
            return Less;
        }
        if x[currind] > y[currind] {
            return Greater;
        }
        currind += 1;
    }
    
    return x.len().cmp(&y.len());
}

pub fn get_substr(x: &Str, start: usize, end: usize) -> Str {
    let mut res : Str = Str::new(0);

    if start==0 {
        let mut currind: usize = 0;
        let mut blockind: usize = 0;
        let refx = x.as_ref();

        while currind+WORD_SIZE <= end {
            currind+=WORD_SIZE;
            push_back(&mut res,refx[blockind]);
            blockind+=1;
        }

        while currind < end {
            res.push(x[currind]);
            currind += 1;
        }   
        res
    }else{
        for i in start..end {
           res.push(x[i]);
        }
        res
    }
}

pub fn push_back(x: &mut Str, val: usize) {
    let len = x.as_ref().len();
    x.resize(len*WORD_SIZE+WORD_SIZE,false);
    x.as_mut()[len] = val;
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
    let mut blockind: usize = 0;
    let mut res = Str::new(0);

    let refx = x.as_ref();
    let refy = y.as_ref();

    while currind+WORD_SIZE <= min(x.len(),y.len()) && refx[blockind]==refy[blockind] {
        push_back(&mut res,refx[blockind]);
        currind+=WORD_SIZE;
        blockind+=1;
    }

    while currind < min(x.len(), y.len()) && x[currind] == y[currind] {
        res.push(x[currind]);
        currind += 1;
    }
    res
}
