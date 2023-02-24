use proconio::input;
use proconio::fastout;
use std::cmp;

#[fastout]
fn main() {
    todo();

    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    }

    let mut dp = vec![0; n];

    for i in 0..n-1 {
        dp[a[i]-1] = cmp::max(dp[a[i]-1], dp[i]+100);
        dp[b[i]-1] = cmp::max(dp[b[i]-1], dp[i]+150);
    }

    //println!("{:?}", dp);
    println!("{}", dp[n-1]);
}

