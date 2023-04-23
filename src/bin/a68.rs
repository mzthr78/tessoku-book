use proconio::{fastout, input};
//use proconio::marker::Usize1;
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
        mf.add_edge(a, b, c);
    }

    println!("{}", mf.max_flow(1, n));
}

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

struct MaxFlow {
    size: usize,
    used: Vec<bool>,
    g: Vec<Vec<Edge>>,
}

impl MaxFlow {
    fn new(n: usize) -> Self {
        Self {
            size: n,
            used: vec![false; n+1],
            g: vec![Vec::new(); n+1],
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let cur_ga = self.g[a].len();
        let cur_gb = self.g[b].len();
        self.g[a].push(Edge{to: b, cap: c, rev: cur_gb});
        self.g[b].push(Edge{to: a, cap: 0, rev: cur_ga});
    }

    fn dfs(&mut self, pos: usize, goal: usize, f: usize) -> usize {
        if pos == goal { return f; }
        self.used[pos] = true;

        for i in 0..self.g[pos].len() {
            if self.g[pos][i].cap == 0 { continue; }

            if self.used[self.g[pos][i].to] { continue; }

            let flow = self.dfs(
                self.g[pos][i].to,
                goal,
                min(f, self.g[pos][i].cap)
            );

            if flow > 0 {
                self.g[pos][i].cap -= flow;
                let to = self.g[pos][i].to;
                let rev = self.g[pos][i].rev;
                self.g[to][rev].cap += flow;
                return flow;
            }
        }
        return 0;
    }

    fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut total_flow = 0;
        loop {
            for i in 0..=self.size {
                self.used[i] = false;
            }

            let f = self.dfs(s, t, std::usize::MAX);

            if f == 0 { break; }

            total_flow += f;
        }
        return total_flow;
    }
}
