use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        a: isize,
        b: isize,
        c: isize,
    }

    println!("{}", if a + b + c == 0 { "Yes" } else { "No" });
}
