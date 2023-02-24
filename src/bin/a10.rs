use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }

    let mut ml = vec![0; n];
    let mut mr = vec![0; n];

    ml[0] = a[0];
    for i in 1..n {
        ml[i] = cmp::max(a[i], ml[i-1]);
    }

    mr[n-1] = a[n-1];
    for i in (0..n-1).rev() {
        mr[i] = cmp::max(a[i], mr[i+1]);
    }

    for i in 0..d {
        println!("{}", cmp::max(ml[lr[i].0-2], mr[lr[i].1]));
    }
}
