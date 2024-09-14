use crate::traits::*;
use crate::static_dicts::minimal_perfect_hash_static_dict::MinimalPerfectHashStaticDict;
use crate::utils::*;
use std::cmp::min;

pub struct ZFastTrieSux<H: Hash<DomainType = Str> + ParametricHash> {
    root: Option<Ptr<dyn TrieNode>>,
    z_map: MinimalPerfectHashStaticDict<Str, Ptr<dyn TrieNode>, H>,
}

trait TrieNode {
    fn is_leaf(&self) -> bool;
    fn get_prev(&self) -> &Option<Ptr<LeafTrieNode>>;
    fn get_next(&self) -> &Option<Ptr<LeafTrieNode>>;
    fn get_left(&self) -> &Option<Ptr<dyn TrieNode>>;
    fn get_right(&self) -> &Option<Ptr<dyn TrieNode>>;
    fn get_leftmost(&self, noderef: Ptr<dyn TrieNode>) -> Ptr<dyn TrieNode>;
    fn get_rightmost(&self, noderef: Ptr<dyn TrieNode>) -> Ptr<dyn TrieNode>;
    fn get_jump_left(&self) -> &Option<Ptr<dyn TrieNode>>;
    fn get_jump_right(&self) -> &Option<Ptr<dyn TrieNode>>;
    fn set_lind(&mut self, x: usize);
    fn get_lind(&self) -> usize;
    fn get_rind(&self) -> usize;
    fn get_extent(&self) -> Str;
    fn get_prefix_extent(&self, x: usize) -> Str;
    fn get_kth_left(&self, noderef: Ptr<dyn TrieNode>, k: usize) -> Ptr<dyn TrieNode>;
    fn get_kth_right(&self, noderef: Ptr<dyn TrieNode>, k: usize) -> Ptr<dyn TrieNode>;
    fn precalc_jumps(&mut self, noderef: Ptr<dyn TrieNode>);
    fn get_handle(&self) -> Option<Str>;
    fn precalc_z_map(
        &self,
        keys: &mut Vec<Str>,
        values: &mut Vec<Ptr<dyn TrieNode>>,
        r: Ptr<dyn TrieNode>
    );
}

struct InternalTrieNode {
    left: Option<Ptr<dyn TrieNode>>,
    right: Option<Ptr<dyn TrieNode>>,
    lind: usize,
    jump_left: Option<Ptr<dyn TrieNode>>,
    jump_right: Option<Ptr<dyn TrieNode>>,
    to_leaf: Option<Ptr<LeafTrieNode>>,
}

struct LeafTrieNode {
    lind: usize,
    prev: Option<Ptr<LeafTrieNode>>,
    next: Option<Ptr<LeafTrieNode>>,
    to_internal: Option<Ptr<InternalTrieNode>>,
    extent: Option<Str>,
}

impl<H: Hash<DomainType = Str> + ParametricHash> Trie for ZFastTrieSux<H> {
    fn build(&mut self, v: &Vec<Str>) {
        let mut x = v.to_vec();
        x.sort();
        self.z_map = MinimalPerfectHashStaticDict::new();
        self.root = ZFastTrieSux::<H>::build_tree(&x, 0, 0, v.len(), &mut None).0;
        if let Some(r) = &self.root {
            let mut keys = Vec::new();
            let mut values = Vec::new();
            r.borrow().precalc_z_map(&mut keys, &mut values, copy_ptr(&r));
            r.borrow_mut().precalc_jumps(copy_ptr(&r));
            self.z_map.build(&keys, &values);
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
            i.len() == x.len() && i[0..min(i.len(), x.len())] == *x
        } else {
            false
        }
    }

    fn ex_range_query(&self, x: &Str, y: &Str) -> bool {
        self.is_nonempty(x, y)
    }
}

impl<H: Hash<DomainType = Str> + ParametricHash> ZFastTrieSux<H> {
    pub fn new() -> ZFastTrieSux<H> {
        ZFastTrieSux::<H> { root: None, z_map: MinimalPerfectHashStaticDict::new() }
    }

