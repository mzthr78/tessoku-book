use proconio::{fastout, input, marker::Chars};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut s: Chars,
    }

    let mut dp = vec![vec![0; n+1]; n+1];

    for i in 1..=n {
        for j in 1..=n {
            dp[i][j] = max(dp[i-1][j], dp[i][j-1]);

            if s[i-1] == s[n-j] {
                dp[i][j] = max(dp[i][j], dp[i-1][j-1]+1);
            } else {
                dp[i][j] = max(dp[i][j], dp[i-1][j-1]);
            }
        }
    }

    println!("{}", dp[n][n]);
}
