use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        mw: usize,
        wv: [(usize, usize); n],
    }

    let mut mv = 0;
    for (_w, v) in &wv {
        mv += v;
    }

    let mut dp = vec![vec![std::usize::MAX; mv+1]; n+1];
    dp[0][0] = 0;
    
    for i in 1..=n {
        let (w, v) = wv[i-1];

        for j in 0..=mv {
            if j < v || dp[i-1][j-v] == std::usize::MAX {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = min(dp[i-1][j], dp[i-1][j-v] + w);
            }
        }
    }

    let mut ans = 0;
    for i in 1..=mv {
        if dp[n][i] <= mw { ans = i; }
    }

    println!("{}", ans);
}
