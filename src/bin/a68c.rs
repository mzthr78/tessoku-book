use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }

    let mut mf = MaxFlow::new(n);

    for (a, b, c) in abc {
        mf.e[a][b] = c;
        mf.g[a].push(b);
        mf.g[b].push(a);
    }

    let f = mf.max_flow(n);

    println!("{}", f);
}

struct MaxFlow {
    e: Vec<Vec<usize>>,
    g: Vec<Vec<usize>>,
    v: Vec<bool>,
}

impl MaxFlow {
    fn new(n: usize) -> Self {
        Self {
            e: vec![vec![0; n+1]; n+1],
            g: vec![Vec::new(); n+1],
            v: vec![false; n+1],
        }
    }

    fn dfs(&mut self, s: usize, g: usize, f: usize) -> usize {
        if s == g { return f; }

        self.v[s] = true;

        for i in 0..self.g[s].len() {
            let to = self.g[s][i];
            if self.e[s][to] == 0 { continue; }
            if self.v[to] { continue; }
            
            let flow = self.dfs(to, g, min(f, self.e[s][to]));

            if flow > 0 {
                self.e[s][to] -= flow;
                self.e[to][s] += flow;

                return flow;
            }
        }

        return 0;
    }

    fn max_flow(&mut self, n: usize) -> usize {
        let mut max_flow = 0;

        loop {
            for i in 1..self.v.len() {
                self.v[i] = false;
            }

            let tmp = self.dfs(1, n, std::usize::MAX);
            max_flow += tmp;

            if tmp == 0 { break; }
        }

        return max_flow;
    }
}
