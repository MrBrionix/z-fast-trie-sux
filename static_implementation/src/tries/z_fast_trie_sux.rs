use crate::static_dicts::minimal_perfect_hash_static_dict::MinimalPerfectHashStaticDict;
use crate::traits::*;
use crate::utils::*;
use refbox::*;
use std::cmp::min;
use std::cmp::Ordering::*;

pub struct ZFastTrieSux<H: Hash<DomainType = Str> + ParametricHash> {
    root: Option<RefBox<dyn TrieNode>>,
    z_map: MinimalPerfectHashStaticDict<Str, Ref<dyn TrieNode>, H>,
}

trait TrieNode {
    fn is_leaf(&self) -> bool;
    fn get_prev(&self) -> &Option<Ref<LeafTrieNode>>;
    fn get_next(&self) -> &Option<Ref<LeafTrieNode>>;
    fn get_left(&self) -> Option<Ref<dyn TrieNode>>;
    fn get_right(&self) -> Option<Ref<dyn TrieNode>>;
    fn get_leftmost(&self, noderef: Ref<dyn TrieNode>) -> Ref<dyn TrieNode>;
    fn get_rightmost(&self, noderef: Ref<dyn TrieNode>) -> Ref<dyn TrieNode>;
    fn get_jump_left(&self) -> Option<Ref<dyn TrieNode>>;
    fn get_jump_right(&self) -> Option<Ref<dyn TrieNode>>;
    fn set_lind(&mut self, x: usize);
    fn get_lind(&self) -> usize;
    fn get_rind(&self) -> usize;
    fn get_extent(&self) -> Str;
    fn get_prefix_extent(&self, x: usize) -> Str;
    fn get_kth_left(&self, noderef: Ref<dyn TrieNode>, k: usize) -> Ref<dyn TrieNode>;
    fn get_kth_right(&self, noderef: Ref<dyn TrieNode>, k: usize) -> Ref<dyn TrieNode>;
    fn precalc_jumps(&mut self, noderef: Ref<dyn TrieNode>);
    fn get_handle(&self) -> Option<Str>;
    fn precalc_z_map(
        &self,
        keys: &mut Vec<Str>,
        values: &mut Vec<Ref<dyn TrieNode>>,
        r: Ref<dyn TrieNode>
    );
}

struct InternalTrieNode {
    left: RefBox<dyn TrieNode>,
    right: RefBox<dyn TrieNode>,
    lind: usize,
    jump_left: Ref<dyn TrieNode>,
    jump_right: Ref<dyn TrieNode>,
    to_leaf: Ref<LeafTrieNode>,
}

struct LeafTrieNode {
    lind: usize,
    prev: Option<Ref<LeafTrieNode>>,
    next: Option<Ref<LeafTrieNode>>,
    to_internal: Option<Ref<InternalTrieNode>>,
    extent: Str,
}

