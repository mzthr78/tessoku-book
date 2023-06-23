use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        l: usize,
        ab: [(usize, char); n],
    }

    let mut m = 0;

    for (a, b) in ab {
        if b == 'E' {
            m = max(m, l - a);
        } else {
            m = max(m, a);
        }
    }

    println!("{}", m);
}