    fn build_tree(
        v: &Vec<Str>,
        ind: usize,
        l: usize,
        r: usize,
        last_leaf_ref: &mut Option<Ptr<LeafTrieNode>>
    ) -> (Option<Ptr<dyn TrieNode>>, Option<Ptr<LeafTrieNode>>) {
        assert!(ind < v[l].len() || l + 1 >= r, "Build error: v is not prefix free");
        if l == r {
            (None, None)
        } else if l + 1 == r {
            let leaf = new_ptr(LeafTrieNode {
                lind: ind,
                prev: last_leaf_ref.clone(),
                next: None,
                to_internal: None,
                extent: Some(v[l].clone()),
            });

            let leafp = copy_ptr(&leaf);
            if let Some(x) = last_leaf_ref {
                x.borrow_mut().next = Some(leafp.clone());
            }
            *last_leaf_ref = Some(leafp.clone());

            (Some(leaf), Some(leafp))
        } else {
            let mut mid = l;
            while mid < r && !v[mid][ind] {
                mid += 1;
            }

            if mid != l && mid != r {
                let (l, pl) = ZFastTrieSux::<H>::build_tree(v, ind + 1, l, mid, last_leaf_ref);
                let (r, pr) = ZFastTrieSux::<H>::build_tree(v, ind + 1, mid, r, last_leaf_ref);

                let res = new_ptr(InternalTrieNode {
                    left: l,
                    right: r,
                    lind: ind,
                    jump_left: None,
                    jump_right: None,
                    to_leaf: pl,
                });

                if let Some(p) = &res.borrow().to_leaf {
                    p.borrow_mut().to_internal = Some(copy_ptr(&res));
                }

                (Some(res), pr)
            } else {
                let (x, y) = ZFastTrieSux::<H>::build_tree(v, ind + 1, l, r, last_leaf_ref);
                assert!(x.is_some());
                let res = x.unwrap();
                res.borrow_mut().set_lind(ind);
                (Some(res), y)
            }
        }
    }

