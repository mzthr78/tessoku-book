use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        mut abc: [(usize, usize, usize); m],
    }

    abc.sort_by_key(|abc| abc.2);

    let mut uf = UnionFind::new(n);
    let mut len = 0;

    for (a, b, c) in abc {
        if uf.parent(a) != uf.parent(b) {
            uf.unite(a, b);
            len += c;
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
