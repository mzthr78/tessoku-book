use proconio::{fastout, input};
//use std::cmp::min;

#[fastout]
fn main() {
    input!{
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    }

    let mut m = vec![24; d+1];

    for (l, r, h) in lrh {
        for i in l..=r {
            m[i] = m[i].min(h);
        }
    }

    let mut ans = 0;

    for i in 1..=d {
        ans += m[i];
    }

    println!("{}", ans);
}
