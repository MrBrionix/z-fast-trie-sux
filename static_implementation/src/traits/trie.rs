use crate::utils::str::*;

pub trait Trie {
    fn build(&mut self, v: &Vec<Str>);
    fn pred_query(&self, x: &Str) -> Option<Str>;
    fn succ_query(&self, x: &Str) -> Option<Str>;
    fn ex_pref_query(&self, x: &Str) -> bool;
    fn ex_range_query(&self, x: &Str, y: &Str) -> bool;
}
