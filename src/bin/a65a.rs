use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n-1],
    }

    let mut g = vec![vec![]; n+1];

    for i in 0..n-1 {
        g[a[i]].push(i+2);
    }

    let mut dp = vec![0; n+1];

    for i in (1..=n).rev() {
        dp[i] = 0;
        for j in 0..g[i].len() {
            dp[i] += dp[g[i][j]]+1;
        }
    }

    println!("{}", dp[1..=n]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
    );
    /*
    for i in 1..=n {
        print!("{} ", dp[i]);
    }
    print!("\n");
    */
}
