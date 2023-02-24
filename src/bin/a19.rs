use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, isize); n],
    }

    /*
    for i in 0..n {
        println!("wv[{}] w={} v={}", i, wv[i].0, wv[i].1);    
    }
    */

    let mut dp = vec![vec![-1; w+1]; n+1];

    dp[0][0] = 0;
    
    for i in 1..=n {
        for j in 0..=w {
            if j < wv[i-1].0 || dp[i-1][j-wv[i-1].0] < 0 { dp[i][j] = dp[i-1][j]; }
            else {
                dp[i][j] = cmp::max(dp[i-1][j], dp[i-1][j - wv[i-1].0] + wv[i-1].1);
            }
        }
    }

    /*
    for i in 0..=n {
        println!("{:?}", dp[i])
    }
    */

    let mut max = 0;
    for i in 1..=w {
        max = cmp::max(max, dp[n][i]);
    }

    println!("{}", max);
}
