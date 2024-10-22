use crate::traits::*;
use crate::utils::*;
use std::cmp::min;
use std::cmp::Ordering::*;
use std::collections::HashMap;

pub struct ZFastTrie<H: Hash<DomainType = Str>> {
    root: Option<Ptr<TrieNode>>,
    z_map: HashMap<H::HashType, Ptr<TrieNode>>,
}

struct TrieNode {
    left: Option<Ptr<TrieNode>>,
    right: Option<Ptr<TrieNode>>,
    lind: usize,
    jump_left: Option<Ptr<TrieNode>>,
    jump_right: Option<Ptr<TrieNode>>,
    to_leaf: Option<Ptr<TrieNode>>,
    to_internal: Option<Ptr<TrieNode>>,
    extent: Option<Str>,
}

impl<H: Hash<DomainType = Str>> Trie for ZFastTrie<H> {
    fn build(&mut self, v: &Vec<Str>) {
        let mut x = v.to_vec();
        x.sort_by(cmp);
        self.z_map = HashMap::new();
        self.root = ZFastTrie::<H>::build_tree(&x, 0, 0, v.len(), &mut None).0;
        if let Some(r) = &self.root {
            r.borrow().precalc_z_map(&mut self.z_map, &mut H::new(), copy_ptr(&r));
            r.borrow_mut().precalc_jumps(copy_ptr(&r));
        }
    }

    fn pred_query(&self, x: &Str) -> Option<Str> {
        self.query(x).0
    }

    fn succ_query(&self, x: &Str) -> Option<Str> {
        self.query(x).1
    }

    fn ex_pref_query(&self, x: &Str) -> bool {
        if let Some(i) = &self.pref_query(x) {
            i.len() == x.len() && get_substr(i,0,min(i.len(), x.len())) == *x
        } else {
            false
        }
    }

    fn ex_range_query(&self, x: &Str, y: &Str) -> bool {
        self.is_nonempty(x, y)
    }
}

impl<H: Hash<DomainType = Str>> ZFastTrie<H> {
    pub fn new() -> ZFastTrie<H> {
        ZFastTrie::<H> { root: None, z_map: HashMap::new() }
    }

    fn build_tree(
        v: &Vec<Str>,
        ind: usize,
        l: usize,
        r: usize,
        last_leaf_ref: &mut Option<Ptr<TrieNode>>
    ) -> (Option<Ptr<TrieNode>>, Option<Ptr<TrieNode>>) {
        assert!(ind < v[l].len() || l + 1 >= r, "Build error: v is not prefix free");
        if l == r {
            (None, None)
        } else if l + 1 == r {
            let leaf = new_ptr(TrieNode {
                left: None,
                right: None,
                lind: ind,
                jump_left: last_leaf_ref.clone(),
                jump_right: None,
                to_leaf: None,
                to_internal: None,
                extent: Some(v[l].clone()),
            });

            let leafp = copy_ptr(&leaf);
            if let Some(x) = last_leaf_ref {
                x.borrow_mut().jump_right = Some(leafp.clone());
            }
            *last_leaf_ref = Some(leafp.clone());

            (Some(leaf), Some(leafp))
        } else {
            let mut mid = l;
            while mid < r && !v[mid][ind] {
                mid += 1;
            }

            if mid != l && mid != r {
                let (l, pl) = ZFastTrie::<H>::build_tree(v, ind + 1, l, mid, last_leaf_ref);
                let (r, pr) = ZFastTrie::<H>::build_tree(v, ind + 1, mid, r, last_leaf_ref);

                let res = new_ptr(TrieNode {
                    left: l,
                    right: r,
                    lind: ind,
                    jump_left: None,
                    jump_right: None,
                    to_leaf: pl,
                    to_internal: None,
                    extent: None,
                });

                if let Some(p) = &res.borrow().to_leaf {
                    p.borrow_mut().to_internal = Some(copy_ptr(&res));
                }

                (Some(res), pr)
            } else {
                let (x, y) = ZFastTrie::<H>::build_tree(v, ind + 1, l, r, last_leaf_ref);
                assert!(x.is_some());
                let res = x.unwrap();
                res.borrow_mut().lind = ind;
                (Some(res), y)
            }
        }
    }

