use proconio::{fastout, input, marker::Chars};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: Chars,
    }

    let mut dp = vec![vec![0; n]; n];

    for i in 0..n {
        dp[i][i] = 1;

        if i < n-1 {
            if s[i] == s[i+1] {
                dp[i][i+1] = 2;
            } else {
                dp[i][i+1] = 1;
            }
        }
    }

    for len in 2..n {
        for l in 0..n-len {
            let r = l + len;

            dp[l][r] = max(dp[l][r-1], dp[l+1][r]);

            if s[l] == s[r] {
                dp[l][r] = max(dp[l][r], dp[l+1][r-1]+2);
            }
        }
    }

    println!("{}", dp[0][n-1]);
}
