use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
    }

    if k >= (n - 1) * 2 && k % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
