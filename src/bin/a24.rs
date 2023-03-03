use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![];

    for i in 0..n {
        let pos = match dp.binary_search(&a[i]) {
            Ok(v) => v,
            Err(v) => v
        };

        if pos == dp.len() {
            dp.push(a[i]);
        } else {
            dp[pos] = a[i];
        }
    }
    println!("{}", dp.len());
}
