use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
        query: [(usize, usize, usize); q],
    }

    let mut uf = UnionFind::new(n);

    for (x, u, v) in query {
        if x == 1 {
            uf.union(u, v);
        } else if x == 2 {
            if uf.parent(u) == uf.parent(v) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
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

    fn union(&mut self, u: usize, v: usize) {
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

    fn parent(&self, mut x: usize) -> usize {
        loop {
            if self.par[x] == x { return x; }
            x = self.par[x];
        }
    }
}
