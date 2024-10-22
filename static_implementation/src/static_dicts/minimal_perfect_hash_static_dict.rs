use crate::traits::*;
use crate::rank_structures::*;
use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;
use sux::prelude::*;

type RankDS = JacobsonRank;
const GAMMA: f64 = 1.23;

pub struct MinimalPerfectHashStaticDict<K, V, H: ParametricHash<DomainType = K>> {
    size: usize,
    h: [H; 3],
    table: Vec<V>,
    w0: BitVec<Vec<usize>>,
    w1: BitVec<Vec<usize>>,
    ds: RankDS,
}

impl<K, V: Clone, H: ParametricHash<DomainType = K>> StaticDict<K, V, H>
for MinimalPerfectHashStaticDict<K, V, H> {
    type State = [H::State; 3];

    fn new() -> MinimalPerfectHashStaticDict<K, V, H> {
        MinimalPerfectHashStaticDict::<K, V, H> {
            size: 0,
            h: [H::new(), H::new(), H::new()],
            table: vec![],
            w0: BitVec::new(0),
            w1: BitVec::new(0),
            ds: RankDS::new(),
        }
    }

    fn build(&mut self, keys: &Vec<K>, values: &Vec<V>) {
        assert!(keys.len() == values.len());        
        self.size = (GAMMA * (keys.len() as f64)) as usize;
        let n = max(self.size, 101);

        let mut cc = 0;
        loop {
            for i in &mut self.h {
                *i = H::new_parametric(n, cc);
                cc += 2;
            }

            let mut edgelists = vec![HashSet::<[usize;3]>::new();n];
            let get_edge = |x: &K| { [self.h[0].hash(x), self.h[1].hash(x), self.h[2].hash(x)] };
            let mut ok = true;
            for i in keys {
                let edge = get_edge(i);
                if edge[0] == edge[1] || edge[0] == edge[2] || edge[1] == edge[2] {
                    ok = false;
                    break;
                }
                for j in edge {
                    edgelists[j].insert(edge);
                }
            }
            if !ok {
                continue;
            }

            let mut q = VecDeque::new();
            for i in 0..n {
                if edgelists[i].len() == 1 {
                    q.push_back(i);
                }
            }

            let mut peeling_sequence = Vec::new();
            let mut cont = 0;
            while !q.is_empty() {
                let node = q.pop_front().unwrap();
                if edgelists[node].len() == 0 {
                    continue;
                }
                assert!(edgelists[node].len() == 1);
                let x = edgelists[node].iter().next().unwrap().clone();
                let ind = {
                    if x[0] == node { 0 } else if x[1] == node { 1 } else { 2 }
                };
                peeling_sequence.push((ind, x));
                for i in x {
                    assert!(edgelists[i].remove(&x) == true);
                    if edgelists[i].len() == 1 {
                        q.push_back(i);
                    }
                }
                cont += 1;
            }
            if cont < keys.len() {
                continue;
            }

            self.w0.resize(n, false);
            self.w1.resize(n, false);
            for (i, j) in peeling_sequence.iter().rev() {
                let mut sum = 0;
                for k in j {
                    sum += self.get_w(*k);

                    if sum >= 3 {
                        sum -= 3;
                    }
                }
                if sum > 0 {
                    sum = 3 - sum;
                }
                sum += i;
                if sum >= 3 {
                    sum -= 3;
                }
                self.set_w(j[*i], sum);
            }

            self.ds.build(&self.w0, &self.w1);
            self.table.resize(n, values[0].clone());
            for i in 0..keys.len() {
                let mut sum = 0;
                let key = &keys[i];
                for j in &self.h {
                    sum += self.get_w(j.hash(key));
                    if sum >= 3 {
                        sum -= 3;
                    }
                }
                let ind = self.ds.rank(self.h[sum].hash(key), &self.w0, &self.w1);
                self.table[ind] = values[i].clone();
            }
            break;
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut res = 0;
        for i in &self.h {
            res += self.get_w(i.hash(key));
            if res >= 3 {
                res -= 3;
            }
        }

        let pos = self.ds.rank(self.h[res].hash(key), &self.w0, &self.w1);
        Some(&self.table[pos])
    }

    fn compute_state(&self, key: &K) -> Self::State {
        [self.h[0].compute_state(key), self.h[1].compute_state(key), self.h[2].compute_state(key)]
    }

    fn fast_prefix_get(&self, key: &K, state: &Self::State, ind: usize) -> Option<&V> {
        let mut res = 0;
        for i in 0..3 {
            res += self.get_w(self.h[i].fast_prefix_hash(key, &state[i], ind));
            if res >= 3 {
                res -= 3;
            }
        }

        let pos = self.ds.rank(
            self.h[res].fast_prefix_hash(key, &state[res], ind),
            &self.w0,
            &self.w1
        );
        Some(&self.table[pos])
    }
}

impl<K, V, H: ParametricHash<DomainType = K>> MinimalPerfectHashStaticDict<K, V, H> {
    fn get_w(&self, ind: usize) -> usize {
        let x = (self.w0[ind] as usize) + 2 * (self.w1[ind] as usize);
        if x == 0 {
            0
        } else {
            x - 1
        }
    }

    fn set_w(&mut self, ind: usize, val: usize) {
        assert!(val < 3);
        self.w0.set(ind, (val + 1) % 2 == 1);
        self.w1.set(ind, val + 1 >= 2);
    }
}
