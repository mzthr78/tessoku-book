use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut mf = MaxFlow::new(n, abc);

    mf.dfs(1, n, std::usize::MAX);

    println!("{}", mf.f);
}

struct MaxFlow {
    e: Vec<Vec<usize>>,
    v: Vec<bool>,
    f: usize,
}

impl MaxFlow {
    fn new(n: usize, abc: Vec<(usize, usize, usize)>) -> Self {
        let mut tmp = vec![vec![0; n+1]; n+1];
        for (a, b, c) in abc {
            tmp[a][b] = c;
        }
        Self {
            e: tmp,
            v: vec![false; n+1],
            f: 0,
        }
    }

    fn dfs(&mut self, s: usize, t: usize, f: usize) -> usize {
        if s == t { 
            self.f += f;
            return f; 
        }

        self.v[s] = true;
        for i in 0..self.e[s].len() {
            if self.v[i] { continue; }
            if self.e[s][i] == 0 { continue; }
            let x = self.dfs(i, t, min(self.e[s][i], f));
            
            self.e[s][i] -= x;
            self.e[i][s] += x;

            //println!("from={} to={} flow={}", s, i, x);
        }

        return 0;
    }
}
