use proconio::input;
use proconio::fastout;
use std::cmp;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let mut x = vec![0; m];
    for i in 0..m {
        for j in 0..n {
            x[i] |= a[i][j] << j;
        }
    }

    let mut dp: Vec<i32> = vec![1<<10; 1<<n];

    for i in 0..m {
        dp[x[i]] = 1;

        for j in 0..(1<<n) {
            dp[j|x[i]] = cmp::min(dp[j|x[i]], dp[j]+1);
        }
    }

    if dp[(1<<n)-1] == (1<<10) { dp[(1<<n)-1] = -1; }
    println!("{}", dp[(1<<n)-1]);
}
