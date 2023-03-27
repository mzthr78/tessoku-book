use proconio::{fastout, input};
use std::cmp;

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut ans = 0;
    for x in 1..=100 {
        for y in 1..=100 {
            let mut score = 0;

            for &(a, b) in &ab {
                if a >= x && a <= x+k && b >= y && b <= y+k {
                    score += 1;
                }
            }

            ans = cmp::max(ans, score);
        }
    }

    println!("{}", ans);
}