    fn locate_exit_or_parex_prob(&self, x: &Str) -> Option<Ptr<TrieNode>> {
        let mut res = None;
        let (mut a, mut b) = (0, x.len());
        let hash = H::new();
        let state = hash.compute_state(x);
        let mut m = {
            if a == 0 { calc(b) } else { calc((a - 1) ^ b) }
        };

        while a <= b {
            if a == 0 || (m & (a - 1)) != (m & b) {
                let f = {
                    if a == 0 { 0 } else { m & b }
                };

                let beta = self.z_map.get(&hash.fast_prefix_hash(x, &state, f));
                if let Some(node_ref) = beta {
                    let rind = node_ref.borrow().get_rind();
                    let lind = node_ref.borrow().lind;
                    //check importante: serve per la validità del teorema 5
                    if get_fattest(rind, lind) == f {
                        a = rind + 1;
                        res = Some(node_ref.clone());
                    } else {
                        //check per evitare underflow
                        if f == 0 {
                            break;
                        }
                        b = f - 1;
                    }
                } else {
                    if f == 0 {
                        break;
                    }
                    b = f - 1;
                }
            }
            m = m >> 1;
        }

        res
    }

    fn locate_parex(&self, x: &Str) -> Option<Ptr<TrieNode>> {
        let mut res = None;
        let (mut a, mut b) = (0, x.len());
        let hash = H::new();
        let state = hash.compute_state(x);

        let mut m = {
            if a == 0 { calc(b) } else { calc((a - 1) ^ b) }
        };

        while a <= b {
            if a == 0 || (m & (a - 1)) != (m & b) {
                let f = {
                    if a == 0 { 0 } else { m & b }
                };

                let beta = self.z_map.get(&hash.fast_prefix_hash(x, &state, f));
                if let Some(node_ref) = beta {
                    let extent = node_ref.borrow().get_extent();
                    let rind = node_ref.borrow().get_rind();
                    let lind = node_ref.borrow().lind;
                    if
                        extent.len() < x.len() &&
                        get_fattest(rind, lind) == f &&
                        extent == get_substr(x,0,extent.len())
                    {
                        a = rind + 1;
                        res = Some(node_ref.clone());
                    } else {
                        if f == 0 {
                            break;
                        }
                        b = f - 1;
                    }
                } else {
                    if f == 0 {
                        break;
                    }
                    b = f - 1;
                }
            }
            m = m >> 1;
        }

        res
    }

    fn locate_exit_or_parex(&self, x: &Str) -> Option<Ptr<TrieNode>> {
        let mut res = self.locate_exit_or_parex_prob(x);
        let handle = {
            if res.is_some() {
                res.as_ref().unwrap().borrow().get_handle().unwrap()
            } else {
                Str::new(0)
            }
        };

        if handle != get_substr(x,0,min(x.len(), handle.len())) {
            res = self.locate_parex(x);
        }
        res
    }

    fn locate_exit_from_node(
        &self,
        x: &Str,
        sigma: Option<Ptr<TrieNode>>
    ) -> Option<Ptr<TrieNode>> {
        if let Some(node_ref) = &sigma {
            let node = node_ref.borrow();
            let extent = node.get_extent();

            if extent.len() < x.len() && extent == get_substr(x,0,min(extent.len(), x.len())) {
                if !x[extent.len()] {
                    assert!(node.left.is_some());
                    Some(copy_ptr(&node.left.as_ref().unwrap()))
                } else {
                    assert!(node.right.is_some());
                    Some(copy_ptr(&node.right.as_ref().unwrap()))
                }
            } else {
                sigma.clone()
            }
        } else {
            assert!(self.root.is_some());
            let r = self.root.as_ref().unwrap();
            Some(copy_ptr(&r))
        }
    }

