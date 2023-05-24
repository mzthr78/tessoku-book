use proconio::{fastout, input, marker::Chars};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: Chars,
    }

    let mut t = s.clone();
    t.reverse();
    
    let mut dp = vec![vec![0; n+1]; n+1];

    for i in 0..n {
        for j in 0..n {
            dp[i+1][j+1] = max(dp[i][j+1], dp[i+1][j]);

            if s[i] == t[j] {
                dp[i+1][j+1] = max(dp[i+1][j+1], dp[i][j]+1);
            } else {
                dp[i+1][j+1] = max(dp[i+1][j+1], dp[i][j]);
            }
        }
    }

    println!("{}", dp[n][n]);
}
