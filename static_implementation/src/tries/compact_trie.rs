use crate::traits::*;
use crate::utils::str::*;
use std::cmp::min;
use std::cmp::Ordering::*;

pub struct CompactTrie {
    root: Option<Box<TrieNode>>,
}

struct TrieNode {
    s: Str,
    left: Option<Box<TrieNode>>,
    right: Option<Box<TrieNode>>,
}

impl CompactTrie {
    pub fn new() -> CompactTrie {
        CompactTrie { root: None }
    }
}

impl Trie for CompactTrie {
    fn build(&mut self, v: &Vec<Str>) {
        let mut x = v.to_vec();
        x.sort_by(cmp);

        self.root = TrieNode::new(&x, 0, 0, v.len());
    }

    fn pred_query(&self, x: &Str) -> Option<Str> {
        if let Some(r) = &self.root {
            (*r).pred_query(x, &mut Str::new(0), &mut false)
        } else {
            None
        }
    }

    fn succ_query(&self, x: &Str) -> Option<Str> {
        if let Some(r) = &self.root {
            (*r).succ_query(x, &mut Str::new(0), &mut false)
        } else {
            None
        }
    }

    fn ex_pref_query(&self, x: &Str) -> bool {
        if let Some(i) = &self.succ_query(x) { get_substr(i,0,min(i.len(), x.len())) == *x } else { false }
    }

    fn ex_range_query(&self, x: &Str, y: &Str) -> bool {
        if let Some(i) = &self.succ_query(x) { cmp(i,y) == Less } else { false }
    }
}

impl TrieNode {
    fn new(v: &Vec<Str>, ind: usize, l: usize, r: usize) -> Option<Box<TrieNode>> {
        assert!(ind < v[l].len() || l + 1 >= r, "Build error: v is not prefix free");
        if l == r {
            None
        } else if l + 1 == r {
            Some(
                Box::new(TrieNode {
                    s: get_substr(&v[l],ind,v[l].len()),
                    left: None,
                    right: None,
                })
            )
        } else {
            let mut mid = l;
            while mid < r && !v[mid][ind] {
                mid += 1;
            }

            if mid != l && mid != r {
                Some(
                    Box::new(TrieNode {
                        s: Str::new(0),
                        left: TrieNode::new(v, ind + 1, l, mid),
                        right: TrieNode::new(v, ind + 1, mid, r),
                    })
                )
            } else {
                let x = TrieNode::new(v, ind + 1, l, r);
                assert!(x.is_some());
                let mut res = x.unwrap();
                push_front(&mut (*res).s, v[l][ind]);
                Some(res)
            }
        }
    }

    fn upd_curr_pred(x: &Str, curr: &mut Str, found: &mut bool, c: bool) -> bool {
        if !*found {
            if curr.len() == x.len() {
                return false;
            } else if c != x[curr.len()] {
                if c {
                    return false;
                } else {
                    *found = true;
                }
            }
        }
        curr.push(c);
        true
    }

    fn pred_query(&self, x: &Str, curr: &mut Str, found: &mut bool) -> Option<Str> {
        assert!(!(self.left.is_some() ^ self.right.is_some())); // invariante: ogni nodo ha 0 o 2 figli
        for c in &self.s {
            if !Self::upd_curr_pred(x, curr, found, c) {
                return None;
            }
        }
        if *found {
            if let Some(r) = &self.right {
                assert!(Self::upd_curr_pred(x, curr, found, true));
                r.pred_query(x, curr, found)
            } else {
                Some(curr.clone())
            }
        } else {
            if x.len() == curr.len() {
                None
            } else if !x[curr.len()] {
                if let Some(l) = &self.left {
                    assert!(Self::upd_curr_pred(x, curr, found, false));
                    l.pred_query(x, curr, found)
                } else {
                    Some(curr.clone())
                }
            } else {
                if let Some(r) = &self.right {
                    let tmp = &mut curr.clone();
                    assert!(Self::upd_curr_pred(x, tmp, found, true));
                    if let Some(res) = r.pred_query(x, tmp, found) {
                        Some(res)
                    } else {
                        assert!(self.left.is_some());
                        let l = self.left.as_ref().unwrap();
                        assert!(Self::upd_curr_pred(x, curr, found, false));
                        l.pred_query(x, curr, found)
                    }
                } else {
                    Some(curr.clone())
                }
            }
        }
    }

    fn upd_curr_succ(x: &Str, curr: &mut Str, found: &mut bool, c: bool) -> bool {
        if !*found {
            if c != x[curr.len()] {
                if !c {
                    return false;
                } else {
                    *found = true;
                }
            }
        }
        curr.push(c);
        if curr.len() == x.len() {
            *found = true;
        }
        true
    }

    fn succ_query(&self, x: &Str, curr: &mut Str, found: &mut bool) -> Option<Str> {
        assert!(!(self.left.is_some() ^ self.right.is_some())); // invariante: ogni nodo ha 0 o 2 figli
        for c in &self.s {
            if !Self::upd_curr_succ(x, curr, found, c) {
                return None;
            }
        }
        if *found {
            if let Some(l) = &self.left {
                assert!(Self::upd_curr_succ(x, curr, found, false));
                l.succ_query(x, curr, found)
            } else {
                Some(curr.clone())
            }
        } else {
            if x[curr.len()] {
                if let Some(r) = &self.right {
                    assert!(Self::upd_curr_succ(x, curr, found, true));
                    r.succ_query(x, curr, found)
                } else {
                    None
                }
            } else {
                if let Some(l) = &self.left {
                    let tmp = &mut curr.clone();
                    assert!(Self::upd_curr_succ(x, tmp, found, false));
                    if let Some(res) = l.succ_query(x, tmp, found) {
                        Some(res)
                    } else {
                        assert!(self.right.is_some());
                        let r = self.right.as_ref().unwrap();
                        assert!(Self::upd_curr_succ(x, curr, found, true));
                        r.succ_query(x, curr, found)
                    }
                } else {
                    None
                }
            }
        }
    }
}