    fn locate_exit_or_parex_prob(&self, x: &Str) -> Option<Ptr<dyn TrieNode>> {
        let mut res = None;
        let (mut a, mut b) = (0, x.len());
        let mut m = {
            if a == 0 { calc(b) } else { calc((a - 1) ^ b) }
        };
        let state = self.z_map.compute_state(x);

        while a <= b {
            if a == 0 || (m & (a - 1)) != (m & b) {
                let f = {
                    if a == 0 { 0 } else { m & b }
                };

                let beta = self.z_map.fast_prefix_get(x, &state, f);
                if let Some(node_ref) = beta {
                    let rind = node_ref.borrow().get_rind();
                    let lind = node_ref.borrow().get_lind();
                    if get_fattest(rind, lind) == f {
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

    fn locate_parex(&self, x: &Str) -> Option<Ptr<dyn TrieNode>> {
        let mut res = None;
        let (mut a, mut b) = (0, x.len());
        let mut m = {
            if a == 0 { calc(b) } else { calc((a - 1) ^ b) }
        };
        let state = self.z_map.compute_state(x);

        while a <= b {
            if a == 0 || (m & (a - 1)) != (m & b) {
                let f = {
                    if a == 0 { 0 } else { m & b }
                };

                let beta = self.z_map.fast_prefix_get(x, &state, f);
                if let Some(node_ref) = beta {
                    let extent = node_ref.borrow().get_extent();
                    let rind = node_ref.borrow().get_rind();
                    let lind = node_ref.borrow().get_lind();
                    if
                        extent.len() < x.len() &&
                        get_fattest(rind, lind) == f &&
                        extent == x[0..extent.len()]
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

    fn locate_exit_or_parex(&self, x: &Str) -> Option<Ptr<dyn TrieNode>> {
        let mut res = self.locate_exit_or_parex_prob(x);
        let handle = {
            if res.is_some() {
                res.as_ref().unwrap().borrow().get_handle().unwrap()
            } else {
                Str::new()
            }
        };

        if handle != x[0..min(x.len(), handle.len())] {
            res = self.locate_parex(x);
        }
        return res;
    }

    fn locate_exit_from_node(
        &self,
        x: &Str,
        sigma: Option<Ptr<dyn TrieNode>>
    ) -> Option<Ptr<dyn TrieNode>> {
        if let Some(node_ref) = &sigma {
            let node = node_ref.borrow();
            let extent = node.get_extent();

            if extent.len() < x.len() && extent == x[0..min(extent.len(), x.len())] {
                if !x[extent.len()] {
                    assert!(node.get_left().is_some());
                    Some(copy_ptr(&node.get_left().as_ref().unwrap()))
                } else {
                    assert!(node.get_right().is_some());
                    Some(copy_ptr(&node.get_right().as_ref().unwrap()))
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

    fn locate_exit(&self, x: &Str) -> Option<Ptr<dyn TrieNode>> {
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
            if *x <= eta.borrow().get_extent() {
                leaf = eta.borrow().get_leftmost(eta.clone());
                succ = Some(leaf.borrow().get_extent());

                if let Some(leaf2) = &leaf.borrow().get_prev() {
                    prev = Some(leaf2.borrow().get_extent());
                }
            } else {
                leaf = eta.borrow().get_rightmost(eta.clone());
                prev = Some(leaf.borrow().get_extent());

                if let Some(leaf2) = &leaf.borrow().get_next() {
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
            return false;
        } else if let Some(mut alpha) = self.locate_exit(x) {
            if *x <= alpha.borrow().get_extent() {
                while !alpha.borrow().is_leaf() {
                    if alpha.borrow().get_extent().len() < y.len() {
                        let tmp = alpha.borrow().get_jump_left().clone().unwrap();
                        alpha = tmp;
                    } else {
                        break;
                    }
                }
                return alpha.borrow().get_extent() < *y;
            }

            let mut beta = self.locate_exit(y).unwrap();
            if *x > beta.borrow().get_extent() {
                while !beta.borrow().is_leaf() {
                    if beta.borrow().get_extent().len() < x.len() {
                        let tmp = beta.borrow().get_jump_right().clone().unwrap();
                        beta = tmp;
                    } else {
                        break;
                    }
                }
                return *x <= beta.borrow().get_extent();
            }

            let z = lcp(x, y);
            let eta = self.locate_exit(&z).unwrap();

            alpha = copy_ptr(&eta.borrow().get_left().as_ref().unwrap());
            while !alpha.borrow().is_leaf() {
                if alpha.borrow().get_extent().len() < x.len() {
                    let tmp = alpha.borrow().get_jump_right().clone().unwrap();
                    alpha = tmp;
                } else {
                    break;
                }
            }
            if *x <= alpha.borrow().get_extent() {
                return true;
            }

            beta = copy_ptr(&eta.borrow().get_right().as_ref().unwrap());
            while !beta.borrow().is_leaf() {
                if beta.borrow().get_extent().len() < y.len() {
                    let tmp = beta.borrow().get_jump_left().clone().unwrap();
                    beta = tmp;
                } else {
                    break;
                }
            }

            let candidate = beta.borrow().get_extent();
            candidate < *y
        } else {
            false
        }
    }
}

impl TrieNode for InternalTrieNode {
    fn is_leaf(&self) -> bool {
        false
    }

    fn get_prev(&self) -> &Option<Ptr<LeafTrieNode>> {
        &None
    }

    fn get_next(&self) -> &Option<Ptr<LeafTrieNode>> {
        &None
    }

    fn get_left(&self) -> &Option<Ptr<dyn TrieNode>> {
        &self.left
    }

    fn get_right(&self) -> &Option<Ptr<dyn TrieNode>> {
        &self.right
    }

    fn get_leftmost(&self, _noderef: Ptr<dyn TrieNode>) -> Ptr<dyn TrieNode> {
        self.jump_left
            .as_ref()
            .unwrap()
            .borrow()
            .get_leftmost(self.jump_left.as_ref().unwrap().clone())
    }

    fn get_rightmost(&self, _noderef: Ptr<dyn TrieNode>) -> Ptr<dyn TrieNode> {
        self.jump_right
            .as_ref()
            .unwrap()
            .borrow()
            .get_rightmost(self.jump_right.as_ref().unwrap().clone())
    }

    fn get_jump_left(&self) -> &Option<Ptr<dyn TrieNode>> {
        &self.jump_left
    }

    fn get_jump_right(&self) -> &Option<Ptr<dyn TrieNode>> {
        &self.jump_right
    }

    fn set_lind(&mut self, x: usize) {
        self.lind = x;
    }

    fn get_lind(&self) -> usize {
        self.lind
    }

    fn get_rind(&self) -> usize {
        assert!(self.left.is_some());
        self.left.as_ref().unwrap().borrow().get_lind() - 1
    }

    fn get_extent(&self) -> Str {
        let rind = self.get_rind();
        assert!(self.to_leaf.is_some());
        let p = self.to_leaf.as_ref().unwrap();
        assert!(p.borrow().extent.is_some());
        p.borrow().extent.as_ref().unwrap()[0..rind].to_bitvec()
    }

    fn get_prefix_extent(&self, x: usize) -> Str {
        let rind = min(self.get_rind(), x);

        assert!(self.to_leaf.is_some());
        let p = self.to_leaf.as_ref().unwrap();
        p.borrow().get_prefix_extent(rind)
    }

    fn get_kth_left(&self, noderef: Ptr<dyn TrieNode>, k: usize) -> Ptr<dyn TrieNode> {
        let rind = self.get_rind();
        if k >= self.lind && k <= rind {
            noderef
        } else {
            assert!(self.left.is_some());
            let x = self.left.as_ref().unwrap();
            x.borrow().get_kth_left(copy_ptr(&x), k)
        }
    }

    fn get_kth_right(&self, noderef: Ptr<dyn TrieNode>, k: usize) -> Ptr<dyn TrieNode> {
        let rind = self.get_rind();
        if k >= self.lind && k <= rind {
            noderef
        } else {
            assert!(self.right.is_some());
            let x = self.right.as_ref().unwrap();
            x.borrow().get_kth_right(copy_ptr(&x), k)
        }
    }

    fn precalc_jumps(&mut self, noderef: Ptr<dyn TrieNode>) {
        let k = get_fattest(self.get_rind(), self.lind);
        if let Some(x) = &self.left {
            self.jump_left = Some(
                self.get_kth_left(noderef.clone(), if k == 0 {
                    usize::MAX
                } else {
                    k + (1 << k.trailing_zeros())
                })
            );
            x.borrow_mut().precalc_jumps(copy_ptr(&x));
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
        assert!(self.to_leaf.is_some());
        let p = self.to_leaf.as_ref().unwrap();
        assert!(p.borrow().extent.is_some());
        Some(p.borrow().extent.as_ref().unwrap()[0..get_fattest(rind, self.lind)].to_bitvec())
    }

    fn precalc_z_map(
        &self,
        keys: &mut Vec<Str>,
        values: &mut Vec<Ptr<dyn TrieNode>>,
        r: Ptr<dyn TrieNode>
    ) {
        let s = self.get_handle();
        assert!(s.is_some());
        keys.push(s.unwrap());
        values.push(r);

        if let Some(x) = &self.left {
            x.borrow().precalc_z_map(keys, values, copy_ptr(&x));
        }
        if let Some(x) = &self.right {
            x.borrow().precalc_z_map(keys, values, copy_ptr(&x));
        }
    }
}

impl TrieNode for LeafTrieNode {
    fn is_leaf(&self) -> bool {
        true
    }

    fn get_prev(&self) -> &Option<Ptr<LeafTrieNode>> {
        &self.prev
    }

    fn get_next(&self) -> &Option<Ptr<LeafTrieNode>> {
        &self.next
    }

    fn get_left(&self) -> &Option<Ptr<dyn TrieNode>> {
        &None
    }

    fn get_right(&self) -> &Option<Ptr<dyn TrieNode>> {
        &None
    }

    fn get_leftmost(&self, noderef: Ptr<dyn TrieNode>) -> Ptr<dyn TrieNode> {
        noderef
    }

    fn get_rightmost(&self, noderef: Ptr<dyn TrieNode>) -> Ptr<dyn TrieNode> {
        noderef
    }

    fn get_jump_left(&self) -> &Option<Ptr<dyn TrieNode>> {
        &None
    }

    fn get_jump_right(&self) -> &Option<Ptr<dyn TrieNode>> {
        &None
    }

    fn set_lind(&mut self, x: usize) {
        self.lind = x;
    }

    fn get_lind(&self) -> usize {
        self.lind
    }

    fn get_rind(&self) -> usize {
        assert!(self.extent.is_some());
        self.extent.as_ref().unwrap().len()
    }

    fn get_extent(&self) -> Str {
        assert!(self.extent.is_some());
        self.extent.as_ref().unwrap().clone()
    }

    fn get_prefix_extent(&self, x: usize) -> Str {
        let rind = min(self.get_rind(), x);

        assert!(self.extent.is_some());
        self.extent.as_ref().unwrap()[0..rind].to_bitvec()
    }

    fn get_kth_left(&self, noderef: Ptr<dyn TrieNode>, _k: usize) -> Ptr<dyn TrieNode> {
        noderef
    }

    fn get_kth_right(&self, noderef: Ptr<dyn TrieNode>, _k: usize) -> Ptr<dyn TrieNode> {
        noderef
    }

    fn precalc_jumps(&mut self, _noderef: Ptr<dyn TrieNode>) {}

    fn get_handle(&self) -> Option<Str> {
        None
    }

    fn precalc_z_map(
        &self,
        _keys: &mut Vec<Str>,
        _values: &mut Vec<Ptr<dyn TrieNode>>,
        _r: Ptr<dyn TrieNode>
    ) {}
}
