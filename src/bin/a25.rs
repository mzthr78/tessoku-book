use proconio::input;
use proconio::fastout;
use proconio::marker::{Chars};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                dp[i][j] = 1;
            } else if c[i][j] == '#' {
                continue;
            } else {
                if i >= 1 { dp[i][j] += dp[i-1][j]; }
                if j >= 1 { dp[i][j] += dp[i][j-1]; }
            }
        }
    }
    
    /*
    for i in 0..h {
        println!("{:?}", dp[i]);
    }
    */

    println!("{}", dp[h-1][w-1]);
}