impl<H: Hash<DomainType = Str> + ParametricHash> Trie for ZFastTrieSux<H> {
    fn build(&mut self, v: &Vec<Str>) {
        let mut x = v.to_vec();
        x.sort_by(cmp);
        self.z_map = MinimalPerfectHashStaticDict::new();
        self.root = ZFastTrieSux::<H>::build_tree(&x, 0, 0, v.len(), &mut None).0;
        if let Some(r) = &self.root {
            let mut keys = Vec::new();
            let mut values = Vec::new();
            r.try_borrow_mut().unwrap().precalc_z_map(&mut keys, &mut values, r.create_ref());
            r.try_borrow_mut().unwrap().precalc_jumps(r.create_ref());
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
            i.len() == x.len() && get_substr(i,0,min(i.len(), x.len())) == *x
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
        last_leaf_ref: &mut Option<Ref<LeafTrieNode>>
    ) -> (Option<RefBox<dyn TrieNode>>, Option<Ref<LeafTrieNode>>) {
        assert!(ind < v[l].len() || l + 1 >= r, "Build error: v is not prefix free");
        if l == r {
            (None, None)
        } else if l + 1 == r {
            let leaf = RefBox::new(LeafTrieNode {
                lind: ind,
                prev: last_leaf_ref.clone(),
                next: None,
                to_internal: None,
                extent: v[l].clone(),
            });

            let leafp = leaf.create_ref();
            if let Some(x) = last_leaf_ref {
                x.try_borrow_mut().unwrap().next = Some(leafp.clone());
            }
            *last_leaf_ref = Some(leafp.clone());

            (Some(coerce!(leaf => dyn TrieNode)), Some(leafp))
        } else {
            let mut mid = l;
            while mid < r && !v[mid][ind] {
                mid += 1;
            }

            if mid != l && mid != r {
                let (l, pl) = ZFastTrieSux::<H>::build_tree(v, ind + 1, l, mid, last_leaf_ref);
                let (r, pr) = ZFastTrieSux::<H>::build_tree(v, ind + 1, mid, r, last_leaf_ref);

                let tmpl = l.expect("left child in ZFastTrieSux build");
                let tmpr = r.expect("right child in ZFastTrieSux build");
                let res = RefBox::new(InternalTrieNode {
                    jump_left: tmpl.create_ref(),
                    jump_right: tmpr.create_ref(),
                    left: tmpl,
                    right: tmpr,
                    lind: ind,
                    to_leaf: pl.clone().expect("to leaf pointer in ZFastTrieSux build"),
                });
                pl.expect("ok").try_borrow_mut().unwrap().to_internal = Some(res.create_ref());

                (Some(coerce!(res => dyn TrieNode)), pr)
            } else {
                let (x, y) = ZFastTrieSux::<H>::build_tree(v, ind + 1, l, r, last_leaf_ref);
                assert!(x.is_some());
                let res = x.unwrap();
                res.try_borrow_mut().unwrap().set_lind(ind);
                (Some(res), y)
            }
        }
    }

    fn locate_exit_or_parex_prob(&self, x: &Str) -> Option<Ref<dyn TrieNode>> {
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
                    let rind = node_ref.try_borrow_mut().unwrap().get_rind();
                    let lind = node_ref.try_borrow_mut().unwrap().get_lind();
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

    fn locate_parex(&self, x: &Str) -> Option<Ref<dyn TrieNode>> {
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
                    let extent = node_ref.try_borrow_mut().unwrap().get_extent();
                    let rind = node_ref.try_borrow_mut().unwrap().get_rind();
                    let lind = node_ref.try_borrow_mut().unwrap().get_lind();
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

    fn locate_exit_or_parex(&self, x: &Str) -> Option<Ref<dyn TrieNode>> {
        let mut res = self.locate_exit_or_parex_prob(x);
        let handle = {
            if res.is_some() {
                res.as_ref().unwrap().try_borrow_mut().unwrap().get_handle().unwrap()
            } else {
                Str::new(0)
            }
        };

        if handle != get_substr(x,0,min(x.len(), handle.len())) {
            res = self.locate_parex(x);
        }
        return res;
    }

    fn locate_exit_from_node(
        &self,
        x: &Str,
        sigma: Option<Ref<dyn TrieNode>>
    ) -> Option<Ref<dyn TrieNode>> {
        if let Some(node_ref) = &sigma {
            let node = node_ref.try_borrow_mut().unwrap();
            let extent = node.get_extent();

            if extent.len() < x.len() && extent == get_substr(x,0,min(extent.len(), x.len())) {
                if !x[extent.len()] {
                    assert!(node.get_left().is_some());
                    Some(node.get_left().unwrap().clone())
                } else {
                    assert!(node.get_right().is_some());
                    Some(node.get_right().unwrap().clone())
                }
            } else {
                sigma.clone()
            }
        } else {
            assert!(self.root.is_some());
            let r = self.root.as_ref().unwrap();
            Some(r.create_ref())
        }
    }

    fn locate_exit(&self, x: &Str) -> Option<Ref<dyn TrieNode>> {
        if !self.root.is_some() {
            None
        } else {
            let res = self.locate_exit_or_parex(x);
            let eta = {
                if res.is_some() {
                    self.locate_exit_from_node(x, res)
                } else {
                    Some(self.root.as_ref().unwrap().create_ref())
                }
            };
            eta.clone()
        }
    }

    fn query(&self, x: &Str) -> (Option<Str>, Option<Str>) {
        if let Some(eta) = self.locate_exit(x) {
            let (mut prev, mut succ) = (None, None);

            let leaf;
            if cmp(x,&eta.try_borrow_mut().unwrap().get_extent()) != Greater {
                leaf = eta.try_borrow_mut().unwrap().get_leftmost(eta.clone());
                succ = Some(leaf.try_borrow_mut().unwrap().get_extent());

                if let Some(leaf2) = &leaf.try_borrow_mut().unwrap().get_prev() {
                    prev = Some(leaf2.try_borrow_mut().unwrap().get_extent());
                }
            } else {
                leaf = eta.try_borrow_mut().unwrap().get_rightmost(eta.clone());
                prev = Some(leaf.try_borrow_mut().unwrap().get_extent());

                if let Some(leaf2) = &leaf.try_borrow_mut().unwrap().get_next() {
                    succ = Some(leaf2.try_borrow_mut().unwrap().get_extent());
                }
            }

            (prev, succ)
        } else {
            (None, None)
        }
    }

    fn pref_query(&self, x: &Str) -> Option<Str> {
        if let Some(exit_node) = self.locate_exit(x) {
            Some(exit_node.try_borrow_mut().unwrap().get_prefix_extent(x.len()))
        } else {
            None
        }
    }

    fn is_nonempty(&self, x: &Str, y: &Str) -> bool {
        if x == y {
            return false;
        } else if let Some(mut alpha) = self.locate_exit(x) {
            if cmp(x,&alpha.try_borrow_mut().unwrap().get_extent()) != Greater {
                while !alpha.try_borrow_mut().unwrap().is_leaf() {
                    if alpha.try_borrow_mut().unwrap().get_extent().len() < y.len() {
                        let tmp = alpha
                            .try_borrow_mut()
                            .unwrap()
                            .get_jump_left()
                            .clone()
                            .unwrap()
                            .clone();
                        alpha = tmp;
                    } else {
                        break;
                    }
                }
                return cmp(&alpha.try_borrow_mut().unwrap().get_extent(),y) == Less;
            }

            let mut beta = self.locate_exit(y).unwrap();
            if cmp(x,&beta.try_borrow_mut().unwrap().get_extent()) == Greater {
                while !beta.try_borrow_mut().unwrap().is_leaf() {
                    if beta.try_borrow_mut().unwrap().get_extent().len() < x.len() {
                        let tmp = beta
                            .try_borrow_mut()
                            .unwrap()
                            .get_jump_right()
                            .clone()
                            .unwrap()
                            .clone();
                        beta = tmp;
                    } else {
                        break;
                    }
                }
                return cmp(x,&beta.try_borrow_mut().unwrap().get_extent()) != Greater;
            }

            let z = lcp(x, y);
            let eta = self.locate_exit(&z).unwrap();

            alpha = eta.try_borrow_mut().unwrap().get_left().unwrap().clone();
            while !alpha.try_borrow_mut().unwrap().is_leaf() {
                if alpha.try_borrow_mut().unwrap().get_extent().len() < x.len() {
                    let tmp = alpha
                        .try_borrow_mut()
                        .unwrap()
                        .get_jump_right()
                        .clone()
                        .unwrap()
                        .clone();
                    alpha = tmp;
                } else {
                    break;
                }
            }
            if cmp(x,&alpha.try_borrow_mut().unwrap().get_extent()) != Greater {
                return true;
            }

            beta = eta.try_borrow_mut().unwrap().get_right().unwrap().clone();
            while !beta.try_borrow_mut().unwrap().is_leaf() {
                if beta.try_borrow_mut().unwrap().get_extent().len() < y.len() {
                    let tmp = beta
                        .try_borrow_mut()
                        .unwrap()
                        .get_jump_left()
                        .clone()
                        .unwrap()
                        .clone();
                    beta = tmp;
                } else {
                    break;
                }
            }

            let candidate = beta.try_borrow_mut().unwrap().get_extent();
            cmp(&candidate,y) == Less
        } else {
            false
        }
    }
}

impl TrieNode for InternalTrieNode {
    fn is_leaf(&self) -> bool {
        false
    }

    fn get_prev(&self) -> &Option<Ref<LeafTrieNode>> {
        &None
    }

    fn get_next(&self) -> &Option<Ref<LeafTrieNode>> {
        &None
    }

    fn get_left(&self) -> Option<Ref<dyn TrieNode>> {
        Some(self.left.create_ref())
    }

    fn get_right(&self) -> Option<Ref<dyn TrieNode>> {
        Some(self.right.create_ref())
    }

    fn get_leftmost(&self, _noderef: Ref<dyn TrieNode>) -> Ref<dyn TrieNode> {
        self.jump_left.try_borrow_mut().unwrap().get_leftmost(self.jump_left.clone())
    }

    fn get_rightmost(&self, _noderef: Ref<dyn TrieNode>) -> Ref<dyn TrieNode> {
        self.jump_right.try_borrow_mut().unwrap().get_rightmost(self.jump_right.clone())
    }

    fn get_jump_left(&self) -> Option<Ref<dyn TrieNode>> {
        Some(self.jump_left.clone())
    }

    fn get_jump_right(&self) -> Option<Ref<dyn TrieNode>> {
        Some(self.jump_right.clone())
    }

    fn set_lind(&mut self, x: usize) {
        self.lind = x;
    }

    fn get_lind(&self) -> usize {
        self.lind
    }

    fn get_rind(&self) -> usize {
        self.left.try_borrow_mut().unwrap().get_lind() - 1
    }

    fn get_extent(&self) -> Str {
        let rind = self.get_rind();

        let p = &self.to_leaf;
        get_substr(&p.try_borrow_mut().unwrap().extent,0,rind)
    }

    fn get_prefix_extent(&self, x: usize) -> Str {
        let rind = min(self.get_rind(), x);

        let p = &self.to_leaf;
        p.try_borrow_mut().unwrap().get_prefix_extent(rind)
    }

    fn get_kth_left(&self, noderef: Ref<dyn TrieNode>, k: usize) -> Ref<dyn TrieNode> {
        let rind = self.get_rind();
        if k >= self.lind && k <= rind {
            noderef
        } else {
            let x = &self.left;
            x.try_borrow_mut().unwrap().get_kth_left(x.create_ref(), k)
        }
    }

    fn get_kth_right(&self, noderef: Ref<dyn TrieNode>, k: usize) -> Ref<dyn TrieNode> {
        let rind = self.get_rind();
        if k >= self.lind && k <= rind {
            noderef
        } else {
            let x = &self.right;
            x.try_borrow_mut().unwrap().get_kth_right(x.create_ref(), k)
        }
    }

    fn precalc_jumps(&mut self, noderef: Ref<dyn TrieNode>) {
        let k = get_fattest(self.get_rind(), self.lind);

        self.jump_left = self.get_kth_left(noderef.clone(), if k == 0 {
            usize::MAX
        } else {
            k + (1 << k.trailing_zeros())
        });

        self.left.try_borrow_mut().unwrap().precalc_jumps(self.left.create_ref());

        self.jump_right = self.get_kth_right(noderef, if k == 0 {
            usize::MAX
        } else {
            k + (1 << k.trailing_zeros())
        });
        self.right.try_borrow_mut().unwrap().precalc_jumps(self.right.create_ref());
    }

    fn get_handle(&self) -> Option<Str> {
        let rind = self.get_rind();

        let p = &self.to_leaf;
        Some(get_substr(&p.try_borrow_mut().unwrap().extent,0,get_fattest(rind, self.lind)))
    }

    fn precalc_z_map(
        &self,
        keys: &mut Vec<Str>,
        values: &mut Vec<Ref<dyn TrieNode>>,
        r: Ref<dyn TrieNode>
    ) {
        let s = self.get_handle();
        assert!(s.is_some());
        keys.push(s.unwrap());
        values.push(r);

        self.left.try_borrow_mut().unwrap().precalc_z_map(keys, values, self.left.create_ref());
        self.right.try_borrow_mut().unwrap().precalc_z_map(keys, values, self.right.create_ref());
    }
}

impl TrieNode for LeafTrieNode {
    fn is_leaf(&self) -> bool {
        true
    }

    fn get_prev(&self) -> &Option<Ref<LeafTrieNode>> {
        &self.prev
    }

    fn get_next(&self) -> &Option<Ref<LeafTrieNode>> {
        &self.next
    }

    fn get_left(&self) -> Option<Ref<dyn TrieNode>> {
        None
    }

    fn get_right(&self) -> Option<Ref<dyn TrieNode>> {
        None
    }

    fn get_leftmost(&self, noderef: Ref<dyn TrieNode>) -> Ref<dyn TrieNode> {
        noderef
    }

    fn get_rightmost(&self, noderef: Ref<dyn TrieNode>) -> Ref<dyn TrieNode> {
        noderef
    }

    fn get_jump_left(&self) -> Option<Ref<dyn TrieNode>> {
        None
    }

    fn get_jump_right(&self) -> Option<Ref<dyn TrieNode>> {
        None
    }

    fn set_lind(&mut self, x: usize) {
        self.lind = x;
    }

    fn get_lind(&self) -> usize {
        self.lind
    }

    fn get_rind(&self) -> usize {
        self.extent.len()
    }

    fn get_extent(&self) -> Str {
        self.extent.clone()
    }

    fn get_prefix_extent(&self, x: usize) -> Str {
        let rind = min(self.get_rind(), x);
        get_substr(&self.extent,0,rind)
    }

    fn get_kth_left(&self, noderef: Ref<dyn TrieNode>, _k: usize) -> Ref<dyn TrieNode> {
        noderef
    }

    fn get_kth_right(&self, noderef: Ref<dyn TrieNode>, _k: usize) -> Ref<dyn TrieNode> {
        noderef
    }

    fn precalc_jumps(&mut self, _noderef: Ref<dyn TrieNode>) {}

    fn get_handle(&self) -> Option<Str> {
        None
    }

    fn precalc_z_map(
        &self,
        _keys: &mut Vec<Str>,
        _values: &mut Vec<Ref<dyn TrieNode>>,
        _r: Ref<dyn TrieNode>
    ) {}
}
