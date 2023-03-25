use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: i64,
        a: [usize; n],
    }

    let mut x : Vec<i64> = vec![0; 101];

    for &i in &a {
        x[i] += 1;
    }

    let mut ans : i64 = 0;
    for i in 1..=100 {
        ans += (x[i] * (x[i]-1) * (x[i]-2)) / (3 * 2 * 1);
    }

    println!("{}", ans);
}
