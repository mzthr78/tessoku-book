use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
        a: [usize; n],
        xy: [(usize, usize); q],
    }

    let mut dp = vec![vec![0; n+1]; 30];

    for i in 1..=n {
        dp[0][i] = a[i-1];
    }

    for d in 1..=29 {
        for i in 1..=n {
            dp[d][i] = dp[d-1][dp[d-1][i]];
        }
    }

    for &(x, y) in &xy {
        let mut p = x;
        for d in (0..=29).rev() {
            if (y / (1 << d)) % 2 != 0 {
                p = dp[d][p];
            }
        }
        println!("{}", p);
    }
}
