use proconio::{fastout, input};
use std::cmp;

#[fastout]
fn main() {
    input!{
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    }

    let mut x = vec![24; d+1];

    for &(l, r, h) in &lrh {
        for i in l..=r {
            x[i] = cmp::min(x[i], h);
        }
    }

    let mut ans = 0;
    for i in 1..=d {
        ans += x[i];
    }

    println!("{}", ans);
}
