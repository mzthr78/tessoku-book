use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
    }

    let mut dp = vec![false; n+1];

    for i in 0..=n {
        if i >= a && !dp[i-a] { dp[i] = true; }
        else if i >= b && !dp[i-b] { dp[i] = true; }
        else { dp[i] = false; }
    }

    if dp[n] { println!("First"); }
    else { println!("Second"); }
}
