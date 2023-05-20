use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        wv: [(usize, isize); n],
    }

    let mut dp = vec![vec![-1; m+1]; n+1];

    dp[0][0] = 0;

    for i in 1..=n {
        let (w, v) = wv[i-1];

        for j in 0..=m {
            if j < w || dp[i-1][j - w] < 0 {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = max(dp[i-1][j], dp[i-1][j-w] + v);
            }
        }
    }

    let mut ans = 0;
    for i in 0..=m {
        ans = max(ans, dp[n][i]);
    }

    println!("{}", ans);
}
