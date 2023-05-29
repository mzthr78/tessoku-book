use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    }

    let mut dp = vec![-1; n+1];
    dp[1] = 0;

    for i in 0..n-1 {
        if dp[i+1] >= 0 {
            dp[a[i]] = max(dp[a[i]], dp[i+1]+100);
            dp[b[i]] = max(dp[b[i]], dp[i+1]+150);
        }
    }

    println!("{}", dp[n]);
}
