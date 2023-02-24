use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; n]; n];

    dp[0][n-1] = 0;
    for l in 0..n {
        for r in (l..n).rev() {
            //println!("({}, {})", l, r);

            let mut lscore = 0;
            let mut rscore = 0;

            if l > 0 && l <= pa[l-1].0-1 && r >= pa[l-1].0-1 {
                lscore = pa[l-1].1;
            }

            if r < n - 1 && l < pa[r+1].0 && r >= pa[r+1].0-1  {
                rscore = pa[r+1].1;    
            }

            if r == n - 1 {
                if l > 0 { dp[l][r] = dp[l-1][r]+lscore; }
            } else if l == 0 {
                dp[l][r] = dp[l][r+1]+rscore;
            } else {
                dp[l][r] = cmp::max(dp[l][r+1]+rscore, dp[l-1][r]+lscore);
            }
        }
    }

    /*
    for i in 0..n {
        println!("{:?}", dp[i]);
    }
    */

    let mut answer = 0;
    for i in 0..n {
        answer = cmp::max(answer, dp[i][i]);
    }
    println!("{}", answer);
}