    fn locate_exit(&self, x: &Str) -> Option<Ptr<TrieNode>> {
        if !self.root.is_some() {
            None
        } else {
            let res = self.locate_exit_or_parex(x);
            let eta = {
                if res.is_some() {
                    self.locate_exit_from_node(x, res)
                } else {
                    Some(copy_ptr(&self.root.as_ref().unwrap()))
                }
            };
            eta.clone()
        }
    }

    fn query(&self, x: &Str) -> (Option<Str>, Option<Str>) {
        if let Some(eta) = self.locate_exit(x) {
            let (mut prev, mut succ) = (None, None);

            let leaf;
            if cmp(x , &eta.borrow().get_extent()) != Greater {
                leaf = eta.borrow().get_leftmost(eta.clone());
                succ = Some(leaf.borrow().get_extent());

                if let Some(leaf2) = &leaf.borrow().jump_left {
                    prev = Some(leaf2.borrow().get_extent());
                }
            } else {
                leaf = eta.borrow().get_rightmost(eta.clone());
                prev = Some(leaf.borrow().get_extent());

                if let Some(leaf2) = &leaf.borrow().jump_right {
                    succ = Some(leaf2.borrow().get_extent());
                }
            }

            (prev, succ)
        } else {
            (None, None)
        }
    }

    fn pref_query(&self, x: &Str) -> Option<Str> {
        if let Some(exit_node) = self.locate_exit(x) {
            Some(exit_node.borrow().get_prefix_extent(x.len()))
        } else {
            None
        }
    }

    fn is_nonempty(&self, x: &Str, y: &Str) -> bool {
        if x == y {
            false
        } else if let Some(mut alpha) = self.locate_exit(x) {
            if cmp(x,&alpha.borrow().get_extent()) != Greater {
                while
                    alpha.borrow().to_leaf.is_some() &&
                    alpha.borrow().get_extent().len() < y.len()
                {
                    let tmp = alpha.borrow().jump_left.clone().unwrap();
                    alpha = tmp;
                }
                return cmp(&alpha.borrow().get_extent(),y) == Less;
            }

            let mut beta = self.locate_exit(y).unwrap();
            if cmp(x,&beta.borrow().get_extent()) == Greater {
                while beta.borrow().to_leaf.is_some() && beta.borrow().get_extent().len() < x.len() {
                    let tmp = beta.borrow().jump_right.clone().unwrap();
                    beta = tmp;
                }
                return cmp(x,&beta.borrow().get_extent()) != Greater;
            }

            let z = lcp(x, y);
            let eta = self.locate_exit(&z).unwrap();

            alpha = copy_ptr(&eta.borrow().left.as_ref().unwrap());
            while alpha.borrow().to_leaf.is_some() && alpha.borrow().get_extent().len() < x.len() {
                let tmp = alpha.borrow().jump_right.clone().unwrap();
                alpha = tmp;
            }
            if cmp(x,&alpha.borrow().get_extent()) != Greater {
                return true;
            }

            beta = copy_ptr(&eta.borrow().right.as_ref().unwrap());
            while beta.borrow().to_leaf.is_some() && beta.borrow().get_extent().len() < y.len() {
                let tmp = beta.borrow().jump_left.clone().unwrap();
                beta = tmp;
            }

            let candidate = beta.borrow().get_extent();
            cmp(&candidate,y) == Less
        } else {
            false
        }
    }
}

impl TrieNode {
    fn get_leftmost(&self, noderef: Ptr<TrieNode>) -> Ptr<TrieNode> {
        if !self.extent.is_some() {
            self.jump_left
                .as_ref()
                .unwrap()
                .borrow()
                .get_leftmost(self.jump_left.as_ref().unwrap().clone())
        } else {
            noderef
        }
    }

