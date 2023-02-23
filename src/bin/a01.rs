use proconio::input;
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    println!("{}", n * n);
}
