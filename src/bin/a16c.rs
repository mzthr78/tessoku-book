use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut a: [usize; n-1],
        mut b: [usize; n-2],
    }

    for i in 0..=1 { a.insert(i, 0); }
    for i in 0..=2 { b.insert(i, 0); }

    let mut dp = vec![0; n+1];

    dp[2] = a[2];

    for i in 3..=n {
        dp[i] = min(a[i] + dp[i-1], b[i] + dp[i-2]);
    }

    println!("{}", dp[n]);
}
