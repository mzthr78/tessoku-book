use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n+1];

    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut d: Vec<isize> = vec![-1; n+1];

    d[1] = 0;
    q.push_back(1);

    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        for i in 0..g[p].len() {
            if d[g[p][i]] == -1 {
                d[g[p][i]] = d[p] + 1;
                q.push_back(g[p][i]);
            }
        }
    }

    for i in 1..d.len() {
        println!("{}", d[i]);
    }
}
