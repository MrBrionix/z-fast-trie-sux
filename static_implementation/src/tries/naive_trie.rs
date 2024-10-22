use crate::traits::*;
use crate::utils::str::*;
use std::cmp::min;
use std::cmp::Ordering::*;

pub struct NaiveTrie {
    v: Vec<Str>,
}

impl NaiveTrie {
    pub fn new() -> NaiveTrie {
        NaiveTrie { v: vec![] }
    }
}

impl Trie for NaiveTrie {
    fn build(&mut self, v: &Vec<Str>) {
        self.v = v.to_vec();
        self.v.sort_by(cmp);
    }

    fn pred_query(&self, x: &Str) -> Option<Str> {
        let mut res = None;
        for i in &self.v {
            if cmp(i,x) == Less {
                res = Some(i);
            } else {
                break;
            }
        }
        res.cloned()
    }

    fn succ_query(&self, x: &Str) -> Option<Str> {
        let mut res = None;
        for i in &self.v {
            if cmp(i,x) != Less {
                res = Some(i);
                break;
            }
        }
        res.cloned()
    }

    fn ex_pref_query(&self, x: &Str) -> bool {
        if let Some(i) = &self.succ_query(x) { get_substr(i,0,min(i.len(), x.len())) == *x } else { false }
    }

    fn ex_range_query(&self, x: &Str, y: &Str) -> bool {
        if let Some(i) = &self.succ_query(x) { cmp(i,y) == Less } else { false }
    }
}
