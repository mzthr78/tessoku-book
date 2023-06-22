use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = 0;

    for i in 1..=100 {
        for j in 1..=100 {
            let mut tmp = 0;

            for &(a, b) in &ab {
                if a >= i && a <= (i + k) && b >= j && b <= (j + k) {
                    tmp += 1;
                }
            }

            ans = max(ans, tmp);
        }
    }

    println!("{}", ans);
}
