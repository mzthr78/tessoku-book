use proconio::input;
use proconio::fastout;
use std::cmp;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![0; n];

    for i in 0..n {
        dp[i] = 1;
        for j in 0..i {
            if a[j] < a[i] { dp[i] = cmp::max(dp[i], dp[j]+1); }
        }
    }
    
    let mut max = 0;
    for i in 0..n {
        if dp[i] > max { max = dp[i]; }
    }
    println!("{}", max);
}
