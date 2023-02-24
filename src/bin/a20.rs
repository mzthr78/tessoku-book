use proconio::input;
use proconio::fastout;
use proconio::marker::Chars;
use std::cmp;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i-1] == t[j-1] {
                dp[i][j] = cmp::max(cmp::max(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1]+1);
            } else {
                dp[i][j] = cmp::max(dp[i-1][j], dp[i][j-1]);
            }
        }
    }

    /*
    for i in 0..=s.len() {
        println!("{:?}", dp[i]);
    }
    */
    println!("{}", dp[s.len()][t.len()]);
}
