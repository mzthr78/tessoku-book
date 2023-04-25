use proconio::{fastout, input, marker::Chars};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: [Chars; n],
    }

    let mut mf = MaxFlow::new(n*2+2);

    let start = 0;
    let goal = n * 2 + 1;

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                let from = i + 1;
                let to = n + j + 1;

                mf.e[from][to] = 1;
                mf.g[from].push(to);
                mf.g[to].push(from);

                mf.e[start][from] = 1;
                mf.g[start].push(from);
                mf.g[from].push(start);

                mf.e[to][goal] = 1;
                mf.g[to].push(goal);
                mf.g[goal].push(to);
            }
        }
    }

    let max_flow = mf.max_flow(start, goal);

    println!("{}", max_flow);
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

    fn max_flow(&mut self, s: usize, g: usize) -> usize {
        let mut max_flow = 0;

        loop {
            for i in 1..self.v.len() {
                self.v[i] = false;
            }

            let tmp = self.dfs(s, g, std::usize::MAX);
            max_flow += tmp;

            if tmp == 0 { break; }
        }

        return max_flow;
    }
}
