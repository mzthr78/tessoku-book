use proconio::{fastout, input};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        abc: [(usize, usize, isize); m],
    }

    let mut g = vec![vec![]; n+1];

    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    
    let mut heap = BinaryHeap::new();
    let mut v = vec![-1; n+1];
    
    heap.push(Reverse((0, 1)));

    while !heap.is_empty() {
        let (c, p) = heap.pop().unwrap().0;

        if v[p] == -1 {
            v[p] = c;
            for &(pos, cost) in &g[p] {
                heap.push(Reverse((c + cost, pos)));
            }
        }
    }

    for i in 1..v.len() {
        println!("{}", v[i]);
    }
}
