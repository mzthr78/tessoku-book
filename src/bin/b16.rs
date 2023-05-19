use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut h: [isize; n],
    }
    h.insert(0, 0);

    let mut dp = vec![0; n+1];

    dp[2] = (h[2] - h[1]).abs();

    for i in 3..=n {
        let c2 = (h[i-2]-h[i]).abs() + dp[i-2];
        let c1 = (h[i-1]-h[i]).abs() + dp[i-1];
        dp[i] = min(c2, c1);
    }

    println!("{}", dp[n]);
}
