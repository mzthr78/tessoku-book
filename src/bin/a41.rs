use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        _n: usize,
        s: String,
    }

    if s.contains("RRR") || s.contains("BBB") {
        println!("Yes");
    } else {
        println!("No");
    }
}
