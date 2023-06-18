use proconio::{fastout, input};
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input!{
        n: usize,
        d: usize,
        //mut xy: [(usize, usize); n],
    }

    let mut xy: Vec<Vec<usize>> = vec![vec![]; d+1];

    for _ in 0..n {
        input!{
            x: usize,
            y: usize,
        }
        xy[x].push(y);
    }

    let mut bh = BinaryHeap::new();
    let mut ans = 0;

    for i in 1..=d {
        for y in &xy[i] {
            bh.push(y);
        }

        if !bh.is_empty() {
            ans += bh.pop().unwrap();
        }
    }

    println!("{}", ans);
}
