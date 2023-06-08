use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let mut dp = vec![false; n+1];

    for i in 0..=n {
        for j in 0..k {
            if i >= a[j] && !dp[i-a[j]] {
                dp[i] = true;
                break;
            }
        }
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