    fn get_rightmost(&self, noderef: Ptr<TrieNode>) -> Ptr<TrieNode> {
        if !self.extent.is_some() {
            self.jump_right
                .as_ref()
                .unwrap()
                .borrow()
                .get_rightmost(self.jump_right.as_ref().unwrap().clone())
        } else {
            noderef
        }
    }

    fn get_rind(&self) -> usize {
        if let Some(p) = &self.left {
            p.borrow().lind - 1
        } else {
            assert!(self.extent.is_some());
            let s = self.extent.as_ref().unwrap();
            s.len()
        }
    }

    fn get_extent(&self) -> Str {
        let rind = self.get_rind();

        if let Some(p) = &self.to_leaf {
            assert!(p.borrow().extent.is_some());
            get_substr(p.borrow().extent.as_ref().unwrap(),0,rind)
        } else {
            assert!(self.extent.is_some());
            get_substr(self.extent.as_ref().unwrap(),0,rind)
        }
    }

    fn get_prefix_extent(&self, x: usize) -> Str {
        let rind = min(self.get_rind(), x);

        if let Some(p) = &self.to_leaf {
            assert!(p.borrow().extent.is_some());
            let s = get_substr(p.borrow().extent.as_ref().unwrap(),0,rind);
            s
        } else {
            assert!(self.extent.is_some());
            let s = get_substr(self.extent.as_ref().unwrap(),0,rind);
            s
        }
    }

    fn get_kth_left(&self, noderef: Ptr<TrieNode>, k: usize) -> Ptr<TrieNode> {
        let rind = self.get_rind();
        if k >= self.lind && k <= rind {
            noderef
        } else if let Some(x) = &self.left {
            x.borrow().get_kth_left(copy_ptr(&x), k)
        } else {
            noderef
        }
    }

    fn get_kth_right(&self, noderef: Ptr<TrieNode>, k: usize) -> Ptr<TrieNode> {
        let rind = self.get_rind();
        if k >= self.lind && k <= rind {
            noderef
        } else if let Some(x) = &self.right {
            x.borrow().get_kth_right(copy_ptr(&x), k)
        } else {
            noderef
        }
    }

    fn precalc_jumps(&mut self, noderef: Ptr<TrieNode>) {
        let k = get_fattest(self.get_rind(), self.lind);
        if let Some(x) = &self.left {
            self.jump_left = Some(
                self.get_kth_left(noderef.clone(), if k == 0 {
                    usize::MAX
                } else {
                    k + (1 << k.trailing_zeros())
                })
            );
            x.borrow_mut().precalc_jumps(copy_ptr(x));
        }

        if let Some(x) = &self.right {
            self.jump_right = Some(
                self.get_kth_right(noderef, if k == 0 {
                    usize::MAX
                } else {
                    k + (1 << k.trailing_zeros())
                })
            );
            x.borrow_mut().precalc_jumps(copy_ptr(&x));
        }
    }

    fn get_handle(&self) -> Option<Str> {
        let rind = self.get_rind();
        if let Some(p) = &self.to_leaf {
            if let Some(s) = &p.borrow().extent {
                Some(get_substr(s,0,get_fattest(rind, self.lind)))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn precalc_z_map<T: Hash<DomainType = Str>>(
        &self,
        z_map: &mut HashMap<T::HashType, Ptr<TrieNode>>,
        hash: &mut T,
        r: Ptr<TrieNode>
    ) {
        if let Some(s) = &self.get_handle() {
            z_map.insert(hash.slow_prefix_hash(s, s.len()), r);
        }

        if let Some(x) = &self.left {
            x.borrow().precalc_z_map(z_map, hash, copy_ptr(&x));
        }
        if let Some(x) = &self.right {
            x.borrow().precalc_z_map(z_map, hash, copy_ptr(&x));
        }
    }
}
