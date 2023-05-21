use proconio::{fastout, input, marker::Chars};
use std::cmp::{min, max};

#[fastout]
fn main() {
    input!{
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();

    let mut dp = vec![vec![max(n, m); m+1]; n+1];
    dp[0] = (0..=m).collect();

    for i in 1..=n {
        dp[i][0] = dp[i-1][0] + 1;
        for j in 1..=m {
            dp[i][j] = min(dp[i-1][j]+1, dp[i][j-1]+1);

            if s[i-1] == t[j-1] {
                dp[i][j] = min(dp[i][j], dp[i-1][j-1]);
            } else {
                dp[i][j] = min(dp[i][j], dp[i-1][j-1]+1);
            }
        }
    }

    println!("{}", dp[n][m]);
}
