use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        _n: usize,
        s: String,
    }

    let mut ans = "No";

    if s.contains("RRR") || s.contains("BBB") { ans = "Yes"; }

    println!("{}", ans);
}
