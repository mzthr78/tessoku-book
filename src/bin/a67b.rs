use proconio::{fastout, input};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut heap = BinaryHeap::new();
    
    for i in 0..m {
        heap.push(Reverse((abc[i].2, i)));
    }

    let mut uf = UnionFind::new(n);
    let mut len = 0;

    while !heap.is_empty() {
        let (l, i) = heap.pop().unwrap().0;
        let (a, b, _c) = abc[i];

        if uf.parent(a) != uf.parent(b) {
            len += l;
            uf.unite(a, b);
        }
    }

    println!("{}", len);
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            par: (0..=n).collect(),
            siz: vec![1; n+1],
        }
    }

    fn unite(&mut self, u: usize, v: usize) {
        let pu = self.parent(u);
        let pv = self.parent(v);

        if pu == pv { return; }

        if self.siz[pu] > self.siz[pv] {
            self.par[pv] = pu;
            self.siz[pu] += self.siz[pv];
        } else {
            self.par[pu] = pv;
            self.siz[pv] += self.siz[pu];
        }
    }

    fn parent(&self, n: usize) -> usize {
        if n != self.par[n] {
            return self.parent(self.par[n]);
        } 
        return n;
    }
}
