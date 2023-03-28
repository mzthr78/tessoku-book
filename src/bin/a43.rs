use proconio::{fastout, input};
use std::cmp;

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        ab: [(usize, char); n],
    }

    let mut ans = 0;
    for &(a, b) in &ab {
        if b == 'E' {
            ans = cmp::max(ans, k - a);
        } else {
            ans = cmp::max(ans, a);
        }
    }

    println!("{}", ans);
}
