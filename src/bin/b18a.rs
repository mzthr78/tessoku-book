use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        s: usize,
        mut a: [usize; n],
    }
    a.insert(0, 0);

    let mut dp = vec![vec![false; s+1]; n+1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if j < a[i] {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = dp[i-1][j] || dp[i-1][j-a[i]];
            }
        }
    }

    if !dp[n][s] { println!("-1"); return; }

    let mut c: Vec<usize> = Vec::new();
    let mut j = s;
    for i in (1..=n).rev() {
        if !dp[i-1][j] {
            c.push(i);
            j -= a[i];
        }
    }

    c.reverse();
    println!("{}", c.len());

    let mut d = "";
    for i in 0..c.len() {
        print!("{}{}", d, c[i]);
        d = " ";
    }
    print!("\n");
}
