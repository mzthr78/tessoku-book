use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut pa: [(usize, usize); n],
    }
    pa.insert(0, (0, 0));

    let mut dp = vec![vec![0; n+1]; n+1];
    
    for i in 1..=n {
        for j in (i..=n).rev() {
            let mut sl = 0;
            let mut sr = 0;

            if j < n {
                let cr = j + 1;
                sr = dp[i][cr];

                if i <= pa[cr].0 && pa[cr].0 <= j {
                    sr += pa[cr].1;
                }
            }

            if i > 1 {
                let cl = i - 1;
                sl = dp[cl][j];

                if i <= pa[cl].0 && pa[cl].0 <= j {
                    sl += pa[cl].1;
                }
            }

            dp[i][j] = max(sr, sl);
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = max(ans, dp[i][i]);
    }

    println!("{}", ans);
}
